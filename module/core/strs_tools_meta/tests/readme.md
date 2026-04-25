# tests

Test suite for strs_tools_meta procedural macros.

## Overview

This directory contains comprehensive integration tests for the `optimize_split` and `optimize_match` procedural macros. Tests verify compile-time optimizations, pattern matching strategies, and code generation correctness.

## Organization

Tests are organized by macro functionality with dedicated test modules for each procedural macro. The `integration_tests.rs` file serves as the main entry point importing feature-gated test modules.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `integration_tests.rs` | Aggregate test modules for integration |
| `optimize_match_tests.rs` | Test optimize_match macro |
| `optimize_split_tests.rs` | Test optimize_split macro |

## Test Coverage

- **optimize_split macro**: 10 test cases covering single-char delimiters, multi-char delimiters, multiple delimiters, preserve options, debug mode, and parameter validation
- **optimize_match macro**: 10 test cases covering single pattern, multiple patterns, matching strategies (first_match, longest_match, all_matches), debug mode, and parameter validation

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
