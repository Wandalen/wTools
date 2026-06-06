# Feature: Section Extraction

### Scope

- **Purpose**: Enable embedding a single named section from a markdown file as a compile-time string constant.
- **Responsibility**: Documents the section extraction feature — design rationale, heading matching decisions, boundary rules, and cross-references to all implementation and specification artifacts.
- **In Scope**: include_md_section! macro, heading identification, section boundary detection, duplicate heading handling.
- **Out of Scope**: Full-file inclusion (see feature/001), runtime file access, markdown rendering.

### Design

Allows precise embedding of one section from a larger markdown file without splitting that file into many single-section files. The motivating case: embedding only the "Usage" or "Configuration" section of a README into a doc comment, rather than the entire file.

Heading matching is case-sensitive and exact: the heading argument must match the file heading verbatim, including leading marker characters and any trailing whitespace. This avoids the ambiguity that case-insensitive matching would introduce for markdown files with mixed-case headings of the same text.

Section boundaries are level-aware: extraction ends at the next heading of equal or greater depth, capturing all nested subsections within the matched section's scope. This means a top-level section heading captures its entire tree of subsections as a self-contained unit, matching the reader's natural expectation.

When a heading appears multiple times in the file, the first occurrence is extracted without error. This is deterministic and predictable; raising an error on duplicates would break on common markdown patterns like repeated subheadings within different sections.

Path arguments resolve relative to `CARGO_MANIFEST_DIR` (the crate root containing the invocation). This differs from `include_md!`, which delegates to `include_str!` and resolves relative to the calling source file. The asymmetry arises from a stable Rust constraint: source-file-relative resolution in proc-macros requires `proc_macro::Span::source_file()`, which is gated on the unstable `proc_macro_span` feature.

### Sources

| File | Responsibility |
|------|----------------|
| `src/lib.rs` | Implements the `include_md_section!` proc-macro — `heading_level()`, `extract_section()`, and macro entry point |

### Tests

| File | Responsibility |
|------|----------------|
| `tests/section_extraction.rs` | Test suite covering heading match, boundary, nesting, duplicate, missing heading |

### Apis

| File | Responsibility |
|------|----------------|
| [api/002_include_md_section.md](../api/002_include_md_section.md) | Macro contract: parameters, heading semantics, error conditions |

### Invariants

| File | Responsibility |
|------|----------------|
| [invariant/001_path_resolution.md](../invariant/001_path_resolution.md) | Path resolution semantics |
| [invariant/002_compile_time_errors.md](../invariant/002_compile_time_errors.md) | Compile-time error guarantee |
| [invariant/003_size_limit.md](../invariant/003_size_limit.md) | 10 MB size constraint |
| [invariant/004_section_extraction_rules.md](../invariant/004_section_extraction_rules.md) | Formal behavioral rules for heading matching and boundary detection |

### Provenance

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Public API and §Section Extraction Behavior; deleted commit `c13cf485` (not migrated); recoverable from git history |
