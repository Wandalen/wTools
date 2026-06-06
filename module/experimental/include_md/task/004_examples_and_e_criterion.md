# Task 004: Examples and E Criterion

## Execution State

- **State:** ✅ (Completed)
- **Created:** 2026-06-06
- **Closes:** null

## Goal (MOST)

**Motivated:** The E criterion in TDCFREMS requires at least one working example demonstrating the crate's public API. Without examples, the crate cannot be promoted from experimental to stable, and new users have no quick-start reference. Not completing this leaves E = `·` indefinitely and blocks stable promotion.

**Observable:** `cargo run --example include_md_trivial --features enabled` exits 0; `examples/include_md_trivial.rs` compiles and demonstrates both `include_md!` and `include_md_section!` with real files; `examples/readme.md` lists the example.

**Scoped:** `examples/` directory (two files: `include_md_trivial.rs` + `readme.md`); `[[example]]` entry in `Cargo.toml`; `readme.md` Quick Start section update. No src/ or docs/ changes.

**Testable:** `cargo build --example include_md_trivial --features enabled` exits 0; Level 1 passes; E criterion in TDCFREMS becomes `E`.

## In Scope

- Create `examples/include_md_trivial.rs` demonstrating both `include_md!` and `include_md_section!` with a real markdown file
- Create `examples/readme.md` with a Responsibility Table (mandatory for new directories)
- Add `[[example]]` entry to `Cargo.toml` with `required-features = ["enabled"]`
- Update `readme.md` Quick Start section to replace "Not yet implemented" with actual usage examples
- Verify E criterion is satisfied (no doc/layers.md update needed in this task — that belongs in a post-completion audit)

## Out of Scope

- Additional examples beyond `include_md_trivial`
- Example test coverage (examples are compiled but not tested via nextest)
- Changes to docs/ files
- src/ implementation changes (Tasks 002/003 prerequisite)

## Work Procedure

1. **Confirm Tasks 001, 002, and 003 are complete** — both macros are implemented and all tests pass.
2. **Create `examples/include_md_trivial.rs`** demonstrating both macros:
   - Include the crate's own `readme.md` via `include_md!("../readme.md")` (path relative to `examples/include_md_trivial.rs`)
   - Include a named section from the readme via `include_md_section!("../readme.md", "## Quick Start")` (same relative base)
   - Print both results in `fn main()` to show the output at run time
   - No `#[cfg(feature = "enabled")]` guard needed inside the file — `required-features = ["enabled"]` in Cargo.toml prevents building without the feature
3. **Create `examples/readme.md`** with Responsibility Table:
   ```
   | File | Responsibility |
   |------|----------------|
   | `include_md_trivial.rs` | Demonstrates include_md! and include_md_section! |
   ```
4. **Add `[[example]]` to Cargo.toml**:
   ```toml
   [[example]]
   name = "include_md_trivial"
   path = "examples/include_md_trivial.rs"
   required-features = [ "enabled" ]
   ```
5. **Update `readme.md` Quick Start section**: replace the "Not yet implemented" placeholder with a two-snippet example showing `include_md!` and `include_md_section!` usage.
6. **Run `cargo build --example include_md_trivial --features enabled`** — fix any compile errors.
7. **Run Level 1** (`cargo nextest run --all-features`) — confirm all tests still pass.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| Build example with features | `cargo build --example include_md_trivial --features enabled` | Exits 0; binary produced |
| Run example | `cargo run --example include_md_trivial --features enabled` | Exits 0; prints markdown content to stdout |
| Build with no features | `cargo build --example include_md_trivial` | Exits non-zero (`required-features = ["enabled"]` enforces this) |
| Level 1 test suite | `cargo nextest run --all-features` | All existing tests still pass |

## Acceptance Criteria

- AC-001: `cargo build --example include_md_trivial --features enabled` exits 0.
- AC-002: `examples/readme.md` exists with a Responsibility Table listing `include_md_trivial.rs`.
- AC-003: `readme.md` Quick Start section contains at least one code block showing `include_md!` usage.
- AC-004: `[[example]]` entry in `Cargo.toml` has `required-features = ["enabled"]`.
- AC-005: Level 1 passes — no existing tests broken by this change.

## Related Documentation

- `docs/feature/001_file_inclusion.md` — feature the example demonstrates
- `docs/feature/002_section_extraction.md` — feature the example demonstrates
- `docs/api/001_include_md.md` — API contract the example exercises
- `docs/api/002_include_md_section.md` — API contract the example exercises

## History

- **[2026-06-06]** `CREATED` — Create include_md_trivial example demonstrating both macros; satisfy E criterion for stable promotion.
- **[2026-06-06]** `COMPLETED` — examples/include_md_trivial.rs created, examples/readme.md created, [[example]] wired in Cargo.toml with required-features = ["enabled"], readme.md Quick Start updated. cargo build --example exits 0. All ACs verified: AC-001 ✅ AC-002 ✅ AC-003 ✅ AC-004 ✅ AC-005 ✅ (Level 1 21/21).

## Verification Record

- **Date:** 2026-06-06
- **Scope Coherence:** PASS — In Scope/Out of Scope non-empty, clear observable end state, no contradictions (after fixes)
- **MOST Goal Quality:** PASS — all four dimensions satisfied with concrete, measurable criteria
- **Value/YAGNI:** PASS — null hypothesis answered (E criterion blocks stable promotion); user-directed work satisfies "committed need" requirement
- **Implementation Readiness:** PASS — Work Procedure steps executable; Test Matrix deterministic
- **Issues fixed before final PASS:** (1) Scope contradiction resolved — `doc/layers.md` update removed from In Scope (post-completion audit item, not a task deliverable); (2) Relative path corrected `../../readme.md` → `../readme.md`; (3) Test Matrix row 3 made deterministic (removed "OR exits 0" branch); (4) Feature gating note clarified (required-features in Cargo.toml is sufficient, no in-file cfg guard needed); (5) Step 8 (doc/layers.md update) removed from Work Procedure
