# Questions for Issue #3

Reply beneath each question. Keep the selected option or provide a replacement decision, then add
any relevant rationale.

## Q1 - Markdown input set

Which Markdown input set should `linter lychee` pass to lychee?

- **Option A (recommended):** Pass `**/*.md` plus `.github/**/*.md`, with lychee configured or
  invoked so generated `target/` content is not checked.
- **Option B:** Use the explicit Tracker input set:
  `README.md`, `SECURITY.md`, `docs/**/*.md`, `**/AGENTS.md`, `packages/*/README.md`,
  `console/**/*.md`, `contrib/**/*.md`, `share/**/*.md`, and `.github/**/*.md`.
- **Option C:** Another input set (describe it below).

**Answer:** Option A, with explicit dot-directory coverage: pass `**/*.md` and
`.github/**/*.md`. Do not duplicate the Tracker-specific input list in the shared linter.

The shared tool must apply one general Markdown-discovery policy across Torrust repositories;
each consuming repository provides its own exclusions and lychee behavior through root
`lychee.toml`. Include `.github/**/*.md` explicitly because ordinary recursive glob expansion may
not include hidden directories. Generated `target/` content must remain excluded through normal
ignore behavior (or the same narrowly scoped discovery exclusion used by existing linters).

## Q2 - Lychee installation version

Which automatic installation policy should the linter use?

- **Option A (recommended):** `cargo install lychee` without a version pin, matching the existing
  external-tool installation approach.
- **Option B:** Install a pinned lychee release (state the exact version), trading ongoing update
  maintenance for reproducibility.
- **Option C:** Another policy (describe it below).

**Answer:** Option A: use `cargo install lychee` without a version pin.

This follows the repository dependency-freshness policy and the existing external-tool
installation approach. The Tracker report pins `0.24.2` only to reproduce the historical baseline
that was measured there; it is not a compatibility constraint for the shared linter. If a future
lychee release requires a behavior/configuration change, handle it through the normal dependency
update and validation workflow rather than retaining a stale pin.

## Q3 - Automated tests

Should this issue introduce test infrastructure for an external command runner?

- **Option A (recommended):** Do not introduce process abstraction solely for tests. Verify through
  `cargo run -- lychee`, a manual invalid-link scenario, and the mandatory `cargo run` gate.
- **Option B:** Introduce a test seam and focused automated tests for command construction,
  configuration discovery, output forwarding, and failure propagation.
- **Option C:** Another testing approach (describe it below).

**Answer:** Option A: do not introduce a process abstraction solely for this linter.

`torrust-linting` currently has no unit-test suite and its established verification is running the
real CLI against the repository (`cargo run`, which CI also executes). Keep the implementation
consistent with existing direct `Command`-based linters, verify `cargo run -- lychee` on valid and
temporarily invalid local Markdown links, and confirm that `cargo run` includes lychee through
`all`. A test seam is justified only if the project adopts one across external-tool runners, not
for this single linter.

## Q4 - Enforce offline mode by the shared CLI

Should `linter lychee` always pass Lychee's `--offline` flag?

- **Option A (recommended):** Yes. The shared CLI always enforces offline mode, so `linter all`
  can never perform network-dependent checks—even when a consuming repository has no
  `lychee.toml`. A root `lychee.toml` still supplies fragment checking and consumer-owned
  exclusions.
- **Option B:** No. The CLI relies entirely on each consuming project's `lychee.toml` for
  `offline = true`. This means projects without that configuration could accidentally perform
  online checks.
- **Option C:** Another policy (describe it below).

**Answer:** Option A: `linter lychee` must always pass Lychee's `--offline` flag.

The shared CLI's contract is deterministic, local Markdown link validation. Relying only on a
consumer's `lychee.toml` makes `linter all` network-dependent whenever that file is absent or
misconfigured, which conflicts with the purpose of a fast, reliable pre-commit and normal-CI
check. A root `lychee.toml` remains the consumer-owned place for `include_fragments`, exclusions,
timeouts, and other compatible local-check settings; `--offline` is the shared safety boundary.

Online external-link validation belongs to a separately invoked Lychee command in an advisory
scheduled/manual workflow, not to `linter lychee` or `linter all`.

## Q5 - Scheduled online link checking

How should the shared CLI support an advisory scheduled workflow that checks external URLs?

- **Option A (recommended):** Add `linter lychee --online --config <PATH>`. The default command
  and `linter all` remain offline. `--online` omits the wrapper's `--offline` flag; the scheduled
  workflow explicitly selects a separate config file, such as `lychee-online.toml`, which does
  not set `offline = true`. The wrapper passes the config path to lychee.
- **Option B:** Add only `linter lychee --online`, relying on the root `lychee.toml` not to set
  `offline = true`. This conflicts with the Tracker's intended local-check configuration unless
  that configuration is changed for every scheduled run.
- **Option C:** Do not add online support to `linter`; the scheduled workflow invokes the lychee
  executable directly with an online-specific config file.
- **Option D:** Another interface (describe it below).

**Answer:** Option C: do not add online support to `linter` in this issue.

Issue #3 establishes the shared CLI's deterministic offline contract. An online mode would add a
second configuration/secret/network policy to a tool intended for normal local and CI linting,
despite being neither required nor safe for `linter all`. The tracker follow-up,
`torrust/torrust-tracker#2162`, should invoke the Lychee executable directly from its advisory
weekly and manually dispatched workflow using a dedicated online configuration file (for example,
`lychee-online.toml`) that does not set `offline = true`.

This preserves a clear boundary: the shared linter protects deterministic local documentation
links, while the consuming repository owns optional external-link monitoring, `GITHUB_TOKEN`,
request limits, artifacts, and operational triage. A future dedicated issue can add shared online
support only if several consuming repositories demonstrate the same stable requirements.
