use mut_cell::MutCell;
use pm::*;
use pm_common::loop_timing::LoopTimingManager;
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

/// TODO: returns boxed shared buffers for tcp streams to read or write into.
#[derive(StateTrait)]
pub struct SharedBufferPool {
    pool: Vec<MutCell<Vec<u8>>>,
}

#[derive(StateTrait)]
pub struct TcpListenerState {
    pub listener: TcpListener,
    pub connections_per_loop: usize,
    pub buffer_size: usize,
}

pub struct TcpListenerHandler {
    pub listener: State<TcpListenerState>,
    pub connections: State<TcpConnectionsState>,
    pub buffer_pool: State<SharedBufferPool>,
}

impl DoerTrait for TcpListenerHandler {
    fn new_state(state: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        let mut local_state = state.local.get()?;
        let listener = TcpListener::bind("127.0.0.1:7878").map_err(|_| PmError::AddNewState)?;
        listener
            .set_nonblocking(true)
            .map_err(|_| PmError::AddNewState)?;

        local_state.add_state(TcpListenerState {
            listener,
            connections_per_loop: 100,
            buffer_size: 4096,
        })?;

        local_state.add_state(TcpConnectionsState {
            connections: HashMap::new(),
        })?;

        local_state.add_state(SharedBufferPool { pool: Vec::new() })
    }

    fn new(pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        let local_state = pm.state.local.get()?;

        Ok(Box::new(Self {
            listener: local_state.get_state::<TcpListenerState>()?,
            connections: local_state.get_state::<TcpConnectionsState>()?,
            buffer_pool: local_state.get_state::<SharedBufferPool>()?,
        }))
    }

    fn update(&self) -> Result<(), PmError> {
        let listener = self.listener.get()?;
        let mut connections = self.connections.get()?;
        let mut buffer_pool = self.buffer_pool.get()?;

        for _ in 0..listener.connections_per_loop {
            if let Ok(stream) = listener.listener.accept() {
                connections.connections.insert(stream.1, BufferedTcpConnection {
                    stream: stream.0,
                    // TODO: if we cant get buffer pool, put this stream into some waiting list?
                    buffer: buffer_pool.get()?,
                    socket_addr: stream.1,
                });
            } else {
                break;
            }
        }

        Ok(())
    }
}

pub struct BufferedTcpConnection {
    pub stream: TcpStream,
    pub buffer: MutCell<Vec<u8>>,
    pub socket_addr: SocketAddr,
}

#[derive(StateTrait)]
pub struct TcpConnectionsState {
    pub connections: HashMap<SocketAddr, BufferedTcpConnection>,
}

#[derive(StateTrait)]
pub struct HttpRequests {
    pub requests: Vec<HttpRequest>,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub keep_alive: Option<Duration>,
}

pub struct TcpConnectionToHttpRequest {
    pub connections: State<TcpConnectionsState>,
    pub requests: State<HttpRequests>,
}

impl DoerTrait for TcpConnectionToHttpRequest {
    fn new_state(state: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        let mut local_state = state.local.get()?;

        local_state.add_state(HttpRequests {
            requests: Vec::new(),
        })
    }

    fn new(pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        let local_state = pm.state.local.get()?;

        Ok(Box::new(TcpConnectionToHttpRequest {
            requests: local_state.get_state::<HttpRequests>()?,
            connections: local_state.get_state::<TcpConnectionsState>()?,
        }))
    }

    fn update(&self) -> Result<(), PmError> {
        let mut requests = self.requests.get()?;
        let mut connections = self.connections.get()?;
        let connections = std::mem::replace(&mut connections.connections, Vec::new());

        for (socket_addr, connection) in connections.iter_mut() {
            let buf_reader = BufReader::with_buffer(inner)
            let http_request: Vec<_> = connection
                .stream
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            requests.requests.push(http_request);
        }

        Ok(())
    }
}

pub struct ResponseSender {
    requests: State<HttpRequests>,
}

fn main() -> Result<(), PmError> {
    let mut pm = pm!(
        LoopTimingManager,
        TcpListenerHandler,
        TcpConnectionToHttpRequest
    );
    pm.run()
}
