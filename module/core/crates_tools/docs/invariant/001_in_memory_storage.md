# Invariant: In-Memory Storage

### Scope

- **Purpose**: Define the storage contract that all archive content is held in memory after loading.
- **Responsibility**: Documents the in-memory storage guarantee and its consequences.
- **In Scope**: Internal storage type, read-only access, no disk extraction guarantee.
- **Out of Scope**: Network download details (see `002_blocking_network.md`), API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | CrateArchive(HashMap< PathBuf, Vec< u8 > >) definition |
| test | `tests/smoke_test.rs` | Basic archive decode contract tests |
| test | `tests/corner_cases_comprehensive.rs` | Edge cases for archive content access |
| doc | `../feature/001_archive_inspection.md` | Feature relying on this storage model |
| doc | `../api/001_crate_archive.md` | API surface for content access |

### Invariant Statement

`CrateArchive` stores the complete archive as `HashMap< PathBuf, Vec< u8 > >`. After any successful load (via `read`, `decode`, or `download*`), all file contents are fully materialized in memory. No deferred loading, streaming, or lazy evaluation occurs.

Access via `list()` and `content_bytes()` is always O(1) per file after the initial load. The HashMap maps each archive file path (as `PathBuf`) to its full byte content (`Vec< u8 >`). The bytes are the raw file content exactly as encoded in the tar entry.

### Enforcement Mechanism

Enforced structurally: `CrateArchive( HashMap< PathBuf, Vec< u8 > > )` is the sole representation. The `decode()` method iterates the tar archive exactly once at load time, inserting all entries into the HashMap. No path to partial materialization exists in the API.

### Violation Consequences

- **Memory pressure**: Large archives consume proportional heap. No size limit is enforced; the caller is responsible for choosing appropriate crates.
- **No streaming**: Files cannot be accessed incrementally during download. The full archive must be received before any content is readable.
- **Borrow lifetime**: Content returned by `content_bytes()` borrows from `CrateArchive`. The archive must remain live for the duration of any borrow.
