# Manual Testing Plan: variadic_from_meta

## Overview

This directory contains manual testing procedures and test cases for corner cases and edge conditions not suitable for automated testing.

## Test Categories

### 1. Boundary Conditions
- **0-field structs (unit structs)**: Verify macro generates no code
- **4+ field structs**: Verify macro generates no code — see [`variadic_from/docs/invariant/001_field_count_boundary.md`](../../../variadic_from/docs/invariant/001_field_count_boundary.md)

### 2. Type Complexity
- **String types**: Verify cloning behavior for convenience implementations
- **Lifetimes**: Verify generic parameter propagation with lifetime parameters
- **Const generics**: Verify handling of const generic parameters
- **PhantomData**: Verify handling of zero-sized type fields

### 3. Compilation Behavior
- **Unsupported struct types**: Verify proper error messages for unit structs
- **Type mismatches**: Verify compile-time type checking works correctly
- **Trait bound propagation**: Verify where clauses propagate correctly

## Testing Procedure

For each corner case:
1. Create test case demonstrating the behavior
2. Compile and verify expected outcome
3. Document any issues found
4. Create reproducing test in automated test suite if issue found

## Test Status

| Category | Test Case | Status | Notes |
|----------|-----------|--------|-------|
| Boundary | 0-field unit struct | ⏳ Pending | Should generate no code |
| Boundary | 4-field struct | ⏳ Pending | Should generate no code |
| Type Complexity | String cloning | ⏳ Pending | Verify convenience impl clones |
| Type Complexity | Lifetime parameters | ⏳ Pending | Verify propagation |
| Type Complexity | Const generics | ⏳ Pending | Verify support |
| Compilation | Unit struct error | ⏳ Pending | Should provide clear error |

## Execution

Manual tests are executed by:
1. Creating temporary test files in this directory
2. Compiling them manually with `cargo build` or `cargo test`
3. Observing compilation output and runtime behavior
4. Cleaning up temporary files after testing

## Results

Testing results will be documented here after execution.
