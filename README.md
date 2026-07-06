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

This section and `SKILL.md` are generated from the clap definition by
`cargo run --bin gen-docs`. CI fails if they drift from the committed
copies, so regenerate after any CLI change.

## Development

Requires [Rust](https://rustup.rs/) (the pinned toolchain in
`rust-toolchain.toml` is installed automatically by rustup)

Build: `cargo build`

Run: `cargo run -- --help`

Lint: `cargo clippy --all-targets --all-features -- -D warnings`

Format: `cargo fmt`

Test: `cargo test`

Benchmark: `cargo bench` (HTML report at `target/criterion/report/index.html`;
fast sanity pass: `cargo bench --bench cli_bench -- --quick`)

Regenerate docs: `cargo run --bin gen-docs`

Security audit: `cargo audit` (requires `cargo install cargo-audit`)

Type checking is the compiler's job — there is no separate step. Fast
feedback without a full build: `cargo check`.

## Versioning

`build.rs` embeds `git describe` output at compile time, so the binary
reports how it relates to the last release (the hatch-vcs experience):

| Build state | `--version` reports |
|---|---|
| Exactly on tag `v1.2.3` | `1.2.3` |
| 4 commits past `v1.2.3` | `1.2.3-4-gabc1234` |
| Uncommitted changes | previous, plus `-dirty` |
| No tags yet | `0.1.0+gabc1234` (Cargo.toml version + commit) |
| No git at all (tarball) | `0.1.0` (Cargo.toml version) |

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

## Layout

- `src/main.rs` — thin binary entry point
- `src/lib.rs` — library root, so tests and downstream code can reuse everything
- `src/cli.rs` — argument parsing (clap), logging setup, dispatch
- `src/commands/` — one module per subcommand; copy `example.rs` to add one
- `src/bin/gen_docs.rs` — regenerates the Usage section above and `SKILL.md`
- `build.rs` — embeds the `git describe` version
- `tests/cli.rs` — end-to-end tests that run the compiled binary
- `benches/cli_bench.rs` — criterion benchmark scaffold

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
- **Docs are generated**: the Usage section and `SKILL.md` come from the
  clap definition via `gen-docs`; CI rejects drift.

## Tracked considerations

Deliberately **not** enabled, so each project can opt in with eyes open:

- **Failing CI on `cargo audit` findings** — the audit step is
  warnings-only (`|| true` in `ci-cd.yml`). Delete the `|| true` to make
  vulnerable dependencies block merges.
- **cargo-deny** — a superset of `cargo audit`: advisories plus license
  allowlists, source pinning, and duplicate-dependency bans. Start with
  `cargo deny init` and swap it into the audit step.
- **Publishing to crates.io** — `publish = false` in `Cargo.toml` blocks
  accidental publication. To ship a crate: remove that line, add a
  `CARGO_REGISTRY_TOKEN` secret, and add a `cargo publish` step to the
  release job (or adopt `cargo-release` / `release-plz` to drive the
  whole tag-bump-publish flow).
- **cargo-dist** — replaces the hand-rolled release matrix with generated
  installers (shell/PowerShell scripts, Homebrew, MSI), checksums, and
  more targets. More capable, but it owns your release workflow YAML and
  its maintenance story has been bumpier since axo.dev wound down; the
  hand-rolled matrix stays until the installer features are needed.
- **MSRV enforcement** — `rust-version` is declared in `Cargo.toml` but
  nothing verifies it; add a `cargo-msrv verify` CI job if the crate has
  downstream consumers who care.
- **Benchmark regression gates** — criterion runs locally with manual
  baselines (`cargo bench -- --save-baseline main`). CI-enforced
  regression tracking needs a service or action (Bencher, CodSpeed,
  `criterion-compare`), plus a quiet runner to be meaningful.
- **Binary size budget** — `lto`/`strip` are on; if size matters, track
  it with `cargo bloat` and consider `opt-level = "z"` / `panic = "abort"`.
