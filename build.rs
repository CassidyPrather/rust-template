//! Embed a `git describe`-based version at compile time.
//!
//! This is the Rust analogue of hatch-vcs: a binary built exactly on a tag
//! reports the tag (`1.2.3`), while a development build several commits
//! past the tag reports how it has mutated (`1.2.3-4-gabc1234`, plus
//! `-dirty` for uncommitted changes). Builds without git history (source
//! tarballs, `cargo install` from a registry) fall back to Cargo.toml's
//! version.

use std::env;
use std::process::Command;

fn main() {
    // Rebuild when the checked-out commit, tags, or working-tree state change
    println!("cargo::rerun-if-changed=.git/HEAD");
    println!("cargo::rerun-if-changed=.git/index");
    println!("cargo::rerun-if-changed=.git/refs/tags");

    let pkg_version = env::var("CARGO_PKG_VERSION").expect("cargo always sets CARGO_PKG_VERSION");
    let version = git_describe().map_or_else(
        || pkg_version.clone(),
        |describe| {
            let describe = describe.strip_prefix('v').unwrap_or(&describe);
            if describe.contains('.') {
                // Tag-based: "1.2.3" or "1.2.3-4-gabc1234[-dirty]"
                describe.to_string()
            } else {
                // No tags yet: bare commit hash. Mimic hatch-vcs's
                // local-version style: "0.1.0+gabc1234[-dirty]"
                format!("{pkg_version}+g{describe}")
            }
        },
    );
    println!("cargo::rustc-env=GIT_DESCRIBE_VERSION={version}");
}

/// Run `git describe` and return its output, or `None` when git or the
/// repository is unavailable.
fn git_describe() -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let describe = String::from_utf8(output.stdout).ok()?;
    let describe = describe.trim();
    if describe.is_empty() {
        None
    } else {
        Some(describe.to_string())
    }
}
