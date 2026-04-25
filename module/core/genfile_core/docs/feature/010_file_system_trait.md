# Feature: File System Trait

### Scope

- **Purpose**: Abstracts file I/O so generation can be tested without touching the real filesystem.
- **Responsibility**: Documents the `FileSystem` trait and its required operations.
- **In Scope**: Write, read, and create-directory operations; the abstraction contract.
- **Out of Scope**: Real filesystem implementation (→ 011), in-memory implementation (→ 012).

### Design

The file system trait defines three operations: write (file path + bytes → disk), read (file path → bytes), and create-directory-all (path → directory tree). Any type implementing this trait can back template generation. The abstraction boundary enables unit testing via the in-memory file system without filesystem side effects.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/filesystem.rs` | `FileSystem` trait definition |
| doc | `docs/feature/011_real_file_system.md` | Production implementation |
| doc | `docs/feature/012_memory_file_system.md` | In-memory testing implementation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR10 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
