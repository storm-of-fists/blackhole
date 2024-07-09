use std::{time::{Duration, Instant}, collections::HashMap};

pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct VelocityComponent {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

pub struct AccelerationComponent {
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
}

pub struct SimulationSettings {
    time_step: Duration,
    last_step_time: Instant,
}

pub struct World {
    next_entity_id: u64,
    
    simulation_settings: SimulationSettings,

    positions: HashMap<u64, PositionComponent>,
    velocities: HashMap<u64, VelocityComponent>,
    accelerations: HashMap<u64, AccelerationComponent>,
}


pub fn main() {
    let mut world = World {
        next_entity_id: 0,
        
        simulation_settings: SimulationSettings {
            time_step: Duration::from_millis(1000),
            last_step_time: Instant::now(),    
        },
        
        positions: HashMap::new(),
        velocities: HashMap::new(),
        accelerations: HashMap::new(),
    };
    
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
    
    pub fn run(&mut self) {
        loop {
            self.simulation_settings.last_step_time = Instant::now();
            
            update_positions(&self.simulation_settings, &mut self.positions, &self.velocities);
            update_velocities(&self.simulation_settings, &mut self.velocities, &self.accelerations);
            
            let overhead = self.simulation_settings.time_step - self.simulation_settings.last_step_time.elapsed();
            
            if overhead > Duration::ZERO {
                println!("sleeping for {overhead:?}");
                std::thread::sleep(overhead);
            }
        }
    }
}

pub fn update_positions(
    sim: &SimulationSettings,
    positions: &mut HashMap<u64, PositionComponent>,
    velocities: &HashMap<u64, VelocityComponent>
) {
    for (id, velocity) in velocities.iter() {
        if let Some(position) = positions.get_mut(id) {
            position.x += velocity.vx * sim.time_step.as_secs_f32();
        }
    }    
}

pub fn update_velocities(
    sim: &SimulationSettings,
    velocities: &mut HashMap<u64, VelocityComponent>,
    accelerations: &HashMap<u64, AccelerationComponent>
) {
    for (id, acceleration) in accelerations.iter() {
        if let Some(velocity) = velocities.get_mut(id) {
            velocity.vx += acceleration.ax * sim.time_step.as_secs_f32();
        }
    }    
}
