use tokio::runtime::Runtime;
use tokio;

async fn function0() {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("==== function0 ====");
}

async fn function1() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("==== function1 ====");
}

async fn function2() {
    println!("==== function2 ====");
}

// #[tokio::main]
// async fn main() {
async fn async_main() {
    let f0 = function0();
    let f1 = function1();
    let f2 = function2();

    // 使用 await 则会顺序执行，使用 join 则会并发执行 f0, f1 和 f2
    // f0.await;
    // f1.await;
    // f2.await;

    tokio::join!(f0, f1, f2);
    // futures::join!(f0, f1, f2);
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());
}

// #[tokio::main]
// async fn main() {
//     tokio::join!(function0(), function1(), function2());
// }
