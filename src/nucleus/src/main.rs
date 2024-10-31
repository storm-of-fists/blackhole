#![feature(portable_simd)]

use std::{simd::Simd, thread::JoinHandle, time::Duration};

use nucleus::*;

#[derive(Debug)]
pub struct Position {
    pub positions: Vec<Simd<f64, 4>>,
}

impl State for Position {}

#[derive(Debug)]
pub struct Velocity {
    pub velocities: Vec<Simd<f64, 4>>,
}

impl State for Velocity {}

#[derive(Debug)]
pub struct PositionUpdater {
    positions: WriteState<Position>,
    velocities: ReadState<Velocity>,
}

impl UpdaterTrait for PositionUpdater {
    fn register(runner_ptr: RunnerPtr) -> Self {
        PositionUpdater {
            positions: runner_ptr
                .register_write_state(Position {
                    positions: Vec::new(),
                })
                .unwrap(),
            velocities: runner_ptr
                .register_read_state(Velocity {
                    velocities: Vec::new(),
                })
                .unwrap(),
        }
    }

    fn update(&mut self) {
        for (index, position) in self.positions.positions.iter_mut().enumerate() {
            if let Some(velocity) = self.velocities.velocities.get(index) {
                *position += *velocity;
            }
        }

        println!("{:?}", *self.positions);

        std::thread::sleep(Duration::from_secs(1));
    }
}

fn setup_main(nucleus_ptr: NucleusPtr) -> JoinHandle<()> {
    std::thread::spawn(|| {
        Runner::new(nucleus_ptr)
            .register_updater::<PositionUpdater>()
            .unwrap()
            .run();
    })
}

fn main() {
    Nucleus::new().add_runner(setup_main).join();
}
