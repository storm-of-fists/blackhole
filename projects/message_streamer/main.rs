/// Message Streamer
/// Use protobuf and hashmaps to connect and stream data to websockets
/// or ports.
///
/// https://crates.io/crates/fnv/1.0.2 use for messages in hashmaps
/// and their integer token ids. or https://github.com/tkaitchuck/aHash ??
///
///
/// this is a cool website https://cpp.libhunt.com/
///

// pub struct MessageToken {}

// pub struct Connection {
//     /// How long until a connection and its data are considered "dead".
//     keep_alive_duration: Duration,
//     /// Should we be able to do tcp OR udp? probs not.
//     websocket_connection: WebsocketConnection,
// }

// pub struct MessageStore<T: ProtobufMessage> {
//     message_cache: HashMap<MessageToken, T>,
//     serialized_message_log: Vec<u8>,
//     serialized_message_indices: Vec<usize>,
// }

// pub struct Server

use std::{
    ops::Deref,
    collections::HashMap,
    sync::{Arc, Mutex},
    net::ToSocketAddrs,
}
;

use websocket::{
    stream::sync::TcpStream,
    sync::Server,
};

use prost;

pub struct Connection {
    tcp_stream: TcpStream,
    client_requests: Vec<ClientMessages>,
}

pub struct MessageHub<T: prost::Message> {
    name: String,
    message_cache: HashMap<usize, T>,
    serialized_message_log: Vec<u8>,
    /// this might need to be hashmap since people will likely use
    /// things that filter out messages, creating sparse entry into
    /// the Vec.
    serialized_message_indices: HashMap<usize, usize>,
    estimated_message_size: usize,
    /// If this message store will use a separate process for serializing
    /// and managing connections.
    use_message_repeater: bool,
    /// Restrict this to just a couple hundred.
    max_direct_connections: u8,
    /// The path for the log to be stored to disk. Maybe combine into
    /// a "DiskStorageSettings" thing.
    disk_log_path: Option<String>,
    /// 
    disk_log_duration: Duration,
}

impl<T> MessageHub<T> where T: prost::Message {
    pub fn new(name: impl Into<String>, _bind_to: impl ToSocketAddrs) -> Self {
        Self {
            name: name.into(),
            message_cache: HashMap::new(),
            serialized_message_log: Vec::new(),
            serialized_message_indices: HashMap::new(),
            estimated_message_size: 100,
            use_message_repeater: false,
            max_direct_connections: 1,
            disk_log_path: None,
            disk_log_duration: Duration::from_secs(1),
        }
    }

    pub fn new_as_handle(name: impl Into<String>, _bind_to: impl ToSocketAddrs) -> MessageHubHandle<T> {
        MessageHubHandle::new(Self::new(name, _bind_to))
    }

    // pub fn write(&mut self, token_num: usize, message: T) {
    //     let index = self.serialized_message_log.len() - 1;
    //     self.serialized_message_indices.insert(token_num, index);
    //     // TODO: check this.
    //     message.encode_length_delimited(self.serialized_message_log)?;
    // }

    // /// feels ugly
    // pub fn set_estimated_message_size(&mut self, count: usize) -> &mut Self {
    //     self.estimated_message_size = count;
    //     self
    // }

    // /// feels ugly
    // pub fn try_reserve(&mut self, count: usize) -> Result<&mut Self, MessageStoreError> {
    //     if self.message_cache.try_reserve(count).is_err()
    //     || self.serialized_message_log.try_reserve_exact(count * self.estimated_message_size).is_err()
    //     || self.serialized_message_indices.try_reserve_exact(count).is_err() {
    //         Err(MessageStoreError::UnableToReserve)
    //     } else {
    //         Ok(self)
    //     }
    // }


}

#[derive(Clone)]
pub struct MessageHubHandle<T: prost::Message> {
    hub: Arc<Mutex<MessageHub<T>>>,
}

impl<T> Deref for MessageHubHandle<T> where T: prost::Message {
    type Target = Arc<Mutex<MessageHub<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.hub
    }
}

impl<T> MessageHubHandle<T> where T: prost::Message {
    pub fn new(hub: MessageHub<T>) -> Self {
        Self {
            hub: Arc::new(Mutex::new(hub)),
        }
    }
}

pub enum MessageStoreError {
    UnableToReserve,
}

fn main() {
    let mut server = Server::bind("127.0.0.1:1234").unwrap();

    server.accept().unwrap().accept().unwrap().send_message(&websocket::Message::text("piss off"));
}
