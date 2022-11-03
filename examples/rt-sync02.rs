use tokio::{self, sync};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = sync::mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        for i in 1..=20 {
            if let Err(e) = tx.send(i).await {
                println!("error occurs \"{}\"", e);
                break;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("received: \"{}\"", i);
    }
}
