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

pub struct SharedDoer1 {
    shared_state: SharedState<SomeSharedState>,
    doer_control: State<DoerControl>,
    start_instant: Instant,
}

impl DoerTrait for SharedDoer1 {
    fn new_state(state: &StateStore) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        let mut shared_state = state.shared.blocking_get()?;

        shared_state.add_state(SomeSharedState {
            ready_for_work: false,
            number: 1.0,
        })?;

        Ok(())
    }

    fn new(nucleus: &Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>
    where
        Self: Sized,
    {
        let shared_state = nucleus.state.shared.blocking_get()?;

        Ok(Box::new(SharedDoer1 {
            shared_state: shared_state.get_state::<SomeSharedState>()?,
            start_instant: Instant::now(),
            doer_control: nucleus.doers.control.clone(),
        }))
    }

    fn update(&self) -> Result<(), NucleusError> {
        let Ok(mut shared_state) = self.shared_state.get() else {
            return Ok(());
        };

        shared_state.number += 1.0;
        shared_state.ready_for_work = true;

        if self.start_instant.elapsed() > Duration::from_secs(3) {
            let mut doer_control = self.doer_control.get()?;

            doer_control
                .message_queue
                .push(DoerControlMessage::AddDoerToEnd(Box::new(SharedDoer2::new)));

            doer_control
                .message_queue
                .push(DoerControlMessage::RemoveDoer(TypeId::of::<SharedDoer1>()));
        }

        Ok(())
    }
}

fn fun_thread(shared_state: SharedState<SharedStore>) -> Result<(), NucleusError> {
    let mut nucleus = Nucleus::new(shared_state)?;
    nucleus.add_doer::<LoopTimingManager>()?;
    nucleus.run()
}

pub struct SharedDoer2 {
    shared_state: SharedState<SomeSharedState>,
    timing: State<LoopTiming>,
    thread_requests: SharedState<ThreadRequest>,
}

impl DoerTrait for SharedDoer2 {
    fn new(nucleus: &Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>
    where
        Self: Sized,
    {
        let shared_state = nucleus.state.shared.blocking_get()?;
        let local_state = nucleus.state.local.get()?;

        shared_state_wait!(
            shared_state,
            SomeSharedState;
            Duration::from_millis(100),
            Duration::MAX
        );

        Ok(Box::new(SharedDoer2 {
            timing: local_state.get_state::<LoopTiming>()?,
            shared_state: shared_state.get_state::<SomeSharedState>()?,
            thread_requests: shared_state.get_state::<ThreadRequest>()?,
        }))
    }

    fn first(&self, nucleus: &Nucleus) -> Result<(), NucleusError> {
        let mut doer_control = nucleus.doers.control.get()?;
        let mut thread_requests = self.thread_requests.blocking_get()?;

        doer_control
            .message_queue
            .push(DoerControlMessage::RemoveDoer(TypeId::of::<SharedDoer2>()));

        thread_requests.add_thread(fun_thread);

        Ok(())
    }

    fn update(&self) -> Result<(), NucleusError> {
        let Ok(mut shared_state) = self.shared_state.get() else {
            self.timing.get()?.loop_sleep_duration += Duration::from_millis(5);

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
    let mut nucleus = Nucleus::with_shared_state()?;
    nucleus.add_doer::<LoopTimingManager>()?;
    nucleus.add_doer::<SharedDoer1>()?;
    nucleus.add_doer::<SharedDoer2>()?;
    nucleus.add_doer::<ThreadManager>()?;

    nucleus.run()
}
