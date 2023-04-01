use base::log;
use mini_redis::client;
use std::time::Instant;

#[tokio::main]
async fn main() {
    log::init();

    let (tx, mut rx) = watch::channel(12);

    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    client.set("hello", "world".into()).await.unwrap();
   
    tx.send(8).unwrap();

    log::info!("got value: {:?}", *rx.borrow());
}
