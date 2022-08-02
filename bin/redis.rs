extern crate redis;
use redis::Commands;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;

    let v = conn.get::<_, i32>("my_key");
    if let Ok(v) = v {
        println!("my_key: {:?}", v);
    } else {
        println!("my_key: not found, reason: {:?}", v.unwrap_err());
    }

    conn.set/*::<_, i32, _>*/("my_key", 42)?;
    conn.expire("my_key", 10)?;

    // let result : i32 = conn.get("my_key")?;
    let result = conn.get::<_, i32>("my_key")?;
    println!("{:#?}", result);

    Ok(())
}
