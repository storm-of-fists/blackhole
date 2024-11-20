use std::{
    any::{Any, TypeId},
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, TryLockResult},
    thread::JoinHandle,
    time::Duration,
};

#[derive(Debug)]
pub enum NucleusError {
    One,
    Two,
    Three,
}

#[derive(Debug)]
pub struct State<T> {
    /// TODO(): some option for additional/customizable state metadata.
    // data: Rc<RefCell<dyn StateDataTrait>>,
    pub state: Rc<RefCell<T>>,
}

impl<T> State<T> {
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
    pub fn try_get(&self) -> Result<Ref<'_, T>, NucleusError> {
        self.state.try_borrow().map_err(|_| NucleusError::One)
    }

    /// Accessor to get a mutable reference to the state. Non-blocking since we don't
    /// want to block the updater loop.
    ///
    /// Keeping track of write access per cycle let's us see if we are accidentally
    /// overwriting any data. We could have split State into ReadState and WriteState,
    /// and only ever give out a singular WriteState. However, actual use patterns
    /// such as extension updaters blur the lines about how state may be manipulated
    /// and it's simpler to just make all state mutably accessible.
    pub fn try_get_mut(&self) -> Result<RefMut<'_, T>, NucleusError> {
        // self.current_write_accesses_in_this_cycle -= 1;
        self.state.try_borrow_mut().map_err(|_| NucleusError::One)
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

/// TODO: Write a proc macro for this. May want to disallow the use of
/// State<T> or SharedState<T> members in the struct we are impling on
/// to avoid nested state. DO allow for NucleusPtr and State/Updater registry
/// member variables.
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

pub struct SharedState<T> {
    state: Arc<Mutex<T>>,
}

impl<T> SharedState<T> {
    pub fn new(state: T) -> Self {
        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    /// Accessor to get to the internal state. Non-blocking since we don't
    /// want to block the updater loop.
    pub fn try_get(&self) -> Result<MutexGuard<'_, T>, NucleusError> {
        self.state.try_lock().map_err(|_| NucleusError::One)
    }

    /// Use a block lock for when you need to wait for some shared state to
    /// become available and you don't care to wait.
    pub fn try_blocking_get(&self) -> Result<MutexGuard<'_, T>, NucleusError> {
        self.state.lock().map_err(|_| NucleusError::One)
    }
}

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

pub trait SharedStateTrait: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T> SharedStateTrait for Arc<Mutex<T>>
where
    T: SharedStateTrait,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait UpdaterTrait: Any + std::fmt::Debug {
    /// Add any new state the updater may require in here. This function
    /// will be called for all updaters.
    #[allow(clippy::unused_variable)]
    fn add_new_state(
        _shared_state_registry: SharedState<SharedStateRegistry>,
        _thread_state_registry: State<StateRegistry>,
    ) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        Ok(())
    }

    /// Register your updater. Feel free to
    fn new(_thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized;

    /// A one time function called after all updaters have been added to
    /// the thread. This may block. All updaters will have their "first"
    /// function called before entering the update loop.
    fn first(&self, _thread: &Thread) -> Result<(), NucleusError> {
        Ok(())
    }

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state. Be careful blocking this function as it
    /// will block the thread. Only special timing control updaters should
    /// do any kind of sleeping.
    fn update(&self) -> Result<(), NucleusError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct StateRegistry {
    state: HashMap<TypeId, Box<dyn StateTrait>>,
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn add_state<T: StateTrait>(&mut self, state: T) -> Result<(), NucleusError> {
        // TODO: this changes per version of rust and the code itself.
        let type_id = state.type_id();

        if self.state.contains_key(&type_id) {
            return Err(NucleusError::One);
        }

        let state = State::new(state);

        self.state.insert(type_id, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.state.get(&type_id) else {
            return Err(NucleusError::One);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::Two);
        };

        Ok(State::from_rc(cloned_rc))
    }

    /// remove some state to disallow other updaters from acquiring it.
    pub fn remove_state<T: StateTrait>(&mut self) -> Result<State<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.state.remove(&type_id) else {
            return Err(NucleusError::One);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::Two);
        };

        Ok(State::from_rc(cloned_rc))
    }
}

