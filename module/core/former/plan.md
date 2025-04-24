# Project Plan: Fix Failing Former Enum Tests

## Initial Task

Check crates at
- module/core/former
- module/core/former_meta
- module/core/macro_tools

Run tests for former and fix all failing tests.
Before planning run tests to determine list of test files which fails and fix one by one.
Before starting analyze ALL sources at module/core/former_meta/src
Note that manual tests for enums probably has wrong outdated implementation.

Strictly follow code/gen, design rules and codestyle rules and prioritize it over codestyle and design used in repository.
Do plan according to requirments of code/gen after running tests. Don't edit file before plan is ready.

## Progress

*   ⚫ Increment 1: Fix `scalar_generic_tuple_manual.rs` test failures (E0412, E0433)
*   ⚫ Increment 2: Fix `multi_field_only_test.rs` test failures (E0061, E0599)
*   ⚫ Increment 3: Fix `generics_independent_tuple_only_test.rs` test failures (E0599)
*   ⚫ Increment 4: Review and potentially update manual enum tests

## Increments

*   ⚫ Increment 1: Fix `scalar_generic_tuple_manual.rs` test failures (E0412, E0433)
*   ⚫ Increment 2: Fix `multi_field_only_test.rs` test failures (E0061, E0599)
*   ⚫ Increment 3: Fix `generics_independent_tuple_only_test.rs` test failures (E0599)
*   ⚫ Increment 4: Review and potentially update manual enum tests (as noted in initial task)

## Notes & Insights

*   [2025-04-24/Init] Test run revealed failures in:
    *   `scalar_generic_tuple_manual.rs` (E0412, E0433 - missing types)
    *   `multi_field_only_test.rs` (E0061, E0599 - wrong args, missing method)
    *   `generics_independent_tuple_only_test.rs` (E0599 - missing variant/method)
*   [2025-04-24/Init] Analysis of `former_meta` suggests fixes likely involve `former_enum.rs` and `field.rs`, focusing on generic variant handling, naming, and setter generation.
*   [2025-04-24/Init] Initial task mentions potential issues with manual enum tests needing updates. Added Increment 4 to address this.
