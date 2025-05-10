# Plan

## Initial Task

Check crates at
- module/core/former
- module/core/former_meta
- module/core/macro_tools

Fix module\core\former\tests\inc\struct_tests\parametrized_dyn_manual.rs
- uncomment code
- duplicate the manual terive and do derive test actually using macro Former
- make macro working taking into account this corner case
- for your conveniency there expansion of macro in parametrized_dyn_manual.rs

Strictly follow code/gen, design rules and codestyle rules and prioritize it over codestyle and design used in repository.

---

## Project Plan: Fix Former Macro for Generics/dyn Trait (Following Proc Macro Workflow)

## Progress

*   ⏳ Increment 1: Finalize Manual Implementation (`parametrized_dyn_manual.rs`)
*   ⚫ Increment 2: Prepare Shared Test Logic (`parametrized_dyn_only_test.rs`)
*   ⚫ Increment 3: Verify Manual Implementation
*   ⚫ Increment 4: Create Macro Invocation Site (`parametrized_dyn_derive.rs`)
*   ⚫ Increment 5: Analyze Macro Failure & Implement Fix in `former_meta`
*   ⚫ Increment 6: Verify Macro Fix

## Increments

*   ⏳ Increment 1: Finalize Manual Implementation (`parametrized_dyn_manual.rs`).
    *   Goal: Ensure the manual code is uncommented, correct, and compilable.
    *   Rules: Strictly follow code/gen, design rules, and codestyle rules.
    *   Detailed Plan:
        *   Read `parametrized_dyn_manual.rs`.
        *   Identify and uncomment the main struct definition (`struct Struct1`) and its `impl` block.
        *   Identify and uncomment the associated `_Former` struct definition (`struct Struct1Former`) and its `impl` block.
        *   Identify and uncomment the `FormingEnd` trait implementation (`impl<...> FormingEnd<...> for Struct1Former<...>`).
        *   Apply required codestyle adjustments (spacing, newlines, indentation) to the uncommented code according to `code/rules/codestyle.md`.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily](code/rules/design.md#code-style-do-not-reformat-arbitrarily), [Comments: Focus on Rationale, Preserve Existing Tasks](code/rules/codestyle.md#comments-focus-on-rationale-preserve-existing-tasks)
    *   Verification Strategy: Compile check (`cargo check --tests` in `module/core/former`), manual review of uncommented code and codestyle.
*   ⚫ Increment 2: Prepare Shared Test Logic (`parametrized_dyn_only_test.rs`).
    *   Goal: Isolate test logic for reuse between manual and derive tests.
    *   Rules: Strictly follow code/gen, design rules, and codestyle rules.
*   ⚫ Increment 3: Verify Manual Implementation.
    *   Goal: Confirm the manual code passes its tests before touching the macro.
    *   Rules: Strictly follow code/gen, design rules, and codestyle rules.
*   ⚫ Increment 4: Create Macro Invocation Site (`parametrized_dyn_derive.rs`).
    *   Goal: Set up the test file that uses `#[derive(Former)]`.
    *   Rules: Strictly follow code/gen, design rules, and codestyle rules.
*   ⚫ Increment 5: Analyze Macro Failure & Implement Fix in `former_meta`.
    *   Goal: Identify the macro's shortcomings with the derive test and correct the macro logic.
    *   Rules: Strictly follow code/gen, design rules, and codestyle rules.
*   ⚫ Increment 6: Verify Macro Fix.
    *   Goal: Ensure both `_manual` and `_derive` tests pass with the updated macro.
    *   Rules: Strictly follow code/gen, design rules, and codestyle rules.

## Notes & Insights

*   *(No notes yet)*
