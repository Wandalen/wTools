# Feature: Archive Inspection

### Scope

- **Purpose**: Enable in-memory inspection of `.crate` archive files without disk extraction.
- **Responsibility**: Document the archive inspection capability and its usage.
- **In Scope**: Downloading archives from crates.io, reading local `.crate` files, decoding gzip/tar archives, listing file paths, accessing file bytes.
- **Out of Scope**: Disk extraction, Cargo.toml parsing, checksum verification, version resolution, diff generation.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | CrateArchive implementation |
| test | `tests/crates_tools_tests.rs` | Download and file listing tests |
| test | `tests/corner_cases_comprehensive.rs` | Edge cases and no-panic guarantees |
| doc | `../invariant/001_in_memory_storage.md` | In-memory storage invariant |
| doc | `../invariant/002_blocking_network.md` | Blocking network invariant |
| doc | `../api/001_crate_archive.md` | CrateArchive API surface |

### Design

#### Loading Archives

`CrateArchive` supports three loading paths:

- **`read(path)`** — Reads a `.crate` file from the filesystem. Returns `io::Result<Self>`.
- **`decode(bytes)`** — Decodes raw gzip-compressed tar bytes already in memory. Returns `io::Result<Self>`. Used when bytes arrive from any source (custom download, test fixtures, embedded data).
- **`download_crates_io(name, version)`** — Downloads by crate name and exact version from `crates.io`. Feature-gated on `network`. Returns `Result<Self, ureq::Error>`.
- **`download(url)`** — Downloads from any arbitrary URL. Feature-gated on `network`. Returns `Result<Self, ureq::Error>`.

All paths produce an identical `CrateArchive` containing every file from the archive stored as `HashMap< PathBuf, Vec< u8 > >`.

#### Inspecting Contents

Once loaded, two read-only methods provide content access:

- **`list()`** — Returns `Vec< &Path >` of all file paths in the archive. Order is non-deterministic (HashMap iteration).
- **`content_bytes(path)`** — Returns `Option< &[u8] >` for a given path. Returns `None` if path is not in the archive.

#### Archive Path Format

Archive paths include the crate name and version prefix as encoded by `cargo package`. For example, a crate named `my_crate` at version `1.0.0` stores files as `my_crate-1.0.0/src/lib.rs`. This prefix is included verbatim in `list()` output and must be provided when calling `content_bytes()`.

#### Feature Flags

```toml
[features]
default = ["enabled"]
full    = ["enabled"]
enabled = ["dep:flate2", "dep:tar", "network"]
network = ["dep:ureq"]
```

All functionality requires the `enabled` feature (on by default). Network-based download methods additionally require the `network` feature (also on by default via `enabled`).
