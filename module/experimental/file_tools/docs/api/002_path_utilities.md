# API: Path Utilities

### Scope

- **Purpose**: Document the public contract of the five path traversal functions in `file_tools::path`.
- **Responsibility**: Authoritative reference for function signatures, parameters, return values, and depth semantics.
- **In Scope**: `traverse_upward`, `collect_files_in_ancestors`, `file_upward_find`, `dir_upward_find`, `matching_upward_find`; `max_depth` semantics; deduplication flag.
- **Out of Scope**: Internal traversal mechanics (see source), usage workflows (see `../feature/003_path_traversal.md`).

### Abstract

Five functions for walking the directory hierarchy upward from a starting path. All are available under `cfg(feature = "enabled")` and require `std`. Four functions return the first match; one accumulates across all ancestor levels.

### Operations

**`traverse_upward<T, F>(start_dir: &Path, predicate: F, max_depth: usize) -> Option<T>`**
Generic upward walker. Calls `predicate(current_dir)` at each level starting from `start_dir`. Returns `Some(T)` on first truthy result; returns `None` if no level matches within `max_depth` steps or the filesystem root is reached.

**`collect_files_in_ancestors<F>(target: &Path, predicate: F, max_depth: Option<usize>, deduplicate: bool) -> io::Result<Vec<PathBuf>>`**
Collects all matching files at every ancestor level from root down to `target`. Result order is root-first (root to target), enabling override semantics. When `deduplicate: true`, files with the same name appearing at multiple levels are deduplicated, keeping the root-level occurrence.

**`file_upward_find(start: &Path, filename: &str, max_depth: usize) -> Option<PathBuf>`**
Walks upward from `start`, returning the first directory containing a file with the exact name `filename`.

**`dir_upward_find(start: &Path, dirname: &str, max_depth: usize) -> Option<PathBuf>`**
Walks upward from `start`, returning the first path where a subdirectory named `dirname` exists.

**`matching_upward_find<F>(start: &Path, predicate: F, max_depth: usize) -> Option<PathBuf>`**
Walks upward from `start`, returning the first directory path for which `predicate(path)` returns `true`.

### Error Handling

`collect_files_in_ancestors` returns `io::Result<Vec<PathBuf>>`; errors from directory reads (permissions, I/O) propagate. All other functions return `Option`; filesystem errors cause the affected level to be skipped (treated as non-matching).

### Compatibility Guarantees

Available under `cfg(feature = "enabled")`. Requires `std`. No compatibility guarantees at `0.x.y`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/path.rs` | All five function implementations |
| test | `../../tests/path_test.rs` | 33 tests covering all functions and edge cases |
| doc | `../feature/003_path_traversal.md` | User-facing capability description |
