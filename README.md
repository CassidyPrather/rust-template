# Rust Template

Cassidy's opinionated Rust template. Enter at your own peril.

## Usage

<!-- generated-usage:start -->

```text
Cassidy's opinionated Rust template

Usage: rust-template [OPTIONS] [COMMAND]

Commands:
  version  Display package version
  example  Example subcommand (replace with your own)
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Increase verbosity (can be repeated: -v, -vv, -vvv)

  -h, --help
          Print help

  -V, --version
          Print version
```

<!-- generated-usage:end -->


## Development

Requires [Rust](https://rustup.rs/)

Build: `cargo build`

Run: `cargo run -- --help`

Lint: `cargo clippy --all-targets --all-features -- -D warnings`

Format: `cargo fmt`

Test: `cargo test`

Regenerate docs: `cargo run --bin gen-docs`

### Hooks

Formatting and doc regeneration run automatically, so there's nothing to
remember:

- **Humans**: enable once per clone with `git config core.hooksPath .githooks`.
  The pre-commit hook runs `cargo fmt` and `gen-docs`, and fails the commit
  if they changed anything — review and re-stage.
- **Claude Code**: the PostToolUse hook in `.claude/settings.json` runs both
  after every Rust source edit.
- **CI** stays the backstop for anyone with neither.

###  Advanced

Benchmark: `cargo bench` (HTML report at `target/criterion/report/index.html`;
fast sanity pass: `cargo bench --bench cli_bench -- --quick`)

Security audit: `cargo audit` (requires `cargo install cargo-audit`)

## Versioning

`build.rs` embeds `git describe` output at compile time, so the binary
reports how it relates to the last release (the hatch-vcs experience):

| Build state             | `--version` reports                            |
| ----------------------- | ---------------------------------------------- |
| Exactly on tag `v1.2.3` | `1.2.3`                                        |
| 4 commits past `v1.2.3` | `1.2.3-4-gabc1234`                             |
| Uncommitted changes     | previous, plus `-dirty`                        |
| No tags yet             | `0.1.0+gabc1234` (Cargo.toml version + commit) |
| No git at all (tarball) | `0.1.0` (Cargo.toml version)                   |

Keep Cargo.toml's `version` in sync with tags when you release;
`cargo-release` automates the bump-tag-push dance if you want it.

## Template Setup

1. **Create a new repository** from this template on GitHub (click "Use this template")

2. **Clone your new repository** and navigate to it

3. **Update project metadata** in `Cargo.toml`:
   - Change `name` to your project name (use kebab-case, e.g., `my-awesome-tool`)
   - Update the `[lib]` name to match (snake_case, e.g., `my_awesome_tool`)
   - Update the first `[[bin]]` name and `default-run` to match (kebab-case)
   - Update `authors`, `description`, `repository`, `keywords`, and `categories`
   - Update dependencies as required

4. **Update the crate references in source**:
   - `src/main.rs`: change `rust_template::cli::run()` to `your_crate_name::cli::run()`
   - `src/bin/gen_docs.rs` and `benches/cli_bench.rs`: update the `use rust_template::...` imports
   - Everything else uses `crate::` paths or `env!("CARGO_PKG_NAME")` and
     picks up the rename automatically

5. **Update `.github/workflows/ci-cd.yml`**:
   - Change `BIN="rust-template"` in the Package binary step to your binary name

6. **Update `.vscode/launch.json`** (if using VS Code):
   - Replace `rust-template` / `rust_template` in the `cargo` sections

7. **Update the README**:
   - Replace the title and description
   - Remove or customize this Template Setup section

8. **Verify everything works**:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --check
   cargo run --bin gen-docs
   ```

## Tracked considerations

Deliberately not enabled; opt in per project:

- **Fail CI on `cargo audit`** — delete the `|| true` in `ci-cd.yml`.
- **cargo-deny** — audit plus license allowlists and duplicate-dep bans.
- **Publish to crates.io** — remove `publish = false`, add a
  `CARGO_REGISTRY_TOKEN` secret and a publish step (or `cargo-release` /
  `release-plz` for the whole tag-bump-publish flow).
- **cargo-dist** — generated installers (shell/PowerShell/Homebrew/MSI)
  replacing the hand-rolled release matrix; adopt when installer
  distribution matters.
- **MSRV enforcement** — `rust-version` is declared but unverified; add a
  `cargo-msrv verify` job if downstream consumers care.
- **Bench regression gates** — criterion baselines are local-only; CI
  tracking needs Bencher/CodSpeed and a quiet runner.
- **Named frames in release backtraces** — `strip = "symbols"` favors
  size; switch to `"debuginfo"` for debuggable crash reports.
