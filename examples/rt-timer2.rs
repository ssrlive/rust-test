use chrono::Local;
use tokio::{self, runtime::Runtime, time};

#[allow(dead_code)]
fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("start: {}", now());
        let slp = time::sleep(time::Duration::from_secs(1));
        tokio::pin!(slp);

        // 注意调用 slp.as_mut().await，而不是 slp.await，后者会 move 消费掉 slp.
        slp.as_mut().await;
        println!("end 1: {}", now());

        slp.as_mut()
            .reset(time::Instant::now() + time::Duration::from_secs(2));

        slp.await;
        println!("end 2: {}", now());
    });
}
