# Manual Testing Directory

This directory contains manual verification tests for corner cases and edge behaviors in the pth crate.

## Purpose

Manual tests verify behaviors that:
- May be implementation-specific or platform-dependent
- Document edge case behaviors explicitly
- Provide additional verification beyond automated CI tests
- Explore boundary conditions not fully covered by unit tests

## Files

| File | Responsibility |
|------|----------------|
| `corner_case_verification.rs` | Verify edge case behaviors for all public path manipulation functions |

## Running Manual Tests

```bash
# Run all manual tests
cargo test --test corner_case_verification --all-features

# Run specific function tests
cargo test --test corner_case_verification verify_is_glob_edge_cases --all-features
cargo test --test corner_case_verification verify_normalize_edge_cases --all-features
cargo test --test corner_case_verification verify_iter_join_edge_cases --all-features
```

## Test Coverage

- **is_glob**: Empty strings, unclosed brackets/braces, double escapes, mixed escapes
- **normalize**: Empty paths, multiple slashes, root with dotdots
- **iter_join**: Empty iterators, all empty strings, multiple absolutes, backslash conversion
- **exts**: Hidden files, dots at end, only dots, paths without filenames
- **without_ext**: No extension, multiple extensions, hidden files, trailing dots
- **change_ext**: No existing extension, empty extension, extension with dot, multiple extensions
- **path_common**: Empty iterators, single paths, identical paths, no common paths
- **rebase**: Root as file/old path
- **path_relative**: Same path, direct child/parent, root cases, no common ancestor
- **ext**: No extension, multiple extensions, hidden files, trailing dots
- **unique_folder_name**: Uniqueness verification across 100 generations

## Expected Behavior

All tests document expected behaviors explicitly. Some behaviors are platform-specific or implementation-dependent and are noted as such in test output.

## Test Philosophy

These manual tests follow the principle of "explicit over implicit" - documenting actual behavior rather than assuming it. This helps:
1. Catch behavioral regressions
2. Document implementation decisions
3. Verify cross-platform consistency
4. Provide examples for users
