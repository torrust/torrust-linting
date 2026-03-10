# Torrust Linting

[![Linting](https://github.com/torrust/torrust-linting/actions/workflows/linting.yml/badge.svg)](https://github.com/torrust/torrust-linting/actions/workflows/linting.yml)

A unified linting library and CLI for Torrust projects. Provides a single binary that runs all project linters consistently across the organization.

## Features

- **Multiple Linters**: Markdown, YAML, TOML, Rust (clippy + rustfmt), shell scripts, and spell checking
- **CLI Ready**: Pre-built CLI binary (`linter`) with subcommands for each linter
- **Library**: Use individual linter functions in your own Rust code or build a custom CLI
- **Extensible**: Easy to add new linters
- **Auto-install**: Automatically installs missing tools (npm-based, system package manager)

## Usage

### As a binary

Run the `linter` binary directly:

```sh
cargo run             # Run all linters (default)
cargo run -- markdown   # Run markdown linter
cargo run -- yaml       # Run YAML linter
cargo run -- toml       # Run TOML linter
cargo run -- cspell     # Run CSpell spell checker
cargo run -- clippy     # Run Rust clippy linter
cargo run -- rustfmt    # Run Rust formatter check
cargo run -- shellcheck # Run ShellCheck linter
cargo run -- all        # Run all linters
```

### As a library dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
torrust-linting = { git = "https://github.com/torrust/torrust-linting" }
```

#### Option 1: Use the complete CLI (easiest)

```rust
use anyhow::Result;

fn main() -> Result<()> {
    torrust_linting::run_cli()
}
```

#### Option 2: Custom CLI implementation

```rust
use anyhow::Result;
use clap::Parser;
use torrust_linting::{Cli, execute_command, init_tracing};

fn main() -> Result<()> {
    init_tracing();
    let cli = Cli::parse();
    execute_command(cli.command.as_ref())?;
    Ok(())
}
```

#### Option 3: Use individual linters programmatically

```rust
use anyhow::Result;
use torrust_linting::{run_rustfmt_linter, run_clippy_linter, run_all_linters};

fn main() -> Result<()> {
    run_rustfmt_linter()?;
    run_clippy_linter()?;
    // Or run everything at once:
    run_all_linters()?;
    Ok(())
}
```

See the [`examples/`](examples/) directory for more usage patterns.

## External tools required

Some linters delegate to external tools. The library will attempt to install missing tools automatically, but you can also install them manually:

| Linter      | Tool                        | Install                                          |
| ----------- | --------------------------- | ------------------------------------------------ |
| Markdown    | `markdownlint`              | `npm install -g markdownlint-cli`                |
| YAML        | `yamllint`                  | `apt install yamllint` / `pip3 install yamllint` |
| TOML        | `taplo`                     | `cargo install taplo-cli --locked`               |
| Spell check | `cspell`                    | `npm install -g cspell`                          |
| Shell       | `shellcheck`                | `apt install shellcheck`                         |
| Rust        | `cargo clippy`, `cargo fmt` | bundled with `rustup`                            |

## Configuration files expected in the project root

The linters read configuration from well-known files in the working directory:

| File                 | Used by      |
| -------------------- | ------------ |
| `.markdownlint.json` | markdownlint |
| `.yamllint-ci.yml`   | yamllint     |
| `.taplo.toml`        | taplo        |
| `cspell.json`        | cspell       |

## License

Copyright (c) 2026 Torrust. See [LICENSE](LICENSE).
