use base::log;
use clap::{Command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct FooArgs {
    #[command(flatten)]
    common: CommonArgs,

    /// Name of the rocket.
    #[arg(long)]
    bar: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommonArgs {
    /// Name of the rocket.
    #[arg(long)]
    foo1: String,

    #[arg(long)]
    foo2: String,

    #[arg(long)]
    foo3: String,
}

fn main() {
    log::default().init();
    let args = FooArgs::parse();
    log::info!("{:?}", args.common);
}