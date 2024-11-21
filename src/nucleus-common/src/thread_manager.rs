use nucleus::*;
use std::thread::JoinHandle;

#[derive(StateTrait)]
pub struct ThreadStore {
    pub join_handles: Vec<JoinHandle<Result<(), NucleusError>>>,
}

impl ThreadStore {
    pub fn new() -> Self {
        Self {
            join_handles: Vec::new(),
        }
    }
}

pub struct ThreadManager {
    shared_state: SharedState<SharedStateStore>,
    thread_store: State<ThreadStore>,
}

// Put this into a Updater and some state?
// pub fn add_thread(
//     &mut self,
//     thread_fn: impl FnOnce(SharedState<SharedStateStore>) -> Result<(), NucleusError>
//         + Send
//         + 'static,
// ) -> Result<(), NucleusError> {
//     let shared_state = self.shared_state.clone();

//     thread_store
//         .join_handles
//         .push(std::thread::spawn(|| thread_fn(shared_state)));

//     Ok(())
// }

impl UpdaterTrait for ThreadManager {
    fn add_new_state(
        _shared_state: SharedState<SharedStateStore>,
        local_state: State<StateStore>,
    ) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        let mut local_state = local_state.get_mut()?;

        local_state.add_state(ThreadStore::new())
    }

    fn new(thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        let local_state = thread.local_state.get_mut()?;

        Ok(Box::new(Self {
            shared_state: thread.shared_state.clone(),
            thread_store: local_state.get_state::<ThreadStore>()?,
        }))
    }

    fn update(&self) -> Result<(), NucleusError> {
        let mut store = self.thread_store.get_mut()?;

        let join_handles = std::mem::replace(&mut store.join_handles, Vec::new());

        let mut complete_handles = Vec::new();

        for handle in join_handles.into_iter() {
            if handle.is_finished() {
                complete_handles.push(handle);
            } else {
                store.join_handles.push(handle);
            }
        }

        // Exit if any of the threads have errored.
        for complete_handle in complete_handles.into_iter() {
            let result = complete_handle.join().unwrap();

            println!("Completed handle: {:?}", result);

            if result.is_err() {
                println!("Nucleus error: {:?}", result);
                return result;
            }
        }

        Ok(())
    }
}
