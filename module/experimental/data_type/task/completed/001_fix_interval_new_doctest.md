# Fix stale Interval::new call in data_type readme doctest

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ Done

## Goal

`module/experimental/data_type/readme.md` contains a doctest calling `interval_adapter::Interval::new( 0, 10 )`. The actual signature is `fn new(left: Bound<T>, right: Bound<T>)` — the API changed from bare values to `core::ops::Bound` wrappers but the documentation was not updated (Motivated: doc tests fail with `E0308: arguments to this function are incorrect`; Observable: `RUSTDOCFLAGS="-D warnings" cargo test --doc` exits 0 after fix; Scoped: one code block in readme.md; Testable: doctest for the `enabled` feature compiles and passes).

## In Scope

- `module/experimental/data_type/readme.md` — update the first `cfg(feature = "enabled")` code block to call `Interval::new( core::ops::Bound::Included(0), core::ops::Bound::Included(10) )`
- No change to the `make` feature code block

## Out of Scope

- Changing `interval_adapter` source or API
- Changing `data_type` source code
- Adding or removing features from `Cargo.toml`

## Requirements

- All work must adhere to applicable rulebooks (discover via `kbase .rulebooks`)
- Doc examples must compile — no `no_run` or `ignore` escape hatches
- Fix the call to use real API; do not suppress the test

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Confirm Interval::new signature** — `grep -n "fn new" module/experimental/interval_adapter/src/lib.rs`
2. **Check data_type prelude** — verify whether `Bound` is re-exported under `data_type::prelude::*`; if not, use `core::ops::Bound::Included`
3. **Edit readme.md** — change `interval_adapter::Interval::new( 0, 10 )` to use `Bound::Included` wrappers
4. **Test doctest** — `cd module/experimental/data_type && RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`
5. **Walk Validation Checklist** — every item answers YES

## Validation

### Measurements

- [x] M1 — old call gone: `grep "Interval::new( 0" readme.md` → 0 matches (was: 1)
- [x] M2 — new call present: `grep "Bound::Included" readme.md` → 2 matches
- [x] M3 — doctest passes: `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` → exit 0 (2 passed)

### Invariants

- [x] I1 — test suite clean: doctests pass, 0 warnings

### Anti-faking checks

- [x] AF1 — not `no_run`: no new `no_run` tags added
- [x] AF2 — not `ignore`: no new `ignore` tags added

### Checklist

- [x] C1 — Is `Interval::new( 0, 10 )` absent from the readme?
- [x] C2 — Does the doctest call `Bound::Included`?
- [x] C3 — Does `cargo test --doc --all-features` pass for data_type?
- [x] C4 — Are all Validation checks met?

## Outcomes

Fix applied 2026-04-17. Changed `interval_adapter::Interval::new( 0, 10 )` to `interval_adapter::Interval::new( core::ops::Bound::Included( 0 ), core::ops::Bound::Included( 10 ) )` in `readme.md` line 29. Doctest passes: 2/2 passed.
