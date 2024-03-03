use base::log;
use tokio::{join, time::{interval, sleep, timeout}};
use std::time::{Instant, Duration};

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

    pub async fn controller(&self) -> State {
        let mut controller_interval = interval(self.controller_interval);

        loop {
            let tick = controller_interval.tick().await;
            log::info!("running init controller for {:?}", tick.elapsed());
            if false {
                return State::Idle;
            }
        }
    }

    pub async fn sequence(&self) -> State {
        log::info!("running init sequence");
        sleep(Duration::from_secs(5)).await;
        log::info!("more init sequence");
        sleep(Duration::from_secs(5)).await;
        log::info!("done with init sequence, return");
        return State::Idle;
    }

    pub async fn run(&mut self) -> State {
        tokio::select! {
            sequence_result = timeout(self.timeout, self.sequence()) => {
                match sequence_result {
                    Ok(sequence_state) => {
                        return sequence_state;
                    }
                    Err(_) => {
                        log::debug!("Timed out of init sequence after timeout of {:?}", self.timeout);
                        return State::Idle;
                    }
                }
            }
            controller_state = self.controller() => {
                return controller_state;
            }
        }
    }
}

struct IdleState {
    pub timeout: Duration,
    pub controller_interval: Duration,
}

impl IdleState {
    pub fn new() -> Self {
        Self {
            timeout: Duration::MAX,
            controller_interval: Duration::from_millis(1000),
        }
    }

    pub async fn controller(&self) -> State {
        let mut controller_interval = interval(self.controller_interval);
        let controller_start = Instant::now();

        loop {
            controller_interval.tick().await;
            log::info!("running idle controller for {:?}", controller_start.elapsed());
            if controller_start.elapsed() > Duration::from_secs(10) {
                return State::Exit;
            }
        }
    }

    pub async fn sequence(&self) -> State {
        sleep(Duration::MAX).await;
        return State::Exit;
    }

    pub async fn run(&mut self) -> State {
        tokio::select! {
            sequence_result = timeout(self.timeout, self.sequence()) => {
                match sequence_result {
                    Ok(sequence_state) => {
                        return sequence_state;
                    }
                    Err(_) => {
                        log::debug!("Timed out of init sequence after timeout of {:?}", self.timeout);
                        return State::Exit;
                    }
                }
            }
            controller_state = self.controller() => {
                return controller_state;
            }
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    Init,
    Idle,
    Exit,
}

struct StateMachine {
    target_state: State,
    current_state: State,
    init_state: InitState,
    idle_state: IdleState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            target_state: State::Init,
            current_state: State::Init,
            init_state: InitState::new(),
            idle_state: IdleState::new(),
        }
    }

    pub async fn run(&mut self) {
        let mut target_state = self.target_state;
        loop {
            target_state = match self.current_state {
                State::Init => {
                    self.init_state.run().await
                }
                State::Idle => {
                    self.idle_state.run().await
                }
                State::Exit => {
                    return;
                }
            };

            self.current_state = target_state;
        }
    }
}

#[tokio::main]
async fn main() {
    log::default().init();

    let mut state_machine = StateMachine::new();
    // let mut state_machine_2 = StateMachine::new();

    join!(
        state_machine.run(),
        // state_machine_2.run()
    );
}