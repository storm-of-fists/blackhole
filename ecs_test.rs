use std::{
    time::{Duration, Instant, SystemTime, UNIX_EPOCH}, 
    collections::BTreeMap,
    // rc::Rc,
    // cell::RefCell,
};

/// The period for the server logic to run.
const SERVER_PERIOD: u8 = 8;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn update_positions(
    timing: &Timing,
    positions: &mut BTreeMap<u64, Position>,
    velocities: &BTreeMap<u64, Velocity>
) {
    for (id, velocity) in velocities.iter() {
        let Some(position) = positions.get_mut(id) else {
            continue;
        };

        position.x += velocity.vx * timing.time_step.as_secs_f32();
        position.y += velocity.vy * timing.time_step.as_secs_f32();
        position.z += velocity.vz * timing.time_step.as_secs_f32();

        // println!("position: {:?})", position);
    }
}

pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

pub struct Acceleration {
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
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
    
    timing.start_instant.elapsed() > Duration::from_millis(40)
}

pub struct World {
    next_entity_id: u64,
    
    timing: Timing,

    positions: BTreeMap<u64, Position>,
    velocities: BTreeMap<u64, Velocity>,
    accelerations: BTreeMap<u64, Acceleration>,
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
        accelerations: BTreeMap::new(),
    };
    
    for _ in 0..1000 {
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
        self.accelerations.remove(&id);
    }
    
    pub fn register_mover(
        &mut self,
        entity_id: u64,
        position: Position,
        velocity: Velocity
    ) {
        self.positions.insert(entity_id, position);
        self.velocities.insert(entity_id, velocity);
    }
    
    pub fn run(&mut self) {
        loop {
            update_timing_instant(&mut self.timing);
            
            update_positions(
                &self.timing, 
                &mut self.positions, 
                &self.velocities
            );
            
            if update_rest_of_timing(&mut self.timing) {
                return;
            }
        }
    }
}



// pub fn update_velocities(
//     timing: &Timing,
//     velocities: &mut BTreeMap<u64, Rc<RefCell<Velocity>>>,
//     accelerations: &BTreeMap<u64, Rc<RefCell<Acceleration>>>
// ) {
//     for (id, acceleration) in accelerations.iter() {
//         let Some(refcell_acceleration) = positions.get(id) else {
//             continue;
//         };
        
//         let Ok(position) = refcell_position.try_borrow_mut() else {
//             continue;
//         };
        
//         velocity.vx += acceleration.ax * sim.time_step.as_secs_f32();    
//     }
// }
