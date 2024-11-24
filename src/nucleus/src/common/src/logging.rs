use env_logger::{Builder, Logger};
use pm::*;
use std::{
    fs::File,
    io::Write,
    time::{Duration, Instant},
};

pub struct LoggingManager {}

impl DoerTrait for LoggingManager {
    fn new(_pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        Builder::new()
            .default_format()
            .try_init()
            .map_err(|_| PmError::NewDoer)?;

        Ok(Box::new(Self {}))
    }
}
