use movement::*;
use hog_spawner::HogSpawnUpdater;
use user_input::UserInputUpdater;
use connors_ecs::*;

fn main() {
    let mut nucleus = Nucleus::new("demo");
    let mut main = Runner::new("main", nucleus.clone());



    main.register_updater::<MovementUpdater>();
    main.register_updater::<HogSpawnUpdater>();
    main.register_updater::<UserInputUpdater>();

    let nucleus_clone = nucleus.clone();

    std::thread::spawn(move || {
        let mut input = Runner::new("input", nucleus_clone);

        input.add_updater::<UserInputUpdater>();

        input.run();
    });




    main.add_updater::<MovementUpdater>();
    main.add_updater::<HogSpawnUpdater>();



    main.run();
}