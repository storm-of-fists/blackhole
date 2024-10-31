/// nucleus
/// - contains all data in a large hashmap 
/// - gives access to each updater to disable or enable it
/// this gives you the ability to write mods a program can willingly
/// ingest to modify its used data and running

// thread
// updater
// data

struct NucleusData {
    // name: String,
    data: HashMap<name, dyn DataTrait>>,
    runners: HashMap<u16, LocalRunner>,
}

pub struct LocalRunner {
    thread: JoinHandle,
    disable_thread: Tx<()>,
    disable_updater: Tx<u16>,
}

pub struct RemoteRunner<T> {
    disable_thread: Rx<()>,
    disable_updater: Rx<u16>,
    updaters: HashMap<u32, Box<dyn UpdaterTrait>>,
    runner: T,
}

/// so updaters and runners have IDs that are just u16s. You can create
/// const MY_UPDATER_ID: u16 = 245; to help here.
///
/// Mods can then use these IDs.

pub fn run(&self) -> {
    if disable_receive.pop() {
        std::thread::park();
    }

    if disable_updater.pop() {
        self.updaters.disable();
    }

    loop {
        runner.update();
    }
}

pub trait RemoteRunner

pub struct Nucleus {
    data: Arc<Mutex<NucleusData>>,
}

impl Nucleus {
    fn disable_thread(&mut self, thread_name: &'static str) {
        self.threads.get(thread_name)?.
    }

    /// This would load shared object files for mods into games. Mods would be
    /// just the single .so entry point. They would need to have been compiled against
    /// certain versions.
    fn load_mods(&mut self) Result {
       for mod_so in self.mods_directory.items().iter() {
            unsafe {
                let mod_lib = libloading::Library::new(mod_so)?;
                let load_mod: libloading::Symbol<unsafe extern fn(nucleus: Nucleus)> = lib.get(b"load_mod")?;
                Ok(load_mod(self.clone()))
            }
       }
    }
}

pub trait DataTrait: Sized + 'static {
    fn name() -> &'static str where Self: Sized;
    fn metadata() -> DataSettings where Self: Sized;
}

pub struct DataSettings {
    name: String,
    description: Option<String>,
    max_num_parents: Option<usize>,
}



/// Users are encouraged to put whatever they want in here. It could even
/// be sets of data. Do what you want.
/// We dont want to compose data, since that muddies the waters, so
/// enforce it via the trait.
///
/// Look at https://github.com/amethyst/specs/blob/master/src/storage/storages.rs for inspo
pub struct Data<T: DataTrait> {
    data: Arc<Mutex<T>>,
    // visibility settings
    // editing settings (by default a single editor)
    updated: bool,
    //
    // Simple shared access across threads?
}

impl<T> DataSettings for Data<T> where T: DataTrait {
    fn metadata() -> DataSettings {
        T::metadata()
    }
}

impl<T> RWData<T> where T: DataTrait {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data))
        }
    }

    /// Sit and wait for the lock until we get it.
    pub fn block_lock(&self) -> MutexGuard<T> {
        self.lock().unwrap()
    }

    /// Attempt to get the lock, if we cant, break out.
    pub fn try_lock(&self) -> Result<MutexGuard<T>> {
        self.try_lock()
    }
}

/// This data is good for config files, sprites, and other meshes.
pub struct ROData<T: DataTrait> {
    data: Arc<T>
}

impl<T> ReadOnlyData<T> where T: DataTrait {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(data)
        }
    }
}

impl<T> ReadOnlyData<T> where T: DataTrait {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

pub struct Updater<T> {
    nucleus: Nucleus,
    disable: bool,
    updater: T
}

impl<T> UpdaterTrait for Updater<T> where T: UpdaterTrait {
    fn name() -> &'static str {
        T::name()
    }

    fn register_new_data(nucleus: Nucleus) {
        T::register_new_data(nucleus);
    }

    fn new(nucleus: Nucleus) -> Self {
        Self {
            nucleus: nucleus.clone(),
            disable: false,
            updater: T::new(nucleus)
        }
    }

    fn update(&self) {
        if !self.disable {
            self.updater.update();
        }
    }
}

pub trait UpdaterTrait: 'static {
    /// Name that this updater is registered with.
    fn name() -> &'static str where Self: Sized;
    /// Its almost always better to recalculate something instead of getting
    /// it from elsewhere in memory. So, you should try to register as little
    /// data as possible. Reduce intermediate stages as much as possible.
    ///
    /// You should also be splitting out "special cases" of data like player
    /// positions from all positions. It depends on how you use the data.
    fn register_new_data(nucleus: Nucleus) where Self: Sized;
    fn new(nucleus: Nucleus) -> Self where Self: Sized;
    fn update(&self);
}


/// TODO: want some concept of a "dirty" btree map. One where we delay its rearrangement and balancing
/// until some point in the future where we determine it. Like it can bear to have a couple of
/// cycles where its not in a pristine shape. This way, we can do whatever manipulations we want on it
/// then rectify its dirty nature later when we have time to do it.
///
/// Maybe this is just kicking the can down the road? Hmmmmm.

/// Really like SPECS single vs multi threaded here https://github.com/amethyst/specs/blob/master/src/world/comp.rs#L63
///
/// TODO: entity id as u64 might not be the best thing. we want something that gives us lots of space
/// in terms of entity count (so like u48), but would also want some bits to be the mask
/// for a thread. so then, each thread gets a mask it applies to its entity_id generator.
/// Maybe this is the thread ID. it needs to be a u16, then it masks that with 48 more bits that are the entity
/// id.