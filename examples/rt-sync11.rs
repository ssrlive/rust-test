use std::sync::{Arc, Mutex};
use tokio::{
    self,
    runtime::Runtime,
    time::{self, Duration},
};

async fn add_1(mutex: &Mutex<u64>) -> u64 {
    let mut lock = mutex.lock().unwrap();
    *lock += 1;
    *lock
} // 申请的互斥锁在此被释放

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mutex = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _i in 0..100 {
            let lock = mutex.clone();
            let h = tokio::spawn(async move {
                let n = add_1(&lock).await;
                time::sleep(Duration::from_millis(n)).await;
            });
            handles.push(h);
        }

        for h in handles {
            h.await.expect("msg");
        }

        println!("data: {}", mutex.lock().unwrap());
    });
}
