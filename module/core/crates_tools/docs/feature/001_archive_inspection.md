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

`CrateArchive` supports four loading paths:

- **`read`** — Loads from a filesystem path.
- **`decode`** — Decodes raw bytes already in memory. Useful for bytes from any source: custom download, test fixtures, embedded data.
- **`download_crates_io`** — Downloads from crates.io by exact crate name and version. Requires the `network` feature.
- **`download`** — Downloads from any URL pointing directly to a `.crate` file. Requires the `network` feature.

All four paths produce an equivalent in-memory archive. See [CrateArchive API](../api/001_crate_archive.md) for full contracts, error types, and timeout configuration.

#### Inspecting Contents

Once loaded, two read-only operations provide content access:

- **`list`** — Enumerates all file paths in the archive. Order is non-deterministic.
- **`content_bytes`** — Retrieves the byte content for a given path. A missing path is distinct from an empty file — absent paths return cleanly.

See [CrateArchive API](../api/001_crate_archive.md) for the full access contract.

#### Archive Path Format

Archive paths include the crate name and version prefix as encoded by `cargo package`. For example, a crate named `my_crate` at version `1.0.0` stores files as `my_crate-1.0.0/src/lib.rs`. This prefix is included verbatim in `list` output and must be provided when calling `content_bytes`.

#### Feature Flags

All functionality requires the `enabled` feature (on by default). Network-based download methods additionally require the `network` feature (also on by default via `enabled`).
