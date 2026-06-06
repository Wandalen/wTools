# Feature: File Inclusion

### Scope

- **Purpose**: Enable embedding a complete markdown file as a compile-time string constant in Rust source code.
- **Responsibility**: Documents the file inclusion feature — design rationale, key constraints, and cross-references to all implementation and specification artifacts.
- **In Scope**: include_md! macro, path resolution behavior, compile-time error guarantees, typical use cases.
- **Out of Scope**: Section-level extraction (see feature/002), runtime file access, markdown parsing or rendering.

### Design

Provides an ergonomic way to embed complete markdown files at compile time, eliminating the manual copy-paste of documentation content into source code. Primary use cases: injecting a crate's readme into module-level doc comments, embedding a changelog section as a const, including markdown-formatted API guides in generated documentation.

Path resolution follows the established compile-time include convention: relative to the source file containing the invocation, not the working directory. This consistency prevents confusion for developers already familiar with the standard built-in.

Files larger than 10 MB are rejected at compile time. The threshold prevents unbounded compile-time memory and time cost for repositories with large binary assets accidentally matched by glob patterns.

### Sources

| File | Responsibility |
|------|----------------|
| `src/lib.rs` | Implements the `include_md!` proc-macro entry point |

### Tests

| File | Responsibility |
|------|----------------|
| `tests/file_inclusion.rs` | Test suite covering file inclusion: found, missing, oversized, utf8-invalid |

### Apis

| File | Responsibility |
|------|----------------|
| [api/001_include_md.md](../api/001_include_md.md) | Macro contract: parameters, output type, error conditions |

### Invariants

| File | Responsibility |
|------|----------------|
| [invariant/001_path_resolution.md](../invariant/001_path_resolution.md) | Path resolution semantics |
| [invariant/002_compile_time_errors.md](../invariant/002_compile_time_errors.md) | Compile-time error guarantee |
| [invariant/003_size_limit.md](../invariant/003_size_limit.md) | 10 MB size constraint |

### Provenance

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Public API and §Implementation Details; deleted commit `c13cf485` (not migrated); recoverable from git history |
