use websocket::ClientBuilder;

fn main() {
    let mut client = ClientBuilder::new("ws://localhost:1234").unwrap().connect_insecure().unwrap();
    println!("{:?}", client.recv_message().unwrap());
}