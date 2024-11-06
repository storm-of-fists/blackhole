

impl NucleusPtr {
    pub fn add_runner(
        &self,
        name: impl Into<String>,
        runner_fn: impl FnOnce(NucleusPtr) -> Result<(), RunnerError> + Send + 'static,
    ) -> &Self {
        let nucleus_ptr = self.clone();

        if let Ok(mut nucleus) = nucleus_ptr.clone().lock() {
            nucleus
                .join_handles
                .insert(name.into(), std::thread::spawn(|| runner_fn(nucleus_ptr)));
        }

        self
    }

    pub fn go(&self) {
        loop {
            // probably want a try lock here
            if let Ok(mut nucleus) = self.nucleus.lock() {
                let join_handles = std::mem::replace(&mut nucleus.join_handles, Vec::new());
                let complete_handles = Vec::new();

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

            // do something if there are any pending updaters.
        }
    }
}
