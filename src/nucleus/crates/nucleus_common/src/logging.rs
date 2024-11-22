use env_logger::{Builder, Logger};
use nucleus::*;
use std::{
    fs::File,
    io::Write,
    time::{Duration, Instant},
};

pub struct LoggingManager {}

impl UpdaterTrait for LoggingManager {
    fn new(_nucleus: &Nucleus) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        Builder::new()
            .default_format()
            .try_init()
            .map_err(|_| NucleusError::NewUpdater)?;

        Ok(Box::new(Self {}))
    }
}
