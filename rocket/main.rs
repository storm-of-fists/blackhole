use base::log;
use raw_pointer::create_raw_ptr;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use context::{ContextPtr, init_context};
use std::future::Future;
use std::pin::Pin;

pub struct StateInit {
    ctx: ContextPtr,
    timeout: Duration,
}

impl StateInit {
    pub fn new(ctx: ContextPtr) -> Self {
        Self {
            ctx,
            timeout: Duration::from_secs(50),
        }
    }

    pub async fn sequence() {
        log::info!("running init sequence");
        sleep(Duration::from_millis(1500)).await;
        log::info!("running another part of init sequence.");
        sleep(Duration::from_millis(1500)).await;
        log::info!("done running init sequence.");
    }

    pub fn controller() {
        log::info!("running control loop");
    }
}

pub struct Rocket {
    ctx: ContextPtr,
    name: String,
    control_period: u64,
    // start_time: Duration,
    start_instant: Instant,
}

impl Rocket {
    pub fn new(ctx: ContextPtr) -> Self {
        let start_instant = Instant::now();
        return Self {
            ctx,
            name: "Rocket".to_string(),
            control_period: 500,
            start_instant
        }
    }
}

#[tokio::main]
async fn main() {
    let mut ctx = unsafe { init_context!() };

    let mut rocket = Rocket::new(ctx);
    let mut rkt = unsafe { create_raw_ptr!(rocket) };

    let state = StateInit::new();

    let fut = state::sequence();

    let pin_fut = Pin::new(&mut fut);

    loop {
        state::controller();
        pin_fut.poll();
    }
    // select! {
    //     val = seq_fut => {}
    //     // val = ctrl_fut => {}
    // }

    // rocket.shutdown_sequence().await;
}