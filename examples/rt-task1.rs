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
        time::sleep(time::Duration::from_secs(5)).await;
        println!("local task1 done");
    });
    local_tasks.spawn_local(async {
        println!("local task2");
        time::sleep(time::Duration::from_secs(3)).await;
        println!("local task2 done");
    });
    println!("before local tasks running: {}", now());
    rt.block_on(async {
        local_tasks.await;
    });
}
