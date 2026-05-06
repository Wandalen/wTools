# Invariant: File Persistence Contracts

### Scope

- **Purpose**: Define the behavioral contracts for all file write operations in config_hierarchy.
- **Responsibility**: Document the atomic-write guarantee and the created_at preservation rule.
- **In Scope**: Every config file save and atomic-modify operation.
- **Out of Scope**: File format structure (→ format/001); path construction (→ api/001).

### Invariant Statement

#### Atomic Write Guarantee

Every config file write operation is atomic with respect to other processes:

1. An exclusive advisory lock is acquired on the target file before any read or write
2. The lock is held for the duration of the read-modify-write cycle
3. The file is written via seek-to-start + truncate + rewrite on the same file descriptor
4. The lock is released only after the write is complete

Consequence: concurrent writes from multiple processes cannot corrupt the file — one writer blocks until the other finishes.

#### Created-At Preservation Rule

The `created_at` metadata field, once written on first save, is never overwritten:

- On first save: `created_at` is set to the current timestamp
- On every subsequent save: the existing `created_at` value is read from the file before writing, and the same value is written back
- Only `last_modified` is updated on subsequent saves

Consequence: `created_at` is a stable record of when the config file was first created, regardless of how many times the file is later modified.

### Enforcement Mechanism

The lock-acquire → read → write → release sequence is enforced by the file I/O layer. The `created_at` preservation is enforced by reading the existing timestamp before constructing the new file content.

### Violation Consequences

- Broken atomic-write: concurrent writes can produce a partially-written file, which may fail to parse
- Broken created-at preservation: config file creation time is lost on the first subsequent save, making audit trails unreliable

### APIs

| File | Relationship |
|------|--------------|
| [api/004_config_manager.md](../api/004_config_manager.md) | File I/O operations that must follow these contracts |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that exposes file persistence operations |

### Formats

| File | Relationship |
|------|--------------|
| [format/001_config_file_format.md](../format/001_config_file_format.md) | Format that defines the created_at field |

### Sources

| File | Relationship |
|------|--------------|
| [src/file_ops.rs](../../src/file_ops.rs) | Complete enforcement of both contracts |

### Tests

| File | Relationship |
|------|--------------|
| [tests/concurrent_access_tests.rs](../../tests/concurrent_access_tests.rs) | Atomic-write contract tests |
| [tests/edge_cases_tests.rs](../../tests/edge_cases_tests.rs) | created_at preservation tests |
