use std::sync::Arc;
use tokio::{
    self,
    sync,
    // time::{self, Duration},
};

#[tokio::main]
async fn main() {
    let mutex = Arc::new(sync::Mutex::new(0));

    let mut handles = Vec::new();

    for i in 0..10 {
        let lock = Arc::clone(&mutex);
        let handle = tokio::spawn(async move {
            let mut data = lock.lock().await;
            *data += 1;
            println!("task: {}, data: {}", i, data);
        });
        handles.push(handle);
    }

    //time::sleep(Duration::from_secs(1)).await;
    for handle in handles {
        handle.await.expect("something went wrong");
    }
}
