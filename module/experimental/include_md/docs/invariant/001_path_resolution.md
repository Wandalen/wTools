# Invariant: Path Resolution

### Scope

- **Purpose**: Guarantee that path arguments to both macros resolve relative to the invoking source file, identical to the standard compile-time include built-in.
- **Responsibility**: Documents the path resolution invariant — the rule itself, its enforcement mechanism, and the consequences of violation.
- **In Scope**: Relative path semantics, path base anchor, both macros uniformly.
- **Out of Scope**: Absolute path handling, runtime file access, file system traversal behavior.

### Invariant Statement

Path arguments are resolved relative to the file that contains the macro invocation, using the invoking source file's directory as the base. This is identical to the resolution behavior of the standard compile-time include built-in.

### Enforcement Mechanism

The macro resolver captures the invoking source file location at macro expansion time and constructs the absolute path by joining the source file's parent directory with the supplied path argument. No runtime path lookup is involved.

### Violation Consequences

If path resolution were anchored to the working directory or any location other than the invoking source file, path arguments would silently become depth-dependent: the same argument would resolve to different files depending on where the build was invoked, breaking reproducibility and making library crate path arguments unusable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/_blank/standard_lib.rs` | Placeholder; future home of macro entry points |
| doc | [api/001_include_md.md](../api/001_include_md.md) | Full-file macro contract |
| doc | [api/002_include_md_section.md](../api/002_include_md_section.md) | Section extraction macro contract |
| doc | [feature/001_file_inclusion.md](../feature/001_file_inclusion.md) | User-facing rationale for path resolution convention |
| doc | [feature/002_section_extraction.md](../feature/002_section_extraction.md) | Section extraction feature; also uses caller-relative path resolution |
| doc | [invariant/002_compile_time_errors.md](002_compile_time_errors.md) | Compile-time error contract that receives path resolution failures |

### Sources

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Path Resolution; deleted commit `c13cf485` (not migrated); recoverable from git history |
