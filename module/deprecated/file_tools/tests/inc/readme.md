# inc Test Modules

## Overview

Core functionality test modules organized by functional domain.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `basic_test.rs` | Basic TempDir structure creation and field access tests |
| `glob_test.rs` | Glob crate re-export and pattern matching integration tests |
| `mod.rs` | Test module aggregator |
| `tempdir_test.rs` | TempDir RAII lifecycle and cleanup semantics tests |

## Domain Organization

### TempDir Basic Tests (basic_test.rs)
**3 tests** covering structure creation, field initialization, and path manipulation.

**Coverage:**
- TempDir::new() with empty defaults
- Field access and modification
- full_path() construction from components

### TempDir RAII Tests (tempdir_test.rs)
**10 tests** covering Drop cleanup semantics and directory creation lifecycle.

**Coverage:**
- create() and create_all() operations
- Automatic cleanup on Drop
- Preservation of user-specified directories
- Empty TempDir instance handling

### Glob Integration Tests (glob_test.rs)
**9 tests** covering glob crate re-export and pattern matching.

**Coverage:**
- Module and type accessibility (glob, Pattern, MatchOptions, Paths, errors)
- Pattern compilation and matching
- Recursive glob patterns
- Pattern options and configuration

## Test Count

**Total: 22 tests** organized across 3 domain-focused test files.
