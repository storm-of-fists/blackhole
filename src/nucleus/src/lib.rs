use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

// These derive names don't conflict with the trait names? Nice.
pub use nucleus_macros::{SharedStateTrait, StateTrait};

#[derive(Debug)]
pub enum NucleusError {
    GetState,
    GetStateBlocking,
    AddNewState,
    NewUpdater,
    UpdaterFirst,
    UpdaterUpdate,
    AddState,
    StateExists,
    CouldNotCastState,
    StateDoesNotExist,
    RemoveState,
    AddSharedState,
    RemoveSharedState,
    ControlUpdater,
    ThreadErrored,
}

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
    pub fn get(&self) -> Result<Ref<'_, T>, NucleusError> {
        self.state.try_borrow().map_err(|_| NucleusError::GetState)
    }

    /// Accessor to get a mutable reference to the state. Non-blocking since we don't
    /// want to block the updater loop.
    ///
    /// Keeping track of write access per cycle let's us see if we are accidentally
    /// overwriting any data. We could have split State into ReadState and WriteState,
    /// and only ever give out a singular WriteState. However, actual use patterns
    /// such as extension updaters blur the lines about how state may be manipulated
    /// and it's simpler to just make all state mutably accessible.
    pub fn get_mut(&self) -> Result<RefMut<'_, T>, NucleusError> {
        // self.current_write_accesses_in_this_cycle -= 1;
        self.state
            .try_borrow_mut()
            .map_err(|_| NucleusError::GetState)
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

pub trait StateTrait: Any {
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

    pub fn from_arc(state: Arc<Mutex<T>>) -> Self {
        Self { state }
    }

    /// Accessor to get to the internal state. Non-blocking since we don't
    /// want to block the updater loop.
    pub fn get(&self) -> Result<MutexGuard<'_, T>, NucleusError> {
        self.state.try_lock().map_err(|_| NucleusError::GetState)
    }

    /// Use a block lock for when you need to wait for some shared state to
    /// become available and you don't care to wait.
    pub fn blocking_get(&self) -> Result<MutexGuard<'_, T>, NucleusError> {
        self.state
            .lock()
            .map_err(|_| NucleusError::GetStateBlocking)
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

unsafe impl<T> Send for SharedState<T> {}
unsafe impl<T> Sync for SharedState<T> {}

#[macro_export]
macro_rules! wait_for_shared_state {
    ($shared_state:expr, $($state_type:ident)*, $check_interval:expr) => {
        let mut all_exist = true;

        loop {
            $(all_exist &= $shared_state.state_exists::<$state_type>();)*

            if all_exist {
                break;
            }

            std::thread::sleep($check_interval);
        }
    };

    ($shared_state:expr, $($state_type:ident)*, $check_interval:expr, $total_wait_duration:expr) => {
        let mut all_exist = true;
        let start_instant = std::time::Instant::now();

        loop {
            if start_instant.elapsed() > $total_wait_duration {
                return Err(NucleusError::StateDoesNotExist)
            }
            $(all_exist &= $shared_state.state_exists::<$state_type>();)*

            if all_exist {
                break;
            }

            std::thread::sleep($check_interval);
        }
    };
}

pub struct LocalStore {
    store: HashMap<TypeId, Box<dyn StateTrait>>,
}

impl LocalStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// This doesn't currently have a way to error, but I'd like to not change the
    /// API in the future if there is.
    pub fn add_state<T: StateTrait>(&mut self, state: T) -> Result<(), NucleusError> {
        // TODO: this changes per version of rust and the code itself.
        let type_id = state.type_id();

        // If someone wants to error because the state exists already, they can
        // do so explicitly.
        if self.state_exists::<T>() {
            return Ok(());
        }

        let state = State::new(state);

        self.store.insert(type_id, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.store.get(&type_id) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(State::from_rc(cloned_rc))
    }

    /// remove some state to disallow other updaters from acquiring it.
    pub fn remove_state<T: StateTrait>(&mut self) -> Result<State<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.store.remove(&type_id) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(State::from_rc(cloned_rc))
    }

    pub fn state_exists<T: StateTrait>(&self) -> bool {
        self.store.contains_key(&TypeId::of::<T>())
    }
}

pub struct SharedStore {
    store: HashMap<TypeId, Box<dyn SharedStateTrait>>,
}

impl SharedStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn add_state<T: SharedStateTrait>(&mut self, state: T) -> Result<(), NucleusError> {
        // TODO: this changes per version of rust and the code itself.
        let type_id = state.type_id();

        if self.store.contains_key(&type_id) {
            return Err(NucleusError::StateExists);
        }

        let state = SharedState::new(state);

