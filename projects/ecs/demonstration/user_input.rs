use connors_ecs::*;
use std::time::Duration;

#[derive(Debug)]
pub struct RawUserInput {
    // recent_inputs: [KeyState]
    input_count: u32,
}

impl DataTrait for RawUserInput {}

pub struct UserInputUpdater {
    raw_input: DataSingleton<RawUserInput>,
}

impl UpdaterTrait for UserInputUpdater {
    fn register(nucleus: Nucleus) {
        nucleus.add_data_singleton(RawUserInput {
            input_count: 0,
        });
    }

    fn new(nucleus: Nucleus) -> Self {
        Self {
            raw_input: nucleus.get_data_singleton::<RawUserInput>()
        }
    }

    fn update(&self) {
        println!("getting user input");
        std::thread::sleep(Duration::from_millis(10));
    }
}