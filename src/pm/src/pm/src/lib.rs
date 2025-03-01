//! Pm is a meant to standardize program state and its modification. The goal of this
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
//! I also wanted the framework to be easily testable. This is possible by simply
//! adding test Doers that inspect state after a Doer has manipulated it.
//! 
//! Generally the framework encourages doing all the "slow" and "blocking" work
//! at startup. Adding doers, adding [State], changing doer ordering, should all
//! happen early in the doer lifecycle. The [DoerTrait::update] method should NOT
//! block the thread unless it is explicitly designed to do so.
//! 
//! The framework also encourages using as little [SharedState] as possible. This
//! reduces the risk of performance degradation and lockups.
//! 
//! Inspiration
//! https://matklad.github.io/2021/09/05/Rust100k.html

mod doer;
mod pm;
mod state;

pub use doer::*;
pub use pm::*;
pub use state::*;

#[derive(Debug)]
pub enum PmError {
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
