use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, task, time};

fn _now() -> String {
    Local::now().format("%F %T").to_string()
}

fn _main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    let _h = task::spawn(async {
        time::sleep(time::Duration::from_secs(3)).await;
        println!("task over: {}", _now());
    });
    println!("task over: {}", _now());
    thread::sleep(time::Duration::from_secs(4));
}

use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
