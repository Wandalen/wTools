# Fix phantom use include_md in wca smoke test

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ Done

## Goal

`wca/tests/smoke_test.rs` has `use include_md;` inside `published_smoke_test`. `include_md` is not a dependency of `wca`. This is a copy-paste error from another crate's smoke test template (Motivated: `wca` cannot compile — `E0432: unresolved import include_md`; Observable: `E0432` error is absent after fix; Scoped: one file, one line; Testable: `cargo build -p wca --all-features` exits 0 after fix).

## In Scope

- `module/experimental/wca/tests/smoke_test.rs` — replace `use include_md;` with `use wca;`

## Out of Scope

- Changing `Cargo.toml` dependencies
- Modifying the `wca` crate source
- Changes to any other test file

## Requirements

- All work must adhere to applicable rulebooks (discover via `kbase .rulebooks`)
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- No mocking; no workarounds; proper import fix only

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read the file** — confirm current content of `tests/smoke_test.rs`
2. **Apply fix** — change `use include_md;` to `use wca;`
3. **Verify** — `cargo build -p wca --all-features` exits 0
4. **Walk Validation Checklist** — every item answers YES

## Validation

### Measurements

- [x] M1 — old import gone: `grep "use include_md" tests/smoke_test.rs` → 0 matches (was: 1 match)
- [x] M2 — new import present: `grep "use wca" tests/smoke_test.rs` → 1 match (was: 0)
- [x] M3 — crate builds: `cargo build -p wca --all-features` → exit 0

### Invariants

- [x] I1 — test suite clean: `w3 .test level::3` → 49/49 passed, 0 warnings for wca

### Anti-faking checks

- [x] AF1 — not commented out: no `// use include_md` or `// use wca` lines
- [x] AF2 — no cfg gate added: no `#[cfg(...)]` wrapper added to hide the fix

### Checklist

- [x] C1 — Is `use include_md;` absent from the file?
- [x] C2 — Is `use wca;` present in its place?
- [x] C3 — Does `cargo build -p wca --all-features` succeed?
- [x] C4 — Are all Validation checks met?

## Outcomes

Fix applied 2026-04-17. Changed `use include_md;` to `use wca;` in `tests/smoke_test.rs` line 27. `wca` L3 validation passes: 49/49 tests, 0 clippy warnings.
