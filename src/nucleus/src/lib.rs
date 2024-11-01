#![feature(mpmc_channel)]

mod common_shared_state;

use std::{
    any::{Any, TypeId},
    cell::{RefCell, UnsafeCell},
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Deref, DerefMut},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

pub struct Nucleus {
    pub join_handles: Vec<JoinHandle<Result<(), RunnerStartError>>>,
    pub shared_state_registry: HashMap<TypeId, Arc<Mutex<dyn State>>>,
}

impl Nucleus {
    pub fn new() -> NucleusPtr {
        NucleusPtr {
            nucleus: Arc::new(Mutex::new(Self {
                join_handles: Vec::new(),
                shared_state_registry: HashMap::new(),
            })),
        }
    }
}

pub enum NucleusError {
    NoControlMessageDestination,
}

#[derive(Clone)]
pub struct NucleusPtr {
    nucleus: Arc<Mutex<Nucleus>>,
}

// SAFETY: Guarded by Arc and Mutex.
unsafe impl Sync for NucleusPtr {}
// SAFETY: Guarded by Arc and Mutex.
unsafe impl Send for NucleusPtr {}

impl Deref for NucleusPtr {
    type Target = Arc<Mutex<Nucleus>>;

    fn deref(&self) -> &Self::Target {
        &self.nucleus
    }
}

impl NucleusPtr {
    pub fn add_runner(
        &self,
        runner_fn: impl FnOnce(NucleusPtr) -> Result<(), RunnerStartError> + Send + 'static,
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
        if let Ok(mut nucleus) = self.clone().lock() {
            for handle in std::mem::replace(&mut nucleus.join_handles, Vec::new()) {
                handle.join().unwrap();
            }
        }
    }
}

pub enum RunnerStartError {
    FailedToStart,
}

pub struct Runner {
    pub nucleus_ptr: NucleusPtr,
    pub state_registry: StateRegistry,
    pub updater_registry: UpdaterRegistry,
}

impl Runner {
    pub fn new(nucleus_ptr: NucleusPtr) -> RunnerPtr {
        RunnerPtr {
            runner: Rc::new(RefCell::new(Self {
                nucleus_ptr,
                state_registry: StateRegistry::new(),
                updater_registry: UpdaterRegistry::new(),
            })),
        }
    }
    // pub fn add_shared_state() {}
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

impl RunnerPtr {
    pub fn register_updater<T: UpdaterTrait>(&self) -> Result<&Self, UpdaterRegistryError> {
        let updater = T::new(self.clone());
        if let Ok(mut runner) = self.clone().try_borrow_mut() {
            runner.updater_registry.register_updater(updater);

            return Ok(self);
        } else {
            return Err(UpdaterRegistryError::UpdaterExistsAlready);
        }
    }

    pub fn get_read_state<T: State>(&self) -> Result<ReadState<T>, StateRegistryError> {
        if let Ok(runner) = self.clone().try_borrow_mut() {
            // runner.state_registry.register_state(state).unwrap();
            Ok(runner.state_registry.get_read_state::<T>().unwrap())
        } else {
            Err(StateRegistryError::StateExists)
        }
    }

    pub fn register_read_state<T: State>(
        &self,
        state: T,
    ) -> Result<ReadState<T>, StateRegistryError> {
        if let Ok(mut runner) = self.clone().try_borrow_mut() {
            runner.state_registry.register_state(state).unwrap();
            Ok(runner.state_registry.get_read_state::<T>().unwrap())
        } else {
            Err(StateRegistryError::StateExists)
        }
    }

    pub fn register_write_state<T: State>(
        &self,
        state: T,
    ) -> Result<WriteState<T>, StateRegistryError> {
        if let Ok(mut runner) = self.clone().try_borrow_mut() {
            runner.state_registry.register_state(state).unwrap();
            Ok(runner.state_registry.get_write_state::<T>().unwrap())
        } else {
            // TODO: fix
            Err(StateRegistryError::StateExists)
        }
    }

