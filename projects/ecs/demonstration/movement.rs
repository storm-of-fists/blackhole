use connors_ecs::*;
use std::time::{Duration, Instant};
use user_input::RawUserInput;

#[derive(Debug)]
pub struct Mover {
    x: f32,
    y: f32,
    z: f32,
    vx: f32,
    vy: f32,
    vz: f32,
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
}

impl Mover {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
        }
    }
}

impl DataTrait for Mover {}

pub struct MovementSystem {
    update_count: u32,
}

impl DataTrait for MovementSystem {}

pub struct MovementUpdater {
    raw_user_input: DataSingleton<RawUserInput>,
    system: DataSingleton<MovementSystem>,
    movers: DataSet<Mover>,
}

impl UpdaterTrait for MovementUpdater {
    fn register(nucleus: Nucleus) {
        nucleus.add_data_singleton(MovementSystem { update_count: 0 });
        nucleus.add_data_set::<Mover>();
    }

    fn new(nucleus: Nucleus) -> Self {
        Self {
            raw_user_input: nucleus.get_data_singleton::<RawUserInput>(),
            system: nucleus.get_data_singleton::<MovementSystem>(),
            movers: nucleus.get_data_set::<Mover>(),
        }
    }

    fn update(&self) {
        let now = Instant::now();
        let mut system = self.system.get();
        let mut movers = self.movers.get();
        let raw_user_input = self.raw_user_input.get();

        println!("raw input count: {:?}", raw_user_input);

        drop(raw_user_input);

        for mover in movers.values_mut() {
            mover.x += mover.vx * 0.1;
            mover.y += mover.vy * 0.1;
            mover.z += mover.vz * 0.1;

            mover.vx += mover.ax * 0.1;
            mover.vy += mover.ay * 0.1;
            mover.vz += mover.az * 0.1;

            // println!("{:?}", mover);
        }

        system.update_count += 1;

        println!("run {} took {:?}", system.update_count, now.elapsed());

        std::thread::sleep(Duration::from_millis(20));
    }
}




