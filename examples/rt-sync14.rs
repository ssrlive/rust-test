use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());

    let notify2 = notify.clone();
    let h = tokio::spawn(async move {
        notify2.notified().await;
        println!("received notification");
    });

    println!("sending notification");
    notify.notify_one();

    h.await.expect("msg");
}
