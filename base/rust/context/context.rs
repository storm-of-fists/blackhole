use arrayvec::ArrayString;
use base::log;
use clap::Parser;
use raw_pointer::RawPointer;

pub type ApplicationName = ArrayString<100>;

/// Arguments for the rocket program.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ContextArgs {
    /// Name of the rocket.
    #[arg(short, long)]
    application_id: ApplicationName,

    /// Name of the rocket.
    #[arg(short, long)]
    asset_id: ApplicationName,
}

pub struct Context {
    application_id: ApplicationName,
    asset_id: ApplicationName
}

impl Context {
    pub fn new() -> Self {
        let args = ContextArgs::parse();
        log::init();

        Self {
            application_id: args.application_id,
            asset_id: args.asset_id,
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