# tests

Test suite for strs_tools_meta procedural macros.

## Overview

This directory contains comprehensive integration tests for the `optimize_split` and `optimize_match` procedural macros. Tests verify compile-time optimizations, pattern matching strategies, and code generation correctness.

## Organization

Tests are organized by macro functionality with dedicated test binaries for each procedural macro. `integration_tests.rs` tests both macros together in the same expansion context.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `integration_tests.rs` | Test both macros used in same expansion context |
| `optimize_match_tests.rs` | Test optimize_match macro |
| `optimize_split_tests.rs` | Test optimize_split macro |
| `corner_cases_test.rs` | Test edge cases for both macros |
| `compile_fail_test.rs` | Compile-fail tests for non-literal delimiter and pattern |
| `manual/` | Manual testing plan and known-behavior documentation |
| `docs/` | Test surface spec files (feature/, api/, invariant/) |

## Test Coverage

- **optimize_split macro** (`optimize_split_tests.rs`): 10 test cases — single-char delimiter, multi-char delimiter, multiple delimiters, preserve options, debug mode, parameter validation
- **optimize_match macro** (`optimize_match_tests.rs`): 11 test cases — single pattern, multiple patterns, matching strategies, debug mode, parameter validation, strategy equivalence
- **corner cases** (`corner_cases_test.rs`): 28 tests — empty input, boundary positions, Unicode, UTF-8 emoji, overlapping patterns/delimiters, optimization threshold edges (8/9 split, 16/17 match)

## Running Tests

```bash
# Run all tests
cargo nextest run --all-features

# Run with workspace context
w3 .test l::3

# Run specific test module
cargo nextest run optimize_split_tests
cargo nextest run optimize_match_tests
```

## Test Matrix Documentation

Each test module contains detailed Test Matrix documentation in file-level doc comments describing test scenarios, input variations, and expected behaviors.
