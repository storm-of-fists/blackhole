use std::{
    time::{Duration, Instant, SystemTime, UNIX_EPOCH}, 
    collections::BTreeMap,
    rc::Rc,
    cell::RefCell,
};

/// The period for the server logic to run.
const SERVER_PERIOD: u8 = 8;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

#[derive(Debug)]
pub struct Mover {
    pub position: Rc<RefCell<Position>>,
    pub velocity: Rc<RefCell<Velocity>>,
}

pub fn update_movers(
    timing: &Timing,
    movers: &BTreeMap<u64, Rc<RefCell<Mover>>>,
) {
    for (_id, rc_mover) in movers.iter() {
        let Ok(mover) = rc_mover.try_borrow() else {
            continue;
        };
        
        let Ok(mut position) = mover.position.try_borrow_mut() else {
            continue;
        };
        
        let Ok(velocity) = mover.velocity.try_borrow() else {
            continue;
        };

        position.x += velocity.vx * timing.time_step.as_secs_f32();
        position.y += velocity.vy * timing.time_step.as_secs_f32();
        position.z += velocity.vz * timing.time_step.as_secs_f32();

        // println!("position: {:?})", position);
    }
}

pub struct Timing {
    time_step: Duration,
    step_instant: Instant,
    start_epoch_timestamp: Duration,
    start_instant: Instant,
    step_overhead: Duration,
}

pub fn update_timing_instant(
    timing: &mut Timing,
) {
    timing.step_instant = Instant::now();
}

pub fn update_rest_of_timing(
    timing: &mut Timing,
) -> bool {
    timing.step_overhead = timing.time_step - timing.step_instant.elapsed();
            
    if timing.step_overhead > Duration::ZERO {
        println!("sleeping for {:?}", timing.step_overhead);
        std::thread::sleep(timing.step_overhead);
    }
    
    timing.start_instant.elapsed() > Duration::from_millis(100000)
}

pub struct World {
    next_entity_id: u64,
    
    timing: Timing,

    positions: BTreeMap<u64, Rc<RefCell<Position>>>,
    velocities: BTreeMap<u64, Rc<RefCell<Velocity>>>,
    movers: BTreeMap<u64, Rc<RefCell<Mover>>>,
}


pub fn main() {
    let mut world = World {
        next_entity_id: 0,

        timing: Timing {
            time_step: Duration::from_millis(SERVER_PERIOD as u64),
            start_epoch_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            step_instant: Instant::now(),
            start_instant: Instant::now(),
            step_overhead: Duration::ZERO,
        },
        
        positions: BTreeMap::new(),
        velocities: BTreeMap::new(),
        movers: BTreeMap::new(),
    };
    
    for _ in 0..10000 {
        let entity_id = world.new_entity_id();
        world.register_mover(
            entity_id, 
            Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }, 
            Velocity {
                vx: 0.0,
                vy: 1.0,
                vz: -1.0,
            },
        );
    }
    
    world.run();
}

impl World {
    pub fn new_entity_id(&mut self) -> u64 {
        let new_id = self.next_entity_id + 1;
        std::mem::replace(&mut self.next_entity_id, new_id)
    }
    
    pub fn remove_entity_by_id(&mut self, id: u64) {
        self.positions.remove(&id);
        self.velocities.remove(&id);
        self.movers.remove(&id);
    }
    
    pub fn register_mover(
        &mut self,
        entity_id: u64,
        position: Position,
        velocity: Velocity
    ) -> Rc<RefCell<Mover>> {
        let mover = Rc::new(RefCell::new(Mover {
            position: self.register_position(entity_id, position),
            velocity: self.register_velocity(entity_id, velocity),
        }));
        
        self.movers.insert(entity_id, mover.clone());
        
        mover
    }
    
