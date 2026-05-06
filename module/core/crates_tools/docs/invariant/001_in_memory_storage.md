# Invariant: In-Memory Storage

### Scope

- **Purpose**: Define the storage contract that all archive content is held in memory after loading.
- **Responsibility**: Documents the in-memory storage guarantee and its consequences.
- **In Scope**: In-memory data model, read-only access, no disk extraction guarantee.
- **Out of Scope**: Network download details (see `002_blocking_network.md`), API contracts (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | CrateArchive path-to-bytes mapping definition |
| test | `tests/smoke_test.rs` | Basic archive decode contract tests |
| test | `tests/corner_cases_comprehensive.rs` | Edge cases for archive content access |
| doc | `../feature/001_archive_inspection.md` | Feature relying on this storage model |
| doc | `../api/001_crate_archive.md` | API surface for content access |

### Invariant Statement

`CrateArchive` stores the complete archive as an in-memory mapping from file path to byte content. After any successful load, all file contents are fully materialized in memory. No deferred loading, streaming, or lazy evaluation occurs.

Access via `list` and `content_bytes` is always constant-time per file after the initial load. Each file path maps to its full byte content exactly as encoded in the tar entry.

### Enforcement Mechanism

Enforced structurally: the sole internal representation is a complete path-to-bytes mapping. The decoding step iterates the tar archive exactly once at load time, inserting all entries before returning. No path to partial materialization exists in the API.

### Violation Consequences

- **Memory pressure**: Large archives consume proportional heap. No size limit is enforced; the caller is responsible for choosing appropriate crates.
- **No streaming**: Files cannot be accessed incrementally during download. The full archive must be received before any content is readable.
- **Lifetime dependency**: Byte content retrieved via `content_bytes` is tied to the archive's lifetime. The archive must remain in scope for the duration of any access derived from it.
