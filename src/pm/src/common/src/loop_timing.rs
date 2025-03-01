use pm::*;
use std::time::{Duration, Instant};

#[derive(StateTrait)]
pub struct LoopTiming {
    pub start_of_loop: Instant,
    pub desired_loop_duration: Duration,
    pub loop_sleep_duration: Duration,
}

pub struct LoopTimingManager {
    timing_data: State<LoopTiming>,
}

impl DoerTrait for LoopTimingManager {
    fn new_state(state: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        let mut local_state = state.local.get()?;

        local_state.add_state(LoopTiming {
            start_of_loop: Instant::now(),
            desired_loop_duration: Duration::from_millis(100),
            loop_sleep_duration: Duration::from_millis(100),
        })?;

        Ok(())
    }

    fn new(pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        let local_state = pm.state.local.get()?;

        Ok(Box::new(Self {
            timing_data: local_state.get_state::<LoopTiming>()?,
        }))
    }

    fn update(&self) -> Result<(), PmError> {
        let mut timing_data = self.timing_data.get()?;

        let start_of_previous_loop =
            std::mem::replace(&mut timing_data.start_of_loop, Instant::now());

        let elapsed_since_last_loop = start_of_previous_loop.elapsed();

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

        std::thread::sleep(timing_data.loop_sleep_duration);

        Ok(())
    }
}
