# Invariant: Compile-Time Errors

### Scope

- **Purpose**: Guarantee that all failure conditions surface as compile-time errors — no runtime panics, no propagated error values.
- **Responsibility**: Documents the compile-time error invariant — the rule, its enforcement mechanism, and all covered failure conditions.
- **In Scope**: All error conditions for both macros: file not found, unreadable, oversized, invalid UTF-8, heading not found.
- **Out of Scope**: Warnings, deprecation notices, linter diagnostics, runtime behavior of compiled output.

### Invariant Statement

Every failure condition that can occur during macro expansion — file not found, file unreadable, file exceeds the size limit, file content is not valid UTF-8, heading not found in file — produces a compile-time error at the macro invocation site. No failure condition results in a runtime panic or a propagated error value in the compiled binary.

### Enforcement Mechanism

Every error path inside the macro implementation terminates by emitting a compiler diagnostic via the proc-macro error mechanism. The macro never returns a fallback value and never emits code that could panic at runtime. The compiler rejects the invocation before producing any executable artifact.

### Violation Consequences

A runtime panic or propagated error would move a build-time configuration mistake into production code paths, making it observable only at runtime. Compile-time rejection makes broken invocations impossible to ship: a build that compiles is guaranteed to have valid, accessible, correctly-encoded files at all invocation sites.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/_blank/standard_lib.rs` | Placeholder; future home of macro entry points |
| doc | [api/001_include_md.md](../api/001_include_md.md) | Full-file macro error handling section |
| doc | [api/002_include_md_section.md](../api/002_include_md_section.md) | Section macro error handling section |
| doc | [invariant/001_path_resolution.md](001_path_resolution.md) | Path resolution contract (feeds file-not-found errors) |
| doc | [invariant/003_size_limit.md](003_size_limit.md) | Size limit contract (feeds oversized-file errors) |
| doc | [feature/001_file_inclusion.md](../feature/001_file_inclusion.md) | File inclusion feature; all error conditions feed into this invariant |
| doc | [feature/002_section_extraction.md](../feature/002_section_extraction.md) | Section extraction feature; all error conditions feed into this invariant |
| doc | [invariant/004_section_extraction_rules.md](004_section_extraction_rules.md) | Section extraction rules; heading-not-found is one of the covered error conditions |

### Sources

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Error Handling; deleted commit `c13cf485` (not migrated); recoverable from git history |
