use tokio::{
    self,
    sync::RwLock,
    time::{self, Duration},
};

#[tokio::main]
async fn main() {
    let lock = std::sync::Arc::new(RwLock::new(0));

    let lock1 = lock.clone();
    let h = tokio::spawn(async move {
        let _n = lock1.read().await;
        // drop(_n); // 鎖必須儘快釋放，否則造成死鎖
        time::sleep(Duration::from_secs(2)).await;
        let _nn = lock1.read().await;
        // drop(_nn); // 鎖必須儘快釋放，否則造成死鎖
    });

    time::sleep(Duration::from_secs(1)).await;
    let mut wn = lock.write().await;
    *wn = 2;
    // drop(wn); // 鎖必須儘快釋放，否則造成死鎖

    h.await.expect("msg");
}
