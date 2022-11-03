use tokio::{self, sync};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = sync::mpsc::channel::<i32>(10);

    for i in 1..=10 {
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tx.send(i).await {
                println!("error occurs \"{}\"", e);
            }
        });
    }
    drop(tx);

    while let Some(i) = rx.recv().await {
        println!("received: {}", i);
    }
}
