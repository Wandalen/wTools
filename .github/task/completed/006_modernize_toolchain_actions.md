
# 006: Modernize Toolchain Actions — Replace wretry+actions-rs

## Execution State

- **Executor Type:** any
- **Actor:** self
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)
- **Validated By:** self
- **Validation Date:** 2026-04-18

## Goal

Two GitHub Actions deprecations affect CI reliability: (1) `Wandalen/wretry.action/main@v3.8.0_js_action` uses Node.js 20 — deprecated, forced to Node.js 24 on June 2, 2026; (2) `actions-rs/toolchain@v1` is archived/unmaintained. Both are used in three workflow files (workspace_push.yml, standard_rust_push.yml, for_pr_rust_push.yml) and their template counterparts in `willbe/template/workflow/`. Fix: replace the `wretry.action + actions-rs/toolchain@v1` pattern (8–18 YAML lines per occurrence) with `dtolnay/rust-toolchain` (1–3 lines), which is a composite action (no Node.js dependency), actively maintained, and the de-facto standard for Rust CI.

## In Scope

- Replace `Wandalen/wretry.action/main@master` + `actions-rs/toolchain@v1` with `dtolnay/rust-toolchain@stable` / `dtolnay/rust-toolchain@nightly` in all live workflow files
- Apply same changes to template counterparts in `module/experimental/willbe/template/workflow/`
- Preserve `components: clippy` for the nightly toolchain in `standard_rust_push.yml` checkmate job

## Out of Scope

- Updating `actions/checkout` version (already at v4)
- Fixing test failures (P2 — explicitly deferred)
- Modifying cicd_renew.rs logic

## Work Procedure

1. Replace toolchain steps in `workspace_push.yml` (stable + nightly, 2 blocks × 9 lines → 2 × 1 line)
2. Replace in `standard_rust_push.yml` (3 blocks: nightly+clippy in checkmate, stable+nightly in will_test)
3. Replace in `for_pr_rust_push.yml` (stable + nightly, same as workspace_push)
4. Apply same to three template counterparts
5. Verify no `wretry.action` or `actions-rs` references remain outside commented blocks

## Outcomes

**Live files updated:**
- `.github/workflows/workspace_push.yml` — 2 blocks → 2 `dtolnay/rust-toolchain` steps
- `.github/workflows/standard_rust_push.yml` — 3 blocks → 3 `dtolnay/rust-toolchain` steps (nightly+clippy, stable, nightly)
- `.github/workflows/for_pr_rust_push.yml` — 2 blocks → 2 `dtolnay/rust-toolchain` steps

**Template files updated (same changes):**
- `module/experimental/willbe/template/workflow/workspace_push.yml`
- `module/experimental/willbe/template/workflow/standard_rust_push.yml`
- `module/experimental/willbe/template/workflow/for_pr_rust_push.yml`

**Net reduction:** ~90 YAML lines removed (6 files × ~15 lines saved per file)

## Acceptance Criteria

- Zero `wretry.action` references in non-commented workflow lines
- Zero `actions-rs/toolchain` references in non-commented workflow lines
- All `dtolnay/rust-toolchain` steps have correct inputs (`components: clippy` where needed)
- All 6 modified YAML files are syntactically valid

## Validation

### Checklist

- [x] C1 — Are all wretry.action references gone (excluding comments)?
- [x] C2 — Are all actions-rs/toolchain references gone (excluding comments)?
- [x] C3 — Does checkmate nightly step retain `components: clippy`?
- [x] C4 — Are all 6 YAML files syntactically valid?

### Measurements

- [x] M1 — no active wretry: `grep -r "wretry.action" .github/workflows/*.yml | grep -v "^#" | wc -l` → 0
- [x] M2 — no actions-rs (live): `grep -r "actions-rs" .github/workflows/*.yml | grep -v "#" | wc -l` → 0
- [x] M3 — no wretry in templates: `grep -r "wretry.action" module/experimental/willbe/template/workflow/*.yml | grep -v "#" | wc -l` → 0
- [x] M4 — dtolnay present: `grep -r "dtolnay/rust-toolchain" .github/workflows/ | wc -l` → 6 (3 stable + 3 nightly in workspace_push, for_pr; 1+2 in standard)

### Invariants

- [x] I1 — cargo +stable install willbe still present: unchanged from task 002 fix
- [x] I2 — checkout@v4 still present: unchanged from task 002 fix

### Anti-faking checks

- [x] AF1 — not merely renamed: `grep -r "Node.js 20" .github/workflows/*.yml | wc -l` → 0 (no Node.js 20 pinning added)
