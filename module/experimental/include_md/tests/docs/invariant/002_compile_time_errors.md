# Invariant Spec: Compile-Time Errors

**Source:** `docs/invariant/002_compile_time_errors.md`
**Test files:** `tests/file_inclusion.rs`, `tests/section_extraction.rs`
**Case prefix:** `IN-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| IN-1 | File not found produces compile error, not runtime panic | ✅ |
| IN-2 | Heading not found produces compile error, not runtime panic | ✅ |
| IN-3 | Wrong argument count produces compile error | ✅ |
| IN-4 | Invalid UTF-8 file produces compile error, not runtime panic | ✅ |

---

### IN-1: File not found produces compile error, not runtime panic

- **Given:** A path argument that refers to a non-existent file
- **When:** Code invoking either `include_md!` or `include_md_section!` with the missing path is compiled
- **Then:** `cargo check` exits non-zero (compilation fails); no binary is produced; the failure cannot be observed at runtime because no executable is generated

### IN-2: Heading not found produces compile error, not runtime panic

- **Given:** A valid file path and a heading string that does not appear in that file
- **When:** Code containing `include_md_section!(path, "## NonExistentHeading")` is compiled
- **Then:** `cargo check` exits non-zero; the "heading not found" error is reported at the macro invocation site; no binary is produced

### IN-3: Wrong argument count produces compile error

- **Given:** An invocation of either macro with an incorrect number of string literal arguments (zero, one for the two-arg macro, two for the one-arg macro, or three for the two-arg macro)
- **When:** The containing source file is compiled
- **Then:** `cargo check` exits non-zero; the arity error is reported at the macro invocation site; no binary is produced

### IN-4: Invalid UTF-8 file produces compile error, not runtime panic

- **Given:** A file containing bytes that are not valid UTF-8 (e.g., `[0xFF, 0xFE, 0x00]`), created at test time
- **When:** Code invoking `include_md!` or `include_md_section!` with the file's absolute path is compiled via subprocess `cargo check`
- **Then:** `cargo check` exits non-zero; the UTF-8 error is reported at compile time; no binary is produced; for `include_md!` the rejection comes from `include_str!`, for `include_md_section!` from `fs::read_to_string`