    pub fn run(&self) {
        if let Ok(mut runner) = self.clone().try_borrow_mut() {
            loop {
                for updater in runner.updater_registry.active_updaters.iter_mut() {
                    updater.update();
                }
            }
        }
    }
}

/// A special kind of updater that can recursively access its own runner
/// and updater registry. Use these when you need to modify
/// the program itself.
// #[cfg(meta_updaters)]
pub trait MetaUpdaterTrait: UpdaterTrait {
    fn update(&mut self, _runner_ptr: RunnerPtr) {}
}

pub trait UpdaterTrait: Debug + Any {
    fn new(_runner_ptr: RunnerPtr) -> Self
    where
        Self: Sized;
    /// An updater sometimes only registers state, so default the
    /// update to be empty. A updater doing this should immediately go
    /// inactive.
    fn update(&mut self) {}
}

#[derive(Debug)]
pub enum UpdaterRegistryError {
    UpdaterExistsAlready,
}

pub struct UpdaterRegistry {
    /// TODO: meta updaters are going to get tangled in the Runner Refcell.
    /// Need to either be okay with multiple mutability or find some way
    /// to avoid it with these.
    pub meta_updaters: Vec<Box<dyn MetaUpdaterTrait>>,
    pub active_updaters: Vec<Box<dyn UpdaterTrait>>,
    pub inactive_updaters: Vec<Box<dyn UpdaterTrait>>,
}

impl UpdaterRegistry {
    pub fn new() -> Self {
        Self {
            meta_updaters: Vec::new(),
            active_updaters: Vec::new(),
            inactive_updaters: Vec::new(),
        }
    }

    pub fn register_updater(&mut self, updater: impl UpdaterTrait) {
        self.active_updaters.push(Box::new(updater));
    }

    pub fn toggle_updater<T: UpdaterTrait>(&mut self, activate_updater: bool) {
        if activate_updater {
            let mut maybe_index = None;
            for (index, updater) in self.inactive_updaters.iter().enumerate() {
                if updater.type_id() == TypeId::of::<T>() {
                    maybe_index = Some(index);
                }
            }

            if let Some(index) = maybe_index {
                self.active_updaters
                    .push(self.inactive_updaters.remove(index));
            }
        } else {
            let mut maybe_index = None;
            for (index, updater) in self.active_updaters.iter().enumerate() {
                if updater.type_id() == TypeId::of::<T>() {
                    maybe_index = Some(index);
                }
            }

            if let Some(index) = maybe_index {
                self.inactive_updaters
                    .push(self.active_updaters.remove(index));
            }
        }
    }
}

pub trait State: Debug + Any {}

#[derive(Debug)]
pub struct ReadState<T>
where
    T: State,
{
    state: *const T,
}

impl<T> Deref for ReadState<T>
where
    T: State,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.state.as_ref().unwrap() }
    }
}

#[derive(Debug)]
pub struct WriteState<T>
where
    T: State,
{
    state: *mut T,
}

impl<T> Deref for WriteState<T>
where
    T: State,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.state.as_ref().unwrap() }
    }
}

impl<T> DerefMut for WriteState<T>
where
    T: State,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.state.as_mut().unwrap() }
    }
}

// Need a way for users to access something mutably. This is for extending
// program functionality.
// pub struct MultiWriteState<T> where T: State {
//     state: RefCell<>
// }

pub struct StateRegistry {
    state: HashMap<TypeId, Pin<Box<UnsafeCell<dyn State>>>>,
    write_states_given: HashSet<TypeId>,
}

#[derive(Debug)]
pub enum StateRegistryError {
    SingleWriteState,
    StateExists,
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
            write_states_given: HashSet::new(),
        }
    }

    pub fn register_state<T: State>(&mut self, state: T) -> Result<(), StateRegistryError> {
        if self.state.contains_key(&state.type_id()) {
            return Err(StateRegistryError::StateExists);
        }

        self.state
            .insert(state.type_id(), Box::pin(UnsafeCell::new(state)));

        Ok(())
    }

    pub fn get_read_state<T: State>(&self) -> Result<ReadState<T>, StateRegistryError> {
        let type_id = TypeId::of::<T>();

        Ok(ReadState {
            state: self.state.get(&type_id).unwrap().get().cast(),
        })
    }

    pub fn get_write_state<T: State>(&mut self) -> Result<WriteState<T>, StateRegistryError> {
        let type_id = TypeId::of::<T>();

        if self.write_states_given.contains(&type_id) {
            return Err(StateRegistryError::SingleWriteState);
        }

        self.write_states_given.insert(type_id);

        Ok(WriteState {
            state: self.state.get(&type_id).unwrap().get().cast(),
        })
    }
}
