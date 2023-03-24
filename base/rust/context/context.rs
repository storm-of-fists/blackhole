use arrayvec::ArrayString;
use base::log;
use clap::Parser;
use raw_pointer::RawPointer;

pub type ApplicationName = ArrayString<100>;

/// Arguments for the rocket program.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the rocket.
    #[arg(short, long)]
    application_name: ApplicationName,
}

pub struct Context {
    application_name: ApplicationName,
}

impl Context {
    pub fn new() -> Self {
        let args = Args::parse();
        log::init();

        Self {
            application_name: args.application_name,
        }
    }
}

pub type ContextPtr = RawPointer<Context>;

#[macro_export]
macro_rules! init_context {
    () => {{
        use context::Context;

        let mut context = Context::new();
        create_raw_ptr!(context)
    }}
}