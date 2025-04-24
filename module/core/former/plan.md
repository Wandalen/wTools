# Plan

## Initial Task

Check crates at
- module/core/former
- module/core/former_meta
- module/core/macro_tools

Run tests for former and fix all failing tests.

Strictly follow code/gen, design rules and codestyle rules and prioritize it over codestyle and design used in repository.

## Progress

*   ⏳ **Increment 1: Fix compilation errors in `generics_independent_tuple_only_test.rs` and `generics_independent_tuple_derive.rs`** <-- Current
*   ⚫ Increment 2: Analyze remaining failing tests in `former`
*   ⚫ Increment 3: Fix remaining failing tests in `former`
*   ⚫ Increment 4: Analyze and fix tests in `former_meta` and `macro_tools` (if necessary)

## Increments

*   ⏳ Increment 1: Fix compilation errors in `generics_independent_tuple_only_test.rs` and `generics_independent_tuple_derive.rs`
    *   Detailed Plan Step 1: Read `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs`.
    *   Detailed Plan Step 3: Add `use std::marker::PhantomData;` to `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs`.
    *   Detailed Plan Step 4: Correct the calls to construct `EnumG5` in `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs` to use the correct method name (assuming `v1` is generated) and arguments, removing the incorrect `PhantomData` argument.
    *   Detailed Plan Step 5: Address the unused type parameter `T` in `EnumG5` in `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs` by adding `PhantomData<T>` to the enum definition.
    *   Detailed Plan Step 6: Re-run `cargo test` for the `former` crate.
    *   Crucial Design Rules: [Comments and Documentation](#comments-and-documentation), [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors), [Code Style: Do Not Reformat Arbitrarily](#code-style-do-not-reformat-arbitrarily)
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   Verification Strategy: `cargo test` for the `former` crate passes without the previously seen errors related to `generics_independent_tuple`.
*   ⚫ Increment 2: Analyze remaining failing tests in `former`
*   ⚫ Increment 3: Fix remaining failing tests in `former`
*   ⚫ Increment 4: Analyze and fix tests in `former_meta` and `macro_tools` (if necessary)

## Notes & Insights

*   [2025-04-24/Increment 1] Identified compilation errors in `generics_independent_tuple_only_test.rs` and `generics_independent_tuple_derive.rs` after initial `cargo test` run. Errors include missing `PhantomData` import, unused type parameter, incorrect variant construction calls, and incorrect number of arguments in test calls.
