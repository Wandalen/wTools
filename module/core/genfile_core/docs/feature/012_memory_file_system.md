# Feature: Memory File System

### Scope

- **Purpose**: Provides an in-memory filesystem implementation enabling fast, isolated unit tests.
- **Responsibility**: Documents the in-memory file system implementation and its in-memory semantics.
- **In Scope**: In-memory storage, no real disk access, full read/write round-trip in memory.
- **Out of Scope**: The trait contract (→ 010), production implementation (→ 011).

### Design

The in-memory file system stores file content in a map from path to byte vector. Writes add or replace entries; reads return the stored bytes or an error for missing paths. The create-directory-all operation is a no-op since in-memory storage requires no directory creation. No real filesystem operations are performed, making tests fast and side-effect free.

### Features

| File | Relationship |
|------|--------------|
| [`feature/010_file_system_trait.md`](010_file_system_trait.md) | Trait that the in-memory file system implements |

### Invariants

| File | Relationship |
|------|--------------|
| [`invariant/002_memory_efficiency.md`](../invariant/002_memory_efficiency.md) | Memory ceiling that applies to this implementation |

### Sources

| File | Relationship |
|------|--------------|
| [`src/filesystem.rs`](../../src/filesystem.rs) | In-memory file system implementation |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/filesystem_test.rs`](../../tests/inc/filesystem_test.rs) | In-memory file system round-trip tests |
