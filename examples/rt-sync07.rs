use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // 通道容量2
    let (tx, mut rx) = broadcast::channel(2);

    // 写入3个数据，将出现接收端落后于发送端的情况，
    // 此时，第一个数据(10)将被剔除，剔除后，20将位于队列的头部
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    tx.send(30).unwrap();

    // 落后于发送端之后的第一次recv()操作，返回RecvError::Lagged错误
    assert!(rx.recv().await.is_err());

    // 之后可正常获取通道中的数据
    assert_eq!(20, rx.recv().await.unwrap());
    assert_eq!(30, rx.recv().await.unwrap());

    println!("done");
}
