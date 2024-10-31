use nucleus::Nucleus;
use std::time::Duration;

pub trait UpdaterTrait: 'static {
    fn register(nucleus: Nucleus) where Self: Sized;
    /// TODO(MakeFallible)
    fn new(nucleus: Nucleus) -> Self where Self: Sized;
    /// This must be implemented in some way, otherwise why are you
    /// even using a function in the first place?
    fn update(&self);
}
