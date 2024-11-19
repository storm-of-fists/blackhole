use std::{
    any::{Any, TypeId},
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    collections::{BTreeMap, HashMap},
    ops::{Deref, DerefMut},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, TryLockError, TryLockResult, atomic::AtomicUsize},
    thread::JoinHandle,
    time::Duration,
};

// #[derive(Clone)]
// pub struct SharedState<T>
// where
//     T: Any,
// {
//     state: Arc<Mutex<T>>,
// }

// impl<T> SharedState<T>
// where
//     T: Any,
// {
//     pub fn new(state: T) -> Self {
//         Self {
//             state: Arc::new(Mutex::new(state)),
//         }
//     }

//     /// Accessor to get to the internal state. Non-blocking since we don't
//     /// want to block the updater loop.
//     pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
//         self.state.try_lock()
//     }
// }

// pub trait SharedStateTrait: Sized + Any {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

/// This struct helps us see how often some state is being accessed.
// pub struct InnerState<T> {
//     mut_access_count: usize,
//     state: T,
// }

#[derive(Clone, Debug)]
pub struct State<T>
where
    T: StateTrait,
{
    /// TODO(): some option for additional/customizable state metadata.
    // data: Rc<RefCell<dyn StateDataTrait>>,
    pub state: Rc<RefCell<T>>,
}

impl<T> State<T>
where
    T: StateTrait,
{
    pub fn new(state: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(state)),
        }
    }

    pub fn from_rc(rc_state: Rc<RefCell<T>>) -> Self {
        Self { state: rc_state }
    }

    /// Accessor to get a reference to the state. Non-blocking since we don't
    /// want to block the updater loop.
    pub fn try_get(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.state.try_borrow()
    }

    /// Accessor to get a mutable reference to the state. Non-blocking since we don't
    /// want to block the updater loop.
    ///
    /// Keeping track of write access per cycle let's us see if we are accidentally
    /// overwriting any data. We could have split State into ReadState and WriteState,
    /// and only ever give out a singular WriteState. However, actual use patterns
    /// such as extension updaters blur the lines about how state may be manipulated
    /// and it's simpler to just make all state mutably accessible.
    pub fn try_get_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        // self.current_write_accesses_in_this_cycle -= 1;
        self.state.try_borrow_mut()
    }
}

pub trait StateTrait: std::fmt::Debug + Any {
    fn as_any(&self) -> &dyn Any;
}


impl<T> StateTrait for Rc<RefCell<T>>
where
    T: StateTrait,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait UpdaterTrait: Any {
    /// Add any new state the updater may require in here. This function
    /// will be called for all updaters.
    #[allow(clippy::unused_variable)]
    fn add_new_state(_nucleus: NucleusPtr, _runner: &mut Runner)
    where
        Self: Sized,
    {
    }

    /// Register your updater. Feel free to
    fn register(nucleus: NucleusPtr, runner: &mut Runner)
    where
        Self: Sized;

    /// A one time function called after all updaters have been added to
    /// the runner. This may block. All updaters will have their "first"
    /// function called before entering the update loop.
    fn first(&self, _nucleus: NucleusPtr, _runner: &mut Runner) {}

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state. Be careful blocking this function as it
    /// will block the thread. Only special timing control updaters should
    /// do any kind of sleeping.
    fn update(&self) {}
}

pub trait MetaUpdaterTrait: UpdaterTrait {}

pub struct Runner {
    nucleus: NucleusPtr,
    piss: Vec<Box<dyn FnOnce(NucleusPtr, &mut Runner)>>,
    state: HashMap<TypeId, Box<dyn StateTrait>>,
    pub active_updaters: BTreeMap<TypeId, Box<dyn UpdaterTrait>>,
    inactive_updaters: BTreeMap<TypeId, Box<dyn UpdaterTrait>>,
    active_meta_updaters: BTreeMap<TypeId, Box<dyn MetaUpdaterTrait>>,
    inactive_meta_updaters: BTreeMap<TypeId, Box<dyn MetaUpdaterTrait>>,
}

#[derive(Debug)]
pub enum StateError {
    One,
    Two,
    Three,
}

