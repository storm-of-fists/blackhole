use crate::{doer::*, state::*, PmError};

/// TODO: maybe see about making StateStore a trait instead?
///
/// Pm<D: DoerStore, S: StateStore> {
///     pub state: S,
///     pub doers: D
/// }
///
pub struct Pm {
    pub state: StateStore,
    pub doers: DoerStore,
}

impl Pm {
    pub fn with_shared_state() -> Result<Self, PmError> {
        Self::new(SharedState::new(SharedStore::new()))
    }

    pub fn new(shared_state: SharedState<SharedStore>) -> Result<Self, PmError> {
        let state = StateStore::new(shared_state);
        let doers = DoerStore::new(&state)?;

        Ok(Self { state, doers })
    }

    pub fn add_doer<T: DoerTrait>(&self) -> Result<(), PmError> {
        let mut doer_state = self.doers.state.get()?;

        T::new_state(&self.state)?;

        doer_state
            .message_queue
            .push(DoerControl::AddDoerToEnd(Box::new(T::new)));

        Ok(())
    }

    /// Add doers in a group. This will call new_state and new, then add the doer
    /// to the active list. After all doers are added to the list, a
    /// Self::manage_control_messages call occurs so doers get situated.
    ///
    /// Doer groups allow doers to remove state they may want to hide. Each
    /// doer can get_state during its new call. A final doer can remove the
    /// state from the store while all the doers still hold a State instance.
    pub fn add_doer_group(&mut self, doer_group: DoerGroup) -> Result<(), PmError> {
        let new_state_funcs = doer_group.add_state;
        let new_funcs = doer_group.doers;
        let mut new_doers: Vec<Box<dyn DoerTrait>> = Vec::new();

        for new_state in new_state_funcs.into_iter() {
            new_state(&self.state)?;
        }

        for new in new_funcs.into_iter() {
            new_doers.push(new(&self)?);
        }

        for doer in new_doers.into_iter() {
            self.doers.active.push(doer);
        }

        self.manage_control_messages()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), PmError> {
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
    fn manage_control_messages(&mut self) -> Result<(), PmError> {
        let mut doer_state = self.doers.state.get()?;

        let control_messages = std::mem::replace(&mut doer_state.message_queue, Vec::new());

        drop(doer_state);

        for control_message in control_messages.into_iter() {
            match control_message {
                DoerControl::AddDoerToStart(new_fn) => {
                    self.doers.doer_to_start(new_fn(&self)?)?;
                }
                DoerControl::AddDoerToEnd(new_fn) => {
                    self.doers.doer_to_end(new_fn(&self)?)?;
                }
                DoerControl::MoveDoerBefore(move_doer_type_id, before_doer_type_id) => {
                    self.doers
                        .move_doer_before_other(before_doer_type_id, move_doer_type_id)?;
                }
                DoerControl::MoveDoerAfter(move_doer_type_id, after_doer_type_id) => {
                    self.doers
                        .move_doer_after_other(after_doer_type_id, move_doer_type_id)?;
                }
                DoerControl::RemoveDoer(type_id) => {
                    self.doers.remove_doer(type_id)?;
                }
            }
        }

        Ok(())
    }

    pub fn first(&mut self) -> Result<(), PmError> {
        self.manage_control_messages()?;

        let doers_len = self.doers.active.len();
        let mut doer_state = self.doers.state.get()?;

        let doers = std::mem::replace(&mut self.doers.active, Vec::with_capacity(doers_len));
        let mut doers_after_first = Vec::new();

        for doer in doers.into_iter() {
            match doer.first(self) {
                Ok(()) => doers_after_first.push(doer),
                Err(err) => doer_state.inactive.push(DoerInactive::FirstErr(doer, err)),
            }
        }

        drop(doer_state);

        for doer in doers_after_first.into_iter() {
            self.doers.active.push(doer);
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), PmError> {
        self.manage_control_messages()?;

        let mut errored_doer_indices = Vec::new();
        let mut errored_doers = Vec::new();

        if self.doers.active.len() == 0 {
            return Err(PmError::DoerUpdate);
        }

        for (index, doer) in self.doers.active.iter().enumerate() {
            match doer.update() {
                Ok(()) => continue,
                Err(err) => errored_doer_indices.push((index, err)),
            }
        }

        for (index, err) in errored_doer_indices.into_iter() {
            errored_doers.push((self.doers.active.remove(index), err));
        }

        if errored_doers.len() > 0 {
            let mut doer_state = self.doers.state.get()?;

            for (errored_doer, err) in errored_doers.into_iter() {
                doer_state
                    .inactive
                    .push(DoerInactive::UpdateErr(errored_doer, err))
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! pm {
    ($($doer:ident),+) => {{
        let mut pm = Pm::with_shared_state()?;
        $(
            pm.add_doer::<$doer>()?;
        )*
        pm
    }};

    ($shared_state:expr; $($doer:ident),+) => {{
        let mut pm = Pm::new($shared_state)?;
        $(
            pm.doer::<$doer>()?;
        )*
        pm
    }};
}
