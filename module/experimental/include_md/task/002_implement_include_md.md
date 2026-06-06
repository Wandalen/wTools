# Task 002: Implement include_md!

## Execution State

- **State:** ✅ (Completed)
- **Created:** 2026-06-06
- **Closes:** null

## Goal (MOST)

**Motivated:** `include_md!` is the crate's primary user-facing macro; without it, the crate provides no value. Developers cannot embed markdown files at compile time, which is the whole purpose of the crate. Not implementing it leaves the crate permanently a placeholder.

**Observable:** `tests/file_inclusion.rs` passes all test cases under Level 3; the macro correctly returns the full file content for a valid path; all error conditions (missing file, oversized file, invalid UTF-8) produce compile-time errors, not runtime panics.

**Scoped:** Implementation of the `include_md` function in `src/lib.rs`; the `tests/file_inclusion.rs` test file; fixture files in `tests/fixture/`; one `[[test]]` entry in `Cargo.toml`. No changes to other macros.

**Testable:** Level 3 passes; positive test returns exact file content; compile-fail tests confirm each error condition produces a compile error; no test uses mocks or runtime fallbacks.

## In Scope

- Implement `include_md` in `src/lib.rs`: parse one `LitStr` path argument, emit the size-checking const-assertion block plus `include_str!(path)`
- Create `tests/file_inclusion.rs` with tests for all behavioral scenarios
- Create test fixture markdown files under `tests/fixture/` (a valid `.md` file; a file with known content)
- Implement compile-fail test helpers for error conditions using subprocess `cargo check` (same approach as `strs_tools_meta`)
- Wire `[[test]]` entry in `Cargo.toml` for `file_inclusion.rs`
- Update `tests/readme.md` to register the new test file

## Out of Scope

- `include_md_section!` implementation (Task 003)
- Examples (Task 004)
- Absolute path arguments (not required by invariant/001)
- Changes to any docs/ files (already updated)
- Proc-macro crate scaffolding (Task 001 prerequisite)

## Work Procedure

1. **Confirm Task 001 is complete** — `src/lib.rs` with proc-macro stubs exists; `cargo check --all-features` passes.
2. **Implement `include_md` in `src/lib.rs`**:
   - Parse input as a single `LitStr` using `syn::parse_macro_input!` or `syn::parse::<syn::LitStr>()`
   - On parse error, return `e.to_compile_error().into()`
   - Emit the following token stream:
     ```rust
     {
       const _ : () = assert!(
         :: core :: mem :: size_of_val( include_bytes!( #path ) ) <= 10_000_000_usize,
         "include_md: file exceeds 10 MB limit"
       );
       include_str!( #path )
     }
     ```
   - Use `quote::quote!` and `proc_macro2` tokens via `macro_tools`
3. **Write failing tests first** (TDD): create `tests/file_inclusion.rs` with all test function bodies that call `assert!(false, "not yet implemented")` — they must compile and fail on first run. Do not use `todo!()` (forbidden by AC-004).
4. **Create fixture files**:
   - `tests/fixture/sample.md` — a short valid markdown file with known content (e.g., `# Hello\n\nThis is a test.\n`)
   - `tests/fixture/readme.md` — Responsibility Table for fixture directory
5. **Implement positive test**: call `include_md!("fixture/sample.md")` and assert the result equals the expected file content string.
6. **Implement compile-fail tests** using subprocess `cargo check` (do not use trybuild — see `module/core/strs_tools_meta/tests/compile_fail_test.rs` for the exact `std::process::Command` pattern to adapt):
   - `missing_file_is_compile_error`: `include_md!("fixture/does_not_exist.md")` → compile fails
   - `oversized_file_is_compile_error`: create a temp >10MB file at test time, invoke `include_md!` on it → compile fails via the const assertion
   - `invalid_utf8_is_compile_error`: `include_str!` natively rejects non-UTF-8 files → compile fails
   - `wrong_argument_count_is_compile_error`: `include_md!()` or `include_md!("a", "b")` → parse error → compile fails
7. **Wire `[[test]]` in Cargo.toml**: add a new `[[test]]` entry (there is no existing `file_inclusion` entry to uncomment): `name = "file_inclusion"`, `path = "tests/file_inclusion.rs"`, `required-features = ["enabled"]`.
8. **Update `tests/readme.md`**: add `file_inclusion.rs` row and update Domain Map.
9. **Run Level 3** — nextest + doc tests + clippy; fix any issues.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| Valid path to existing UTF-8 markdown file | `include_md!("fixture/sample.md")` | Expands to `&'static str` equal to the file's full UTF-8 contents |
| Path to non-existent file | `include_md!("fixture/does_not_exist.md")` | Compile-time error: file not found |
| Path to file > 10 MB | `include_md!("fixture/big.md")` where file is 11 MB | Compile-time error: assert fires ("exceeds 10 MB limit") |
| Path to file with invalid UTF-8 bytes | `include_md!("fixture/invalid.bin")` | Compile-time error: `include_str!` rejects non-UTF-8 |
| No arguments passed | `include_md!()` | Compile-time error: parse error (expected string literal) |
| Two arguments passed | `include_md!("a.md", "b")` | Compile-time error: parse error (unexpected token) |

## Acceptance Criteria

- AC-001: `include_md!("fixture/sample.md")` in test code returns the exact bytes of `tests/fixture/sample.md` as a `&'static str`.
- AC-002: Each compile-fail scenario (AC-002a missing, AC-002b oversized, AC-002c invalid UTF-8, AC-002d wrong arity) produces a non-zero exit from `cargo check`; the test asserts on the exit status.
- AC-003: Level 3 (`cargo nextest run` + `cargo test --doc` + `cargo clippy`) exits 0 for the crate.
- AC-004: No test uses `#[ignore]`, `todo!()`, or any form of test bypass.
- AC-005: `tests/fixture/readme.md` exists with a Responsibility Table listing all fixture files.

## Related Documentation

- `docs/feature/001_file_inclusion.md` — feature definition; specifies the four error conditions
- `docs/api/001_include_md.md` — API contract: single LitStr path, output type, all error conditions
- `docs/invariant/001_path_resolution.md` — path resolution via emitted `include_str!`
- `docs/invariant/002_compile_time_errors.md` — all failures must be compile-time
- `docs/invariant/003_size_limit.md` — 10 MB limit enforced via const assertion
- `task/decisions.md` — Q-01 (include_md! delegates to include_str!), Q-04 (size limit via const assertion)

## History

- **[2026-06-06]** `CREATED` — Implement include_md! proc-macro: parse LitStr path, emit include_str! + const size assertion, compile-fail tests for all error conditions.
- **[2026-06-06]** `COMPLETED` — All ACs verified: valid file returns exact contents (AC-001), all 4 compile-fail scenarios pass (AC-002a-d including oversized + invalid UTF-8), Level 3 passes 21/21 (AC-003).

## Verification Record

- **Date:** 2026-06-06
- **Scope Coherence:** PASS — In Scope/Out of Scope non-empty, clear observable end state, no contradictions
- **MOST Goal Quality:** PASS — all four dimensions satisfied with concrete, measurable criteria
- **Value/YAGNI:** PASS — null hypothesis answered; user-directed work satisfies "committed need" requirement
- **Implementation Readiness:** PASS — Work Procedure steps executable; Test Matrix present
- **Issues fixed before final PASS:** (1) TDD step changed from `todo!()` stubs (forbidden by AC-004) to `assert!(false, ...)` stubs; (2) compile-fail step now references strs_tools_meta pattern; (3) step 7 wording changed from "uncomment or add" to "add a new entry" (no existing entry to uncomment)
