use base::log;
use raw_pointer::create_raw_ptr;
use clock::Clock;
use std::time::Duration;
use context::{ContextPtr, init_context};
use std::thread::sleep;

pub enum RocketState {
    Init,
    Running,
    Shutdown,
}

// USE THIS AS A SINGLETON AND PASS TO CONTROL STATE MACHINE
//  REP THE ROCKET AS DATA ESSENTIALLY, AND CREATE SHARED ACCESS
// VIA UNSAFE RAW POINTER
pub struct Rocket {
    ctx: ContextPtr,

    clock: Clock,
    control_period: Duration,
    control_time: Duration,


    channels: HashMap<u8, SomeChannelStruct>,
}

// impl Rocket {
//     pub fn new(ctx: ContextPtr) -> Self {
//         return Self {
//             ctx,
//             clock: Clock::new(),
//             control_period: Duration::from_millis(100),
//             control_time: Duration::ZERO,
//             state: Some(RocketState::Init),
//         }
//     }

//     fn init(&mut self) {
//         log::info!("running init!");
//         if self.clock.duration_since_start() > Duration::from_secs(1) {
//             self.transition(RocketState::Running);
//         }
//     }

//     fn running(&mut self) {
//         log::info!("running running");
//         if self.clock.duration_since_start() > Duration::from_secs(2) {
//             self.transition(RocketState::Shutdown);
//         }
//     }

//     fn shutdown(&mut self) {
//         log::info!("running shutdown");
//         self.exit();
//     }

//     pub fn run_control(&mut self) {
//         loop {
//             match self.state {
//                 Some(RocketState::Init) => self.init(),
//                 Some(RocketState::Running) => self.running(),
//                 Some(RocketState::Shutdown) => self.shutdown(),
//                 None => return,
//             };

//             self.sleep_rest_of_period();
//         }
//     }

//     fn transition(&mut self, state: RocketState) {
//         self.state = Some(state);
//     }

//     fn exit(&mut self) {
//         self.state = None;
//     }

//     fn advance_time(&mut self) {
//         self.control_time += self.control_period;
//     }

//     fn sleep_rest_of_period(&mut self) {
//         // TODO: what if we run over the cycle time?
//         let cycle_duration = self.clock.duration_since_start() - self.control_time;

//         let sleep_duration = match self.control_period.checked_sub(cycle_duration) {
//             Some(duration) => duration,
//             None => {
//                 log::debug!("Cycle time ran over control period by {:?}", cycle_duration - self.control_period);
//                 Duration::ZERO
//             },
//         };

//         if sleep_duration > Duration::ZERO {
//             sleep(sleep_duration);
//         }
        
//         self.advance_time();
//     }
// }

fn main() {
    let mut ctx = unsafe { init_context!() };

    let mut rocket = Rocket::new(ctx);
    let mut rkt = unsafe { create_raw_ptr!(rocket) };

    
    // select! {
    //     val = seq_fut => {}
    //     // val = ctrl_fut => {}
    // }

    // rocket.shutdown_sequence().await;
}