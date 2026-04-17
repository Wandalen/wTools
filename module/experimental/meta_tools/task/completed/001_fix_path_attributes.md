# Fix broken #[path] attributes for impls_index and mod_interface

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ Done

## Goal

`meta_tools` was moved from `module/core/` to `module/experimental/` but its test module `tests/inc/mod.rs` uses `#[path]` to re-use tests from `impls_index` and `mod_interface`. Both those crates stayed in `module/core/`, so the relative paths `../../../impls_index/` and `../../../mod_interface/` now resolve to non-existent paths under `module/experimental/`, causing build failure (Motivated: compilation is broken — crate cannot be tested; Observable: `couldn't read ../../../impls_index/tests/inc/mod.rs` error is absent after fix; Scoped: one file `tests/inc/mod.rs`, two line changes; Testable: `cargo build -p meta_tools --all-features` exits 0 after fix).

## In Scope

- `module/experimental/meta_tools/tests/inc/mod.rs` — change two `#[path]` prefixes from `../../../` to `../../../../core/` for `impls_index` and `mod_interface` entries
- Leave the `for_each` path (`../../../for_each/tests/inc/mod.rs`) unchanged — `for_each` is in `module/experimental/`

## Out of Scope

- Moving any crates
- Modifying `impls_index` or `mod_interface` source files
- Changes to `Cargo.toml` or feature flags

## Requirements

- All work must adhere to applicable rulebooks (discover via `kbase .rulebooks`)
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- No mocking; no workarounds; proper path fix only

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Verify target paths exist** — confirm both `module/core/impls_index/tests/inc/mod.rs` and `module/core/mod_interface/tests/inc/mod.rs` exist
2. **Fix `impls_index` path** — change `#[ path = "../../../impls_index/tests/inc/mod.rs" ]` to `#[ path = "../../../../core/impls_index/tests/inc/mod.rs" ]`
3. **Fix `mod_interface` path** — change `#[ path = "../../../mod_interface/tests/inc/mod.rs" ]` to `#[ path = "../../../../core/mod_interface/tests/inc/mod.rs" ]`
4. **Verify** — `cargo build -p meta_tools --all-features` exits 0; `for_each` path is unchanged
5. **Walk Validation Checklist** — every item answers YES

## Validation

### Measurements

- [x] M1 — impls_index path fixed: `grep "path.*impls_index" tests/inc/mod.rs` → contains `../../../../core/impls_index` (was: `../../../impls_index`)
- [x] M2 — mod_interface path fixed: `grep "path.*mod_interface" tests/inc/mod.rs` → contains `../../../../core/mod_interface` (was: `../../../mod_interface`)
- [x] M3 — for_each unchanged: `grep "path.*for_each" tests/inc/mod.rs` → still `../../../for_each`
- [x] M4 — crate builds: `cargo build -p meta_tools --all-features` → exit 0

### Invariants

- [ ] I1 — test suite clean: `w3 .test level::3` → 0 failures, 0 warnings for meta_tools (blocked by pre-existing mod_interface E0425 errors, separate issue)

### Anti-faking checks

- [x] AF1 — not cfg-gated away: `#[cfg]` guards for `meta_impls_index` and `meta_mod_interface` features unchanged
- [x] AF2 — paths not commented out: no `// #[ path` lines added

### Checklist

- [x] C1 — Both changed `#[path]` attrs point to existing files?
- [x] C2 — Is `for_each` path unchanged?
- [x] C3 — Does crate build without errors?
- [x] C4 — Are all Validation checks met (except I1 which is blocked by pre-existing issue)?

## Outcomes

Fix applied 2026-04-17. Changed two `#[path]` attributes in `tests/inc/mod.rs`:
- `../../../impls_index/tests/inc/mod.rs` → `../../../../core/impls_index/tests/inc/mod.rs`
- `../../../mod_interface/tests/inc/mod.rs` → `../../../../core/mod_interface/tests/inc/mod.rs`

`cargo build -p meta_tools --all-features` exits 0. Test suite compilation still fails due to pre-existing E0425 errors inside the included `mod_interface` tests — this is a separate issue outside the scope of this task.
