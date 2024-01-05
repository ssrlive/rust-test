use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut client, mut server) = tokio::io::duplex(64);

    client.write_all(b"ping").await?;

    let mut buf = [0u8; 4];
    server.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"ping");

    server.write_all(b"pong").await?;

    client.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"pong");

    Ok(())
}
