#![feature(portable_simd)]

use std::{
    any::TypeId,
    time::{Duration, Instant},
};

use nucleus::*;
use nucleus_common::{
    // logging::{Logging, LoggingManager},
    loop_timing::{LoopTiming, LoopTimingManager},
    thread_manager::{ThreadManager, ThreadRequest},
};

#[derive(SharedStateTrait)]
pub struct SomeSharedState {
    ready_for_work: bool,
    number: f32,
}

pub struct SharedUpdater1 {
    shared_state: SharedState<SomeSharedState>,
    updater_control: State<UpdaterControl>,
    start_instant: Instant,
}

impl UpdaterTrait for SharedUpdater1 {
    fn add_new_state(
        shared_state: SharedState<SharedStore>,
        _thread_state: State<StateStore>,
    ) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        let mut shared_state = shared_state.blocking_get()?;

        shared_state.add_state(SomeSharedState {
            ready_for_work: false,
            number: 1.0,
        })?;

        Ok(())
    }

    fn new(nucleus: &Nucleus) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        let shared_state = thread.shared_state.blocking_get()?;

        Ok(Box::new(SharedUpdater1 {
            shared_state: shared_state.get_state::<SomeSharedState>()?,
            start_instant: Instant::now(),
            updater_control: thread.updater_control.clone(),
        }))
    }

    fn update(&self) -> Result<(), NucleusError> {
        let Ok(mut shared_state) = self.shared_state.get() else {
            return Ok(());
        };

        shared_state.number += 1.0;
        shared_state.ready_for_work = true;

        if self.start_instant.elapsed() > Duration::from_secs(3) {
            let mut updater_control = self.updater_control.get_mut()?;

            updater_control
                .message_queue
                .push(UpdaterControlMessage::AddUpdaterToEnd(Box::new(
                    SharedUpdater2::new,
                )));

            updater_control
                .message_queue
                .push(UpdaterControlMessage::RemoveUpdater(TypeId::of::<
                    SharedUpdater1,
                >()));
        }

        Ok(())
    }
}

fn fun_thread(shared_state: SharedState<SharedStore>) -> Result<(), NucleusError> {
    println!("starting fun thread!");
    let mut thread = Nucleus::new(shared_state);
    thread.add_updater::<LoopTimingManager>()?;
    thread.run()
}

pub struct SharedUpdater2 {
    shared_state: SharedState<SomeSharedState>,
    timing: State<LoopTiming>,
    thread_requests: SharedState<ThreadRequest>,
}

impl UpdaterTrait for SharedUpdater2 {
    fn new(nucleus: &Nucleus) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        let shared_state = thread.shared_state.blocking_get()?;
        let local_state = thread.local_state.get_mut()?;

        wait_for_shared_state!(
            shared_state,
            SomeSharedState,
            Duration::from_millis(100),
            Duration::from_millis(500)
        );

        Ok(Box::new(SharedUpdater2 {
            timing: local_state.get_state::<LoopTiming>()?,
            shared_state: shared_state.get_state::<SomeSharedState>()?,
            thread_requests: shared_state.get_state::<ThreadRequest>()?,
        }))
    }

    fn first(&self, thread: &Nucleus) -> Result<(), NucleusError> {
        let mut updater_control = thread.updater_control.get_mut()?;
        let mut thread_requests = self.thread_requests.blocking_get()?;

        updater_control
            .message_queue
            .push(UpdaterControlMessage::RemoveUpdater(TypeId::of::<
                SharedUpdater2,
            >()));

        thread_requests.add_thread(fun_thread);

        Ok(())
    }

    fn update(&self) -> Result<(), NucleusError> {
        let Ok(mut shared_state) = self.shared_state.get() else {
            self.timing.get_mut()?.loop_sleep_duration += Duration::from_millis(5);

            return Ok(());
        };

        if shared_state.ready_for_work {
            shared_state.number -= 1.0;
            shared_state.ready_for_work = false;
        }

        Ok(())
    }
}

fn main() -> Result<(), NucleusError> {
    let mut nucleus = Nucleus::new()?;
    nucleus.add_updater::<LoopTimingManager>()?;
    nucleus.add_updater::<SharedUpdater1>()?;
    nucleus.add_updater::<SharedUpdater2>()?;
    nucleus.add_updater::<ThreadManager>()?;

    nucleus.run()
}
