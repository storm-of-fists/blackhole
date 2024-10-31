#![feature(portable_simd)]
#![feature(mpmc_channel)]

use std::{
    simd::Simd,
    sync::{mpmc::{channel, Sender}, Mutex, RwLock},
    time::{Duration, Instant},
};

use criterion::{Criterion, criterion_group, criterion_main};
use nucleus::{ReadState, State, StateRegistry, WriteState};
use std::hint::black_box;

#[derive(Debug)]
pub struct PositionState {
    pub positions: [Simd<f64, 4>; 10000],
}

impl State for PositionState {}

#[derive(Debug)]
pub struct VelocityState {
    pub velocities: [Simd<f64, 4>; 10000],
}

impl State for VelocityState {}

fn no_simd_iterators(c: &mut Criterion) {
    let mut registry = StateRegistry::new();

    registry.register_state(PositionState {
        positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
    });

    registry.register_state(VelocityState {
        velocities: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
    });

    let mut w_positions = registry.get_write_state::<PositionState>().unwrap();
    let r_velocities = registry.get_read_state::<VelocityState>().unwrap();

    c.bench_function("no_simd_iterators", |b| {
        b.iter(|| {
            for (index, position) in w_positions.positions.iter_mut().enumerate() {
                if let Some(velocity) = r_velocities.velocities.get(index) {
                    black_box(position[0] += velocity[0]);
                    black_box(position[1] += velocity[1]);
                    black_box(position[2] += velocity[2]);
                    black_box(position[3] += velocity[3]);
                }
            }
        })
    });
}

fn direct_index_simd(c: &mut Criterion) {
    let mut registry = StateRegistry::new();

    registry.register_state(PositionState {
        positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
    });

    registry.register_state(VelocityState {
        velocities: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
    });

    let mut w_positions = registry.get_write_state::<PositionState>().unwrap();
    let r_velocities = registry.get_read_state::<VelocityState>().unwrap();

    let len = w_positions.positions.len();

    c.bench_function("direct_index_simd", |b| {
        b.iter(|| {
            for i in 0..len {
                black_box(w_positions.positions[i] += r_velocities.velocities[i]);
            }
        })
    });
}

fn getter_simd(c: &mut Criterion) {
    let mut registry = StateRegistry::new();

    registry.register_state(PositionState {
        positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
    });

    registry.register_state(VelocityState {
        velocities: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
    });

    let mut w_positions = registry.get_write_state::<PositionState>().unwrap();
    let r_velocities = registry.get_read_state::<VelocityState>().unwrap();

    c.bench_function("getter_simd", |b| {
        b.iter(|| {
            for (index, position) in w_positions.positions.iter_mut().enumerate() {
                if let Some(velocity) = r_velocities.velocities.get(index) {
                    black_box(*position += *velocity);
                }
            }
        })
    });
}

fn channel_messaging(c: &mut Criterion) {
    let (tx, rx) = channel::<u64>();

    c.bench_function("channel_messaging", |b| {
        b.iter(|| {
            black_box(tx.send(100000000000000).unwrap());
        })
    });
}

fn push_to_vec(c: &mut Criterion) {
    c.bench_function("push_to_vec", |b| {
        b.iter(|| {
            black_box(Vec::<f64>::with_capacity(5000));
        })
    });
}

fn rwlock_array(c: &mut Criterion) {
    let arr = RwLock::new([1.0; 10000]);
    c.bench_function("rwlock_array", |b| {
        b.iter(|| {
            black_box({
                let mut arr = arr.try_write().unwrap();
                arr[100] = 3.0;
            });
        })
    });
}

// criterion_group!(benches, getter_simd, direct_index_simd, no_simd_iterators);
criterion_group!(benches, rwlock_array);

criterion_main!(benches);
