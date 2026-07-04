# Rust Template

Cassidy's opinionated Rust template. Enter at your own peril.

## Development

Requires [Rust](https://rustup.rs/) (the pinned toolchain in
`rust-toolchain.toml` is installed automatically by rustup)

Build: `cargo build`

Run: `cargo run -- --help`

Lint: `cargo clippy --all-targets --all-features -- -D warnings`

Format: `cargo fmt`

Test: `cargo test`

Security audit: `cargo audit` (requires `cargo install cargo-audit`)

Type checking is the compiler's job — there is no separate step. Fast
feedback without a full build: `cargo check`.

## Template Setup

1. **Create a new repository** from this template on GitHub (click "Use this template")

2. **Clone your new repository** and navigate to it

3. **Update project metadata** in `Cargo.toml`:
   - Change `name` to your project name (use kebab-case, e.g., `my-awesome-tool`)
   - Update the `[lib]` name to match (snake_case, e.g., `my_awesome_tool`)
   - Update the `[[bin]]` name to match (kebab-case)
   - Update `authors` and `description` with your information
   - Update dependencies as required

4. **Update the crate references in source**:
   - `src/main.rs`: change `rust_template::cli::run()` to `your_crate_name::cli::run()`
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
   ```

## Layout

- `src/main.rs` — thin binary entry point
- `src/lib.rs` — library root, so tests and downstream code can reuse everything
- `src/cli.rs` — argument parsing (clap), logging setup, dispatch
- `src/commands/` — one module per subcommand; copy `example.rs` to add one
- `tests/cli.rs` — end-to-end tests that run the compiled binary

## Conventions baked in

- **Clippy at maximum grump**: `pedantic`, `nursery`, and `cargo` lint
  groups are enabled in `Cargo.toml`, and CI runs with `-D warnings`.
  Targeted `allow`s go in `[lints.clippy]` with a justifying comment.
- **`unsafe` is forbidden** crate-wide. Delete the `unsafe_code = "forbid"`
  line if you genuinely need it (you probably don't).
- **`Cargo.lock` is committed** — this builds a binary, and binaries want
  reproducible dependency trees.
- **Logging**: `-v`/`-vv`/`-vvv` map to INFO/DEBUG/TRACE (default WARN);
  `RUST_LOG` overrides for full [env_logger](https://docs.rs/env_logger)
  filter syntax.
- **Errors**: subcommands return `anyhow::Result<()>`; the CLI layer logs
  the error chain and converts it to a non-zero exit code.
- **Releases**: publishing a GitHub release builds and attaches binaries
  for Linux (x86_64), macOS (arm64), and Windows (x86_64).
