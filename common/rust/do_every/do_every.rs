#[macro_export]
macro_rules! do_every_duration {
    ($clock:expr, $duration:expr, $do_every:expr) => {
        static mut LAST_DONE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let current_time = $clock.get_time().as_micros() as u64;

        unsafe {
            if current_time - LAST_DONE.get_mut() > $duration.as_micros() as u64 {
                LAST_DONE.store(current_time, std::sync::atomic::Ordering::Relaxed);
                $do_every;
            }
        }
    };

    ($duration:expr, $do_every:expr) => {
        static mut LAST_DONE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        unsafe {
            if current_time - LAST_DONE.get_mut().clone() > $duration.as_micros() as u64 {
                LAST_DONE.store(current_time, std::sync::atomic::Ordering::Relaxed);
                $do_every;
            }
        }

    };
}

fn main() {
    loop {
        do_every_duration!(std::time::Duration::from_millis(500), {
            println!("{:?}", std::time::SystemTime::now());
        });
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
