use base::log;
use tokio::{join, time::{interval, sleep, timeout}};
use std::time::Duration;

struct InitState {
    pub timeout: Duration,
    pub controller_interval: Duration,
}

impl InitState {
    pub fn new() -> Self {
        Self {
            timeout: Duration::MAX,
            controller_interval: Duration::from_millis(1000),
        }
    }

    pub async fn controller(&self) {
        let mut controller_interval = interval(self.controller_interval);

        loop {
            controller_interval.tick().await;
            log::info!("running init controller");
        }
    }

    pub async fn sequence(&self) {
        log::info!("running init sequence");
        sleep(Duration::from_secs(20)).await;
        log::info!("more init sequence");
        sleep(Duration::from_secs(20)).await;
        log::info!("done with init sequence, return");
    }

    pub async fn run(&mut self) {
        tokio::select! {
            sequence_result = timeout(self.timeout, self.sequence()) => {
                if let Err(_) = sequence_result {
                    log::debug!("Timed out of init sequence after timeout of {:?}", self.timeout)
                }
            }
            _controller_result = self.controller() => {}
        }
    }
}

enum States {
    InitState,
}

struct StateMachine {
    current_state: States,
    init_state: InitState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_state: States::InitState,
            init_state: InitState::new(),
        }
    }

    pub async fn run(&mut self) {
        match self.current_state {
            States::InitState => {
                self.init_state.run().await
            }
        }
    }
}

#[tokio::main]
async fn main() {
    log::default().init();

    let mut state_machine = StateMachine::new();
    let mut state_machine_2 = StateMachine::new();

    join!(
        state_machine.run(),
        state_machine_2.run()
    );
}