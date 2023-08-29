#![feature(async_fn_in_trait)]

use context::ContextPtr;

pub enum ComponentLifecycleState {
    Create,
    Start,
    Running,
    Stop,
    Destroy,
    Disabled,
}

pub struct Component<T> {
    context: ContextPtr,
    inner_machine: T,
    lifecycle_state: ComponentLifecycleState,
}

impl<T> Component<T>
where
    T: ComponentTrait,
{
    pub fn context(&self) -> ContextPtr {
        self.context
    }

    pub fn lifecycle_state(&self) -> &ComponentLifecycleState {
        &self.lifecycle_state
    }

    pub fn set_lifecycle_state(&mut self, state: ComponentLifecycleState) {
        self.lifecycle_state = state;
    }

    pub async fn create(context: ContextPtr, inner_machine: T) -> Self {
        Self {
            inner_machine,
            context,
            lifecycle_state: ComponentLifecycleState::Create,
        }
    }

    pub async fn start(&mut self) {
        self.inner_machine.start().await;
    }

    pub fn dispatch(&mut self) {
        self.inner_machine.dispatch();
    }

    pub async fn stop(&mut self) {
        self.inner_machine.stop().await;
    }

    pub async fn destroy(mut self) -> Self {
        self.inner_machine.destroy().await;
        self
    }

    // pub set_enabled(&mut self, enabled: bool) {

    // }
}

pub trait ComponentTrait: Sized {
    async fn create(context: ContextPtr) -> Self;

    async fn start(&mut self) {
        return;
    }

    fn dispatch(&mut self) {
        return;
    }

    async fn stop(&mut self) {
        return;
    }

    async fn destroy(&mut self) {
        return;
    }
}
