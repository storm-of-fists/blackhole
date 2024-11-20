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

/// TODO: Write a proc macro for this. May want to disallow the use of
/// State<T> or SharedState<T> members in the struct we are impling on
/// to avoid nested state.
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

pub trait UpdaterTrait: Any {
    /// Add any new state the updater may require in here. This function
    /// will be called for all updaters.
    #[allow(clippy::unused_variable)]
    fn add_new_state(_nucleus: NucleusPtr, _thread: &mut Thread) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        Ok(())
    }

    /// Register your updater. Feel free to
    fn register(nucleus: NucleusPtr, thread: &mut Thread) -> Result<(), NucleusError>
    where
        Self: Sized;

    /// A one time function called after all updaters have been added to
    /// the thread. This may block. All updaters will have their "first"
    /// function called before entering the update loop.
    fn first(&self, _nucleus: NucleusPtr, _thread: &mut Thread) -> Result<(), NucleusError> {
        Ok(())
    }

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state. Be careful blocking this function as it
    /// will block the thread. Only special timing control updaters should
    /// do any kind of sleeping.
    fn update(&self) -> Result<(), NucleusError> {
        Ok(())
    }

    /// This is a method and not just associated so that it can change at runtime.
    /// A meta updater will still need to perform the order change.
    fn ordering(&self) -> UpdaterOrdering {
        UpdaterOrdering::default()
    }

    // fn is_meta_updater(&self) -> bool {
    //     false
    // }
}

pub trait MetaUpdaterTrait: UpdaterTrait {
    fn update(&self, _nucleus: NucleusPtr, _thread: &mut Thread) -> Result<(), NucleusError> {
        Ok(())
    }

    // fn is_meta_updater(&self) -> bool {
    //     true
    // }
}

pub struct Thread {
    nucleus: NucleusPtr,
    /// TODO: come up with a way to combine the meta and regular updaters? perhaps part of the trait?
    pending_updaters: Vec<Box<dyn FnOnce(NucleusPtr, &mut Thread) -> Result<(), NucleusError>>>,
    pending_meta_updaters:
        Vec<Box<dyn FnOnce(NucleusPtr, &mut Thread) -> Result<(), NucleusError>>>,
    state: HashMap<TypeId, Box<dyn StateTrait>>,
    active_updaters: Vec<Box<dyn UpdaterTrait>>,
    inactive_updaters: Vec<Box<dyn UpdaterTrait>>,
    active_meta_updaters: Vec<Box<dyn MetaUpdaterTrait>>,
    inactive_meta_updaters: Vec<Box<dyn MetaUpdaterTrait>>,
}

impl Thread {
    pub fn new(nucleus: NucleusPtr) -> Self {
        Self {
            nucleus,
            pending_updaters: Vec::new(),
            pending_meta_updaters: Vec::new(),
            state: HashMap::new(),
            active_updaters: Vec::new(),
            inactive_updaters: Vec::new(),
            active_meta_updaters: Vec::new(),
            inactive_meta_updaters: Vec::new(),
        }
    }

    /// Informs the thread that this updater will be part of the thread. Delays creation of
    /// the updater to avoid "State does not exist!" errors.
    pub fn register_updater<T: UpdaterTrait>(&mut self) -> Result<(), NucleusError> {
        // Add new state, but delay the registration until later to reduce annoyingness.
        T::add_new_state(self.nucleus.clone(), self)?;
        self.pending_updaters.push(Box::new(T::register));

        Ok(())
    }

    /// Add an updater to this thread's active_updaters list.
    pub fn add_updater<T: UpdaterTrait>(&mut self, updater: T) -> Result<(), NucleusError> {
        // Check if updater exists already.
        self.add_updater_with_ordering(Box::new(updater))?;

        Ok(())
    }

    /// Enable or disable an updater. Moves it from the active/inactive list to the other.
    pub fn toggle_updater<T: UpdaterTrait>(&mut self, enabled: bool) -> Result<(), NucleusError> {
        let mut maybe_index = None;

        if enabled {
            for (index, updater) in self.inactive_updaters.iter().enumerate() {
                if updater.type_id() == TypeId::of::<T>() {
                    maybe_index = Some(index);
                }
            }

            if let Some(index) = maybe_index {
                let updater = self.inactive_updaters.remove(index);

                self.add_updater_with_ordering(updater)?;
            }
        } else {
            for (index, updater) in self.active_updaters.iter().enumerate() {
                if updater.type_id() == TypeId::of::<T>() {
                    maybe_index = Some(index);
                }
            }

            if let Some(index) = maybe_index {
                let updater = self.active_updaters.remove(index);

                // Don't care about the order inactive updaters are in.
                self.inactive_updaters.push(updater);
            }
        }

        Ok(())
    }

