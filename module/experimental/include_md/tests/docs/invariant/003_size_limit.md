# Invariant Spec: Size Limit

**Source:** `docs/invariant/003_size_limit.md`
**Test files:** `tests/file_inclusion.rs`, `tests/section_extraction.rs`
**Case prefix:** `IN-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| IN-1 | Files within the 10 MB limit are accepted | ✅ |
| IN-2 | `include_md!` — file exceeding 10 MB rejected at compile time | ✅ |
| IN-3 | `include_md_section!` — file exceeding 10 MB rejected before content read | ✅ |

---

### IN-1: Files within the 10 MB limit are accepted

- **Given:** Fixture files `tests/fixture/sample.md` (< 1 KB) and `tests/fixture/multi_section.md` (< 1 KB) are both well within the 10 MB limit
- **When:** Either macro is invoked with these fixtures at compile time
- **Then:** Compilation succeeds and the macro expands correctly; no size error is produced

### IN-2: `include_md!` — file exceeding 10 MB rejected at compile time

- **Given:** A file larger than 10,000,000 bytes created at test time in a temp directory; code using `include_md!` with its absolute path compiled via subprocess `cargo check`
- **When:** The const assertion `include_bytes!(path).len() <= 10_000_000` is evaluated at compile time
- **Then:** The assertion fails; `cargo check` exits non-zero; no binary is produced (`oversized_file_is_compile_error` in `tests/file_inclusion.rs`)

### IN-3: `include_md_section!` — file exceeding 10 MB rejected before content read

- **Given:** A file larger than 10,000,000 bytes created at test time; code using `include_md_section!` with its absolute path compiled via subprocess `cargo check`
- **When:** The proc-macro checks `content.len() > 10_000_000` after `read_to_string`
- **Then:** The size check fires before any heading search; the macro emits a compile-time error; `cargo check` exits non-zero (`oversized_file_is_compile_error` in `tests/section_extraction.rs`)
