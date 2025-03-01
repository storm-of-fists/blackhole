use std::{
    any::{type_name, Any},
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

use mut_cell::{MutCell, MutCellRef};

use crate::PmError;

// These derive names don't conflict with the trait names? Nice.
pub use pm_macros::{SharedStateTrait, StateTrait};

/// This is any state local to the thread. Each piece of state is allocated
/// via an Rc. It is trivialy cloneable to any doer. Users can access the
/// state inside via the MutCell API. See MutCell crate for more info on what
/// it is and why I didn't use a RefCell.
pub struct State<T> {
    /// We use an Rc here so that dropping of the internal MutCell is handled 
    /// automatically. Pm doesn't use any Weak pointers or check ref counts, 
    /// but those don't cost anything here.
    state: Rc<MutCell<T>>,
}

impl<T> State<T> {
    pub fn new(state: T) -> Self {
        Self {
            state: Rc::new(MutCell::new(state)),
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
    ///
    /// TODO: May want to make this part of StateTrait instead, allow users to use
    /// their own sorts of state storage types.
    pub fn get(&self) -> Result<MutCellRef<'_, T>, PmError> {
        self.state.get().map_err(|_| PmError::GetState)
    }
}

impl<T> From<Rc<MutCell<T>>> for State<T>
where
    T: StateTrait,
{
    fn from(state: Rc<MutCell<T>>) -> Self {
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

impl<T> StateTrait for Rc<MutCell<T>>
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
    pub fn get(&self) -> Result<MutexGuard<'_, T>, PmError> {
        self.state.try_lock().map_err(|_| PmError::GetState)
    }

    /// Use a block lock for when you need to wait for some shared state to
    /// become available and you don't care to wait.
    pub fn blocking_get(&self) -> Result<MutexGuard<'_, T>, PmError> {
        self.state.lock().map_err(|_| PmError::GetStateBlocking)
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

unsafe impl<T: Send> Send for SharedState<T> {}
unsafe impl<T: Send> Sync for SharedState<T> {}

/// This macro checks if shared state exists within the SharedStateStore.
/// This makes it easier when "getting" SharedState across threads.
#[macro_export]
macro_rules! shared_state_wait {
    ($shared_state:expr, $($state_type:ident)+; $check_interval:expr, $total_wait_duration:expr) => {
        let mut all_exist = true;
        let start_instant = std::time::Instant::now();

        loop {
            if start_instant.elapsed() > $total_wait_duration {
                return Err(PmError::StateDoesNotExist)
            }
            $(all_exist &= $shared_state.state_exists::<$state_type>();)*

            if all_exist {
                break;
            }

            std::thread::sleep($check_interval);
        }
    };
}

/// A store for thread-local state.
pub struct LocalStore {
    /// Uses a hashmap to store the state by its type name.
    store: HashMap<&'static str, Box<dyn StateTrait>>,
}

impl LocalStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Add state to the store.
    pub fn add_state<T: StateTrait>(&mut self, state: T) -> Result<(), PmError> {
        // Error on if the state already exists. Originally, this didn't error, but
        // this could lead to confusion about initial state values. If a user thinks
        // the state may exist already, they can check.
        if self.state_exists::<T>() {
            return Err(PmError::StateExists);
        }

        let state = State::new(state);

        self.store
            .insert(type_name::<T>(), Box::new(state.state.clone()));

        // Don't return a value since this method is only called inside
        // [DoerTrait::add_state], where the value has no place to be stored.
        Ok(())
    }

    /// Get state from the store. 
    pub fn get_state<T: StateTrait>(&self) -> Result<State<T>, PmError> {
        let Some(boxed_state) = self.store.get(type_name::<T>()) else {
            return Err(PmError::StateDoesNotExist);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<MutCell<T>>>()
            .cloned()
        else {
            return Err(PmError::CouldNotCastState);
        };

        Ok(State::from(cloned_rc))
    }

    /// remove some state to disallow other doers from acquiring it.
    pub fn remove_state<T: StateTrait>(&mut self) -> Result<State<T>, PmError> {
        let Some(boxed_state) = self.store.remove(type_name::<T>()) else {
            return Err(PmError::StateDoesNotExist);
        };

        let Some(cloned_rc) = boxed_state
            .as_any()
            .downcast_ref::<Rc<MutCell<T>>>()
            .cloned()
        else {
            return Err(PmError::CouldNotCastState);
        };

        Ok(State::from(cloned_rc))
    }

    /// Check if some state exists via its type name.
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

    pub fn add_state<T: SharedStateTrait>(&mut self, state: T) -> Result<(), PmError> {
        let type_name = type_name::<T>();

        if self.store.contains_key(type_name) {
            return Err(PmError::StateExists);
        }

        let state = SharedState::new(state);

        self.store.insert(type_name, Box::new(state.state.clone()));

        Ok(())
    }

    pub fn get_state<T: SharedStateTrait>(&self) -> Result<SharedState<T>, PmError> {
        let Some(boxed_state) = self.store.get(type_name::<T>()) else {
            return Err(PmError::StateDoesNotExist);
        };

        let Some(cloned_arc) = boxed_state
            .as_any()
            .downcast_ref::<Arc<Mutex<T>>>()
            .cloned()
        else {
            return Err(PmError::CouldNotCastState);
        };

        Ok(SharedState::from(cloned_arc))
    }

    /// remove some state to disallow other doers from acquiring it.
    pub fn remove_state<T: SharedStateTrait>(&mut self) -> Result<SharedState<T>, PmError> {
        let Some(boxed_state) = self.store.remove(type_name::<T>()) else {
            return Err(PmError::StateDoesNotExist);
        };

        let Some(cloned_arc) = boxed_state
            .as_any()
            .downcast_ref::<Arc<Mutex<T>>>()
            .cloned()
        else {
            return Err(PmError::CouldNotCastState);
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
