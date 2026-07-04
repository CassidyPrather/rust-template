//! Rust template crate.
//!
//! The CLI is split into two layers:
//!
//! - [`cli`] owns argument parsing, logging setup, and dispatch.
//! - [`commands`] holds one module per subcommand.

pub mod cli;
pub mod commands;
