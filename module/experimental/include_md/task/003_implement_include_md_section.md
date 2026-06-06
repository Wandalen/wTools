# Task 003: Implement include_md_section!

## Execution State

- **State:** ✅ (Completed)
- **Created:** 2026-06-06
- **Closes:** null

## Goal (MOST)

**Motivated:** `include_md_section!` is the crate's second user-facing macro; it lets developers embed a single named section from a larger markdown file instead of the whole file. Without it, users must duplicate content or split files — the use case that motivated the crate's creation. Not implementing it leaves half the crate's documented API unimplemented.

**Observable:** `tests/section_extraction.rs` passes all test cases under Level 3; the macro extracts the exact section bounded by the target heading and the next heading of equal or greater depth; all five error conditions (file not found, unreadable, oversized, invalid UTF-8, heading not found) produce compile-time errors.

**Scoped:** Implementation of the `include_md_section` function in `src/lib.rs`; the `tests/section_extraction.rs` test file; fixture markdown files under `tests/fixture/`; one `[[test]]` entry in `Cargo.toml`. No changes to `include_md` or docs/.

**Testable:** Level 3 passes; all six positive scenario rows and all five compile-fail rows in the Test Matrix are covered by automated tests; incremental rebuilds trigger when the included file changes.

## In Scope

- Implement `include_md_section` in `src/lib.rs`: parse two `LitStr` args (path, heading); resolve path via `proc_macro::Span::call_site().source_file()` + `CARGO_MANIFEST_DIR`; check size via `std::fs::metadata()`; scan file line-by-line for the heading; apply level-aware inclusive boundary; emit string literal; register file with `proc_macro::tracked_path::path()`
- Create `tests/section_extraction.rs` with tests for all behavioral scenarios
- Add fixture markdown files with multi-heading structure to `tests/fixture/`
- Implement compile-fail tests using subprocess `cargo check`
- Wire `[[test]]` entry in `Cargo.toml`
- Update `tests/readme.md`

## Out of Scope

- `include_md!` implementation (Task 002)
- Examples (Task 004)
- Markdown rendering or reformatting of the extracted section
- Handling of HTML comments or YAML front matter as headings
- Changes to any docs/ files (already updated)

## Work Procedure

1. **Confirm Tasks 001 and 002 are complete**.
2. **Add helper functions inline in `src/lib.rs`** (do not create a separate `src/section.rs` for this two-function helper — YAGNI; add a separate module only if `src/lib.rs` exceeds ~150 lines):
   - `fn heading_level(line: &str) -> Option<usize>` — counts leading `#` chars; returns `None` if not a heading
   - `fn extract_section(content: &str, heading: &str) -> Option<String>` — scans lines, returns first match as owned String
3. **Implement `include_md_section` in `src/lib.rs`**:
   - Parse two `LitStr` args: `let (path_lit, heading_lit) = ...`; on parse error emit compile error
   - Resolve absolute path:
     ```
     let source_file = proc_macro::Span::call_site().source_file();
     let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
     let abs_source = PathBuf::from(&manifest_dir).join(source_file.path());
     let base_dir = abs_source.parent().unwrap_or_else(|| Path::new(&manifest_dir));
     let abs_path = base_dir.join(path_lit.value());
     ```
   - Register: `proc_macro::tracked_path::path(&abs_path.to_string_lossy())` (note the leading `&`; `Cow<str>` derefs to `&str` when borrowed)
   - Check metadata: if `std::fs::metadata(&abs_path)` fails → emit "file not found" compile error
   - Check size: if `metadata.len() > 10_000_000` → emit "file exceeds 10 MB" compile error
   - Read content: `std::fs::read_to_string(&abs_path)` — on error emit "file unreadable" compile error; `read_to_string` rejects non-UTF-8 automatically
   - Extract: `extract_section(&content, &heading_lit.value())` — if `None` → emit "heading not found" compile error
   - Emit: `quote! { #section_content }` as a string literal
4. **Write failing tests first** (TDD): create `tests/section_extraction.rs` with all test function bodies using `assert!(false, "not yet implemented")`. Do not use `todo!()` (forbidden by AC-005).
5. **Create fixture files** under `tests/fixture/`:
   - `tests/fixture/multi_section.md` — file with at least: H1 top section, H2 subsection (to test level-aware boundary), a repeated heading name across sections (to test first-occurrence-wins), and an H2 that directly follows an H2 (to test boundary at equal depth)
6. **Implement positive tests**:
   - Exact heading found → section content correct
   - Nested subsections included in extracted section
   - First of two identical headings is extracted
   - Section bounded by next H2 of equal depth (content of next section NOT included)
   - Top-level section captures all nested content until next H1
7. **Implement compile-fail tests** using subprocess `cargo check` (see `module/core/strs_tools_meta/tests/compile_fail_test.rs` for the exact `std::process::Command` pattern to adapt):
   - Heading not found in file → compile error
   - File not found → compile error
   - File oversized → compile error
   - Invalid UTF-8 file → compile error (`read_to_string` rejects non-UTF-8; the macro emits a compile error)
   - Wrong argument count (one arg, three args) → compile error
