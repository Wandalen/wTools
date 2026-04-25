# Feature: Memory File System

### Scope

- **Purpose**: Provides an in-memory filesystem implementation enabling fast, isolated unit tests.
- **Responsibility**: Documents `MemoryFileSystem` and its in-memory semantics.
- **In Scope**: In-memory storage, no real disk access, full read/write round-trip in memory.
- **Out of Scope**: The trait contract (→ 010), production implementation (→ 011).

### Design

The in-memory file system stores file content in a map from path to byte vector. Writes add or replace entries; reads return the stored bytes or an error for missing paths. The create-directory-all operation is a no-op since in-memory storage requires no directory creation. No real filesystem operations are performed, making tests fast and side-effect free.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/filesystem.rs` | `MemoryFileSystem` implementation |
| doc | `docs/feature/010_file_system_trait.md` | Trait that `MemoryFileSystem` implements |
| doc | `docs/invariant/002_memory_efficiency.md` | Memory ceiling that applies to this implementation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR12 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
