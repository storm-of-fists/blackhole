use std::sync::atomic::{AtomicU64, Ordering};
use std::ops::Deref;

macro_rules! custom_token {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u64);

        impl $name {
            pub fn new() -> Self {
                static COUNTER: AtomicU64 = AtomicU64::new(1);

                return Self(COUNTER.fetch_add(1, Ordering::Relaxed));
            }

            pub const unsafe fn from_raw(value: u64) -> Self {
                return Self(value);
            }
        }

        impl Deref for $name {
            type Target = u64;

            fn deref(&self) -> &Self::Target {
                return &self.0;
            }
        }
    }
}

custom_token!(Token);

