use clap::Parser;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::io::{stdout, stdin};
use std::io::prelude::*;
use std::env;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Config};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet.
    #[arg()]
    pattern: String,

    /// Number of times to greet
    #[arg()]
    file_to_read_path: PathBuf,
}

fn main() {
    let args = Args::parse();
}
