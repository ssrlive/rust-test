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

    rt.block_on(async {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        let (tx, mut rx) = sync::oneshot::channel::<i32>();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            if let Err(e) = tx.send(33) {
                println!("some error \"{}\"", e);
            }
        });
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    println!("tick");
                }
                msg = &mut rx => {
                    println!("got message: {:?}", msg.unwrap());
                    break;
                }
            }
        }
    });
}
