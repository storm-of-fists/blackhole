pub mod logging {
    use env_logger as logger;
    pub use log;

    pub fn init_logging() {
        logger::init();
    }
}
