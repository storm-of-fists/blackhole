use env_logger as logger;
pub use eyre::*;
pub use log;

pub fn init_logging() {
    logger::init();
}