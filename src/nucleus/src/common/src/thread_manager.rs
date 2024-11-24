use pm::*;
use std::thread::JoinHandle;

#[derive(SharedStateTrait)]
pub struct ThreadRequest {
    pub requests:
        Vec<Box<dyn FnOnce(SharedState<SharedStore>) -> Result<(), PmError> + Send + 'static>>,
}

impl ThreadRequest {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    pub fn add_thread(
        &mut self,
        thread_fn: impl FnOnce(SharedState<SharedStore>) -> Result<(), PmError> + Send + 'static,
    ) {
        self.requests.push(Box::new(thread_fn));
    }
}

#[derive(StateTrait)]
pub struct ThreadStore {
    pub join_handles: Vec<JoinHandle<Result<(), PmError>>>,
}

impl ThreadStore {
    pub fn new() -> Self {
        Self {
            join_handles: Vec::new(),
        }
    }

    pub fn add_thread(
        &mut self,
        shared_state: SharedState<SharedStore>,
        thread_fn: impl FnOnce(SharedState<SharedStore>) -> Result<(), PmError> + Send + 'static,
    ) -> Result<(), PmError> {
        self.join_handles
            .push(std::thread::spawn(|| thread_fn(shared_state)));

        Ok(())
    }
}

pub struct ThreadManager {
    shared_state: SharedState<SharedStore>,
    thread_requests: SharedState<ThreadRequest>,
    thread_store: State<ThreadStore>,
}

impl DoerTrait for ThreadManager {
    fn new_state(state_store: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        let mut local_state = state_store.local.get()?;
        let mut shared_state = state_store.shared.blocking_get()?;

        local_state.add_state(ThreadStore::new())?;
        shared_state.add_state(ThreadRequest::new())?;

        Ok(())
    }

    fn new(pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        let local_state = pm.state.local.get()?;
        let shared_state = pm.state.shared.blocking_get()?;

        Ok(Box::new(Self {
            shared_state: pm.state.shared.clone(),
            thread_store: local_state.get_state::<ThreadStore>()?,
            thread_requests: shared_state.get_state::<ThreadRequest>()?,
        }))
    }

    fn update(&self) -> Result<(), PmError> {
        let mut thread_store = self.thread_store.get()?;
        let mut thread_request = self.thread_requests.get()?;
        let join_handles = std::mem::replace(&mut thread_store.join_handles, Vec::new());
        let requests = std::mem::replace(&mut thread_request.requests, Vec::new());
        let mut complete_handles = Vec::new();

        for request in requests.into_iter() {
            thread_store.add_thread(self.shared_state.clone(), request)?;
        }

        for handle in join_handles.into_iter() {
            if handle.is_finished() {
                complete_handles.push(handle);
            } else {
                thread_store.join_handles.push(handle);
            }
        }

        // Exit if any of the threads have errored.
        for complete_handle in complete_handles.into_iter() {
            let result = complete_handle.join().unwrap();

            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }
}
