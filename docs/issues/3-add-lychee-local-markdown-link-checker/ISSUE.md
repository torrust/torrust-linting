---
doc-type: issue
issue-type: feature
status: draft
github-issue: 3
branch: feat/add-lychee-local-markdown-link-checker
related-tracker-issue: https://github.com/torrust/torrust-tracker/issues/2150
related-tracker-follow-up: https://github.com/torrust/torrust-tracker/issues/2162
---

# Issue #3 - Add lychee local Markdown link checker

## Goal

Add [lychee](https://lychee.cli.rs/) to the shared `linter` CLI so Torrust repositories can
deterministically validate local Markdown file links and fragments with `linter lychee` and
`linter all`.

## Background

The Torrust Tracker repository established the consuming-project convention in
[tracker issue #2150](https://github.com/torrust/torrust-tracker/issues/2150): a root
`lychee.toml` enables `offline = true` and `include_fragments = "full"`. This checks local
Markdown targets, heading anchors, and text fragments without making network requests. Its
baseline is clean after excluding immutable historical issue specifications with a narrowly
documented `exclude_path` rule.

The common `torrust-linting` binary currently supports Markdown, YAML, TOML, CSpell, Clippy,
Rustfmt, and ShellCheck. It needs a corresponding lychee runner before Tracker can integrate
the deterministic local-link check into its pre-commit hook and ordinary CI in issue #2162.

## Scope

### In scope

- Add a `Lychee` CLI command, exposed as `linter lychee`.
- Add `src/linters/lychee.rs`, exporting `run_lychee_linter() -> anyhow::Result<()>` through
  `src/linters/mod.rs` and `src/lib.rs`.
- Run lychee from the current working directory against the repository's Markdown files,
  passing `**/*.md` and `.github/**/*.md` for explicit dot-directory coverage. Preserve normal
  ignore behavior so generated `target/` content is not checked.
- Allow lychee to automatically use a root `lychee.toml` when that file exists; do not require
  another configuration filename or hard-code consumer-specific configuration.
- Always pass `--offline` so the shared CLI is a deterministic local-link check even when a
  consuming project has no `lychee.toml` or its configuration is incomplete.
- Install lychee automatically when it is absent, following the existing external-tool pattern.
  Use Cargo to install the `lychee` package, after confirming that Cargo is available.
- Include the runner in `run_all_linters()` while retaining the existing behavior of attempting
  every linter and returning an error if one or more fail.
- Preserve this repository's self-linting CI contract: `.github/workflows/linting.yml` builds the
  `linter` binary and runs `./target/debug/linter all`, so it will exercise lychee against this
  repository as well as verify automatic tool installation in a clean runner.
- Preserve lychee standard output, standard error, and a nonzero failure result for broken local
  file links or fragments.
- Update `README.md` to document the feature list, `cargo run -- lychee` command, manual
  installation, and root `lychee.toml` configuration file.
- Do not introduce a process abstraction solely for this linter. Verify direct external-command
  execution with valid and deliberately invalid local Markdown links, plus the mandatory
  end-to-end linter run.

### Out of scope

- Enabling online or external URL checks in `linter lychee` or `linter all`.
- Adding an online-mode flag or an alternate-config-file interface to the shared linter. A
  consuming repository's advisory scheduled/manual workflow invokes the lychee executable
  directly with its dedicated online configuration file.
- Adding a `lychee.toml` configuration for this repository solely as part of the shared-linter
  feature.
- Tracker-specific configuration, exclusions, pre-commit wiring, CI wiring, or the advisory
  scheduled external-link workflow. Those are tracked by Tracker issue #2162.
- Hard-coding Tracker paths or exclusions in this crate.

## Design

### Command behavior

`linter lychee` runs the lychee executable from the caller's current working directory with
`--offline`, `**/*.md`, and `.github/**/*.md`. The explicit `.github` input covers authoritative
documentation in hidden directories; ordinary lychee ignore behavior excludes generated content
such as `target/`. Lychee's standard root configuration discovery remains active, so a consuming
project can control compatible local-check policy through `lychee.toml`.

The initial supported policy is the Tracker convention:

```toml
# Consumer-owned configuration example; not embedded in this crate.
offline = true
include_fragments = "full"

# Each consumer exclusion must be narrow and document its rationale.
exclude_path = [ "(^|/)docs/issues/closed/" ]
```

The shared CLI enforces offline mode independently of the configuration. `offline = true` is
still recommended in consumer configuration to make the local-only policy explicit for direct
lychee use. Full fragment checking validates both heading anchors and text fragments in local
Markdown links. Optional external-link monitoring is explicitly outside this CLI: consuming
repositories invoke lychee directly from advisory scheduled/manual workflows with a separate
online configuration file. That repository-owned workflow controls its network policy,
`GITHUB_TOKEN`, request limits, artifacts, and triage process.

### Error handling

The runner follows the external linter pattern already used by Markdown, TOML, and CSpell:

1. Detect the lychee executable in `PATH`.
2. If unavailable, install the latest available release with `cargo install lychee` and propagate
   installation failures with context. Do not pin a version; handle future compatibility changes
   through the normal dependency update and validation workflow.
3. Execute lychee, forwarding its diagnostic output when it exits unsuccessfully.
4. Return an error to make the specific command and `linter all` fail.

## Implementation plan

| ID  | Status | Task                                                                                       |
| --- | ------ | ------------------------------------------------------------------------------------------ |
| T1  | TODO   | Add Cargo-based lychee installation helper and linter runner.                              |
| T2  | TODO   | Register and re-export the new linter module.                                              |
| T3  | TODO   | Add the `lychee` subcommand and run it from `all`.                                         |
| T4  | TODO   | Update README feature, usage, tool-installation, and configuration documentation.          |
| T5  | TODO   | Verify valid and deliberately invalid local-link scenarios with the direct CLI.            |
| T6  | TODO   | Format and run the required validation commands, including the mandatory `cargo run` gate. |

## Acceptance criteria

- [ ] `linter lychee` invokes lychee in the caller's current project directory.
- [ ] A consuming project's root `lychee.toml` is honored through lychee's default discovery.
- [ ] The documented `offline = true` and `include_fragments = "full"` convention supports
      deterministic local Markdown file and fragment validation.
- [ ] The CLI always passes `--offline`, preventing normal linter runs from checking external
      URLs even if consumer configuration is absent or incomplete.
- [ ] Missing lychee is installed through Cargo, or a clear installation error is returned.
- [ ] Lychee's diagnostics are displayed and a broken local link or fragment causes a nonzero
      result from both `linter lychee` and `linter all`.
- [ ] `linter all` continues running remaining linters after a lychee failure and reports an
      aggregate failure.
- [ ] The normal linter path does not enable online external URL checking.
- [ ] README documentation lists lychee, its command, installation command, and `lychee.toml`.
- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo run` exit successfully.

## Verification plan

| ID  | Check                                                                         | Expected result                                                                                                    |
| --- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| V1  | `cargo fmt --check`                                                           | Formatting check succeeds.                                                                                         |
| V2  | `cargo clippy -- -D warnings`                                                 | No Clippy warnings.                                                                                                |
| V3  | `cargo run -- lychee`                                                         | Lychee runs offline from the repository root with `**/*.md` and `.github/**/*.md`, then succeeds.                  |
| V4  | `cargo run`                                                                   | Mandatory self-linting repository gate exits with code 0.                                                          |
| V5  | `.github/workflows/linting.yml` behavior                                      | Its `./target/debug/linter all` invocation installs lychee when absent and successfully self-lints the repository. |
| V6  | Run `linter lychee` in a temporary directory containing an invalid local link | Lychee diagnostic is shown and the command exits nonzero without network access.                                   |

## Risks and decisions needed

- **Markdown input set:** `**/*.md` plus `.github/**/*.md` establishes one general discovery
  policy without duplicating the Tracker-specific input list. Consumer repositories own their
  exclusions through `lychee.toml`.
- **Latest lychee installation:** The unpinned installation policy keeps the tool fresh but can
  introduce future behavior changes. Mitigation: validate releases through the normal dependency
  update workflow rather than retaining a stale version pin.
- **No isolated runner unit tests:** Direct process execution matches every existing linter. The
  valid/invalid local-link scenarios and mandatory end-to-end gate provide the required evidence
  without introducing a one-off abstraction.
- **Self-linting bootstrap:** CI runs `linter all` from a clean runner, so the first lychee run
  may install the tool. Mitigation: use the existing Cargo installation pattern and keep the
  workflow's Rust toolchain setup and cache compatible with `cargo install lychee`.
- **Online policy scope:** Shared online support would require another configuration, secret, and
  network policy that is incompatible with the normal linter contract. Mitigation: consumers run
  the lychee executable directly in separately configured advisory workflows; add shared online
  support only when multiple consumers establish stable, common requirements.

## References

- GitHub issue: <https://github.com/torrust/torrust-linting/issues/3>
- Tracker configuration and baseline: <https://github.com/torrust/torrust-tracker/issues/2150>
- Tracker integration follow-up: <https://github.com/torrust/torrust-tracker/issues/2162>
- Lychee configuration: <https://lychee.cli.rs/guides/config/>
- Lychee fragment checking: <https://lychee.cli.rs/recipes/anchors/>
