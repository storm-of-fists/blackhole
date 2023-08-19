use env_logger as logger;
pub use log::*;

pub fn default() -> logger::Builder {
    let mut logger = custom();
    logger.format_timestamp_micros();

    return logger;
}

pub fn custom() -> logger::Builder {
    return logger::Builder::from_env(logger::Env::default().default_filter_or("info"));
}

