# tests/inc

## Purpose
Shared test infrastructure included by integration test files via `mod inc`.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Re-exports all shared test modules for test file inclusion |
| `test_helpers.rs` | Common test utilities: assertion helpers and sample data builders |
| `alignment_helpers.rs` | Display-width assertion utilities for Unicode alignment tests |