    /// Add an updater back into the active updater list with its preferred ordering.
    fn add_updater_with_ordering(
        &mut self,
        updater: Box<dyn UpdaterTrait>,
    ) -> Result<(), NucleusError> {
        match updater.ordering() {
            UpdaterOrdering::AddToFront => {
                unimplemented!("TODO not impl");
            }
            UpdaterOrdering::AddToEnd => {
                // TODO: need logic in here to detect existing updaters.
                self.active_updaters.push(updater);
            }
            UpdaterOrdering::Before(_type_id) => {
                unimplemented!("TODO not impl");
            }
            UpdaterOrdering::After(_type_id) => {
                unimplemented!("TODO not impl");
            }
        }

        Ok(())
    }

    pub fn register_meta_updater<T: MetaUpdaterTrait>(&mut self) -> Result<(), NucleusError> {
        // Add new state, but delay the registration until later to reduce annoyingness.
        T::add_new_state(self.nucleus.clone(), self)?;
        self.pending_meta_updaters.push(Box::new(T::register));

        Ok(())
    }

    /// Enable or disable an updater. Moves it from the active/inactive list to the other.
    pub fn toggle_meta_updater<T: MetaUpdaterTrait>(
        &mut self,
        enabled: bool,
    ) -> Result<(), NucleusError> {
        let mut maybe_index = None;

        if enabled {
            for (index, updater) in self.inactive_meta_updaters.iter().enumerate() {
                if updater.type_id() == TypeId::of::<T>() {
                    maybe_index = Some(index);
                }
            }

            if let Some(index) = maybe_index {
                let updater = self.inactive_meta_updaters.remove(index);

                self.add_meta_updater_with_ordering(updater)?;
            }
        } else {
            for (index, updater) in self.active_meta_updaters.iter().enumerate() {
                if updater.type_id() == TypeId::of::<T>() {
                    maybe_index = Some(index);
                }
            }

            if let Some(index) = maybe_index {
                let updater = self.active_meta_updaters.remove(index);

                // Don't care about the order inactive updaters are in.
                self.inactive_meta_updaters.push(updater);
            }
        }

        Ok(())
    }

    /// Add an updater back into the active updater list with its preferred ordering.
    fn add_meta_updater_with_ordering(
        &mut self,
        updater: Box<dyn MetaUpdaterTrait>,
    ) -> Result<(), NucleusError> {
        match updater.ordering() {
            UpdaterOrdering::AddToFront => {
                unimplemented!("TODO not impl");
            }
            UpdaterOrdering::AddToEnd => {
                self.active_meta_updaters.push(updater);
            }
            UpdaterOrdering::Before(_type_id) => {
                unimplemented!("TODO not impl");
            }
            UpdaterOrdering::After(_type_id) => {
                unimplemented!("TODO not impl");
            }
        }

        Ok(())
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

    fn first(&mut self) -> Result<(), NucleusError> {
        let register_meta_updaters = std::mem::replace(&mut self.pending_meta_updaters, Vec::new());

        for register_meta_updater in register_meta_updaters.into_iter() {
            register_meta_updater(self.nucleus.clone(), self)?;
        }

        let active_meta_updaters = std::mem::replace(&mut self.active_meta_updaters, Vec::new());

        for updater in active_meta_updaters.into_iter() {
            updater.first(self.nucleus.clone(), self)?;
            self.add_meta_updater_with_ordering(updater)?;
        }

        let register_updaters = std::mem::replace(&mut self.pending_updaters, Vec::new());

        for register_updater in register_updaters.into_iter() {
            register_updater(self.nucleus.clone(), self)?;
        }

        let active_updaters = std::mem::replace(&mut self.active_updaters, Vec::new());

        for updater in active_updaters.into_iter() {
            updater.first(self.nucleus.clone(), self)?;
            self.add_updater_with_ordering(updater)?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), NucleusError> {
        self.first()?;

        loop {
            // TODO: decide how to handle this double mutable borrow.
            // for active_meta_updater in self.active_meta_updaters.iter_mut() {
            //     active_meta_updater.update(self.nucleus.clone(), self);
            // }
            for active_updater in self.active_updaters.iter_mut() {
                active_updater.update()?;
            }
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
