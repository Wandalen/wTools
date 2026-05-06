# Invariant: Size Limit

### Scope

- **Purpose**: Guarantee that files exceeding 10 MB are rejected at compile time before any content is processed.
- **Responsibility**: Documents the file size constraint — the threshold, measurement method, enforcement point, and rationale.
- **In Scope**: Both macros uniformly; size check timing relative to content read; threshold definition.
- **Out of Scope**: Memory usage during section extraction, output string size, build system memory limits.

### Invariant Statement

Files exceeding 10 MB — measured as the total byte count of the file before any content is read or processed — are rejected with a compile-time error. The limit applies uniformly to both macros. For section extraction, the entire file is checked against the limit before any heading search begins; a file that exceeds 10 MB is rejected regardless of the size of the target section.

### Enforcement Mechanism

The macro implementation reads the file metadata to obtain the byte count before opening the file for content. If the byte count exceeds 10,000,000 bytes, the macro emits a compile-time error at the invocation site and does not proceed to read content. This prevents memory exhaustion during compilation from unexpectedly large files.

### Violation Consequences

Without a size limit, an accidentally-included binary artifact, log file, or generated dataset embedded by a path argument would cause the compiler process to allocate the entire file in memory during macro expansion. At sufficient scale this crashes the compiler or degrades build performance. The pre-read check makes the failure fast and explicit rather than resource-exhaustion-based.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/_blank/standard_lib.rs` | Placeholder; future home of macro entry points |
| doc | [api/001_include_md.md](../api/001_include_md.md) | Full-file macro error handling section |
| doc | [api/002_include_md_section.md](../api/002_include_md_section.md) | Section macro error handling section |
| doc | [invariant/002_compile_time_errors.md](002_compile_time_errors.md) | Compile-time error contract |
| doc | [feature/001_file_inclusion.md](../feature/001_file_inclusion.md) | File inclusion feature; size constraint applies uniformly to full-file inclusion |
| doc | [feature/002_section_extraction.md](../feature/002_section_extraction.md) | Section extraction feature; size constraint checked before extraction begins |

### Sources

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Non-Functional Requirements; deleted commit `c13cf485` (not migrated); recoverable from git history |
