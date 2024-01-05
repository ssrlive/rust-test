use chrono::Local;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, DuplexStream};
use tokio::{self, runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn write_duplex(r: &mut DuplexStream) -> io::Result<usize> {
    r.write(now().as_bytes()).await
}

#[allow(unused_mut)]
async fn read_duplex(mut r: DuplexStream) {
    // let (mut r, w) = tokio::io::split(r);
    // drop(w);
    let mut buf = [0u8; 1024];
    loop {
        match r.read(&mut buf).await {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if let Ok(data) = std::str::from_utf8(&buf[..n]) {
                    println!("read from duplex: {}", data);
                }
            }
        };
    }
}

fn main() {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let (client, mut server) = tokio::io::duplex(64);

        // client read data from server
        tokio::spawn(async move {
            read_duplex(client).await;
        });

        // server write now() to client
        loop {
            match write_duplex(&mut server).await {
                Err(_) | Ok(0) => break,
                _ => (),
            }
            time::sleep(time::Duration::from_secs(1)).await;
        }
    });
}
