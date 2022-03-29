use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, task, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    let _h = task::spawn(async {
        time::sleep(time::Duration::from_secs(3)).await;
        println!("task over: {}", now());
    });
    println!("task over: {}", now());
    thread::sleep(time::Duration::from_secs(4));
}
