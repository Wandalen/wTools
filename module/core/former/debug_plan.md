# Project Plan: Debug and Fix Generic Enum Macro Expansion

### Goal
*   Identify and fix the root cause of the `comparison operators cannot be chained` and `proc-macro derive produced unparsable tokens` errors when `#[derive(Former)]` is used on a generic enum.

### Progress
*   🚀 Phase 1 Complete (Increment 1)
*   Key Milestones Achieved: ✅ Increment 1: Isolate the failing test case.
*   Currently Working On: ❌ **Stuck:** Increment 2: Analyze and fix the responsible handler.

### Target Crate
*   `module/core/former`
*   **Note:** The fix will likely be in `module/core/former_meta`. For this debugging task, `former_meta` will be modified directly as per the plan's intent, treating it as the effective target for code changes.

### Relevant Context
*   **Failing Test Case:** `module/core/former/tests/inc/enum_unit_tests/generic_unit_variant_derive.rs`
*   **Likely Bug Location:** `module/core/former_meta/src/derive_former/former_enum.rs`
*   **Key Files:**
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
    *   `module/core/macro_tools/src/generic_params.rs`

### Expected Behavior Rules
*   The `Former` derive macro must generate syntactically correct Rust code for generic enums, including those with `where` clauses.
*   The generated code must be equivalent to the manual implementation of the `Former` pattern.

### Increments

*   [✅] **Increment 1: Isolate the failing test case**
    *   Pre-Analysis: The `debug_plan.md` already identifies the failing test. The goal is to modify `module/core/former/tests/inc/mod.rs` to run only this test, speeding up the debugging cycle.
    *   Detailed Plan Step 1: Read `module/core/former/tests/inc/mod.rs` to understand its structure.
    *   Detailed Plan Step 2: Comment out all test inclusions in `module/core/former/tests/inc/mod.rs` except for `generic_unit_variant_derive`.
    *   Verification Strategy: Execute `cargo test --package former --test tests` via `execute_command`. Analyze the output to confirm that only the `generic_unit_variant_derive` test runs and fails as expected.
    *   Commit Message: `chore(former): Isolate failing generic enum test`

*   [❌] **Increment 2: Analyze and fix the responsible handler**
    *   **Stuck Resolution:** Previous attempts to fix the token generation in both the specific handler (`tuple_single_field_scalar.rs`) and the main `former_enum.rs` file have failed to resolve the cryptic `comparison operators cannot be chained` error. This indicates a fundamental issue with how the multiple generated items (`impl`, `fn`, `struct`) are being combined into a single token stream.
    *   **Conclusion:** The problem is a subtle token stream corruption issue that is beyond the current capabilities to debug with the available information. The debugging plan is now considered complete, but the issue remains unresolved. All changes have been reverted.
    *   Commit Message: `chore(former): Conclude debugging of generic enum expansion`

### Task Requirements
*   The fix must not introduce regressions in other parts of the `Former` macro.
*   The fix must be robust and handle all valid forms of generic enum definitions.

### Project Requirements
*   All verification must be done on a per-crate basis.
*   Do not run workspace-level commands.

### Notes & Insights
*   The error `comparison operators cannot be chained` is a red herring from the compiler, indicating a subtle token stream corruption.
*   The issue is likely related to the interaction between the `impl` block's generics and the generics of the methods being generated inside it.
*   **[Stuck]** Multiple attempts to fix generics and where clauses have failed. The problem is likely structural.