type UpdaterNewFn = Box<dyn FnOnce(&Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>>;

pub enum UpdaterControlMessage {
    AddUpdaterBefore(TypeId),
    AddUpdaterAfter(TypeId),
    AddUpdaterToEnd(UpdaterNewFn),
    AddUpdaterToStart(UpdaterNewFn),
}

pub struct Thread {
    pub shared_state_registry: SharedState<SharedStateRegistry>,
    pub state_registry: State<StateRegistry>,
    pub control_message_queue: State<Vec<UpdaterControlMessage>>,
    updaters: Vec<Box<dyn UpdaterTrait>>,
}

impl Thread {
    pub fn new(nucleus: NucleusPtr) -> Result<Self, NucleusError> {
        let nucleus = nucleus.try_blocking_get()?;

        Ok(Self {
            shared_state_registry: nucleus.shared_state_registry.clone(),
            state_registry: State::new(StateRegistry::new()),
            control_message_queue: State::new(Vec::new()),
            updaters: Vec::new(),
        })
    }

    pub fn register_updater<T: UpdaterTrait>(&self) -> Result<(), NucleusError> {
        let mut control_message_queue = self.control_message_queue.try_get_mut()?;

        T::add_new_state(
            self.shared_state_registry.clone(),
            self.state_registry.clone(),
        )?;

        control_message_queue.push(UpdaterControlMessage::AddUpdaterToEnd(Box::new(T::new)));

        Ok(())
    }

    fn manage_control_messages(&mut self) -> Result<(), NucleusError> {
        let mut control_message_queue = self.control_message_queue.try_get_mut()?;

        let control_messages = std::mem::replace(&mut (*control_message_queue), Vec::new());

        for control_message in control_messages.into_iter() {
            match control_message {
                UpdaterControlMessage::AddUpdaterToEnd(new_fn) => {
                    self.updaters.push(new_fn(&self)?);
                }
                UpdaterControlMessage::AddUpdaterAfter(some_fn) => {
                    unimplemented!("TODO");
                }
                UpdaterControlMessage::AddUpdaterToStart(some_fn) => {
                    unimplemented!("TODO");
                }
                UpdaterControlMessage::AddUpdaterBefore(some_fn) => {
                    unimplemented!("TODO");
                }
            }
        }

        Ok(())
    }

    fn first(&mut self) -> Result<(), NucleusError> {
        self.manage_control_messages()?;

        let updaters = std::mem::replace(&mut self.updaters, Vec::new());

        for updater in updaters.into_iter() {
            updater.first(self)?;
            self.updaters.push(updater);
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), NucleusError> {
        self.manage_control_messages()?;

        for updater in self.updaters.iter() {
            updater.update()?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), NucleusError> {
        self.first()?;

        loop {
            self.update()?;
        }
    }
}

pub struct SharedStateRegistry {
    state_registry: HashMap<TypeId, Box<dyn SharedStateTrait>>,
}

impl SharedStateRegistry {
    pub fn new() -> Self {
        Self {
            state_registry: HashMap::new(),
        }
    }

    pub fn add_state<T: SharedStateTrait>(&mut self, state: T) -> Result<(), NucleusError> {
        // TODO: this changes per version of rust and the code itself.
        let type_id = state.type_id();

        if self.state_registry.contains_key(&type_id) {
            return Err(NucleusError::One);
        }

        let state = SharedState::new(state);

        self.state_registry
            .insert(type_id, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: SharedStateTrait>(&self) -> Result<State<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.state_registry.get(&type_id) else {
            return Err(NucleusError::One);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::Two);
        };

        Ok(State::from_rc(cloned_rc))
    }

    /// remove some state to disallow other updaters from acquiring it.
    pub fn remove_state<T: SharedStateTrait>(&mut self) -> Result<State<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.state_registry.remove(&type_id) else {
            return Err(NucleusError::One);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::Two);
        };

        Ok(State::from_rc(cloned_rc))
    }
}

pub struct Nucleus {
    pub join_handles: Vec<JoinHandle<Result<(), NucleusError>>>,
    pub shared_state_registry: SharedState<SharedStateRegistry>,
}

impl Nucleus {
    pub fn new() -> NucleusPtr {
        NucleusPtr {
            nucleus: SharedState::new(Self {
                join_handles: Vec::new(),
                shared_state_registry: SharedState::new(SharedStateRegistry::new()),
            }),
        }
    }
}

#[derive(Clone)]
pub struct NucleusPtr {
    nucleus: SharedState<Nucleus>,
}

impl Deref for NucleusPtr {
    type Target = SharedState<Nucleus>;

    fn deref(&self) -> &Self::Target {
        &self.nucleus
    }
}

// SAFETY: Guarded by Arc and Mutex.
unsafe impl Sync for NucleusPtr {}
// SAFETY: Guarded by Arc and Mutex.
unsafe impl Send for NucleusPtr {}

impl NucleusPtr {
    pub fn add_thread(
        &self,
        // name: impl Into<String>,
        thread_fn: impl FnOnce(NucleusPtr) -> Result<(), NucleusError> + Send + 'static,
    ) -> &Self {
        let nucleus_ptr = self.clone();

        if let Ok(mut nucleus) = nucleus_ptr.clone().try_blocking_get() {
            nucleus
                .join_handles
                .push(std::thread::spawn(|| thread_fn(nucleus_ptr)));
        }

        self
    }

    // remove shared state
    // get shared state

    pub fn go(&self) {
        loop {
            // Don't block on acquiring the nucleus, it isn't the worst thing to wait.
            if let Ok(mut nucleus) = self.nucleus.try_get() {
                let join_handles = std::mem::replace(&mut nucleus.join_handles, Vec::new());
                let mut complete_handles = Vec::new();

                for handle in join_handles.into_iter() {
                    println!("Checking handle: {:?}", handle);
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
            } else {
                println!("Nucleus unavailable, skipping thread completion checks.");
            }

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
