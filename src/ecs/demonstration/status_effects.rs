use connors_ecs::*;
use std::time::{Duration, Instant};
use user_input::RawUserInput;

pub enum StatusEffectsEnum {
    Fire,
    Poison,
    Frost,
    Acid,
}

#[derive(Debug)]
pub struct StatusEffects {
    status_effects: [4; StatusEffectsEnum]
}

impl StatusEffects {
    pub fn new() -> Self {
        Self {
            status_effects: [],
        }
    }
}

impl DataTrait for StatusEffects {}

pub struct StatusEffectUpdater {
    status_effects: DataSet<StatusEffects>,
    movers: DataSet<Mover>,
}

impl UpdaterTrait for MovementUpdater {
    fn register(nucleus: Nucleus) {
        nucleus.add_data_set(MovementSystem { update_count: 0 });
    }

    fn new(nucleus: Nucleus) -> Self {
        Self {
            status_effects: nucleus.get_data_set::<StatusEffects>(),
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




