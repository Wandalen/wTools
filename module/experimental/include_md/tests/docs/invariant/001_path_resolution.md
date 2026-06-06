# Invariant Spec: Path Resolution

**Source:** `docs/invariant/001_path_resolution.md`
**Test files:** `tests/file_inclusion.rs`, `tests/section_extraction.rs`
**Case prefix:** `IN-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| IN-1 | `include_md!` path resolves relative to calling source file | ✅ |
| IN-2 | `include_md_section!` path resolves relative to CARGO_MANIFEST_DIR (crate root) | ✅ |

---

### IN-1: `include_md!` path resolves relative to calling source file

- **Given:** A fixture file at `tests/fixture/sample.md` and a test in `tests/file_inclusion.rs`
- **When:** `include_md!("fixture/sample.md")` is invoked from that test file
- **Then:** The path resolves relative to `tests/` (the directory of `file_inclusion.rs`), locating the fixture correctly; the full file contents are returned as confirmed by FT-1

### IN-2: `include_md_section!` path resolves relative to CARGO_MANIFEST_DIR (crate root)

- **Given:** A fixture file at `tests/fixture/multi_section.md` relative to the crate root, and `include_md_section!` invoked with that relative path
- **When:** `include_md_section!("tests/fixture/multi_section.md", "# Introduction")` is evaluated during compilation
- **Then:** The path is resolved relative to `CARGO_MANIFEST_DIR` (the crate's `Cargo.toml` directory); the fixture is found and the requested section is returned; the same path argument fails if anchored to a different directory
- **Note:** Source-file-relative resolution requires the unstable `proc_macro_span` feature; `CARGO_MANIFEST_DIR` is the documented stable behavior for this macro
