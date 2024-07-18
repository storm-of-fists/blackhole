struct NucleusData {
    // name: String,
    // data_maps:
    // data_singletons:
    // read_only_data:
    // runners:
    // updaters:
}

pub struct Nucleus {
    data: Arc<Mutex<NucleusData>>,
}

pub struct Data<T> {
    data: Arc<Mutex<T>>,
    // Simple shared access across threads?
    // Should we include a map?
    // access to the entity_id this thing contains
}

pub struct ReadOnlyData<T> {
    data: Arc<T>
}

pub struct UpdaterSchedule {
    // at some period in time, before or after some other thing,
    // only ever
    constraints: Vec<UpdaterScheduleConstraints>,
}

pub struct Updater {
    nucleus: Nucleus,
    schedule: Schedule
    // updater: Box<dyn UpdaterTrait>,
    // access to nucleus
}

pub trait UpdaterTrait: 'static {
    fn register_new_data(nucleus: Nucleus) where Self: Sized;
    fn new(nucleus: Nucleus) -> Self where Self: Sized;
    fn update(&self);
}