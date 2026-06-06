# Invariant: Size Limit

### Scope

- **Purpose**: Guarantee that files exceeding 10 MB are rejected at compile time before any content is processed.
- **Responsibility**: Documents the file size constraint — the threshold, measurement method, enforcement point, and rationale.
- **In Scope**: Both macros uniformly; size check timing relative to content read; threshold definition.
- **Out of Scope**: Memory usage during section extraction, output string size, build system memory limits.

### Invariant Statement

Files exceeding 10 MB — measured as the UTF-8 byte count of the file content — are rejected with a compile-time error. The limit applies uniformly to both macros. For section extraction, the entire file is checked against the limit before any heading search begins; a file that exceeds 10 MB is rejected regardless of the size of the target section.

### Enforcement Mechanism

The two macros enforce the limit by different mechanisms:

**include_md!** emits a const assertion in the expanded code: `const _: () = assert!(include_bytes!(path).len() <= 10_000_000, "file exceeds 10 MB limit")`. The compiler evaluates this constant expression before the `include_str!` output is used; a file that exceeds the limit produces a compile-time error at the invocation site.

**include_md_section!** reads the full file content via `fs::read_to_string()` then checks `content.len() > 10_000_000`. If the byte count exceeds 10,000,000, the macro emits a compile-time error at the invocation site before any heading search begins. The check uses the UTF-8 byte count of the content string, consistent with how `include_md!` measures size via `include_bytes!`.

### Violation Consequences

Without a size limit, an accidentally-included binary artifact, log file, or generated dataset embedded by a path argument would cause the compiler process to allocate the entire file in memory during macro expansion. At sufficient scale this crashes the compiler or degrades build performance. The pre-read check makes the failure fast and explicit rather than resource-exhaustion-based.

### Sources

| File | Responsibility |
|------|----------------|
| `src/lib.rs` | Implements size enforcement — const assertion for `include_md!`, `content.len()` check for `include_md_section!` |

### Apis

| File | Responsibility |
|------|----------------|
| [api/001_include_md.md](../api/001_include_md.md) | Full-file macro error handling section |
| [api/002_include_md_section.md](../api/002_include_md_section.md) | Section macro error handling section |

### Features

| File | Responsibility |
|------|----------------|
| [feature/001_file_inclusion.md](../feature/001_file_inclusion.md) | File inclusion feature; size constraint applies uniformly to full-file inclusion |
| [feature/002_section_extraction.md](../feature/002_section_extraction.md) | Section extraction feature; size constraint checked before extraction begins |

### Invariants

| File | Responsibility |
|------|----------------|
| [invariant/002_compile_time_errors.md](002_compile_time_errors.md) | Compile-time error contract |

### Provenance

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Non-Functional Requirements; deleted commit `c13cf485` (not migrated); recoverable from git history |
