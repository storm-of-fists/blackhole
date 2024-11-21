use nucleus::*;
use std::time::{Duration, Instant};
use crate::logging::Logging;

#[derive(StateTrait)]
pub struct LoopTiming {
    pub start_of_loop: Instant,
    pub desired_loop_duration: Duration,
    pub loop_sleep_duration: Duration,
}

pub struct LoopTimingManager {
    timing_data: State<LoopTiming>,
    log: State<Logging>,
}

impl UpdaterTrait for LoopTimingManager {
    fn add_new_state(
        shared_state: SharedState<SharedStateStore>,
        thread_state: State<StateStore>,
    ) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        let mut thread_state = thread_state.get_mut()?;

        thread_state.add_state(LoopTiming {
            start_of_loop: Instant::now(),
            desired_loop_duration: Duration::from_millis(100),
            loop_sleep_duration: Duration::from_millis(100),
        })?;

        Ok(())
    }

    fn new(thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        let local_state = thread.local_state.get_mut()?;

        Ok(Box::new(Self {
            timing_data: local_state.get_state::<LoopTiming>()?,
            log: local_state.get_state::<Logging>()?,
        }))
    }

    fn update(&self) -> Result<(), NucleusError> {
        let mut timing_data = self.timing_data.get_mut()?;
        let mut log = self.log.get_mut()?;

        let start_of_previous_loop =
            std::mem::replace(&mut timing_data.start_of_loop, Instant::now());

        let elapsed_since_last_loop = start_of_previous_loop.elapsed();

        log.log(&format!("loop duration {:?}", elapsed_since_last_loop));

        let desired_loop_duration = timing_data.desired_loop_duration;

        if elapsed_since_last_loop > desired_loop_duration {
            let mut adjustment = elapsed_since_last_loop - desired_loop_duration;
            if adjustment > timing_data.loop_sleep_duration {
                adjustment = timing_data.loop_sleep_duration;
            }
            timing_data.loop_sleep_duration -= adjustment;
        } else {
            let adjustment = desired_loop_duration - elapsed_since_last_loop;
            timing_data.loop_sleep_duration += adjustment;
        }

        log.log(&format!("loop sleep duration {:?}", timing_data.loop_sleep_duration));

        std::thread::sleep(timing_data.loop_sleep_duration);

        Ok(())
    }
}
