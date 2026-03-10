# AGENTS.md

Instructions for AI coding agents working on `torrust-linting`.

## Project overview

`torrust-linting` is a Rust library and CLI binary that runs all standard Torrust project linters in one command. It wraps external tools (markdownlint, yamllint, taplo, cspell, shellcheck) and Rust built-ins (clippy, rustfmt) behind a unified interface.

- **Crate type**: hybrid — library (`torrust_linting`) + binary (`linter`)
- **Rust edition**: 2021, `rust-version = "1.85"`
- **License**: MIT

## Build commands

```sh
cargo build                        # Build library + binary
cargo build --bin linter           # Build only the CLI binary
cargo build --release              # Release build
```

## Run the linter

```sh
cargo run                          # Run all linters (default)
cargo run -- all                   # Explicit "all"
cargo run -- markdown              # Single linter
cargo run -- yaml
cargo run -- toml
cargo run -- cspell
cargo run -- clippy
cargo run -- rustfmt
cargo run -- shellcheck
```

After a build you can also run the binary directly:

```sh
./target/debug/linter all
```

## Testing

There are no unit tests at the moment. The primary verification is running the linter against itself:

```sh
cargo run   # Must exit 0 — this is the CI gate
```

**This is mandatory before every commit.** The CI workflow (`.github/workflows/linting.yml`) runs the exact same command and will reject any PR that does not pass. Do not open a PR without confirming exit code 0 locally first.

## Code style

- Format with `cargo fmt` before committing. The project uses a custom `rustfmt.toml` (`group_imports = "StdExternalCrate"`, `max_width = 130`).
- No clippy warnings allowed (`cargo clippy -- -D warnings`).
- TOML files must pass `taplo fmt --check **/*.toml`. Auto-fix with `taplo fmt **/*.toml`.
- Markdown files must pass `markdownlint`.
- YAML files must pass `yamllint -c .yamllint-ci.yml`.
- Spell-check all files with `cspell`. Add new technical terms to `project-words.txt` (one word per line, alphabetical order).

## Adding a new linter

1. Create `src/linters/<name>.rs` with a public `run_<name>_linter() -> Result<()>` function.
2. Register it in `src/linters/mod.rs` (`pub mod <name>; pub use <name>::run_<name>_linter;`).
3. Re-export from `src/lib.rs`.
4. Add a `Commands` variant in `src/cli.rs` and handle it in `execute_command()`.
5. Add the call to `run_all_linters()` in `src/cli.rs`.

## Configuration files

Place these in the project root when using this linter on another repo:

| File                 | Used by      |
| -------------------- | ------------ |
| `.markdownlint.json` | markdownlint |
| `.yamllint-ci.yml`   | yamllint     |
| `.taplo.toml`        | taplo        |
| `cspell.json`        | cspell       |

## PR instructions

- **Run `cargo run` and confirm exit code 0 before every commit and before opening a PR. PRs that fail the linting CI will be rejected.**
- Branch naming: `feat/<short-description>`, `fix/<short-description>`, `chore/<short-description>`.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`.
- Keep `Cargo.lock` committed (the crate ships a binary).
