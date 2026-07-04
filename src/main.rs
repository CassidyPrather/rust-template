//! Binary entry point.
//!
//! Kept as thin as possible so all real logic lives in the library,
//! where unit and integration tests can reach it.

use std::process::ExitCode;

fn main() -> ExitCode {
    rust_template::cli::run()
}
