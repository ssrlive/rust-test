use tokio::{self, sync};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = sync::mpsc::channel(10);

    tokio::spawn(async move {
        for i in 1..=20 {
            // if let Err(_) = tx.send(i).await {}
            if tx.send(i).await.is_err() {
                println!("receiver closed");
                return;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("received: {}", i);
    }
}
