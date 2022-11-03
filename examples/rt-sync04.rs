use chrono::Local;
use tokio::{
    self, sync,
    time::{self, Duration},
};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = sync::mpsc::channel::<i32>(5);

    for i in 1..=7 {
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tx.send(i).await {
                println!("error: \"{}\"", e);
                return;
            }
            println!("sended: {}, {}", i, now());
        });
    }

    drop(tx);

    time::sleep(Duration::from_secs(1)).await;
    while let Some(i) = rx.recv().await {
        println!("received: {}", i);
    }
}