    pub fn register_position(
        &mut self,
        entity_id: u64,
        position: Position
    ) -> Rc<RefCell<Position>> {
        let rc_position = Rc::new(RefCell::new(position));
        self.positions.insert(entity_id, rc_position.clone());
        rc_position
    }
    
    pub fn register_velocity(
        &mut self,
        entity_id: u64,
        velocity: Velocity
    ) -> Rc<RefCell<Velocity>> {
        let rc_velocity = Rc::new(RefCell::new(velocity));
        self.velocities.insert(entity_id, rc_velocity.clone());
        rc_velocity
    }
    
    pub fn run(&mut self) {
        loop {
            update_timing_instant(&mut self.timing);
            
            update_movers(
                &self.timing, 
                &self.movers,
            );
            
            if update_rest_of_timing(&mut self.timing) {
                return;
            }
        }
    }
}



// use std::{
//     time::{Duration, Instant, SystemTime, UNIX_EPOCH}, 
//     collections::BTreeMap,
//     rc::Rc,
//     cell::RefCell,
// };

// /// The period for the server logic to run.
// const SERVER_PERIOD: u8 = 8;

// pub struct Component<T> {
//     entity_id: u64,
//     data: Rc<RefCell<T>>,
// }

// impl<T> Component<T> where T: Sized {
//     pub fn new(entity_id: u64, data: T) -> Self {
//         Self {
//             entity_id,
//             data: Rc::new(RefCell::new(data))
//         }
//     }
    
//     pub fn from_rc(entity_id: u64, data: Rc<RefCell<T>>) -> Self {
//         Self {
//             entity_id,
//             data,
//         }
//     }
    
//     pub fn get(&self) -> Result<&T> {
        
//     }
    
//     /// This may seem like it should be mutable, but we are going through
//     /// the refcell for it. We keep this a &self reference so we can mutate
//     /// all along a component tree.
//     pub fn get_mut(&self) -> Result<&mut T> {
        
//     }
    
//     /// Implement try_clone to avoid panicking.
//     pub fn try_clone(&self) -> Result<Self> {
//         if self.get().is_ok() {
//             return Self {
//                 entity_id: self.entity_id,
//                 data: self.data.clone(),
//             }
//         } else {
//             return Result::Error
//         }
//     }
    
//     pub fn instance_count(&self) -> u64 {
//         self.data.strong_count()
//     }
    
//     pub fn entity_id(&self) -> &u64 {
//         &self.entity_id
//     }
// }

// // pub struct Simulation {
    
// // }

// // pub struct ThreadsafeComponent<T> {
// //     data: Arc<Mutex<T>>,
// // }

// // impl<T> ThreadsafeComponent<T> where T: Sized {
    
// // }

// #[derive(Debug)]
// pub struct Position {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

// #[derive(Debug)]
// pub struct Velocity {
//     pub vx: f32,
//     pub vy: f32,
//     pub vz: f32,
// }

// #[derive(Debug)]
// pub struct Mover {
//     pub position: Component<Position>,
//     pub velocity: Component<Velocity>,
// }

// pub fn update_movers(
//     cycle: &Cycle,
//     movers: &BTreeMap<u64, Component<Mover>>,
// ) {
//     for (_id, rc_mover) in movers.iter() {
//         let Ok(mover) = rc_mover.try_borrow() else {
//             continue;
//         };
        
//         let Ok(mut position) = mover.position.try_borrow_mut() else {
//             continue;
//         };
        
//         let Ok(velocity) = mover.velocity.try_borrow() else {
//             continue;
//         };

//         position.x += velocity.vx * cycle.period.as_secs_f32();
//         position.y += velocity.vy * cycle.period.as_secs_f32();
//         position.z += velocity.vz * cycle.period.as_secs_f32();

//         // println!("position: {:?})", position);
//     }
// }

// pub struct StartTiming {
//     instant: Instant,
//     epoch_timestamp: Duration,
// }

