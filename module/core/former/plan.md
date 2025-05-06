# Project Plan: Fix Warnings in `former` Crate

## Goal

*   Fix all warnings reported by `cargo check` and `cargo test` in the `former` crate.

## Context

*   Files to Include in `context.md`:
    *   `module/core/former/src/lib.rs`
    *   `module/core/former_meta/src/lib.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
    *   `module/core/former_meta/src/derive_former/field.rs`
    *   `module/core/former_types/src/lib.rs`
    *   `module/core/macro_tools/src/lib.rs`
    *   `module/core/former/tests/inc/mod.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_only_test.rs`
*   Crates for Documentation in `context.md`:
    *   `former`
    *   `former_meta`
    *   `former_types`
    *   `macro_tools`

## Increments

*   ⚫ Increment 1: Run `cargo check` and `cargo test` in `module/core/former`, analyze warnings, and fix them iteratively.
    *   Detailed Plan Step 1: Execute `cargo check` in `module/core/former` to get the initial list of warnings.
    *   Detailed Plan Step 2: Execute `cargo test --lib` in `module/core/former` to get any additional warnings reported by tests.
    *   Detailed Plan Step 3: Analyze the combined output, identify the first warning.
    *   Detailed Plan Step 4: Propose and apply code changes to fix the warning, adhering to codestyle and design rules.
    *   Detailed Plan Step 5: Repeat steps 3-4 for each subsequent warning until no warnings are reported by either `cargo check` or `cargo test --lib`.
    *   Verification Strategy: Run `cargo check` and `cargo test --lib` in `module/core/former` after each fix or group of fixes to confirm the warning is resolved and no new warnings or errors are introduced.

## Requirements
*   Fix all warnings reported by `cargo check` and `cargo test --lib` in the `former` crate.
*   Do not uncomment or modify the commented-out tests unless a warning specifically relates to them.

## Notes & Insights
