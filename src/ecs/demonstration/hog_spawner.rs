use connors_ecs::*;
use movement::Mover;

pub struct HogSpawnData {
    next_hog_id: usize,
}

impl DataTrait for HogSpawnData {}

pub struct HogSpawnUpdater {
    movers: DataSet<Mover>,
    hog_spawn: DataSingleton<HogSpawnData>,
}

impl UpdaterTrait for HogSpawnUpdater {
    fn register(nucleus: Nucleus) {
        nucleus.add_data_singleton(HogSpawnData {
            next_hog_id: 0,
        });
    }

    fn new(nucleus: Nucleus) -> Self {
        Self {
            movers: nucleus.get_data_set::<Mover>(),
            hog_spawn: nucleus.get_data_singleton::<HogSpawnData>(),
        }
    }

    fn update(&self) {
        let mut movers = self.movers.get();
        let mut hog_spawn = self.hog_spawn.get();

        if movers.len() > 500 {
            return;
        }

        let next_hog_id = hog_spawn.next_hog_id;
        hog_spawn.next_hog_id += 1;

        let mut new_hog_mover = Mover::new();
        new_hog_mover.ax = 0.1;
        new_hog_mover.ay = 0.0;
        new_hog_mover.az = -0.1;

        movers.insert(next_hog_id, new_hog_mover);
    }
}