// pub fn check_total_runtime(
//     start_timing: &StartTiming,
// ) -> bool {
//      start_timing.instant.elapsed() > Duration::from_millis(1000)
// }

// pub struct Cycle {
//     number: u64,
//     start: Instant,
//     overhead: Duration,
//     period: Duration,
// }

// pub fn start_cycle(
//     cycle: &mut Cycle,
// ) {
//     cycle.start = Instant::now();
//     cycle.number += 1;
// }

// pub fn end_cycle(
//     cycle: &mut Cycle,
// ) {
//     let elapsed_from_cycle_start = cycle.start.elapsed();
    
//     if cycle.period > elapsed_from_cycle_start {
//         cycle.overhead = cycle.period - elapsed_from_cycle_start;
//         println!("sleeping for {:?}", cycle.overhead);
//         std::thread::sleep(cycle.overhead);
//     } else {
//         cycle.overhead = Duration::ZERO;
//         println!("Ran over cycle start: {:?}", elapsed_from_cycle_start);
//     }
// }

// pub struct World {
//     next_entity_id: u64,
    
//     start_timing: StartTiming,
//     cycle: Cycle,

//     positions: BTreeMap<u64, Component<Position>>,
//     velocities: BTreeMap<u64, Component<Velocity>>,
//     movers: BTreeMap<u64, Component<Mover>>,
// }


// pub fn main() {
//     let mut world = World {
//         next_entity_id: 0,

//         start_timing: StartTiming {
//             instant: Instant::now(),
//             epoch_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
//         },
        
//         cycle: Cycle {
//             number: 0,
//             start: Instant::now(),
//             overhead: Duration::ZERO,
//             period: Duration::from_millis(SERVER_PERIOD as u64),
//         },
        
//         positions: BTreeMap::new(),
//         velocities: BTreeMap::new(),
//         movers: BTreeMap::new(),
//     };
    
//     for _ in 0..50 {
//         let entity_id = world.new_entity_id();
//         world.register_mover(
//             entity_id, 
//             Position {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//             }, 
//             Velocity {
//                 vx: 0.0,
//                 vy: 1.0,
//                 vz: -1.0,
//             },
//         );
//     }
    
//     world.run();
// }

// impl World {
//     pub fn new_entity_id(&mut self) -> u64 {
//         let new_id = self.next_entity_id + 1;
//         std::mem::replace(&mut self.next_entity_id, new_id)
//     }
    
//     pub fn remove_entity_by_id(&mut self, id: u64) {
//         self.positions.remove(&id);
//         self.velocities.remove(&id);
//         self.movers.remove(&id);
//     }
    
//     pub fn register_mover(
//         &mut self,
//         entity_id: u64,
//         position: Position,
//         velocity: Velocity
//     ) -> Component<Mover> {
//         let mover = Component::new(entity_id, Mover {
//             position: self.register_position(entity_id, position),
//             velocity: self.register_velocity(entity_id, velocity),
//         });
        
//         self.movers.insert(entity_id, mover.clone());
        
//         mover
//     }
    
//     pub fn register_position(
//         &mut self,
//         entity_id: u64,
//         position: Position
//     ) -> ComponentPosition> {
//         let rc_position = Component::new(position);
//         self.positions.insert(entity_id, rc_position.clone());
//         rc_position
//     }
    
//     pub fn register_velocity(
//         &mut self,
//         entity_id: u64,
//         velocity: Velocity
//     ) -> Component<Velocity> {
//         let rc_velocity = Component::new(velocity);
//         self.velocities.insert(entity_id, rc_velocity.clone());
//         rc_velocity
//     }
    
//     pub fn run(&mut self) {
//         loop {
//             start_cycle(&mut self.cycle);
            
//             update_movers(
//                 &self.cycle, 
//                 &self.movers,
//             );
            
//             end_cycle(&mut self.cycle);
            
//             if check_total_runtime(&mut self.start_timing) {
//                 return;
//             }
//         }
//     }
// }

