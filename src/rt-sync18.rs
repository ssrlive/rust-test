use chrono::Local;
use std::sync::Arc;
use tokio::{
    self,
    sync::Semaphore,
    time::{self, Duration},
};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    // 只有 3 个信号灯的信号量
    let semaphore = Arc::new(Semaphore::new(3));

    let mut handles = Vec::new();

    // 5 个并发任务，每个任务执行前都先获取信号灯
    // 因此，同一时刻最多只有3个任务进行并发
    for i in 1..=5 {
        let semaphore = semaphore.clone();
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("{}, {}", i, now());
            time::sleep(Duration::from_secs(1)).await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
}
