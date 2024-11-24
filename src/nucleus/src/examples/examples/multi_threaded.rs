#![feature(portable_simd)]

use std::{
    any::{type_name, TypeId},
    time::{Duration, Instant},
};

use pm::*;
use pm_common::{
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
    fn new_state(state: &StateStore) -> Result<(), PmError>
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

    fn new(pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        let shared_state = pm.state.shared.blocking_get()?;

        Ok(Box::new(SharedDoer1 {
            shared_state: shared_state.get_state::<SomeSharedState>()?,
            start_instant: Instant::now(),
            doer_control: pm.doers.control.clone(),
        }))
    }

    fn update(&self) -> Result<(), PmError> {
        let shared_state = self.shared_state.blocking_get()?;

        shared_state.number += 1.0;
        shared_state.ready_for_work = true;

        if self.start_instant.elapsed() > Duration::from_secs(3) {
            let mut doer_control = self.doer_control.get()?;

            doer_control
                .message_queue
                .push(DoerControl::AddDoerToEnd(Box::new(SharedDoer2::new)));

            doer_control
                .message_queue
                .push(DoerControl::RemoveDoer(type_name::<SharedDoer1>()));
        }

        Ok(())
    }
}

fn fun_thread(shared_state: SharedState<SharedStore>) -> Result<(), PmError> {
    let mut pm = Pm::new(shared_state)?;
    pm.doer::<LoopTimingManager>()?;
    pm.run()
}

pub struct SharedDoer2 {
    shared_state: SharedState<SomeSharedState>,
    timing: State<LoopTiming>,
    thread_requests: SharedState<ThreadRequest>,
}

impl DoerTrait for SharedDoer2 {
    fn new(pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        let shared_state = pm.state.shared.blocking_get()?;
        let local_state = pm.state.local.get()?;

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

    fn first(&self, pm: &Pm) -> Result<(), PmError> {
        let mut doer_control = pm.doers.control.get()?;
        let mut thread_requests = self.thread_requests.blocking_get()?;

        doer_control
            .message_queue
            .push(DoerControl::RemoveDoer(TypeId::of::<SharedDoer2>()));

        thread_requests.add_thread(fun_thread);

        Ok(())
    }

    fn update(&self) -> Result<(), PmError> {
        let mut shared_state = self.shared_state.blocking_get()?;

        if shared_state.ready_for_work {
            shared_state.number -= 1.0;
            shared_state.ready_for_work = false;
        }

        Ok(())
    }
}

fn main() -> Result<(), PmError> {
    let mut pm = pm!(LoopTimingManager, SharedDoer1, SharedDoer2, ThreadManager);
    pm.run()
}
