# Task 001: Proc-Macro Scaffolding

## Execution State

- **State:** ✅ (Completed)
- **Created:** 2026-06-06
- **Closes:** null

## Goal (MOST)

**Motivated:** The crate is currently a normal library crate with a placeholder entry point; no proc-macro functions can be defined until `proc-macro = true` is set and the proc-macro framework (`macro_tools`) is wired in. Without this, Tasks 002 and 003 are blocked. Not completing this leaves the crate permanently non-functional.

**Observable:** `cargo check --all-features` on the `include_md` crate succeeds; the compiled crate exports two `#[proc_macro]` symbols (`include_md` and `include_md_section`) that return empty token streams; no compilation errors or warnings.

**Scoped:** Changes to `Cargo.toml`, `src/lib.rs` (creation), and `src/_blank/standard_lib.rs` (deletion or gutting). No implementation logic — stubs only.

**Testable:** `w3 .test level::1` (or `cargo nextest run --all-features`) passes on the crate after scaffolding; the two proc-macro symbols are reachable.

## In Scope

- Set `proc-macro = true` in `[lib]` section of `Cargo.toml`
- Change `[lib] path` from `"src/_blank/standard_lib.rs"` to `"src/lib.rs"`
- Add `macro_tools = { workspace = true, optional = true }` to `[dependencies]`
- Update `enabled` feature to activate `dep:macro_tools` and `macro_tools/enabled`
- Create `src/lib.rs` with `#[cfg(feature = "enabled")]` guard and two `#[proc_macro]` stub functions
- Delete or empty `src/_blank/standard_lib.rs` (it becomes unreferenced dead code)
- Verify smoke tests still pass

## Out of Scope

- Any macro implementation logic (handled in Tasks 002 and 003)
- Test files for macro behavior (Tasks 002 and 003)
- Examples (Task 004)
- Changes to any docs/ files (already up to date)

## Work Procedure

1. **Read current Cargo.toml** to confirm exact field values before editing.
2. **Edit Cargo.toml**:
   - In `[lib]`: add `proc-macro = true`; change `path` to `"src/lib.rs"`
   - In `[dependencies]`: add `macro_tools = { workspace = true, optional = true }`
   - In `[features]`: update `enabled = ["dep:macro_tools", "macro_tools/enabled"]`
3. **Create `src/lib.rs`** with two `#[proc_macro]` stubs, both returning `TokenStream::new()`:
   ```rust
   #![ cfg_attr( not( feature = "enabled" ), allow( unused ) ) ]

   use ::macro_tools ::prelude ::*;

   #[ cfg( feature = "enabled" ) ]
   #[ proc_macro ]
   pub fn include_md( _input : TokenStream ) -> TokenStream
   {
     TokenStream ::new()
   }

   #[ cfg( feature = "enabled" ) ]
   #[ proc_macro ]
   pub fn include_md_section( _input : TokenStream ) -> TokenStream
   {
     TokenStream ::new()
   }
   ```
4. **Delete `src/_blank/standard_lib.rs`** (no longer the `[lib]` path; deleting prevents confusion).
5. **Update `src/_blank/readme.md`** to remove the reference to `standard_lib.rs` and note it was deleted.
6. **Run `cargo check --all-features`** on the crate; fix any errors.
7. **Run Level 1** (`cargo nextest run --all-features`) to confirm smoke tests still pass.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `cargo check --all-features` after scaffolding | `proc-macro = true`, `macro_tools` dep, stubs | Compiles without errors or warnings |
| `cargo check` with no features | feature-gated stubs behind `#[cfg(feature = "enabled")]` | Compiles without errors |
| `smoke_test.rs` after scaffolding | existing smoke tests | All existing tests pass unmodified |

## Acceptance Criteria

- AC-001: `cargo check --all-features` exits 0 on the crate after all Cargo.toml and src/ changes.
- AC-002: `cargo check` with default features exits 0.
- AC-003: All existing tests in `smoke_test.rs` pass under Level 1.
- AC-004: `src/_blank/standard_lib.rs` is deleted; `src/lib.rs` exists and defines `include_md` and `include_md_section` as `#[proc_macro]` items.
- AC-005: No blanket `#[allow(dead_code)]` or `#[allow(unused)]` without feature gating in `src/lib.rs`. (The `#![ cfg_attr( not( feature = "enabled" ), allow( unused ) ) ]` conditional form is required and permitted.)

## Related Documentation

- `docs/feature/001_file_inclusion.md` — user requirement that drives this macro
- `docs/feature/002_section_extraction.md` — user requirement that drives this macro
- `docs/api/001_include_md.md` — API contract for the stub entry point
- `docs/api/002_include_md_section.md` — API contract for the stub entry point
- `task/decisions.md` — Q-03 (macro_tools as proc-macro framework)

## History

- **[2026-06-06]** `CREATED` — Convert include_md to a proc-macro crate with two #[proc_macro] stubs using macro_tools.
- **[2026-06-06]** `COMPLETED` — All ACs verified: proc-macro = true, src/lib.rs with both stubs, src/_blank/standard_lib.rs deleted, Level 3 passes.

## Verification Record

- **Date:** 2026-06-06
- **Scope Coherence:** PASS — In Scope and Out of Scope non-empty, clear observable end state, no contradictions
- **MOST Goal Quality:** PASS — all four dimensions satisfied with concrete, measurable criteria
- **Value/YAGNI:** PASS — null hypothesis answered; user-directed work satisfies "committed need" requirement
- **Implementation Readiness:** PASS — Work Procedure steps executable; Test Matrix present; AC-005 clarified (cfg_attr conditional form permitted)
- **Issues fixed before final PASS:** AC-005 ambiguity resolved (conditional form explicitly permitted)
