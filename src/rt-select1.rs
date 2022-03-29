use tokio::{
    self,
    time::{self, Duration},
};

async fn sleep(n: u64) -> u64 {
    time::sleep(Duration::from_secs(n)).await;
    n
}

#[tokio::main]
async fn main() {
    tokio::select! {
        v = sleep(5) => println!("sleep 5 secs, branch 1 done: {}", v),
        v = sleep(3) => println!("sleep 3 secs, branch 2 done: {}", v),
    };

    println!("select! done");
}
