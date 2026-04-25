# API: CrateArchive

### Scope

- **Purpose**: Document the complete public API surface for the `CrateArchive` struct.
- **Responsibility**: Define method contracts, error conditions, and compatibility guarantees.
- **In Scope**: All public methods, their operational behavior, error conditions, and stability guarantees.
- **Out of Scope**: Internal storage details (see `../invariant/001_in_memory_storage.md`), usage guides (see `../feature/001_archive_inspection.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | CrateArchive definition and method implementations |
| test | `tests/crates_tools_tests.rs` | Download and list integration tests |
| test | `tests/smoke_test.rs` | Basic decode contract tests |
| doc | `../feature/001_archive_inspection.md` | Feature guide for archive inspection |
| doc | `../invariant/001_in_memory_storage.md` | In-memory storage invariant |
| doc | `../invariant/002_blocking_network.md` | Blocking network invariant |

### Abstract

`CrateArchive` is the sole public type in `crates_tools`. It represents a fully-loaded `.crate` archive (gzip-compressed tar) as an in-memory collection of file paths and their byte contents. All methods are read-only — no mutation API exists. The type supports default construction (empty archive), cloning, equality comparison, and debug output that lists file paths without their byte contents.

### Operations

#### `read`

Loads a `.crate` file from the filesystem at the given path. Reads the file bytes then delegates to `decode`. Returns an I/O error on filesystem failure or a malformed archive.

#### `decode`

Decodes raw gzip-compressed tar bytes. Performs decompression followed by tar entry iteration, inserting each file's path and byte content into the archive. An empty byte input yields an empty archive without error.

#### `download` *(requires `network` feature)*

Downloads and decodes a `.crate` archive from any URL. The URL must point directly to a `.crate` file. Configured with a 5-second read timeout and a 5-second write timeout.

#### `download_crates_io` *(requires `network` feature)*

Constructs the canonical crates.io download URL from `name` and `version`, then delegates to `download`. Both `name` and `version` must be exact — no version range resolution occurs.

#### `list`

Returns all file paths present in the archive. Order is non-deterministic. Returns an empty collection for an empty archive.

#### `content_bytes`

Returns the byte content of the file at the given path, or absent if the path is not in the archive. The returned bytes are valid for the lifetime of the archive.

### Error Handling

- `read` and `decode` return an I/O error. Possible causes: file not found, permission denied, malformed gzip, malformed tar.
- `download` and `download_crates_io` return a network error. Possible causes: DNS failure, connection refused, HTTP error response, read/write timeout (5 seconds), invalid URL.
- No error wrapping or conversion is performed.

### Compatibility Guarantees

`CrateArchive` is a stable public type. Default construction yields an empty archive. Cloning performs a deep copy of all file paths and byte contents. Equality comparison is by path and content.

The absence result from `content_bytes` signals a missing path, never corrupted content. An archive entry that exists but contains zero bytes returns an empty byte slice — not an absence result.
