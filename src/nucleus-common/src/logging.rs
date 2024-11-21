use nucleus::*;
use std::{fs::File, io::Write, time::{Duration, Instant}};

#[derive(StateTrait)]
pub struct Logging {
    log_file: File,
    log_string: String,
    instant: Instant,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            log_file: File::create("logging.txt").unwrap(),
            log_string: String::new(),
            instant: Instant::now(),
        }
    }

    pub fn log(&mut self, msg: &str) {
        self.log_string.push_str(msg);
        self.log_string.push('\n');
    }

    pub fn write_to_file(&mut self) {
        self.log_file.write(&self.log_string.as_bytes()).unwrap();
        self.log_string.clear();
    }
}

pub struct LoggingManager {
    log: State<Logging>,
}

impl UpdaterTrait for LoggingManager {
    fn add_new_state(
        _shared_state: SharedState<SharedStateStore>,
        thread_state: State<StateStore>,
    ) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        thread_state.get_mut()?.add_state(Logging::new())
    }

    fn new(thread: &Thread) -> Result<Box<dyn UpdaterTrait>, NucleusError>
    where
        Self: Sized,
    {
        Ok(Box::new(Self {
            log: thread.local_state.get()?.get_state::<Logging>()?,
        }))
    }

    fn update(&self) -> Result<(), NucleusError> {
        let mut log = self.log.get_mut()?;
        let elapsed_secs = log.instant.elapsed().as_secs();

        if elapsed_secs % 10 == 0 {
            log.write_to_file();
        } 
        Ok(())
    }
}
