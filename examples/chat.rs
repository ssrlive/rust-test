use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:8080").await?;
    let (tx, _rx) = broadcast::channel(10);
    loop {
        let (mut stream, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result? == 0 {
                            break;
                        }
                        tx.send((addr, line.clone()))?;
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (other_addr, msg) = result?;
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await?;
                        }
                    }
                }
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}