        self.store.insert(type_id, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: SharedStateTrait>(&self) -> Result<SharedState<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.store.get(&type_id) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_arc) = boxed_state
            .as_any()
            .downcast_ref::<Arc<Mutex<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(SharedState::from_arc(cloned_arc))
    }

    /// remove some state to disallow other updaters from acquiring it.
    pub fn remove_state<T: SharedStateTrait>(&mut self) -> Result<SharedState<T>, NucleusError> {
        let type_id = TypeId::of::<T>();

        let Some(boxed_state) = self.store.remove(&type_id) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_arc) = boxed_state
            .as_any()
            .downcast_ref::<Arc<Mutex<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(SharedState::from_arc(cloned_arc))
    }

    pub fn state_exists<T: SharedStateTrait>(&self) -> bool {
        self.store.contains_key(&TypeId::of::<T>())
    }
}

pub struct StateStore {
    pub shared: SharedState<SharedStore>,
    pub local: State<LocalStore>,
}

impl StateStore {
    pub fn new(shared_state: SharedState<SharedStore>) -> Self {
        Self {
            shared: shared_state,
            local: State::new(LocalStore::new()),
        }
    }
}

pub trait UpdaterTrait: Any {
    /// Add any new state the updater may require in here. This function
    /// will be called for all updaters.
    fn add_new_state(_state: &StateStore) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        Ok(())
    }

    /// This function tells the system whether the updater should or should
    /// not be used. You can do this based on state that exists.
    /// TODO: delete
    fn should_create(_thread: &Thread) -> bool
    where
        Self: Sized,
    {
        true
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

    /// Function called when updater is removed.
    fn remove(&self) -> Result<(), NucleusError> {
        Ok(())
    }

    /// what to run on app error and exit.
    fn on_exit(&self) -> Result<(), NucleusError> {
        Ok(())
    }
}

