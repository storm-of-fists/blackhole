#![feature(portable_simd)]
#![feature(mpmc_channel)]

use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
};

use criterion::{Criterion, criterion_group, criterion_main};
use nucleus::*;
use std::hint::black_box;

// #[derive(Debug)]
// pub struct PositionState {
//     pub positions: [Simd<f64, 4>; 10000],
// }

// #[derive(Debug)]
// pub struct VelocityState {
//     pub velocities: [Simd<f64, 4>; 10000],
// }

// fn no_simd_iterators(c: &mut Criterion) {
//     let mut registry = StateRegistry::new();

//     registry.register_state(PositionState {
//         positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
//     });

//     registry.register_state(VelocityState {
//         velocities: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
//     });

//     let mut w_positions = registry.get_write_state::<PositionState>().unwrap();
//     let r_velocities = registry.get_read_state::<VelocityState>().unwrap();

//     c.bench_function("no_simd_iterators", |b| {
//         b.iter(|| {
//             for (index, position) in w_positions.positions.iter_mut().enumerate() {
//                 if let Some(velocity) = r_velocities.velocities.get(index) {
//                     black_box(position[0] += velocity[0]);
//                     black_box(position[1] += velocity[1]);
//                     black_box(position[2] += velocity[2]);
//                     black_box(position[3] += velocity[3]);
//                 }
//             }
//         })
//     });
// }

// fn direct_index_simd(c: &mut Criterion) {
//     let mut registry = StateRegistry::new();

//     registry.register_state(PositionState {
//         positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
//     });

//     registry.register_state(VelocityState {
//         velocities: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
//     });

//     let mut w_positions = registry.get_write_state::<PositionState>().unwrap();
//     let r_velocities = registry.get_read_state::<VelocityState>().unwrap();

//     let len = w_positions.positions.len();

//     c.bench_function("direct_index_simd", |b| {
//         b.iter(|| {
//             for i in 0..len {
//                 black_box(w_positions.positions[i] += r_velocities.velocities[i]);
//             }
//         })
//     });
// }

// fn getter_simd(c: &mut Criterion) {
//     let mut registry = StateRegistry::new();

//     registry.register_state(PositionState {
//         positions: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
//     });

//     registry.register_state(VelocityState {
//         velocities: [Simd::from_array([0.0, 0.0, 0.0, 0.0]); 10000],
//     });

//     let mut w_positions = registry.get_write_state::<PositionState>().unwrap();
//     let r_velocities = registry.get_read_state::<VelocityState>().unwrap();

//     c.bench_function("getter_simd", |b| {
//         b.iter(|| {
//             for (index, position) in w_positions.positions.iter_mut().enumerate() {
//                 if let Some(velocity) = r_velocities.velocities.get(index) {
//                     black_box(*position += *velocity);
//                 }
//             }
//         })
//     });
// }

// fn channel_messaging(c: &mut Criterion) {
//     let (tx, rx) = channel::<u64>();

//     c.bench_function("channel_messaging", |b| {
//         b.iter(|| {
//             black_box(tx.send(100000000000000).unwrap());
//         })
//     });
// }

// fn push_to_vec(c: &mut Criterion) {
//     c.bench_function("push_to_vec", |b| {
//         b.iter(|| {
//             black_box(Vec::<f64>::with_capacity(5000));
//         })
//     });
// }

// fn rwlock_array(c: &mut Criterion) {
//     let arr = RwLock::new([1.0; 10000]);
//     c.bench_function("rwlock_array", |b| {
//         b.iter(|| {
//             black_box({
//                 let mut arr = arr.try_write().unwrap();
//                 arr[100] = 3.0;
//             });
//         })
//     });
// }

#[derive(Clone, Copy)]
struct SomeStruct {
    one: f32,
    two: u32,
    three: f64,
}

impl SomeStruct {
    pub fn new() -> Self {
        SomeStruct {
            one: 0.0,
            two: 0,
            three: 0.0,
        }
    }

    pub fn incr(&mut self) {
        self.one += 1.0;
        self.two += 1;
        self.three += 1.0;
    }

    #[inline(always)]
    pub fn incr_inline(&mut self) {
        self.one += 1.0;
        self.two += 1;
        self.three += 1.0;
    }
}

/// Based on this test with 100000 items, using refcell is maybe ~15 us slower than boxes.
/// RefCell is 120 us slower than a raw pointer (gross!). Lets say, worst case, we had
/// a game that was 120Hz. Thats about 8ms per frame. That 120us is 1.5% of the 8ms cycle we get.
/// I dont think the overhead of refcell is all that high, but ill keep doing testing.
fn refcell_item(c: &mut Criterion) {
    let mut non_refcell_items = Vec::new();
    let mut refcell_items = Vec::new();
    let mut raw_ptr_items = Vec::new();

    for _ in 0..100000 {
        let data = SomeStruct::new();

        non_refcell_items.push(Box::new(data));
        refcell_items.push(Rc::new(RefCell::new(data)));
        raw_ptr_items.push(UnsafeCell::new(data));
    }

    c.bench_function("non_refcell_items", |b| {
        b.iter(|| {
            black_box({
                for data in non_refcell_items.iter_mut() {
                    data.incr();
                }
            });
        })
    });

    c.bench_function("refcell_items", |b| {
        b.iter(|| {
            black_box({
                for rc_data in refcell_items.iter() {
                    if let Ok(mut data) = rc_data.try_borrow_mut() {
                        data.incr();
                    }
                }
            });
        })
    });

    c.bench_function("raw_ptr_items", |b| {
        b.iter(|| {
            black_box({
                for data in raw_ptr_items.iter() {
                    unsafe {
                        data.get().as_mut().unwrap().incr();
                    }
                }
            });
        })
    });
}

pub trait MyTrait {
    fn some_function_call(&mut self);
    fn some_function_call1(&mut self) {
        ()
    }
    fn some_function_call2(&mut self) {
        ()
    }
    fn some_function_call3(&mut self) {
        ()
    }
}

impl MyTrait for SomeStruct {
    fn some_function_call(&mut self) {
        self.one += 1.0;
        self.two += 1;
        self.three += 1.0;
    }
}

fn dynamic_func_calls(c: &mut Criterion) {
    let mut inline_calls = Vec::new();
    let mut regular_calls = Vec::new();
    let mut virtual_calls = Vec::new();

    for _ in 0..1024 {
        let data = SomeStruct::new();

        inline_calls.push(data);
        regular_calls.push(data);
        virtual_calls.push(Box::new(data));
    }

    c.bench_function("inline_calls", |b| {
        b.iter(|| {
            black_box({
                for data in inline_calls.iter_mut() {
                    data.incr_inline();
                }
            });
        })
    });

    c.bench_function("regular_calls", |b| {
        b.iter(|| {
            black_box({
                for data in regular_calls.iter_mut() {
                    data.incr();
                }
            });
        })
    });

    c.bench_function("virtual_calls", |b| {
        b.iter(|| {
            black_box({
                for data in virtual_calls.iter_mut() {
                    data.some_function_call();
                }
            });
        })
    });
}

// criterion_group!(benches, getter_simd, direct_index_simd, no_simd_iterators);
criterion_group!(benches, dynamic_func_calls);

criterion_main!(benches);
