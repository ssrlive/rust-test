use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    let local_tasks = tokio::task::LocalSet::new();

    local_tasks.spawn_local(async {
        println!("local task1");
        time::sleep(time::Duration::from_secs(2)).await;
        println!("local task1 done {}", now());
    });

    // task2 要睡眠 10 秒，它将被第一次 local_tasks.block_on 在 3 秒后中断
    local_tasks.spawn_local(async {
        println!("local task2");
        time::sleep(time::Duration::from_secs(10)).await;
        println!("local task2 done, {}", now());
    });

    let task3 = async {
        tokio::task::spawn_local(async {
            println!("local task3");
            time::sleep(time::Duration::from_secs(3)).await;
            println!("local task3 done: {}", now());
        })
        .await
        .unwrap();
    };

    println!("before local tasks running: {}", now());
    local_tasks.block_on(&rt, task3);

    // 线程阻塞 15 秒，此时 task2 睡眠 10 秒的时间已经过去了，
    // 当再次进入 LocalSet 时，task2 将可以直接被唤醒
    std::thread::sleep(std::time::Duration::from_secs(15));

    // 再次进入 LocalSet
    local_tasks.block_on(&rt, async {
        // 先执行该任务，当遇到睡眠 1 秒的任务时，将出现任务切换，
        // 此时，调度器将调度 task2，而此时 task2 已经睡眠完成
        println!("re enter localset context: {}", now());

        // 這個能引起調度切換的語句必不可少，否則 task2 不會被執行。
        tokio::task::yield_now().await;

        println!("re enter localset context done: {}", now());
    });
    println!("all local tasks done: {}", now());
}
