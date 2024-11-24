use std::time::{Duration, Instant};

use pm::*;

pub struct DoerA {
    start: Instant,
}

impl DoerTrait for DoerA {
    fn new_state(_state: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        println!("DoerA is adding new state.");

        Ok(())
    }

    fn new(_pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        println!("DoerA is in new.");

        Ok(Box::new(DoerA {
            start: Instant::now(),
        }))
    }

    fn first(&self, _pm: &Pm) -> Result<(), PmError> {
        println!("DoerA is in first.");

        Ok(())
    }

    fn update(&self) -> Result<(), PmError> {
        println!("DoerA is in update.");

        std::thread::sleep(Duration::from_secs(1));

        if self.start.elapsed() > Duration::from_secs(3) {
            return Err(PmError::DoerUpdate);
        }

        Ok(())
    }

    fn remove(&self) -> Result<(), PmError> {
        println!("DoerA is in remove.");

        Ok(())
    }
}

pub struct DoerB {
    start: Instant,
}

impl DoerTrait for DoerB {
    fn new_state(_state: &StateStore) -> Result<(), PmError>
    where
        Self: Sized,
    {
        println!("DoerB is adding new state.");

        Ok(())
    }

    fn new(_pm: &Pm) -> Result<Box<dyn DoerTrait>, PmError>
    where
        Self: Sized,
    {
        println!("DoerB is in new.");

        Ok(Box::new(DoerB {
            start: Instant::now(),
        }))
    }

    fn first(&self, _pm: &Pm) -> Result<(), PmError> {
        println!("DoerB is in first.");

        Ok(())
    }

    fn update(&self) -> Result<(), PmError> {
        println!("DoerB is in update.");

        std::thread::sleep(Duration::from_secs(1));

        if self.start.elapsed() > Duration::from_secs(7) {
            return Err(PmError::DoerUpdate);
        }

        Ok(())
    }

    fn remove(&self) -> Result<(), PmError> {
        println!("DoerB is in remove.");

        Ok(())
    }
}

fn main() -> Result<(), PmError> {
    let mut pm = pm!(DoerA, DoerB);
    pm.run()
}
