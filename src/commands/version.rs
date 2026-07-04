//! Version subcommand - display package version.

use anyhow::Result;
use log::info;

/// Execute the version command.
///
/// # Errors
///
/// Never fails; the `Result` return keeps the signature uniform across
/// subcommands.
pub fn run() -> Result<()> {
    info!("Displaying version information");
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}
