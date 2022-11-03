use tokio::{self, runtime::Runtime, sync};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, rx) = sync::oneshot::channel::<i32>();

        // drop(tx);
        // drop(rx);
        tokio::spawn(async move {
            if let Err(e) = tx.send(33) {
                println!("receiver dropped first: {:?}", e);
            }
        });

        match rx.await {
            Ok(value) => println!("received: {:?}", value),
            Err(e) => println!("sender dropped first: {:?}", e),
        };
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    });
}
