use std::{
    any::{type_name, type_name_of_val, Any},
    cell::{RefCell, RefMut},
    collections::HashMap,
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
    NewDoer,
    DoerFirst,
    DoerUpdate,
    AddState,
    StateExists,
    CouldNotCastState,
    StateDoesNotExist,
    RemoveState,
    AddSharedState,
    RemoveSharedState,
    ControlDoer,
    ThreadErrored,
}

/// This is any state local to the thread.
pub struct State<T> {
    /// TODO: may not want a RefCell, could use something simpler since we
    /// dont care to differentiate between read only or write only.
    ///
    /// pub struct State<T> {
    ///     state: UnsafeCell<T>,
    ///     is_borrowed: Cell<bool>,
    /// }
    state: Rc<RefCell<T>>,
}

impl<T> State<T> {
    pub fn new(state: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(state)),
        }
    }

    /// Accessor to get a mutable reference to the state. Non-blocking since we don't
    /// want to block the doer loop. The reason I just defaulted to this being mut
    /// is because keeping track of if State is only being accessed via
    /// Ref or RefMut is not tenable for a big enough program. Instead, I'd rather
    /// get users into the habit of dropping the RefMut as often as possible.
    ///
    /// We could have split State into ReadState and WriteState, and only ever
    /// give out a singular WriteState. However, that makes it impossible for
    /// additional Doers to mutate state. I'm thinking of dynamically loaded libraries
    /// that add doers to an existing program.
    pub fn get(&self) -> Result<RefMut<'_, T>, NucleusError> {
        self.state
            .try_borrow_mut()
            .map_err(|_| NucleusError::GetState)
    }
}

