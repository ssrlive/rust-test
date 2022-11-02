use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    sync::mpsc,
};

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    while let Ok((client_stream, client_addr)) = server.accept().await {
        println!("accept client: {}", client_addr);
        // 每接入一个客户端的连接请求，都分配一个子任务，
        // 如果客户端的并发数量不大，为每个客户端都分配一个thread，
        // 然后在thread中创建tokio runtime，处理起来会更方便
        tokio::spawn(async move {
            process_client(client_stream).await;
        });
    }
}

async fn process_client(client_stream: TcpStream) {
    let (client_reader, client_writer) = client_stream.into_split();
    let (msg_tx, msg_rx) = mpsc::channel::<String>(100);

    // 无论是读任务还是写任务的终止，另一个任务都将没有继续存在的意义，因此都将另一个任务也终止
    if let Err(e) = tokio::try_join!(
        read_from_client(client_reader, msg_tx),
        write_to_client(client_writer, msg_rx)
    ) {
        eprintln!("tunnel terminated with error: \"{}\"", e);
    }
    println!("client disconnected");
}

/// 从客户端读取
async fn read_from_client(
    reader: OwnedReadHalf,
    msg_tx: mpsc::Sender<String>,
) -> anyhow::Result<()> {
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut buf = String::new();
    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(e) => {
                eprintln!("read from client error \"{}\"", e);
                break Err(anyhow::anyhow!(e));
            }
            // 遇到了EOF
            Ok(0) => {
                println!("client closed");
                break Ok(());
            }
            Ok(n) => {
                // read_line()读取时会包含换行符，因此去除行尾换行符
                // 将buf.drain(。。)会将buf清空，下一次read_line读取的内容将从头填充而不是追加
                buf.pop();
                let content = buf.drain(..).as_str().to_string();
                println!("read {} bytes from client. content: {}", n, content);
                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if let Err(e) = msg_tx.send(content).await {
                    eprintln!("receiver closed \"{}\"", e);
                    break Err(anyhow::anyhow!(e));
                }
            }
        }
    }
}

/// 写给客户端
async fn write_to_client(
    writer: OwnedWriteHalf,
    mut msg_rx: mpsc::Receiver<String>,
) -> anyhow::Result<()> {
    let mut buf_writer = tokio::io::BufWriter::new(writer);
    loop {
        match msg_rx.recv().await {
            None => {
                eprintln!("sender closed");
                break Err(anyhow::anyhow!("sender closed"));
            }
            Some(mut str) => {
                str.push('\n');
                if let Err(e) = buf_writer.write_all(str.as_bytes()).await {
                    eprintln!("write to client failed: {}", e);
                    break Err(anyhow::anyhow!(e));
                }
            }
        }
    }
}
