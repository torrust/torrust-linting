---
name: publish-rust-crate
description: Publishes a new version of the torrust-linting crate to crates.io. Use this skill when the user asks to release a new version, publish the package, cut a release, bump the version, or prepare a crates.io release for this project.
license: MIT
compatibility: Requires Rust toolchain (cargo), git, and write access to crates.io and the GitHub remote at https://github.com/torrust/torrust-linting. Run inside the repository root.
metadata:
  author: torrust
  version: "1.0"
---

# Publishing `torrust-linting` to crates.io

This skill covers the full end-to-end process of releasing a new version of the
`torrust-linting` crate to [crates.io](https://crates.io/crates/torrust-linting).

Repository: <https://github.com/torrust/torrust-linting>

---

## Overview

The release process consists of six sequential phases:

1. [Pre-release checklist](#1-pre-release-checklist)
2. [Version bump](#2-version-bump)
3. [Local validation](#3-local-validation)
4. [Tag and commit](#4-tag-and-commit)
5. [Publish to crates.io](#5-publish-to-cratesio)
6. [Post-publish verification](#6-post-publish-verification)

---

## 1. Pre-release checklist

### 1.1 Verify `Cargo.toml` metadata

The `[package]` section must contain all of the following fields before publishing.
The current state is already correct — verify nothing has regressed since the last release.

```toml
[package]
name = "torrust-linting"
version = "0.1.0"          # <-- bump this in §2
edition = "2021"
rust-version = "1.85"
description = "Linting utilities for Torrust projects"
authors = ["Torrust Organization <https://torrust.com>"]
documentation = "https://docs.rs/torrust-linting"
homepage = "https://torrust.com"
repository = "https://github.com/torrust/torrust-linting"
license = "MIT"
readme = "README.md"
keywords = ["torrust", "linting", "cli"]
categories = ["development-tools", "command-line-utilities"]
include = ["/src", "/examples", "LICENSE", "README.md"]
```

Key things to check:

- `readme = "README.md"` is present — without it crates.io will not render the README on the crate page.
- `license` is a valid SPDX identifier.
- `keywords` has at most 5 entries, all lowercase.
- `categories` entries are from the [crates.io category list](https://crates.io/categories).

### 1.2 Verify `README.md` dependency snippet

The "As a library dependency" section in `README.md` must show the crates.io form
with the **new** version number — never the git source form.

**Correct:**

```toml
[dependencies]
torrust-linting = "0.2.0"   # use the new version being released
```

**Wrong (must not appear in a release):**

```toml
[dependencies]
torrust-linting = { git = "https://github.com/torrust/torrust-linting" }
```

### 1.3 Verify `Cargo.lock` is committed

`torrust-linting` ships a binary, so `Cargo.lock` must be committed for reproducible builds.

```sh
git status Cargo.lock   # must show "nothing to commit"
```

### 1.4 Inspect the package contents

Verify the `.crate` archive contains the right files and nothing unexpected:

```sh
cargo package --list
```

Expected output includes files under `src/`, `examples/`, `LICENSE`, `README.md`,
and `Cargo.toml`. It must not include `target/`, secrets, or local config files.

---

## 2. Version bump

Edit `version` in `Cargo.toml` following [Semantic Versioning](https://semver.org/):

| Change type                      | Example           |
| -------------------------------- | ----------------- |
| Backwards-compatible bug fix     | `0.1.0` → `0.1.1` |
| New backwards-compatible feature | `0.1.0` → `0.2.0` |
| Breaking change in public API    | `0.1.0` → `1.0.0` |

After bumping the version in `Cargo.toml`, also update the version string in the
`README.md` dependency snippet (§1.2).

> **Note:** While still on `0.x.y`, a MINOR bump may contain breaking changes by
> convention. Stabilize the public API before releasing `1.0.0`.

---

## 3. Local validation

Run every check locally before touching git, to avoid noisy CI failures.

### 3.1 Dry-run publish

Simulates packaging and upload without actually publishing anything:

```sh
cargo publish --dry-run
```

The last line of a successful dry run is:

```text
warning: aborting upload due to dry run
```

Any error before that line must be fixed before continuing.

### 3.2 Run all linters (mandatory CI gate)

`torrust-linting` uses its own binary as the CI gate. The linter **must exit 0**.
The same check runs in `.github/workflows/linting.yml` and will reject any push that fails.

```sh
cargo run
```

This runs: `markdownlint`, `yamllint`, `taplo`, `cspell`, `cargo clippy`, `cargo fmt`, `shellcheck`.

Common fixes:

| Linter         | Fix                                                                         |
| -------------- | --------------------------------------------------------------------------- |
| `rustfmt`      | `cargo fmt`                                                                 |
| `clippy`       | Address the warning, or add `#[allow(...)]` with a justification comment    |
| `markdownlint` | Edit the offending Markdown file                                            |
| `yamllint`     | Edit the offending YAML file                                                |
| `taplo`        | `taplo fmt **/*.toml`                                                       |
| `cspell`       | Add the word to `project-words.txt` (one word per line, alphabetical order) |
| `shellcheck`   | Fix the script per shellcheck's suggestion                                  |

---

## 4. Tag and commit

### 4.1 Commit the release preparation changes

Stage all files changed during §1–§3 in a single commit.
Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```sh
git add Cargo.toml Cargo.lock README.md
git commit -m "chore: release v0.2.0

- Bump version to 0.2.0
- Update README dependency snippet to crates.io form"
```

If preparing on a branch, name it `chore/release-v0.2.0` and open a PR to `main`.
Tag only after the commit is on `main`.

### 4.2 Create an annotated git tag

The tag name must exactly match the version in `Cargo.toml` with a `v` prefix:

```sh
git tag -a v0.2.0 -m "Release v0.2.0"
```

Do not create the tag until the release commit is final — the tag should point
to the exact commit that was published.

### 4.3 Push the commit and tag

```sh
git push origin main
git push origin v0.2.0
```

---

## 5. Publish to crates.io

### 5.1 Authenticate (once per machine)

Generate a token at <https://crates.io/settings/tokens> with the `publish-new`
and `publish-update` scopes:

```sh
cargo login
# paste your API token when prompted
```

### 5.2 Publish

```sh
cargo publish
```

The command packages the crate, verifies it compiles from the packaged source,
then uploads it. This takes up to 60 seconds.

> **Important:** Published versions cannot be deleted — only yanked. Yanking hides
> a version from `cargo add` suggestions but does not break builds that already
> pin that version. Double-check the dry run (§3.1) before running this.

---

## 6. Post-publish verification

### 6.1 Verify on crates.io

Open the crate page and confirm the new version is listed as latest, the README
renders correctly, and all metadata is shown:

<https://crates.io/crates/torrust-linting/0.2.0>

### 6.2 Verify docs.rs built successfully

docs.rs automatically builds documentation after every publish. Check the build log:

<https://docs.rs/crate/torrust-linting/0.2.0/builds>

Build failures are not fatal for the release but should be fixed in a prompt patch release.

### 6.3 Create a GitHub Release

1. Go to <https://github.com/torrust/torrust-linting/releases/new>
2. Select tag `v0.2.0`.
3. Set the title to `v0.2.0`.
4. Write release notes (use `git log` or the commit history as a source).
5. Click **Publish release**.

### 6.4 Announce (optional)

- [Torrust GitHub Discussions](https://github.com/orgs/torrust/discussions)
- The project's Discord or Matrix channel.

---

## Common mistakes to avoid

| Mistake                                           | Consequence                                | Prevention                                         |
| ------------------------------------------------- | ------------------------------------------ | -------------------------------------------------- |
| `readme = "README.md"` missing from `Cargo.toml`  | crates.io page shows no documentation      | Verify §1.1 before every release                   |
| README still shows `git` dependency after publish | New users get the wrong source             | Update README snippet in the release commit (§1.2) |
| Skipping `cargo publish --dry-run`                | Compile errors surface during live publish | Always run §3.1 first                              |
| Skipping `cargo run` linter check                 | CI fails after push                        | Run linters locally before every commit (§3.2)     |
| Tagging before the release commit is on `main`    | Tag points to wrong commit                 | Tag only after merge (§4.2)                        |
| Forgetting `git push origin v0.2.0`               | No GitHub Release can be created           | Always push the tag explicitly (§4.3)              |
| MINOR bump omitted for a breaking change          | Downstream consumers break                 | Follow Semantic Versioning (§2)                    |

---

## Quick-reference checklist

```text
[ ] 1.1  Cargo.toml metadata looks correct (especially readme = "README.md")
[ ] 1.2  README dependency snippet updated to new version number
[ ] 1.3  Cargo.lock is committed
[ ] 1.4  cargo package --list shows expected files
[ ] 2    Version bumped in Cargo.toml
[ ] 3.1  cargo publish --dry-run exits without error
[ ] 3.2  cargo run (all linters) exits 0
[ ] 4.1  Release preparation commit on main (conventional commit message)
[ ] 4.2  Annotated git tag vX.Y.Z created on that commit
[ ] 4.3  git push origin main && git push origin vX.Y.Z
[ ] 5.1  Authenticated with crates.io (cargo login)
[ ] 5.2  cargo publish completed successfully
[ ] 6.1  New version visible on https://crates.io/crates/torrust-linting
[ ] 6.2  docs.rs build passed at https://docs.rs/crate/torrust-linting/X.Y.Z/builds
[ ] 6.3  GitHub Release created at https://github.com/torrust/torrust-linting/releases
```
