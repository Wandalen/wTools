# Feature: Real File System

### Scope

- **Purpose**: Implements the file system abstraction for production use against the actual operating system filesystem.
- **Responsibility**: Documents the real file system implementation and its production I/O behavior.
- **In Scope**: Directory creation, file writes, file reads, I/O error wrapping.
- **Out of Scope**: The trait contract (→ 010), testing alternative (→ 012).

### Design

The real file system implementation uses standard I/O operations. On write, it creates all parent directories before writing the file. On read, it reads bytes from the given path. All OS errors are wrapped in a typed filesystem error variant for uniform handling upstream.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/filesystem.rs` | Real file system implementation |
| test | `tests/` | Real file system I/O operation tests |
| doc | `docs/feature/010_file_system_trait.md` | Trait that the real file system implements |
