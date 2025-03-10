use crate::{pm::*, state::*, PmError};
use std::any::{type_name_of_val, Any};

/// Doers are how [State] is updated. They are trait objects stored in a single [Vec]
/// within the [Pm]'s [DoerStore].
///
/// [DoerTrait] impls [Any] so that the type_name can be used for addressing
/// the doer.
pub trait DoerTrait: Any {
    /// Add any new [State] the [Doer] may require. When adding [Doer]s to
    /// the [Pm] or using [DoerGroup]s, this function is called before any [Doer]s
    /// are created. This avoids errors in [DoerTrait::new] when calling
    /// [StateStore::get_state] for [State] that other [Doer]s add to the [Pm].
    ///
    /// Note that the whole point of this method is to add new [State]. That
    /// is why only a [StateStore] reference is passed in. Don't do anything else.
    fn new_state(_state: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        Ok(())
    }

    /// Create the instance of the [Doer]. Typically, users will access the [Pm]
    /// reference for its [StateStore] to acquire the [State] it needs. Users lock
    /// either the [LocalStore] or [SharedStore] and then retrieve [State] or
    /// [SharedState] from them.
    ///
    /// By default, the doer is added to the end of the [DoerStore]'s active list.
    /// Change the doer's ordering using [DoerStore]'s [DoerState] and calling
    /// the relevant methods.
    fn new(_pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized;

    /// A one time function called after all doers have been added to
    /// the pm. This may block. All doers will have their "first"
    /// function called before entering the update loop.
    fn first(&self, _pm: &Pm) -> Result<(), PmError> {
        Ok(())
    }

    /// Don't pass in any context pointers because we want to only focus
    /// on manipulating the state. Be careful blocking this function as it
    /// will block the pm. Only special timing control doers should
    /// do any kind of sleeping.
    fn update(&self) -> Result<(), PmError> {
        Ok(())
    }

    /// Function called when doer is removed. If you need to differentiate
    /// about why the doer is being removed, you'll need to add that
    /// state yourself (such as if the whole program is exiting).
    fn remove(&self) -> Result<(), PmError> {
        Ok(())
    }
}

/// Typedef for the Doer's new function since it got used in multiple spots.
/// Slight abstraction cost, but code is easier to read.
type DoerNewFn = Box<dyn FnOnce(&Pm) -> Result<Box<dyn DoerTrait>, PmError>>;

/// A message sent to the [DoerState] intended to add/remove/manipulate [Doer]
/// execution order. 
pub enum DoerControlMessage {
    MoveAfter(&'static str, &'static str),
    MoveBefore(&'static str, &'static str),
    MoveToIndex(&'static str, usize),
    AddToEnd(DoerNewFn),
    AddToStart(DoerNewFn),
    Remove(&'static str),
}

/// All info concering Doers BESIDES the list of Doers themselves is kept in
/// a State so that other Doers may access it. This provides as much meta info
/// as possible without creating multiple mutable access of the active DoerList
/// (since they are mutably iterated over by the Pm).
#[derive(StateTrait)]
pub struct DoerState {
    /// The control messages that the Pm will iterate over and consume on each
    /// run loop. 
    pub message_queue: Vec<DoerControlMessage>,
    /// The list of inactive Doers. A Doer can become inactive if it errors or is
    /// removed via a [DoerControlMessage].
    pub inactive: Vec<DoerInactive>,
}

impl DoerState {
    pub fn new() -> Self {
        Self {
            message_queue: Vec::new(),
            inactive: Vec::new(),
        }
    }
}

/// A DoerStore contains the doers and meta information about them.
pub struct DoerStore {
    /// The list of active Doers. If any error, they are put into inactive.
    /// This isn't in the StateStore because the Pm needs to iterate over each.
    /// This could result in a "meta" Doer trying to access the list while the
    /// Pm is iterating over it, causing simultaneous access. If users need
    /// to manipulate this list, they can access DoerState's message_queue.
    pub active: Vec<Box<dyn DoerTrait>>,
    /// This state is special. It does get put into the LocalStore, but
    /// it is also kept here so other Doers don't need to query the LocalStore
    /// in their methods, they can just access it via the &Pm.
    pub state: State<DoerState>,
}

impl DoerStore {
    /// Create a new [Self]. Takes in a StateStore and adds its DoerState to
    /// it for other parts of the program to access.
    pub fn new(state: &StateStore) -> Result<Self, PmError> {
        let mut local_state = state.local.get()?;

        local_state.add_state(DoerState::new())?;

        Ok(Self {
            active: Vec::new(),
            state: local_state.get_state::<DoerState>()?,
        })
    }

    /// Add a doer to the start of the execution order.
    pub fn doer_to_start(&mut self, doer: Box<dyn DoerTrait>) -> Result<(), PmError> {
        self.active.insert(0, doer);

        Ok(())
    }

    /// Add a doer to the end of the execution order.
    pub fn doer_to_end(&mut self, doer: Box<dyn DoerTrait>) -> Result<(), PmError> {
        self.active.push(doer);

        Ok(())
    }

    /// Move one doer before another.
    pub fn move_doer_before_other(
        &mut self,
        before_doer: &'static str,
        move_doer: &'static str,
    ) -> Result<(), PmError> {
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

    /// Move one doer after another.
    pub fn move_doer_after_other(
        &mut self,
        after_doer: &'static str,
        move_doer: &'static str,
    ) -> Result<(), PmError> {
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

    /// Remove a doer from the execution. This calls [DoerTrait::remove] and puts the
    /// doer into the inactive list.
    pub fn remove_doer(&mut self, doer_name: &'static str) -> Result<(), PmError> {
        let mut doer_state = self.state.get()?;

        let mut maybe_index = None;

        for (index, doer) in self.active.iter().enumerate() {
            if type_name_of_val(&(**doer)) == doer_name {
                maybe_index = Some(index);
                break;
            }
        }

        if let Some(index) = maybe_index {
            let doer = self.active.remove(index);

            match doer.remove() {
                Ok(()) => doer_state.inactive.push(DoerInactive::Removed(doer)),
                Err(err) => doer_state.inactive.push(DoerInactive::RemoveErr(doer, err)),
            }
        }

        Ok(())
    }
}

/// A group of doers to simplify adding many at once to a Pm.
pub struct DoerGroup {
    pub add_state: Vec<Box<dyn FnOnce(&StateStore) -> Result<(), PmError>>>,
    pub doers: Vec<DoerNewFn>,
}

impl DoerGroup {
    /// A builder type method for adding doers to the DoerGroup.
    pub fn add_doer<T: DoerTrait>(&mut self) -> Result<(), PmError> {
        self.add_state.push(Box::new(T::new_state));
        self.doers.push(Box::new(T::new));

        Ok(())
    }
}

/// An enum for describing why a doer is inactive. Grouped with the doer itself
/// in the "inactive" doer list on DoerState.
pub enum DoerInactive {
    NewStateErr(PmError),
    NewErr(PmError),
    FirstErr(Box<dyn DoerTrait>, PmError),
    UpdateErr(Box<dyn DoerTrait>, PmError),
    RemoveErr(Box<dyn DoerTrait>, PmError),
    Removed(Box<dyn DoerTrait>),
}
