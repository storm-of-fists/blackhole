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

pub struct LoopTimingUpdater {
    timing_data: State<Timing>,
}

impl UpdaterTrait for LoopTimingUpdater {
    fn add_new_state(nucleus: NucleusPtr, runner: &mut Runner)
    where
        Self: Sized,
    {
        runner.add_state(Timing {
            start_of_loop: Instant::now(),
            desired_loop_duration: Duration::from_millis(100),
            loop_sleep_duration: Duration::from_millis(100),
        });
    }

    fn register(nucleus: NucleusPtr, runner: &mut Runner)
    where
        Self: Sized,
    {
        let updater = Self {
            timing_data: runner.get_state::<Timing>().unwrap(),
        };

        runner
            .active_updaters
            .insert(updater.type_id(), Box::new(updater));
    }

    fn first(&self, _nucleus: NucleusPtr, _runner: &mut Runner) {}

    fn update(&self) {
        let mut timing_data = self.timing_data.try_get_mut().unwrap();

        let start_of_previous_loop =
            std::mem::replace(&mut timing_data.start_of_loop, Instant::now());

        let elapsed_since_last_loop = start_of_previous_loop.elapsed();
        println!("elapsed: {:?}", elapsed_since_last_loop);
        let desired_loop_duration = timing_data.desired_loop_duration;

        if elapsed_since_last_loop > desired_loop_duration {
            let adjustment = elapsed_since_last_loop - desired_loop_duration;
            timing_data.loop_sleep_duration -= adjustment;
        } else {
            let adjustment = desired_loop_duration - elapsed_since_last_loop;
            timing_data.loop_sleep_duration += adjustment;
        }

        println!("actual loop sleep dur: {:?}", timing_data.loop_sleep_duration);

        std::thread::sleep(timing_data.loop_sleep_duration);
    }
}

pub struct OtherUpdater {}

impl UpdaterTrait for OtherUpdater {
    fn register(nucleus: NucleusPtr, runner: &mut Runner)
        where
            Self: Sized {
        let updater = OtherUpdater {};

        runner.active_updaters.insert(updater.type_id(), Box::new(updater));
    }

    fn update(&self) {
        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn main_runner(nucleus: NucleusPtr) -> Result<(), RunnerError> {
    let mut runner = Runner::new(nucleus);
    runner.add_updater::<LoopTimingUpdater>();
    runner.add_updater::<OtherUpdater>();
    runner.run();

    Ok(())
}

fn main() {
    Nucleus::new().add_runner(main_runner).go();
}
