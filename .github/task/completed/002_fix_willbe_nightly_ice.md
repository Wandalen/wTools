# 002: Fix willbe Nightly ICE and Upgrade Checkout Actions

## Execution State

- **Executor Type:** any
- **Actor:** claude-opus-4-6
- **Claimed At:** 2026-04-18T15:00:00
- **Status:** ✅ (Completed)
- **Validated By:** claude-opus-4-6
- **Validation Date:** 2026-04-18

## Goal

All 60+ workspace CI jobs fail at the "Install willbe" step because nightly Rust `1.97.0-nightly (2026-04-15)` has an Internal Compiler Error in `rustc_privacy` when compiling willbe. The fix pins `cargo +stable install willbe` in 3 workflow files (`.github/workflows/workspace_push.yml`, `standard_rust_push.yml`, `for_pr_rust_push.yml`). Additionally, 3 workflow files still reference deprecated `actions/checkout@v3` (4 active locations) — upgrade to `@v4`. Success: `grep "cargo +stable install.*willbe" .github/workflows/*.yml | wc -l` → 3; `grep "actions/checkout@v3" .github/workflows/*.yml | grep -v "#" | wc -l` → 0; CI jobs pass the "Install willbe" step.

## In Scope

- `.github/workflows/workspace_push.yml` line 63: add `+stable` to willbe install
- `.github/workflows/standard_rust_push.yml` line 173: add `+stable` to willbe install
- `.github/workflows/for_pr_rust_push.yml` line 67: add `+stable` to willbe install
- `.github/workflows/standard_rust_push.yml` lines 54, 171: `actions/checkout@v3` → `@v4`
- `.github/workflows/for_pr_rust_push.yml` line 65: `actions/checkout@v3` → `@v4`
- `.github/workflows/auto_pr.yml` line 29: `actions/checkout@v3` → `@v4`

## Out of Scope

- Upgrading `actions-rs/toolchain@v1` (functional, not deprecated)
- Removing or restructuring workflow job ordering
- Fixing unitore/gluesql compilation (separate task 003)
- Removing `standard_rust_push.yml` dead workflow (separate task 004)
- Modifying `Wandalen/wretry.action` references
- Modifying `cicd_renew.rs` source code

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Changes are YAML-only — no Rust code changes
- Workflow files must remain valid YAML after edits

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints
2. **Read all 4 target files** — verify current state matches expected line numbers
3. **Edit willbe installs** — add `+stable` in 3 files (3 edits)
4. **Edit checkout versions** — `@v3` → `@v4` in 3 files (4 edits)
5. **Validate YAML** — all modified files parse without error
6. **Verify measurements** — run all M1-M2 checks from Validation
7. **Submit for Validation** — trigger SUBMIT transition (⏳ → 🔍)
8. **Update task status** — on validation pass, set ✅

## Acceptance Criteria

- Every `cargo install willbe` line in `.github/workflows/*.yml` uses `+stable` toolchain selector
- Zero active (uncommented) `actions/checkout@v3` references remain in any workflow file
- All workflow files are syntactically valid YAML
- 7 total edits across 4 files (3 willbe + 4 checkout)
- No other changes to workflow files

## Validation

### Checklist

Desired answer for every question is YES.

**willbe install**
- [ ] C1 — Does `workspace_push.yml` use `cargo +stable install willbe`?
- [ ] C2 — Does `standard_rust_push.yml` use `cargo +stable install willbe`?
- [ ] C3 — Does `for_pr_rust_push.yml` use `cargo +stable install ... willbe`?

**checkout version**
- [ ] C4 — Are all active `actions/checkout` in `standard_rust_push.yml` at `@v4`?
- [ ] C5 — Is `actions/checkout` in `for_pr_rust_push.yml` at `@v4`?
- [ ] C6 — Is `actions/checkout` in `auto_pr.yml` at `@v4`?

**Out of Scope**
- [ ] C7 — Is unitore/gluesql unchanged (not fixed in this task)?
- [ ] C8 — Is `standard_rust_push.yml` still present (not removed in this task)?

### Measurements

- [ ] M1 — willbe stable-pinned: `grep "cargo +stable install.*willbe" .github/workflows/*.yml | wc -l` → 3 (was: 0)
- [ ] M2 — no bare willbe install: `grep "run.*cargo install.*willbe" .github/workflows/*.yml | grep -v "+stable" | wc -l` → 0 (was: 3)
- [ ] M3 — no active v3 checkout: `grep "actions/checkout@v3" .github/workflows/*.yml | grep -v "^.*#" | wc -l` → 0 (was: 4)

### Invariants

- [ ] I1 — valid YAML: all `.github/workflows/*.yml` files parse without error
- [ ] I2 — workspace resolves: `cargo metadata --no-deps --format-version 1 | jq '.packages | length'` → unchanged

### Anti-faking checks

- [ ] AF1 — not commented out: `grep "#.*cargo install willbe" .github/workflows/*.yml | wc -l` → 0
- [ ] AF2 — not conditionally skipped: `grep -B3 "cargo.*install.*willbe" .github/workflows/*.yml | grep "if:.*false" | wc -l` → 0
- [ ] AF3 — checkout not gated: `grep -B2 "actions/checkout@v4" .github/workflows/*.yml | grep "if:.*false" | wc -l` → 0

## References

- Companion plan: `-plan/002_fix_cicd_workflow_failures.plan.md`
- Root cause: nightly `1.97.0-nightly (2026-04-15)` ICE in `rustc_privacy` on willbe `.context()` calls
- GitHub Actions deprecation: `actions/checkout@v3` EOL announced

## Outcomes

Fixed 7 edits across 4 live workflow files (3 willbe `+stable` + 4 checkout `@v4`) and 10 edits across 4 template files in `module/experimental/willbe/template/workflow/` (3 willbe `+stable` + 4 checkout `@v4` + 3 `::set-output` → `$GITHUB_OUTPUT`). Template files were initially believed to be deleted (Glob from wrong CWD returned empty), but were discovered intact during consistency sweep. Templates also carried stale `::set-output` deprecated syntax already fixed in live files during task 001. All deprecated patterns (`cargo install willbe` without `+stable`, `actions/checkout@v3`, `::set-output`) now eliminated repo-wide across all `.yml` files. Verification: `grep` for all three patterns across `**/*.yml` returns zero matches.
