# API: include_md Macro

### Scope

- **Purpose**: Provide compile-time inclusion of a complete markdown file as a string constant.
- **Responsibility**: Documents the include_md! macro contract — path argument, file reading semantics, output type, and error conditions.
- **In Scope**: Macro invocation, path resolution semantics, success output, and all compile-time error cases.
- **Out of Scope**: Section-level extraction (see api/002), runtime file access, markdown parsing or rendering.

### Abstract

A compile-time macro that reads a markdown file from disk and substitutes its full contents as a string constant at the invocation site. Extends the standard compile-time file inclusion primitive with markdown-specific intent; path resolution semantics are identical to the standard built-in, preserving the established mental model.

### Operations

- **include_md**: Accepts a single file path argument (a string literal). Resolves the path relative to the source file containing the invocation. Returns a compile-time string constant holding the complete UTF-8 contents of the specified file, usable anywhere a string literal is valid.

### Error Handling

All failure modes produce compile-time errors — no runtime panics, no propagated error values. Covered conditions: file not found (reported with the attempted path), file unreadable (reported with the reason), file exceeds the 10 MB size limit (see invariant/003), file contains invalid UTF-8.

### Compatibility Guarantees

Path resolution is identical to the standard compile-time file inclusion built-in: paths resolve relative to the source file, not the working directory or crate root. Output is a compile-time constant; any consumer that accepts a string literal accepts the macro output without modification.

### Sources

| File | Responsibility |
|------|----------------|
| `src/lib.rs` | Implements `include_md!` — argument parsing, size assertion emission, `include_str!` delegation |

### Features

| File | Responsibility |
|------|----------------|
| [feature/001_file_inclusion.md](../feature/001_file_inclusion.md) | User-facing design rationale for file inclusion |

### Invariants

| File | Responsibility |
|------|----------------|
| [invariant/001_path_resolution.md](../invariant/001_path_resolution.md) | Path resolution contract this macro upholds |
| [invariant/002_compile_time_errors.md](../invariant/002_compile_time_errors.md) | Compile-time error contract |
| [invariant/003_size_limit.md](../invariant/003_size_limit.md) | 10 MB file size constraint |

### Provenance

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Public API; deleted commit `c13cf485` (workspace-wide cleanup, not migrated); content recoverable from git history |