impl<T> From<Rc<RefCell<T>>> for State<T>
where
    T: StateTrait,
{
    fn from(state: Rc<RefCell<T>>) -> Self {
        Self { state }
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

    /// Accessor to get to the internal state. Non-blocking since we don't
    /// want to block the doer loop.
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

impl<T> From<Arc<Mutex<T>>> for SharedState<T>
where
    T: SharedStateTrait,
{
    fn from(state: Arc<Mutex<T>>) -> Self {
        Self { state }
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
macro_rules! shared_state_wait {
    ($shared_state:expr, $($state_type:ident)+; $check_interval:expr, $total_wait_duration:expr) => {
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
    store: HashMap<&'static str, Box<dyn StateTrait>>,
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
        // If someone wants to error because the state exists already, they can
        // do so explicitly.
        if self.state_exists::<T>() {
            return Ok(());
        }

        let state = State::new(state);

        self.store
            .insert(type_name::<T>(), Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, NucleusError> {
        let Some(boxed_state) = self.store.get(type_name::<T>()) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(State::from(cloned_rc))
    }

    /// remove some state to disallow other doers from acquiring it.
    pub fn remove_state<T: StateTrait>(&mut self) -> Result<State<T>, NucleusError> {
        let Some(boxed_state) = self.store.remove(type_name::<T>()) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<RefCell<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(State::from(cloned_rc))
    }

    pub fn state_exists<T: StateTrait>(&self) -> bool {
        self.store.contains_key(type_name::<T>())
    }
}

pub struct SharedStore {
    store: HashMap<&'static str, Box<dyn SharedStateTrait>>,
}

impl SharedStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn add_state<T: SharedStateTrait>(&mut self, state: T) -> Result<(), NucleusError> {
        let type_name = type_name::<T>();

        if self.store.contains_key(type_name) {
            return Err(NucleusError::StateExists);
        }

        let state = SharedState::new(state);

        self.store.insert(type_name, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: SharedStateTrait>(&self) -> Result<SharedState<T>, NucleusError> {
        let Some(boxed_state) = self.store.get(type_name::<T>()) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_arc) = boxed_state
            .as_any()
            .downcast_ref::<Arc<Mutex<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(SharedState::from(cloned_arc))
    }

    /// remove some state to disallow other doers from acquiring it.
    pub fn remove_state<T: SharedStateTrait>(&mut self) -> Result<SharedState<T>, NucleusError> {
        let Some(boxed_state) = self.store.remove(type_name::<T>()) else {
            return Err(NucleusError::StateDoesNotExist);
        };

        let Some(cloned_arc) = boxed_state
            .as_any()
            .downcast_ref::<Arc<Mutex<T>>>()
            .cloned()
        else {
            return Err(NucleusError::CouldNotCastState);
        };

        Ok(SharedState::from(cloned_arc))
    }

    pub fn state_exists<T: SharedStateTrait>(&self) -> bool {
        self.store.contains_key(type_name::<T>())
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

pub trait DoerTrait: Any {
    /// Add any new state the doer may require in here. This function
    /// will be called for all doers.
    fn new_state(_state: &StateStore) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        Ok(())
    }

    /// Register your doer. Feel free to
    fn new(_nucleus: &Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>
    where
        Self: Sized;

    /// A one time function called after all doers have been added to
    /// the nucleus. This may block. All doers will have their "first"
    /// function called before entering the update loop.
    fn first(&self, _nucleus: &Nucleus) -> Result<(), NucleusError> {
        Ok(())
    }

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state. Be careful blocking this function as it
    /// will block the nucleus. Only special timing control doers should
    /// do any kind of sleeping.
    fn update(&self) -> Result<(), NucleusError> {
        Ok(())
    }

    /// Function called when doer is removed. If you need to differentiate
    /// about why the doer is being removed, you'll need to add that
    /// state yourself (such as if the whole program is exiting).
    fn remove(&self) -> Result<(), NucleusError> {
        Ok(())
    }
}

type DoerNewFn = Box<dyn FnOnce(&Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>>;

pub enum DoerControlMessage {
    AddDoerBefore(&'static str, &'static str),
    AddDoerAfter(&'static str, &'static str),
    AddDoerToEnd(DoerNewFn),
    AddDoerToStart(DoerNewFn),
    RemoveDoer(&'static str),
}

#[derive(StateTrait)]
pub struct DoerState {
    /// The list of messages
    pub message_queue: Vec<DoerControlMessage>,
    /// The list of inactive Doers. A Doer can become inactive if it errors or
    pub inactive: Vec<Box<dyn DoerTrait>>,
}

impl DoerState {
    pub fn new() -> Self {
        Self {
            message_queue: Vec::new(),
            inactive: Vec::new(),
        }
    }
}

pub struct DoerStore {
    /// The list of active Doers. If any error, they are put into inactive.
    /// This isn't in the StateStore because we need to access it multiple times
    /// to update over it (also why UpdaterTrait::update is &self). If users need
    /// to manipulate this list, they can access DoerState's message_queue.
    pub active: Vec<Box<dyn DoerTrait>>,
    /// This state is special. It does get put into the LocalStore, but
    /// it is also kept here so other Doers don't need to query the LocalStore
    /// in their methods, they can just access it via the &Nucleus.
    pub state: State<DoerState>,
}

impl DoerStore {
    pub fn new(state: &StateStore) -> Result<Self, NucleusError> {
        let mut local_state = state.local.get()?;

        local_state.add_state(DoerState::new())?;

        Ok(Self {
            active: Vec::new(),
            state: local_state.get_state::<DoerState>()?,
        })
    }

    pub fn add_doer_to_start(&mut self, doer: Box<dyn DoerTrait>) -> Result<(), NucleusError> {
        self.active.insert(0, doer);

        Ok(())
    }

    pub fn add_doer_to_end(&mut self, doer: Box<dyn DoerTrait>) -> Result<(), NucleusError> {
        self.active.push(doer);

        Ok(())
    }

    pub fn move_doer_before_other(
        &mut self,
        before_doer: &'static str,
        move_doer: &'static str,
    ) -> Result<(), NucleusError> {
        let mut maybe_before_index = None;
        let mut maybe_move_index = None;

        for (index, doer) in self.active.iter().enumerate() {
            if maybe_move_index.is_some() && maybe_before_index.is_some() {
                break;
            }

            if type_name_of_val(&(**doer)) == before_doer {
                maybe_before_index = Some(index);
            }

            if type_name_of_val(&(**doer)) == move_doer {
                maybe_move_index = Some(index);
            }
        }

        if let Some(before_index) = maybe_before_index {
            if let Some(move_index) = maybe_move_index {
                let move_doer = self.active.remove(move_index);

                if before_index == 0 {
                    self.active.insert(0, move_doer);
                } else {
                    self.active.insert(before_index - 1, move_doer);
                }
            }
        }

        Ok(())
    }

    pub fn move_doer_after_other(
        &mut self,
        after_doer: &'static str,
        move_doer: &'static str,
    ) -> Result<(), NucleusError> {
        let mut maybe_after_index = None;
        let mut maybe_move_index = None;

        for (index, doer) in self.active.iter().enumerate() {
            if maybe_move_index.is_some() && maybe_after_index.is_some() {
                break;
            }

            if type_name_of_val(&(**doer)) == after_doer {
                maybe_after_index = Some(index);
            }

            if type_name_of_val(&(**doer)) == move_doer {
                maybe_move_index = Some(index);
            }
        }

        if let Some(before_index) = maybe_after_index {
            if let Some(move_index) = maybe_move_index {
                let move_doer = self.active.remove(move_index);

                self.active.insert(before_index + 1, move_doer);
            }
        }

        Ok(())
    }

    pub fn remove_doer(&mut self, doer_name: &'static str) -> Result<(), NucleusError> {
        let mut maybe_index = None;

        for (index, doer) in self.active.iter().enumerate() {
            if type_name_of_val(&(**doer)) == doer_name {
                maybe_index = Some(index);
                break;
            }
        }

        if let Some(index) = maybe_index {
            let doer = self.active.remove(index);

            doer.remove()?;
        }

        Ok(())
    }
}

/// TODO: maybe see about making StateStore a trait instead?
///
/// Nucleus<D: DoerStore, S: StateStore> {
///     pub state: S,
///     pub doers: D
/// }
///
pub struct Nucleus {
    pub state: StateStore,
    pub doers: DoerStore,
}

impl Nucleus {
    pub fn with_shared_state() -> Result<Self, NucleusError> {
        Self::new(SharedState::new(SharedStore::new()))
    }

    pub fn new(shared_state: SharedState<SharedStore>) -> Result<Self, NucleusError> {
        let state = StateStore::new(shared_state);
        let doers = DoerStore::new(&state)?;

        Ok(Self { state, doers })
    }

    pub fn add_doer<T: DoerTrait>(&self) -> Result<(), NucleusError> {
        let mut doer_state = self.doers.state.get()?;

        T::new_state(&self.state)?;

        doer_state
            .message_queue
            .push(DoerControlMessage::AddDoerToEnd(Box::new(T::new)));

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), NucleusError> {
        self.first()?;

        loop {
            self.update()?;
        }
    }

    /// I'd honestly like if this fit into the doer/state scheme, but
    /// we would run into double mutable access during the update loop.
    /// As in, if we had doers = State<Doers>, then any time we
    /// loop, we would do doers.get()?, then iterate over them and call "update".
    /// But any doer attempting to mutate doers would then do doers.get()
    /// itself and hit the double access.
    fn manage_control_messages(&mut self) -> Result<(), NucleusError> {
        let mut doer_state = self.doers.state.get()?;

        let control_messages = std::mem::replace(&mut doer_state.message_queue, Vec::new());

        drop(doer_state);

        for control_message in control_messages.into_iter() {
            match control_message {
                DoerControlMessage::AddDoerToStart(new_fn) => {
                    self.doers.add_doer_to_start(new_fn(&self)?)?;
                }
                DoerControlMessage::AddDoerToEnd(new_fn) => {
                    self.doers.add_doer_to_end(new_fn(&self)?)?;
                }
                DoerControlMessage::AddDoerBefore(move_doer_type_id, before_doer_type_id) => {
                    self.doers
                        .move_doer_before_other(before_doer_type_id, move_doer_type_id)?;
                }
                DoerControlMessage::AddDoerAfter(move_doer_type_id, after_doer_type_id) => {
                    self.doers
                        .move_doer_after_other(after_doer_type_id, move_doer_type_id)?;
                }
                DoerControlMessage::RemoveDoer(type_id) => {
                    self.doers.remove_doer(type_id)?;
                }
            }
        }

        Ok(())
    }

    pub fn first(&mut self) -> Result<(), NucleusError> {
        self.manage_control_messages()?;

        let doers_len = self.doers.active.len();

        let doers = std::mem::replace(&mut self.doers.active, Vec::with_capacity(doers_len));

        for doer in doers.into_iter() {
            doer.first(self)?;
            self.doers.add_doer_to_end(doer)?;
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), NucleusError> {
        self.manage_control_messages()?;

        for doer in self.doers.active.iter() {
            doer.update()?;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! nucleus {
    ($($doer:ident)+) => {
        let mut nucleus = Nucleus::with_shared_state()?;
        $(
            nucleus.add_doer::<$doer>()?;
        )*
        return nucleus.run();
    };

    ($shared_state:expr, $($doer:ident)+) => {
        let mut nucleus = Nucleus::new($shared_state)?;
        $(
            nucleus.add_doer::<$doer>()?;
        )*
        return nucleus.run();
    };
}
