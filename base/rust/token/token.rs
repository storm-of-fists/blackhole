use std::sync::atomic::{AtomicU64, Ordering};
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token(u64);

impl Token {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);

        return Token(COUNTER.fetch_add(1, Ordering::Relaxed));
    }

    pub const fn from_raw(value: u64) -> Self {
        return Token(value);
    }
}

impl Deref for Token {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}