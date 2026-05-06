# Tool Module

Low-level utilities for external tool interaction and system operations.

### Responsibility Table

| File | Responsibility |
|------|---------------|
| `cargo.rs` | Executes cargo commands and parses output |
| `files.rs` | Performs file system operations |
| `git.rs` | Executes git commands and manages repository state |
| `graph.rs` | Constructs and analyzes dependency graphs |
| `http.rs` | Handles HTTP requests for crates.io API |
| `iter.rs` | Provides iterator utilities |
| `macros.rs` | Defines utility macros for tool module |
| `mod.rs` | Declares and organizes tool submodules |
| `path.rs` | Re-exports path types for tool layer |
| `query.rs` | Queries crates.io and package registries |
| `repository.rs` | Manages repository operations and metadata |
| `semver_utils.rs` | Provides semantic versioning utilities |
| `tree.rs` | Formats and prints tree structures |
| `url.rs` | Handles URL construction and manipulation |