type UpdaterNewFn = Box<dyn FnOnce(&Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>>;

pub enum UpdaterControlMessage {
    AddUpdaterBefore(TypeId, TypeId),
    AddUpdaterAfter(TypeId, TypeId),
    AddUpdaterToEnd(UpdaterNewFn),
    AddUpdaterToStart(UpdaterNewFn),
    RemoveUpdater(TypeId),
}

#[derive(StateTrait)]
pub struct UpdaterControl {
    pub message_queue: Vec<UpdaterControlMessage>,
}

impl UpdaterControl {
    pub fn new() -> Self {
        Self {
            message_queue: Vec::new(),
        }
    }
}

pub struct UpdaterStore {
    list: Vec<Box<dyn UpdaterTrait>>,
    control: State<UpdaterControl>,
}

impl UpdaterStore {
    pub fn new(state: &StateStore) -> Result<Self, NucleusError> {
        let mut local_state = state.local.get_mut()?;
        local_state.add_state(UpdaterControl::new());

        Ok(Self {
            list: Vec::new(),
            control: local_state.get_state::<UpdaterControl>()?,
        })
    }

    pub fn add_updater_to_start(
        &mut self,
        updater: Box<dyn UpdaterTrait>,
    ) -> Result<(), NucleusError> {
        self.list.insert(0, updater);

        Ok(())
    }

    pub fn add_updater_to_end(
        &mut self,
        updater: Box<dyn UpdaterTrait>,
    ) -> Result<(), NucleusError> {
        self.list.push(updater);

        Ok(())
    }

    pub fn move_updater_before_other(
        &mut self,
        before_updater_type_id: TypeId,
        move_updater_type_id: TypeId,
    ) -> Result<(), NucleusError> {
        let mut maybe_before_index = None;
        let mut maybe_move_index = None;

        for (index, updater) in self.list.iter().enumerate() {
            if maybe_move_index.is_some() && maybe_before_index.is_some() {
                break;
            }

            if (**updater).type_id() == before_updater_type_id {
                maybe_before_index = Some(index);
            }

            if (**updater).type_id() == move_updater_type_id {
                maybe_move_index = Some(index);
            }
        }

        if let Some(before_index) = maybe_before_index {
            if let Some(move_index) = maybe_move_index {
                let move_updater = self.list.remove(move_index);

                if before_index == 0 {
                    self.list.insert(0, move_updater);
                } else {
                    self.list.insert(before_index - 1, move_updater);
                }
            }
        }

        Ok(())
    }

    pub fn move_updater_after_other(
        &mut self,
        after_updater_type_id: TypeId,
        move_updater_type_id: TypeId,
    ) -> Result<(), NucleusError> {
        let mut maybe_after_index = None;
        let mut maybe_move_index = None;

        for (index, updater) in self.list.iter().enumerate() {
            if maybe_move_index.is_some() && maybe_after_index.is_some() {
                break;
            }

            if (**updater).type_id() == after_updater_type_id {
                maybe_after_index = Some(index);
            }

            if (**updater).type_id() == move_updater_type_id {
                maybe_move_index = Some(index);
            }
        }

        if let Some(before_index) = maybe_after_index {
            if let Some(move_index) = maybe_move_index {
                let move_updater = self.list.remove(move_index);

                self.list.insert(before_index + 1, move_updater);
            }
        }

        Ok(())
    }

    pub fn remove_updater(&mut self, type_id: TypeId) -> Result<(), NucleusError> {
        let mut maybe_index = None;

        for (index, updater) in self.list.iter().enumerate() {
            if (**updater).type_id() == type_id {
                maybe_index = Some(index);
                break;
            }
        }

        if let Some(index) = maybe_index {
            let updater = self.list.remove(index);

            updater.remove()?;
        }

        Ok(())
    }
}

pub struct Thread {
    pub state: StateStore,
    pub updaters: UpdaterStore,
}

impl Thread {
    pub fn new(shared_state: SharedState<SharedStore>) -> Result<Self, NucleusError> {
        let state = StateStore::new(shared_state);
        let updaters = UpdaterStore::new(&state)?;

        Ok(Self { state, updaters })
    }

    pub fn add_updater<T: UpdaterTrait>(&self) -> Result<(), NucleusError> {
        let mut updater_control = self.updaters.control.get_mut()?;

        T::add_new_state(&self.state)?;

        updater_control
            .message_queue
            .push(UpdaterControlMessage::AddUpdaterToEnd(Box::new(T::new)));

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), NucleusError> {
        self.first()?;

        loop {
            self.update()?;
        }
    }

    /// I'd honestly like if this fit into the updater/state scheme, but
    /// we would run into double mutable access during the update loop.
    /// As in, if we had updaters = State<Updaters>, then any time we
    /// loop, we would do updaters.get()?, then iterate over them and call "update".
    /// But any updater attempting to mutate updaters would then do updaters.get_mut()
    /// itself and hit the double access.
    fn manage_control_messages(&mut self) -> Result<(), NucleusError> {
        let mut updater_control = self.updaters.control.get_mut()?;

        let control_messages = std::mem::replace(&mut updater_control.message_queue, Vec::new());

        drop(updater_control);

        for control_message in control_messages.into_iter() {
            match control_message {
                UpdaterControlMessage::AddUpdaterToStart(new_fn) => {
                    self.updaters.add_updater_to_start(new_fn(&self)?);
                }
                UpdaterControlMessage::AddUpdaterToEnd(new_fn) => {
                    self.updaters.add_updater_to_end(new_fn(&self)?);
                }
                UpdaterControlMessage::AddUpdaterBefore(
                    move_updater_type_id,
                    before_updater_type_id,
                ) => {
                    self.updaters
                        .move_updater_before_other(before_updater_type_id, move_updater_type_id)?;
                }
                UpdaterControlMessage::AddUpdaterAfter(
                    move_updater_type_id,
                    after_updater_type_id,
                ) => {
                    self.updaters
                        .move_updater_after_other(after_updater_type_id, move_updater_type_id)?;
                }
                UpdaterControlMessage::RemoveUpdater(type_id) => {
                    self.updaters.remove_updater(type_id)?;
                }
            }
        }

        Ok(())
    }

    fn first(&mut self) -> Result<(), NucleusError> {
        self.manage_control_messages()?;

        let updaters_len = self.updaters.list.len();

        let updaters = std::mem::replace(&mut self.updaters.list, Vec::with_capacity(updaters_len));

        for updater in updaters.into_iter() {
            updater.first(self)?;
            self.updaters.add_updater_to_end(updater);
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), NucleusError> {
        self.manage_control_messages()?;

        for updater in self.updaters.list.iter() {
            // TODO: Maybe we should have some State that collects errors and a default Updater
            // that crashes out, but could be replaced/removed.
            // TOOD: maybe if the updater errors, we simply remove that updater from the list?
            // Then we can add other updaters to manage how we fix the issue.
            updater.update()?;
        }

        Ok(())
    }
}

pub struct Nucleus {
    pub thread: Thread,
    pub shared_state: SharedState<SharedStore>,
}

impl Nucleus {
    pub fn new() -> Result<Self, NucleusError> {
        let shared_state = SharedState::new(SharedStore::new());
        let nucleus = Self {
            thread: Thread::new(shared_state.clone())?,
            shared_state,
        };

        Ok(nucleus)
    }
}

impl Deref for Nucleus {
    type Target = Thread;

    fn deref(&self) -> &Self::Target {
        &self.thread
    }
}

impl DerefMut for Nucleus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thread
    }
}
