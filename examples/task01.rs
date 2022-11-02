use chrono::Local;
use tokio::{self, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn async_task() {
    println!("create an aync task: {}", now());
    let h = tokio::spawn(async {
        time::sleep(time::Duration::from_secs(3)).await;
        println!("async task over: {}", now());
    });
    h.await.unwrap();
}

#[tokio::main]
async fn main() {
    async_task().await;

    let task = tokio::spawn(async {
        time::sleep(time::Duration::from_secs(30)).await;
    });
    time::sleep(time::Duration::from_millis(1)).await;
    task.abort(); // task cancaled
    let abort_err = task.await.unwrap_err();
    println!("abort_err: {}", abort_err);

    async fn do_one(id: i32) -> anyhow::Result<()> {
        println!("one {id}: {}", now());
        time::sleep(time::Duration::from_secs(3)).await;
        println!("one {id} done: {}", now());
        Ok(())
    }
    async fn do_two(id: i32) -> anyhow::Result<()> {
        println!("two {id}: {}", now());
        time::sleep(time::Duration::from_secs(1)).await;
        if rand::random::<i32>() % 2 == 0 {
            println!("two {id} done: {}", now());
            Ok(())
        } else {
            // random error
            let err = format!("two {id} ===failed===: {}", now());
            println!("{err}");
            Err(anyhow::anyhow!(err))
        }
    }
    // join! 必须等待所有任务完成, 即使其中有一个任务失败也會等待所有任务完成
    let _ = tokio::join!(do_one(1), do_two(2));

    // try_join! 要么等待所有异步任务正常完成，要么等待第一个返回 Result Err 的任务出现
    if tokio::try_join!(do_one(3), do_two(4)).is_err() {
        println!("one of the tasks failed");
    }

    // LocalSet 让异步任务被放在一个独立的本地任务队列中，它们不会跨线程执行
    let local_task = tokio::task::LocalSet::new();
    local_task.spawn_local(async {
        println!("local task 1: {}", now());
        time::sleep(time::Duration::from_secs(3)).await;
        println!("local task 1 done: {}", now());
    });
    local_task.spawn_local(async {
        println!("local task 2: {}", now());
        time::sleep(time::Duration::from_secs(5)).await;
        println!("local task 2 done: {}", now());
    });
    local_task.await;
}

