#![feature(portable_simd)]

use std::{
    simd::Simd,
    thread::JoinHandle,
    time::{Duration, Instant},
};

use nucleus::*;

#[derive(Debug)]
pub struct Position {
    pub positions: [Simd<f64, 4>; 1000],
}

impl State for Position {}

#[derive(Debug)]
pub struct Velocity {
    pub velocities: [Simd<f64, 4>; 1000],
}

impl State for Velocity {}

#[derive(Debug)]
pub struct PositionUpdater {
    timing: ReadState<Timing>,
    positions: WriteState<Position>,
    velocities: ReadState<Velocity>,
}

impl UpdaterTrait for PositionUpdater {
    fn new(runner_ptr: RunnerPtr) -> Self {
        PositionUpdater {
            timing: runner_ptr.get_read_state::<Timing>().unwrap(),
            positions: runner_ptr
                .register_write_state(Position {
                    positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 1000],
                })
                .unwrap(),
            velocities: runner_ptr
                .register_read_state::<Velocity>(Velocity {
                    velocities: [Simd::from_array([1.0, 1.0, 1.0, 1.0]); 1000],
                })
                .unwrap(),
        }
    }

    fn update(&mut self) {
        for (index, position) in self.positions.positions.iter_mut().enumerate() {
            let loop_duration = self.timing.desired_loop_duration.as_secs_f64();
            if let Some(velocity) = self.velocities.velocities.get(index) {
                *position += *velocity * Simd::from_array([loop_duration; 4]);
            }
        }

        // println!("{:?}", *self.positions);
    }
}

#[derive(Debug)]
pub struct Timing {
    pub start_of_loop: Instant,
    pub desired_loop_duration: Duration,
}

impl State for Timing {}

#[derive(Debug)]
pub struct LoopTimingUpdater {
    timing_data: WriteState<Timing>,
}

impl UpdaterTrait for LoopTimingUpdater {
    /// What if I only wanted to adjust the order of an updater?
    fn new(runner_ptr: RunnerPtr) -> Self
    where
        Self: Sized,
    {
        Self {
            timing_data: runner_ptr
                .register_write_state::<Timing>(Timing {
                    start_of_loop: Instant::now(),
                    desired_loop_duration: Duration::from_millis(100),
                })
                .unwrap(),
        }
    }

    fn update(&mut self) {
        let start_of_previous_loop =
            std::mem::replace(&mut self.timing_data.start_of_loop, Instant::now());

        let sleep_time = self.timing_data.desired_loop_duration - start_of_previous_loop.elapsed();

        println!("sleeping for {:?}", sleep_time);

        // std::thread::sleep This is THE timing control mechanism for the thread.
        std::thread::sleep(sleep_time);
    }
}



fn main_runner(nucleus_ptr: NucleusPtr) -> Result<(), RunnerStartError> {
    Runner::new(nucleus_ptr)
        .register_updater::<PositionUpdater>()?
        .register_updater::<LoopTimingUpdater>()?
        .run()?;
}

fn main() {
    Nucleus::new().add_runner(main_runner).go();
}
