use tokio::{self, runtime::Runtime, time};

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let task = tokio::task::spawn(async {
            time::sleep(time::Duration::from_secs(10)).await;
        });

        // 让上面的异步任务跑起来
        time::sleep(time::Duration::from_millis(1)).await;
        task.abort(); // 取消任务, 之后，可以取得 JoinError
        let abort_err = task.await.unwrap_err();
        println!("{}", abort_err.is_cancelled());
    })
}
