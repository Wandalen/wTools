# Invariant: Path Resolution

### Scope

- **Purpose**: Document the path resolution semantics for both macros — source-file-relative for `include_md!`, CARGO_MANIFEST_DIR-relative for `include_md_section!`.
- **Responsibility**: Documents the path resolution invariant — the rule itself, its enforcement mechanism, and the consequences of violation.
- **In Scope**: Relative path semantics, path base anchor, both macros — with distinct resolution mechanisms per macro.
- **Out of Scope**: Absolute path handling, runtime file access, file system traversal behavior.

### Invariant Statement

The two macros use different path resolution anchors, each the best available under stable Rust constraints:

**include_md!**: Path arguments resolve relative to the source file containing the invocation, identical to the standard compile-time file inclusion built-in.

**include_md_section!**: Path arguments resolve relative to `CARGO_MANIFEST_DIR` — the manifest directory of the crate containing the invocation. Source-file-relative resolution would require `proc_macro::Span::source_file()`, which is gated on the unstable `proc_macro_span` feature. `CARGO_MANIFEST_DIR` is the stable equivalent for proc-macro file I/O.

### Enforcement Mechanism

**include_md!** delegates path resolution to the compiler by emitting `include_str!(path)` in the expanded code. The compiler resolves the path relative to the source file at the macro invocation site — identical to the standard built-in. No proc-macro file I/O is required.

**include_md_section!** reads the file at proc-macro expansion time. The path is resolved by calling `std::env::var("CARGO_MANIFEST_DIR")` and joining the result with the supplied path argument to form an absolute path. `CARGO_MANIFEST_DIR` is set by Cargo to the directory of the `Cargo.toml` for the crate being compiled, making this equivalent to crate-root-relative resolution.

### Known Limitation — Incremental Rebuilds

`include_md_section!` reads the markdown file at proc-macro expansion time via `fs::read_to_string`. Registering the resolved path for incremental-rebuild invalidation would require `proc_macro::tracked_path::path()`, but that API is behind `#![feature(track_path)]` (nightly-only as of Rust 1.94.1). On stable Rust, changes to an included markdown file are not detected by the build system. **Users must run `cargo clean` before rebuilding to pick up changes to files included by `include_md_section!`.**

`include_md!` is unaffected: it delegates to the compiler's own `include_str!`, which handles incremental invalidation automatically.

### Violation Consequences

If either macro silently changed its resolution anchor, the same path argument would resolve to different files depending on where the build was invoked or which source directory hosts the invocation, breaking reproducibility. For `include_md!`, using any anchor other than source-file-relative would break the mental model shared with the standard built-in. For `include_md_section!`, using any anchor other than `CARGO_MANIFEST_DIR` would make file paths non-deterministic across nested source file locations.

### Sources

| File | Responsibility |
|------|----------------|
| `src/lib.rs` | Implements path resolution for both macros — source-file-relative via `include_str!` for `include_md!`, CARGO_MANIFEST_DIR-relative for `include_md_section!` |

### Apis

| File | Responsibility |
|------|----------------|
| [api/001_include_md.md](../api/001_include_md.md) | Full-file macro contract |
| [api/002_include_md_section.md](../api/002_include_md_section.md) | Section extraction macro contract |

### Features

| File | Responsibility |
|------|----------------|
| [feature/001_file_inclusion.md](../feature/001_file_inclusion.md) | User-facing rationale for path resolution convention |
| [feature/002_section_extraction.md](../feature/002_section_extraction.md) | Section extraction feature; uses CARGO_MANIFEST_DIR path resolution |

### Invariants

| File | Responsibility |
|------|----------------|
| [invariant/002_compile_time_errors.md](002_compile_time_errors.md) | Compile-time error contract that receives path resolution failures |

### Provenance

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Path Resolution; deleted commit `c13cf485` (not migrated); recoverable from git history |
