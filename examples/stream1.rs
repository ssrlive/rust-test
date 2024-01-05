use tokio_stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(vec![0, 1, 2]);

    while let Some(value) = stream.next().await {
        println!("Got {}", value);
    }
}
