use std::time::{Duration, Instant};

use nucleus::*;

pub struct Updater {}

impl UpdaterTrait for Updater {
    fn add_new_state(_state: &StateStore) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
    }

    fn first(&self, _thread: &Thread) -> Result<(), NucleusError> {}

    fn new(_thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
    }

    fn update(&self) -> Result<(), NucleusError> {}

    fn remove(&self) -> Result<(), NucleusError> {}

    fn on_exit(&self) -> Result<(), NucleusError> {}
}

fn main() -> Result<(), NucleusError> {
    let mut nucleus = Nucleus::new()?;
    nucleus.add_updater::<LoopTimingManager>()?;
    nucleus.add_updater::<SharedUpdater1>()?;
    nucleus.add_updater::<SharedUpdater2>()?;
    nucleus.add_updater::<ThreadManager>()?;

    nucleus.run()
}
