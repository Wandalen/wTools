# Bundle `terminal_size` dep into `format_table_visual` Cargo feature

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Claimed At:** null
- **Status:** 🎯 (Verified)

## Goal

Ensure that enabling the `format_table_visual` Cargo feature alone is sufficient to activate live terminal-width detection by explicitly including `dep:terminal_size` in that feature's dependency list in `Cargo.toml`.
(Motivated: `terminal_size` is a required dependency for auto-wrap terminal detection, but it may not be bundled into the `format_table_visual` feature — a downstream crate enabling only `format_table_visual` could silently get the hardcoded-120 fallback rather than live detection; Observable: `Cargo.toml` `[features]` entry for `format_table_visual` lists `"dep:terminal_size"` as one of its members so a single `features = ["format_table_visual"]` activation pulls in live detection; Scoped: one-line edit to `Cargo.toml` `[features]` section; no source code changes; Testable: `cargo check --features format_table_visual --no-default-features` compiles without missing-dependency errors and `w3 .test level::3` passes clean)

## In Scope

All paths relative to the crate root (`module/core/data_fmt/`).

**`Cargo.toml`:**
- In the `[features]` section, locate the `format_table_visual` entry and append `"dep:terminal_size"` to its feature list if not already present

## Out of Scope

- `src/` source files — no code changes required
- Any other feature definition
- Dev-dependency or workspace-level manifests

## Work Procedure

1. Read `Cargo.toml`. Locate `[features]` and find the `format_table_visual` entry.
2. Verify whether `terminal_size` already appears as `"dep:terminal_size"` in that feature list.
3. If missing, append `"dep:terminal_size"` to the list.
4. Run `cargo check --features format_table_visual --no-default-features` to confirm compilation succeeds.
5. Run `w3 .test level::3` to confirm no regressions.

## Test Matrix

| Check | Command | Expected |
|-------|---------|---------|
| Feature-isolated compile | `cargo check --features format_table_visual --no-default-features` | exit 0, no errors |
| Full test suite | `w3 .test level::3` | all tests pass |

## Closes

null

## Verification Record

All 4 Verification Gate dimensions passed:

1. **Scope Coherence** — PASS: In Scope is a single Cargo.toml change; Out of Scope excludes everything else; observable outcome is compilation success.
2. **MOST Goal Quality** — PASS: Motivated by silent detection failure; Observable via feature-only compile check; Scoped to one file; Testable via `cargo check` command.
3. **Value/YAGNI** — PASS: Fixes a real bundling gap; concrete committed need (feature should be self-contained); no speculative additions.
4. **Implementation Readiness** — PASS: Work Procedure is a single-step edit; Test Matrix uses a direct compile check; no new code needed.