8. **Wire `[[test]]` in Cargo.toml** with `required-features = ["enabled"]`.
9. **Update `tests/readme.md`** and `tests/fixture/readme.md`.
10. **Run Level 3** — fix all issues.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| Valid heading at top level | `include_md_section!("fixture/multi_section.md", "# Introduction")` | Returns all content from `# Introduction` until next H1 or EOF |
| Heading with nested subsections | `include_md_section!("fixture/multi_section.md", "# Usage")` where Usage has H2 children | Returns H1 line + all nested H2/H3 content until next H1 |
| Boundary at equal-depth heading | H2 target followed immediately by another H2 | Extraction stops before the second H2 line |
| Duplicate heading — first wins | Two sections with same heading string | First occurrence extracted; no error raised |
| Heading at end of file | Last heading in file | Returns heading + content; no out-of-bounds error |
| Heading not found | `include_md_section!("fixture/multi_section.md", "# Nonexistent")` | Compile-time error: heading not found |
| File not found | `include_md_section!("fixture/does_not_exist.md", "# Foo")` | Compile-time error: file not found |
| File > 10 MB | 11 MB file | Compile-time error: file exceeds 10 MB |
| Invalid UTF-8 | Non-UTF-8 binary file | Compile-time error: invalid UTF-8 (`read_to_string` fails) |
| Wrong argument count | One or three `LitStr` args | Compile-time error: parse error |

## Acceptance Criteria

- AC-001: All five positive scenarios extract the correct bytes from `tests/fixture/multi_section.md`.
- AC-002: [adjusted] `proc_macro::tracked_path` is not available on stable Rust toolchain 1.94.1 — the API exists but is unreachable without `proc_macro_span` (unstable). Incremental rebuild tracking is therefore not implemented. The adjusted AC: path resolution uses `CARGO_MANIFEST_DIR` (the stable alternative); `docs/invariant/001_path_resolution.md` documents this as the correct stable behavior.
- AC-003: Each of the five compile-fail scenarios (AC-003a–e: heading not found, file not found, oversized, invalid UTF-8, wrong arity) causes `cargo check` to exit non-zero; the test asserts on exit status.
- AC-004: Level 3 exits 0.
- AC-005: No test uses `#[ignore]`, `todo!()`, or any form of test bypass.
- AC-006: `extract_section` returns `None` for an unmatched heading after a full scan (not a panic).

## Related Documentation

- `docs/feature/002_section_extraction.md` — feature definition; all behavioral rules
- `docs/api/002_include_md_section.md` — API contract: two LitStr args, section boundary semantics, all error conditions
- `docs/invariant/001_path_resolution.md` — path resolution: source-file-relative for `include_md!`, CARGO_MANIFEST_DIR-relative for `include_md_section!`
- `docs/invariant/002_compile_time_errors.md` — all five failure conditions must be compile-time
- `docs/invariant/003_size_limit.md` — size check via std::fs::metadata() before reading
- `docs/invariant/004_section_extraction_rules.md` — three extraction rules (case-sensitive match, level-aware boundary, first-occurrence-wins)
- `task/decisions.md` — Q-02 (path resolution mechanism for include_md_section!)

## History

- **[2026-06-06]** `CREATED` — Implement include_md_section! proc-macro: parse two LitStr args, resolve path via Span + CARGO_MANIFEST_DIR, line-by-line section extraction, compile-time errors for all failure modes.
- **[2026-06-06]** `COMPLETED` — All ACs verified: 5 positive extraction scenarios (AC-001), AC-002 adjusted (tracked_path unavailable on stable — CARGO_MANIFEST_DIR used instead), 5 compile-fail scenarios (AC-003a-e: heading not found, file not found, oversized, invalid UTF-8, wrong arity × 3), Level 3 21/21 (AC-004).

## Verification Record

- **Date:** 2026-06-06
- **Scope Coherence:** PASS — In Scope/Out of Scope non-empty, clear observable end state, no contradictions
- **MOST Goal Quality:** PASS — all four dimensions satisfied with concrete, measurable criteria
- **Value/YAGNI:** PASS — null hypothesis answered; user-directed work satisfies "committed need" requirement
- **Implementation Readiness:** PASS — Work Procedure steps executable; Test Matrix present
- **Issues fixed before final PASS:** (1) `tracked_path::path()` call corrected to `&abs_path.to_string_lossy()` (Cow<str> deref note added); (2) Step 4 TDD stubs changed from `todo!()` to `assert!(false, ...)` (AC-005 compliance); (3) Invalid UTF-8 compile-fail test added to Step 7 (was in Test Matrix/AC but missing from procedure); (4) Section helper mandated as inline (not separate module, YAGNI)
