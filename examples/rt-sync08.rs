use tokio::sync::watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建 watch 通道时，需指定一个初始值存放在通道中
    let (tx, mut rx) = watch::channel("hello");

    // Recevier 端，通过 changed() 来等待通道的数据发生变化
    // 通过 borrow() 引用通道中的数据
    let h = tokio::spawn(async move {
        if rx.changed().await.is_ok() {
            println!("received = {:?}", *rx.borrow());
        }
    });

    // 向通道中发送数据，实际上是修改通道中的那个数据
    tx.send("world")?;

    h.await?;
    Ok(())
}
