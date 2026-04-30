# Feature: Path Traversal Utilities

### Scope

- **Purpose**: Provide upward directory-tree traversal and ancestor file collection to eliminate duplicated traversal logic across workspace tools.
- **Responsibility**: Documents the five path traversal functions extracted from cargo_tools, git_tools, and rulebook discovery into a single authoritative implementation.
- **In Scope**: `traverse_upward` (generic), `collect_files_in_ancestors`, `file_upward_find`, `dir_upward_find`, `matching_upward_find`; deduplication behaviour; `max_depth` control.
- **Out of Scope**: Downward directory traversal, recursive directory walking (`walkdir` use case), file content operations.

### Design

All five functions share a single traversal primitive: `traverse_upward<T, F>(start_dir, predicate, max_depth)`. Walking upward from a start directory, it applies a predicate at each level and returns the first match. Higher-level helpers delegate to this primitive with domain-specific predicates.

`collect_files_in_ancestors` is the only function that accumulates across all ancestor levels rather than stopping at the first match. It collects matching files at every level from the target up to the root, deduplicates across levels (when `deduplicate: true`), and returns results in root-to-target order to enable correct override semantics (closest-to-root wins in config resolution scenarios).

This module was extracted to eliminate 268 lines of duplicate code:
- `cargo_tools::path` — ~196 lines
- `git_tools::path` — ~26 lines
- Rulebook discovery — ~46 lines

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/path.rs` | All five function implementations |
| test | `../../tests/path_test.rs` | 33 tests across all functions and edge cases |
| doc | `../api/002_path_utilities.md` | Function signatures and parameter contracts |
