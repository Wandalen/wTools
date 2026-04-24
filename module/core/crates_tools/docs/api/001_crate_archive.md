# API: CrateArchive

### Scope

- **Purpose**: Document the complete public API surface for the `CrateArchive` struct.
- **Responsibility**: Define type signatures, method contracts, error types, and trait implementations.
- **In Scope**: All public methods, type parameters, return types, error types, derived traits.
- **Out of Scope**: Internal HashMap details (see `../invariant/001_in_memory_storage.md`), usage guides (see `../feature/001_archive_inspection.md`).

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

`CrateArchive` is the sole public type in `crates_tools`. It represents a fully-loaded `.crate` archive (gzip-compressed tar) as an in-memory collection of file paths and their byte contents. All methods are associated functions or `&self` read-only methods — no mutation API exists. The struct derives `Default`, `Clone`, and `PartialEq` and implements `Debug`.

### Operations

#### Struct Definition

```rust
#[ derive( Default, Clone, PartialEq ) ]
pub struct CrateArchive( HashMap< PathBuf, Vec< u8 > > );
```

`Debug` is implemented manually, emitting the list of file paths without file contents.

#### `read`

```rust
pub fn read< P : AsRef< Path > >( path : P ) -> std::io::Result< Self >
```

Reads a `.crate` file from the filesystem at `path`. Internally calls `std::fs::read()` then `decode()`. Returns `io::Error` on filesystem failure or malformed archive.

#### `decode`

```rust
pub fn decode< B : AsRef< [u8] > >( bytes : B ) -> std::io::Result< Self >
```

Decodes raw gzip-compressed tar bytes. Performs GzDecoder decompression followed by tar Archive iteration. Each entry's path and content are inserted into the HashMap. Empty byte slices yield an empty `CrateArchive` without error.

#### `download`

```rust
#[ cfg( feature = "network" ) ]
pub fn download< Url : AsRef< str > >( url : Url ) -> Result< Self, ureq::Error >
```

Downloads and decodes a `.crate` archive from any URL. The URL must point directly to a `.crate` file. Configured with 5-second read and write timeouts.

#### `download_crates_io`

```rust
#[ cfg( feature = "network" ) ]
pub fn download_crates_io< N, V >( name : N, version : V ) -> Result< Self, ureq::Error >
where
  N : core::fmt::Display,
  V : core::fmt::Display,
```

Constructs the canonical crates.io download URL from `name` and `version` then delegates to `download()`. Both `name` and `version` must be exact — no version range resolution occurs.

#### `list`

```rust
pub fn list( &self ) -> Vec< &Path >
```

Returns all file paths in the archive as a `Vec< &Path >`. Order is non-deterministic (HashMap iteration). Returns an empty `Vec` for an empty archive.

#### `content_bytes`

```rust
pub fn content_bytes< P : AsRef< Path > >( &self, path : P ) -> Option< &[u8] >
```

Returns the byte content of the file at `path`, or `None` if the path is not present in the archive. The returned slice borrows from `self` and is valid for the lifetime of the archive.

### Error Handling

- `read()` and `decode()` return `std::io::Error`. Possible causes: file not found, permission denied, malformed gzip, malformed tar, I/O failure during read.
- `download()` and `download_crates_io()` return `ureq::Error`. Possible causes: DNS failure, TCP connection refused, HTTP 4xx/5xx response, read/write timeout (5 seconds), invalid URL.
- No error type wrapping or conversion is performed. Callers receive the raw error type from the underlying library.

### Compatibility Guarantees

`CrateArchive` is a stable public type. The `Default` implementation yields an empty archive (empty HashMap). `Clone` performs a deep copy of all file contents. `PartialEq` compares by HashMap equality (path + content).

The `content_bytes` return type is `Option< &[u8] >` — the `None` variant means path absent, never means corrupted content. An empty file in the archive returns `Some( &[] )` (empty slice), not `None`.
