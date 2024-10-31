// More info on build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html
// More examples: https://doc.rust-lang.org/cargo/reference/build-script-examples.html

// TODO(ban unwrap and expect or panic, https://github.com/rust-lang/rust-clippy/issues/6636).

/// A macro for exiting.
macro_rules! exit_build {
    ($code:expr, $explanation:expr) => {
        println!("cargo::error={}", $explanation);
        std::process::exit($code);
    };
}

fn main() {
    // Environment variables https://doc.rust-lang.org/cargo/reference/environment-variables.html 
    // let cargo_binary_path = env!("CARGO");
    // let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    // let out_dir = env!("OUT_DIR");

    // if std::env::var("DEBUG_CARGO_ENV_VARIABLES").is_ok() {
    //     println!("cargo:warning=Cargo binary path: {}", cargo_binary_path);
    //     println!("cargo:warning=Cargo manifest directory: {}", cargo_manifest_dir);
    // }

    // println!("cargo::clippy");

    let exit_code = 0;

    // This fails the build.
    if exit_code == 12 {
        exit_build!(exit_code, "piss");
    }
}
