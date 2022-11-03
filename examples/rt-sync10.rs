use std::sync::Arc;
// use std::sync::Mutex; // 會導致編譯錯誤，因此棄用。

use tokio::{
    self,
    sync::Mutex,
    time::{self, Duration},
};

async fn add_1(mutex: &Mutex<u64>) {
    // let mut lock = mutex.lock().unwrap(); // 會導致編譯錯誤，因此棄用。
    let mut lock = mutex.lock().await;
    *lock += 1;

    // 子任务，跨 await，且引用了父任务中的数据
    time::sleep(Duration::from_millis(*lock)).await;
    println!("lock = {}", lock);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _i in 0..10 {
        let lock = mutex.clone();
        let h = tokio::spawn(async move {
            add_1(&lock).await;
        });
        handles.push(h);
    }

    for h in handles {
        h.await?;
    }
    Ok(())
}
