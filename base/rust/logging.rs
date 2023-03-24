use env_logger as logger;
pub use log::*;

pub fn init() {
    logger::builder().format_timestamp_micros().init();
}

