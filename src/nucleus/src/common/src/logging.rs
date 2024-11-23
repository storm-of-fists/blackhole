use env_logger::{Builder, Logger};
use nucleus::*;
use std::{
    fs::File,
    io::Write,
    time::{Duration, Instant},
};

pub struct LoggingManager {}

impl DoerTrait for LoggingManager {
    fn new(_nucleus: &Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>
    where
        Self: Sized,
    {
        Builder::new()
            .default_format()
            .try_init()
            .map_err(|_| NucleusError::NewDoer)?;

        Ok(Box::new(Self {}))
    }
}
