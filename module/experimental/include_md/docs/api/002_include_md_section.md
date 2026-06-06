# API: include_md_section Macro

### Scope

- **Purpose**: Provide compile-time extraction of a single named section from a markdown file as a string constant.
- **Responsibility**: Documents the include_md_section! macro contract — path and heading arguments, section boundary semantics, output type, and error conditions.
- **In Scope**: Macro invocation, path resolution, heading matching rules, section boundary detection, and all compile-time error cases.
- **Out of Scope**: Full-file inclusion (see api/001), runtime file access, markdown rendering.

### Abstract

A compile-time macro that reads one named section from a markdown file and substitutes it as a string constant at the invocation site. The section is identified by its heading string; extraction boundaries are level-aware and include all nested subsections. Path arguments resolve relative to `CARGO_MANIFEST_DIR` (crate root), unlike `include_md!` which is source-file-relative; see invariant/001.

### Operations

- **include_md_section**: Accepts two string literal arguments — a file path and a heading string (verbatim, including leading marker characters). Resolves the path relative to `CARGO_MANIFEST_DIR` (the manifest directory of the crate containing the invocation). Returns a compile-time string constant containing all content from the matched heading until the next heading of equal or higher level, inclusive of nested subsections.

### Error Handling

All failure modes produce compile-time errors — no runtime panics, no propagated error values. Covered conditions: file not found, file unreadable, file exceeds 10 MB, invalid UTF-8, heading not found in file. When multiple headings match, no error is raised — first occurrence is extracted deterministically (see invariant/004).

### Compatibility Guarantees

Path resolution uses `CARGO_MANIFEST_DIR` (crate root), which differs from `include_md!`'s source-file-relative resolution; see invariant/001 for details. Section boundary detection is deterministic: first-match extraction for duplicate headings, level-aware inclusive boundary for nested subsections. These guarantees hold regardless of file size, heading depth, or nesting depth, subject to the 10 MB limit.

### Sources

| File | Responsibility |
|------|----------------|
| `src/lib.rs` | Implements `include_md_section!` — argument parsing, CARGO_MANIFEST_DIR path resolution, file reading, section extraction |

### Features

| File | Responsibility |
|------|----------------|
| [feature/002_section_extraction.md](../feature/002_section_extraction.md) | User-facing design rationale for section extraction |

### Invariants

| File | Responsibility |
|------|----------------|
| [invariant/001_path_resolution.md](../invariant/001_path_resolution.md) | Path resolution contract this macro upholds |
| [invariant/002_compile_time_errors.md](../invariant/002_compile_time_errors.md) | Compile-time error contract |
| [invariant/003_size_limit.md](../invariant/003_size_limit.md) | 10 MB file size constraint |
| [invariant/004_section_extraction_rules.md](../invariant/004_section_extraction_rules.md) | Heading matching and boundary detection rules |

### Provenance

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Public API; deleted commit `c13cf485` (not migrated); content recoverable from git history |
