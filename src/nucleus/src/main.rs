#![feature(portable_simd)]

use std::{
    any::Any,
    simd::Simd,
    time::{Duration, Instant},
};

use nucleus::*;

#[derive(Debug)]
pub struct Timing {
    pub start_of_loop: Instant,
    pub desired_loop_duration: Duration,
    pub loop_sleep_duration: Duration,
}

impl StateTrait for Timing {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct LoopTimingUpdater {
    timing_data: State<Timing>,
}

impl UpdaterTrait for LoopTimingUpdater {
    fn add_new_state(nucleus: NucleusPtr, state: State<StateRegistry>) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        let mut state_registry = state.try_get_mut().unwrap();

        state_registry.add_state(Timing {
            start_of_loop: Instant::now(),
            desired_loop_duration: Duration::from_millis(100),
            loop_sleep_duration: Duration::from_millis(100),
        })?;

        Ok(())
    }

    fn new(nucleus: NucleusPtr, thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        Ok(Box::new(Self {
            timing_data: thread.get_state::<Timing>().unwrap(),
        }))
    }

    fn update(&self) -> Result<(), NucleusError> {
        let mut timing_data = self.timing_data.try_get_mut().unwrap();

        let start_of_previous_loop =
            std::mem::replace(&mut timing_data.start_of_loop, Instant::now());

        let elapsed_since_last_loop = start_of_previous_loop.elapsed();

        let desired_loop_duration = timing_data.desired_loop_duration;

        if elapsed_since_last_loop > desired_loop_duration {
            let adjustment = elapsed_since_last_loop - desired_loop_duration;
            timing_data.loop_sleep_duration -= adjustment;
        } else {
            let adjustment = desired_loop_duration - elapsed_since_last_loop;
            timing_data.loop_sleep_duration += adjustment;
        }

        std::thread::sleep(timing_data.loop_sleep_duration);

        Ok(())
    }
}

#[derive(Debug)]
pub struct OtherUpdater2 {}

impl UpdaterTrait for OtherUpdater2 {
    fn new(nucleus: NucleusPtr, thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        Ok(Box::new(OtherUpdater2 {}))
    }

    fn update(&self) -> Result<(), NucleusError> {
        std::thread::sleep(Duration::from_millis(50));

        Ok(())
    }
}

pub fn main_thread(nucleus: NucleusPtr) -> Result<(), NucleusError> {
    let mut thread = Thread::new(nucleus);
    thread.register_updater::<LoopTimingUpdater>()?;
    thread.register_updater::<OtherUpdater2>()?;
    thread.run()?;

    Ok(())
}

fn main() {
    Nucleus::new().add_thread(main_thread).go();
}
