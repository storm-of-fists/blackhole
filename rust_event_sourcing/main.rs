use base::{log, logger};
use std::path::PathBuf;

const OUTPUT: &str = "/home/connor/code/rust_event_sourcing/output/events.json";

fn main() -> base::Result<()> {
    let output_path = PathBuf::from(OUTPUT);
    logger::init();
    log::info!("asd");

    return Ok(());
}
