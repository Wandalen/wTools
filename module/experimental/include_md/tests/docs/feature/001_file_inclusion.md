# Feature Spec: File Inclusion

**Source:** `docs/feature/001_file_inclusion.md`
**Test file:** `tests/file_inclusion.rs`
**Case prefix:** `FT-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| FT-1 | Valid file returns full contents | ✅ |
| FT-2 | Missing file is compile error | ✅ |
| FT-3 | Zero arguments is compile error | ✅ |
| FT-4 | Two arguments is compile error | ✅ |
| FT-5 | Oversized file is compile error | ✅ |
| FT-6 | Path resolves relative to calling source file | ✅ |
| FT-7 | Empty file returns empty string | ✅ |

---

### FT-1: Valid file returns full contents

- **Given:** A valid UTF-8 markdown file `tests/fixture/sample.md` exists with known content `"# Hello\n\nThis is a test fixture for include_md.\n"`
- **When:** `include_md!("fixture/sample.md")` is evaluated at compile time from `tests/file_inclusion.rs`
- **Then:** The macro expands to the complete file content as a `&'static str`; runtime assertion `content == expected` passes

### FT-2: Missing file is compile error

- **Given:** No file exists at the path argument supplied to the macro
- **When:** Code containing `include_md!("does_not_exist.md")` is compiled
- **Then:** Compilation fails with a compile-time error; no binary artifact is produced; the program cannot run

### FT-3: Zero arguments is compile error

- **Given:** No arguments are provided to the macro
- **When:** Code containing `include_md!()` is compiled
- **Then:** Compilation fails at the invocation site; the macro does not accept an empty argument list

### FT-4: Two arguments is compile error

- **Given:** Two string literal arguments are provided instead of the required one
- **When:** Code containing `include_md!("a.md", "b")` is compiled
- **Then:** Compilation fails; the macro requires exactly one string literal argument

### FT-5: Oversized file is compile error

- **Given:** A file whose byte count exceeds 10,000,000 bytes created at test time in a temp directory
- **When:** Code containing `include_md!("abs_path_to_large.md")` is compiled via subprocess `cargo check`
- **Then:** Compilation fails via the const assertion `include_bytes!(...).len() <= 10_000_000`; `cargo check` exits non-zero; no binary is produced

### FT-6: Path resolves relative to calling source file

- **Given:** A fixture file at `tests/fixture/sample.md` and a test function in `tests/file_inclusion.rs`
- **When:** `include_md!("fixture/sample.md")` is invoked from `tests/file_inclusion.rs`
- **Then:** The path `fixture/sample.md` is resolved relative to the directory of `tests/file_inclusion.rs` (i.e., `tests/`), finding the fixture at `tests/fixture/sample.md`; FT-1 passes

### FT-7: Empty file returns empty string

- **Given:** A zero-byte markdown file `tests/fixture/empty.md` exists
- **When:** `include_md!("fixture/empty.md")` is evaluated at compile time
- **Then:** The macro expands to `""` (an empty `&'static str`); no compile error is produced; the size check passes for a zero-byte file
