# API Spec: include_md Macro

**Source:** `docs/api/001_include_md.md`
**Test file:** `tests/file_inclusion.rs`
**Case prefix:** `AP-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| AP-1 | Single string literal argument accepted | ✅ |
| AP-2 | Zero arguments rejected at compile time | ✅ |
| AP-3 | Two arguments rejected at compile time | ✅ |
| AP-4 | Valid invocation returns `&'static str` | ✅ |
| AP-5 | File not found produces compile error at invocation site | ✅ |
| AP-6 | File exceeding 10 MB produces compile error at invocation site | ✅ |

---

### AP-1: Single string literal argument accepted

- **Given:** A valid file path exists and a single string literal is provided as the argument
- **When:** `include_md!("fixture/sample.md")` is evaluated at compile time
- **Then:** The macro expands without error; the file content is returned as a `&'static str` constant

### AP-2: Zero arguments rejected at compile time

- **Given:** No arguments are passed to the macro
- **When:** Code containing `include_md!()` is compiled
- **Then:** `cargo check` exits non-zero; a syn parse error is reported at the invocation site; no binary is produced

### AP-3: Two arguments rejected at compile time

- **Given:** Two string literal arguments are provided where one is expected
- **When:** Code containing `include_md!("a.md", "extra")` is compiled
- **Then:** `cargo check` exits non-zero; the extra token is rejected; no binary is produced

### AP-4: Valid invocation returns `&'static str`

- **Given:** A valid UTF-8 markdown file at the given path, within the 10 MB limit
- **When:** `include_md!("fixture/sample.md")` is used in a context requiring `&'static str`
- **Then:** The expanded value is accepted by the compiler as a `&'static str`; it equals the complete file contents at compile time

### AP-5: File not found produces compile error at invocation site

- **Given:** The path argument does not refer to an existing file
- **When:** Code containing `include_md!("nonexistent.md")` is compiled
- **Then:** `cargo check` exits non-zero; the error is attributed to the macro invocation site (not an internal location)

### AP-6: File exceeding 10 MB produces compile error at invocation site

- **Given:** A file larger than 10,000,000 bytes created at test time in a temp directory
- **When:** Code containing `include_md!("abs_path")` is compiled via subprocess `cargo check`
- **Then:** The const assertion `include_bytes!(path).len() <= 10_000_000` fails; `cargo check` exits non-zero; the error is reported at the macro invocation site; no binary is produced
