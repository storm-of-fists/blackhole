use std::{
    any::{Any, TypeId},
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    collections::{BTreeMap, HashMap},
    ops::{Deref, DerefMut},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, TryLockError, TryLockResult, atomic::AtomicUsize},
    thread::JoinHandle,
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

#[derive(Clone)]
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

pub trait StateTrait: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T> StateTrait for RefCell<T>
where
    T: StateTrait,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait UpdaterTrait: Any {
    /// Create a new updater, given the global and thread runner context
    /// pointers. Be careful to avoid recursive access to the nucleus or
    /// runner due to the potential for lockups.
    fn new(nucleus: NucleusPtr, runner: &mut Runner) -> Self
    where
        Self: Sized;

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state.
    fn update(&self) {}
}

pub trait MetaUpdaterTrait: UpdaterTrait {}

pub struct Runner {
    nucleus: NucleusPtr,
    state: HashMap<TypeId, Rc<dyn StateTrait>>,
    active_updaters: BTreeMap<TypeId, Box<dyn UpdaterTrait>>,
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
            state: HashMap::new(),
            active_updaters: BTreeMap::new(),
            inactive_updaters: BTreeMap::new(),
            active_meta_updaters: BTreeMap::new(),
            inactive_meta_updaters: BTreeMap::new(),
        }
    }

    /// Add an updater to this runner. Pass the runner as a &mut to avoid circular
    /// locks one would encounter passing the RunnerPtr.
    pub fn add_updater(&mut self, updater_fn: impl FnOnce(NucleusPtr, &mut Runner)) {
        updater_fn(self.nucleus.clone(), self)
    }

    pub fn add_state<T: StateTrait>(&mut self, state: T) -> Result<State<T>, StateError> {
        let type_id = state.type_id();

        if self.state.contains_key(&type_id) {
            return Err(StateError::One);
        }

        let state = State::new(state);

        self.state.insert(type_id, state.state.clone());

        Ok(state)
    }

    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, StateError> {
        let type_id = TypeId::of::<T>();

        Ok(State::from_rc(
            self
                .state
                .get(&type_id)
                .cloned()
                .unwrap()
                .as_any()
                .downcast_ref::<Rc<RefCell<T>>>()
                .cloned()
                .unwrap(),
        ))
    }
}

#[derive(Clone)]
pub struct RunnerPtr {
    runner: Rc<RefCell<Runner>>,
}

impl Deref for RunnerPtr {
    type Target = Rc<RefCell<Runner>>;

    fn deref(&self) -> &Self::Target {
        &self.runner
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
