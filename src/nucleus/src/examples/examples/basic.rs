use std::time::{Duration, Instant};

use nucleus::*;

pub struct DoerA {
    start: Instant,
}

impl DoerTrait for DoerA {
    fn new_state(_state: &StateStore) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        println!("DoerA is adding new state.");

        Ok(())
    }

    fn new(_nucleus: &Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>
    where
        Self: Sized,
    {
        println!("DoerA is in new.");

        Ok(Box::new(DoerA {
            start: Instant::now(),
        }))
    }

    fn first(&self, _nucleus: &Nucleus) -> Result<(), NucleusError> {
        println!("DoerA is in first.");

        Ok(())
    }

    fn update(&self) -> Result<(), NucleusError> {
        println!("DoerA is in update.");

        std::thread::sleep(Duration::from_secs(1));

        if self.start.elapsed() > Duration::from_secs(3) {
            return Err(NucleusError::DoerUpdate);
        }

        Ok(())
    }

    fn remove(&self) -> Result<(), NucleusError> {
        println!("DoerA is in remove.");

        Ok(())
    }
}

pub struct DoerB {
    start: Instant,
}

impl DoerTrait for DoerB {
    fn new_state(_state: &StateStore) -> Result<(), NucleusError>
    where
        Self: Sized,
    {
        println!("DoerB is adding new state.");

        Ok(())
    }

    fn new(_nucleus: &Nucleus) -> Result<Box<dyn DoerTrait>, NucleusError>
    where
        Self: Sized,
    {
        println!("DoerB is in new.");

        Ok(Box::new(DoerB {
            start: Instant::now(),
        }))
    }

    fn first(&self, _nucleus: &Nucleus) -> Result<(), NucleusError> {
        println!("DoerB is in first.");

        Ok(())
    }

    fn update(&self) -> Result<(), NucleusError> {
        println!("DoerB is in update.");

        std::thread::sleep(Duration::from_secs(1));

        if self.start.elapsed() > Duration::from_secs(7) {
            return Err(NucleusError::DoerUpdate);
        }

        Ok(())
    }

    fn remove(&self) -> Result<(), NucleusError> {
        println!("DoerB is in remove.");

        Ok(())
    }
}

fn main() -> Result<(), NucleusError> {
    let mut nucleus = nucleus!(DoerA, DoerB);
    nucleus.run()
}
