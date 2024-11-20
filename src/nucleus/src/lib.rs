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

#[derive(Clone)]
pub struct SharedState<T>
where
    T: Any,
{
    state: Arc<Mutex<T>>,
}

impl<T> SharedState<T>
where
    T: Any,
{
    pub fn new(state: T) -> Self {
        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    /// Accessor to get to the internal state. Non-blocking since we don't
    /// want to block the updater loop.
    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
        self.state.try_lock()
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

#[derive(Default)]
pub enum UpdaterOrdering {
    AddToFront,
    #[default]
    AddToEnd,
    Before(TypeId),
    After(TypeId),
}

pub trait UpdaterTrait: Any + std::fmt::Debug {
    /// Add any new state the updater may require in here. This function
    /// will be called for all updaters.
    #[allow(clippy::unused_variable)]
    fn add_new_state(_nucleus: NucleusPtr, _state: State<StateRegistry>) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        Ok(())
    }

    /// Register your updater. Feel free to
    fn new(_nucleus: NucleusPtr, _thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized;

    /// A one time function called after all updaters have been added to
    /// the thread. This may block. All updaters will have their "first"
    /// function called before entering the update loop.
    fn first(&self, _nucleus: NucleusPtr, _thread: &Thread) -> Result<(), NucleusError> {
        Ok(())
    }

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state. Be careful blocking this function as it
    /// will block the thread. Only special timing control updaters should
    /// do any kind of sleeping.
    fn update(&self) -> Result<(), NucleusError> {
        Ok(())
    }

    // / This is a method and not just associated so that it can change at runtime.
    // / A meta updater will still need to perform the order change.
    // fn ordering(&self) -> UpdaterOrdering {
    //     UpdaterOrdering::default()
    // }
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

type UpdaterNewFn =
    Box<dyn FnOnce(NucleusPtr, &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>>;

pub enum ControlMessage {
    AddUpdaterBefore(TypeId),
    AddUpdaterAfter(TypeId),
    AddUpdaterToEnd(UpdaterNewFn),
    AddUpdaterToStart(UpdaterNewFn),
    // AddState(TypeId, Box<dyn FnOnce())
    // RemoveState(TypeId, )
}

pub struct Thread {
    nucleus: NucleusPtr,
    state: State<StateRegistry>,
    control_message_queue: State<Vec<ControlMessage>>,
    updaters: Vec<Box<dyn UpdaterTrait>>,
}

impl Thread {
    pub fn new(nucleus: NucleusPtr) -> Self {
        Self {
            nucleus,
            state: State::new(StateRegistry::new()),
            control_message_queue: State::new(Vec::new()),
            updaters: Vec::new(),
        }
    }

    pub fn register_updater<T: UpdaterTrait>(&self) -> Result<(), NucleusError> {
        let mut control_message_queue = self.control_message_queue.try_get_mut().unwrap();

        T::add_new_state(self.nucleus.clone(), self.state.clone())?;

        control_message_queue.push(ControlMessage::AddUpdaterToEnd(Box::new(T::new)));

        Ok(())
    }

    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, NucleusError> {
        let state_registry = self.state.try_get().unwrap();

        state_registry.get_state::<T>()
    }

    pub fn add_state<T: StateTrait>(&self, state: T) -> Result<(), NucleusError> {
        let mut state_registry = self.state.try_get_mut().unwrap();

        state_registry.add_state(state)
    }

    pub fn remove_state<T: StateTrait>(&self) -> Result<State<T>, NucleusError> {
        let mut state_registry = self.state.try_get_mut().unwrap();

        state_registry.remove_state::<T>()
    }

    fn manage_control_messages(&mut self) -> Result<(), NucleusError> {
        let mut control_message_queue = self.control_message_queue.try_get_mut().unwrap();

        let control_messages = std::mem::replace(&mut (*control_message_queue), Vec::new());

        for control_message in control_messages.into_iter() {
            match control_message {
                ControlMessage::AddUpdaterToEnd(new_fn) => {
                    self.updaters.push(new_fn(self.nucleus.clone(), &self)?);
                }
                ControlMessage::AddUpdaterAfter(some_fn) => {
                    unimplemented!("TODO");
                }
                ControlMessage::AddUpdaterToStart(some_fn) => {
                    unimplemented!("TODO");
                }
                ControlMessage::AddUpdaterBefore(some_fn) => {
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
            updater.first(self.nucleus.clone(), self)?;
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

pub struct Nucleus {
    pub join_handles: Vec<JoinHandle<Result<(), NucleusError>>>,
    pub shared_state: HashMap<TypeId, Box<dyn SharedStateTrait>>,
}

impl Nucleus {
    pub fn new() -> NucleusPtr {
        NucleusPtr {
            nucleus: Arc::new(Mutex::new(Self {
                join_handles: Vec::new(),
                shared_state: HashMap::new(),
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
    pub fn add_thread(
        &self,
        // name: impl Into<String>,
        thread_fn: impl FnOnce(NucleusPtr) -> Result<(), NucleusError> + Send + 'static,
    ) -> &Self {
        let nucleus_ptr = self.clone();

        if let Ok(mut nucleus) = nucleus_ptr.clone().lock() {
            nucleus
                .join_handles
                .push(std::thread::spawn(|| thread_fn(nucleus_ptr)));
        }

        self
    }

    pub fn add_shared_state<T: SharedStateTrait>(&self, state: T) -> Result<(), NucleusError> {
        if let Ok(mut nucleus) = self.clone().lock() {
            nucleus
                .shared_state
                .insert(TypeId::of::<T>(), Box::new(state));
        }

        Ok(())
    }

    // remove shared state
    // get shared state

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

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
