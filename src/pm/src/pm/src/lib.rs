//! Pm is meant to standardize program state and its modification. The goal of this
//! is to provide performance, minimal abstraction, and maintainability.
//!
//! I was trying to avoid:
//! Async.
//! Complicated scheduling/lifecycle.
//! Context pointers.
//! Deeply nested or deeply composed state.
//! Hidden state.
//! Message passing.
//! Abstraction of the work cycle.
//! Building too much into the framework.
//!
//! I also wanted the framework to be easily testable. Doers can be added before
//! or after a certain State manipulation occurs, erroring out on a fail condition.
//! 
//! Generally the framework encourages doing all the "slow" and "blocking" work
//! at startup. Adding doers, adding [State], changing doer ordering, should all
//! happen early in the doer lifecycle. The [DoerTrait::update] method should NOT
//! block the thread unless it is explicitly designed to do so. [DoerTrait::update]
//! does not ingest a &Pm to discourage State and Doer modification at runtime.
//! There is nothing stopping a user from keeping a State<StateStore> as a
//! member of the Doer struct itself to maintain flexibility.
//!
//! State and SharedState are separate because the performance costs of multithreaded
//! access. A single threaded application can do quite a lot on its own. Pm encourages
//! structuring applications to have few, "large" threads instead of many small ones.
//! 
//! Inspiration
//! https://matklad.github.io/2021/09/05/Rust100k.html

mod doer;
mod pm;
mod state;

pub use doer::*;
pub use pm::*;
pub use state::*;

/// The high level errors possible while using Pm.
#[derive(Debug)]
pub enum PmError {
    /// Errored during [State::get] or [SharedState::get].
    GetState,
    /// Errored during [SharedState::blocking_get].
    GetStateBlocking,
    /// Errored inside of [DoerTrait::new_state].
    AddNewState,
    /// Errored inside of [DoerTrait::new].
    NewDoer,
    /// Errored inside of [DoerTrait::first].
    DoerFirst,
    /// Errored inside of [DoerTrait::update].
    DoerUpdate,
    /// Errored when attempting to add [State] to a [StateStore].
    StateExists,
    /// Errored when attempting to cast [State] to the desired type.
    CouldNotCastState,
    /// Errored when attempting to get [State] from a store.
    StateDoesNotExist,
    /// Errored when attempting to remove [State] from a store.
    RemoveState,
}
