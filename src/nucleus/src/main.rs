#![feature(portable_simd)]

use std::{
    simd::Simd,
    thread::JoinHandle,
    time::{Duration, Instant},
};

use nucleus::*;

pub struct Position {
    pub positions: [Simd<f64, 4>; 1000],
}

pub struct Velocity {
    pub velocities: [Simd<f64, 4>; 1000],
}

pub struct PositionUpdater {
    timing: State<Timing>,
    positions: State<Position>,
    velocities: State<Velocity>,
}

impl UpdaterTrait for PositionUpdater {
    fn new(nucleus: NucleusPtr, runner: &mut Runner) -> Self
    where
        Self: Sized,
    {
        PositionUpdater {
            timing: runner.add_state::<Timing>().unwrap(),
            positions: runner
                .add_state(Position {
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

pub struct Timing {
    pub start_of_loop: Instant,
    pub desired_loop_duration: Duration,
}

pub struct LoopTimingUpdater {
    timing_data: State<Timing>,
}

impl UpdaterTrait for LoopTimingUpdater {
    /// What if I only wanted to adjust the order of an updater?
    fn new(nucleus: NucleusPtr, runner: &mut Runner) -> Self
    where
        Self: Sized,
    {
        Self {
            timing_data: runner.add_state(Timing {
                start_of_loop: Instant::now(),
                desired_loop_duration: Duration::from_millis(100),
            }),
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

fn main() {
    // Nucleus::new().add_runner(main_runner).go();
}
