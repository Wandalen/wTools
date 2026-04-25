# fs Module

## Overview

Core filesystem utilities module providing temporary directory path management and path traversal functionality.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `fs.rs` | TempDir structure implementation with RAII cleanup semantics |
| `lib.rs` | Module aggregator and namespace organization (own/orphan/exposed/prelude) |
| `path.rs` | Generic path traversal utilities and ancestor directory discovery |

## Module Organization

This module follows traditional Rust namespace organization:
- **own**: Module's own types and functions
- **orphan**: Shared with parent namespace
- **exposed**: Public API surface
- **prelude**: Essential imports for convenience

## Key Components

### TempDir (fs.rs)
Temporary directory path container with RAII cleanup for directories created via `create()`/`create_all()`.

**Design Decisions:**
- Public fields for flexibility over encapsulation
- Three path components: base, prefix, postfix
- Cleanup only for directories created via methods (not user-specified paths)

### Path Utilities (path.rs)
Generic traversal with closure predicates enabling complex return types and filesystem operations.

**Key Functions:**
- `traverse_upward<T, F>` - Generic ancestor traversal with predicate
- `file_upward_find` - Locate files in ancestor directories
- `dir_upward_find` - Locate directories in ancestor chain
- `collect_files_in_ancestors` - Gather matching files from ancestor hierarchy

### Module Organization (lib.rs)
Aggregates `fs` and `path` submodules with traditional namespace hierarchy. Re-exports glob crate when `glob` feature enabled.