impl Runner {
    pub fn new(nucleus: NucleusPtr) -> Self {
        Self {
            nucleus,
            piss: Vec::new(),
            state: HashMap::new(),
            active_updaters: BTreeMap::new(),
            inactive_updaters: BTreeMap::new(),
            active_meta_updaters: BTreeMap::new(),
            inactive_meta_updaters: BTreeMap::new(),
        }
    }

    /// Add an updater to this runner.
    pub fn add_updater<T: UpdaterTrait>(&mut self) {
        // Add new state, but delay the registration until later to reduce annoyingness.
        T::add_new_state(self.nucleus.clone(), self);
        self.piss.push(Box::new(T::register));
    }

    pub fn add_state<T: StateTrait>(&mut self, state: T) -> Result<(), StateError> {
        let type_id = state.type_id();

        if self.state.contains_key(&type_id) {
            return Err(StateError::One);
        }

        let state = State::new(state);

        self.state.insert(type_id, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, StateError> {
        let type_id = TypeId::of::<T>();

        Ok(State::from_rc(
            self.state
                .get(&type_id)
                .unwrap()
                .as_any()
                .downcast_ref::<Rc<RefCell<T>>>()
                .cloned()
                .unwrap(),
        ))
    }

    fn first(&mut self) {
        let register_updaters = std::mem::replace(&mut self.piss, Vec::new());

        for register_updater in register_updaters.into_iter() {
            register_updater(self.nucleus.clone(), self);
        }

        let active_updaters = std::mem::replace(&mut self.active_updaters, BTreeMap::new());

        for (type_id, updater) in active_updaters.into_iter() {
            updater.first(self.nucleus.clone(), self);
            self.active_updaters.insert(type_id, updater);
        }
    }

    pub fn run(&mut self) {
        self.first();

        loop {
            for active_updater in self.active_updaters.values_mut() {
                active_updater.update();
            }
        }
    }
}

pub enum RunnerError {
    One,
    Two,
    Three,
}

pub struct Nucleus {
    pub join_handles: Vec<JoinHandle<Result<(), RunnerError>>>,
    pub pending_updater_functions: Vec<Box<dyn FnOnce(NucleusPtr)>>,
    // pub shared_state: HashMap<TypeId, Box<dyn SharedStateTrait>>,
}

impl Nucleus {
    pub fn new() -> NucleusPtr {
        NucleusPtr {
            nucleus: Arc::new(Mutex::new(Self {
                join_handles: Vec::new(),
                pending_updater_functions: Vec::new(),
            })),
        }
    }
}

#[derive(Clone)]
pub struct NucleusPtr {
    nucleus: Arc<Mutex<Nucleus>>,
}

impl Deref for NucleusPtr {
    type Target = Arc<Mutex<Nucleus>>;

    fn deref(&self) -> &Self::Target {
        &self.nucleus
    }
}

// SAFETY: Guarded by Arc and Mutex.
unsafe impl Sync for NucleusPtr {}
// SAFETY: Guarded by Arc and Mutex.
unsafe impl Send for NucleusPtr {}

impl NucleusPtr {
    pub fn add_runner(
        &self,
        // name: impl Into<String>,
        runner_fn: impl FnOnce(NucleusPtr) -> Result<(), RunnerError> + Send + 'static,
    ) -> &Self {
        let nucleus_ptr = self.clone();

        if let Ok(mut nucleus) = nucleus_ptr.clone().lock() {
            nucleus
                .join_handles
                .push(std::thread::spawn(|| runner_fn(nucleus_ptr)));
        }

        self
    }

    pub fn go(&self) {
        loop {
            // probably want a try lock here
            if let Ok(mut nucleus) = self.nucleus.lock() {
                let join_handles = std::mem::replace(&mut nucleus.join_handles, Vec::new());
                let mut complete_handles = Vec::new();

                for handle in join_handles.into_iter() {
                    if handle.is_finished() {
                        complete_handles.push(handle);
                    } else {
                        nucleus.join_handles.push(handle);
                    }
                }

                // Exit if any of the threads have errored.
                for complete_handle in complete_handles.into_iter() {
                    if complete_handle.join().is_err() {
                        return;
                    }
                }
            }

            // TODO: do something with pending updaters?

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
