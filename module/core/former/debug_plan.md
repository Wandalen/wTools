# Project Plan: Debug and Fix Generic Enum Macro Expansion

### Goal
*   Identify and fix the root cause of the `comparison operators cannot be chained` and `proc-macro derive produced unparsable tokens` errors when `#[derive(Former)]` is used on a generic enum.

### Progress
*   [ ⏳ ] Phase 1: Isolate and Analyze (Increments 1-3)
*   [ ⚫ ] Phase 2: Implement and Verify Fix (Increments 4-5)
*   Key Milestones Achieved: ✅ Increment 1, ✅ Increment 2, ✅ Increment 3
*   Currently Working On: Increment 4

### Target Crate
*   `module/core/former` (for testing and validation)
*   `module/core/former_meta` (for code modifications and the fix)
*   **Note:** This task requires direct modification of both the user crate (`former`) and its associated proc-macro crate (`former_meta`). They will be treated as a single logical unit for this plan.

### Relevant Context
*   **Failing Test Case:** `module/core/former/tests/inc/enum_unit_tests/generic_unit_variant_derive.rs`
*   **Likely Bug Location:** A handler within `module/core/former_meta/src/derive_former/former_enum.rs` or its submodules.
*   **Key Files:**
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
    *   `module/core/former/tests/inc/mod.rs`
    *   `module/core/former/tests/inc/enum_unit_tests/generic_unit_variant_derive.rs`

### Expected Behavior Rules
*   The `Former` derive macro must generate syntactically correct Rust code for generic enums, including those with `where` clauses.
*   The generated code must be equivalent to a correct manual implementation of the `Former` pattern.

### Increments

*   [✅] **Increment 1: Isolate the failing test case**
    *   Pre-Analysis: To isolate the failing test without adding comments, we will temporarily replace the contents of the main test include file (`tests/inc/mod.rs`) to only run the single problematic test. The original content will be restored in the final increment.
    *   Detailed Plan Step 1: Read `module/core/former/tests/inc/mod.rs` to store its original content for later restoration.
    *   Detailed Plan Step 2: Use `write_to_file` to overwrite `module/core/former/tests/inc/mod.rs` with only the line that includes `generic_unit_variant_derive.rs`.
    *   Verification Strategy: Execute `cargo test --package former --test tests` via `execute_command`. Analyze the output to confirm that only the `generic_unit_variant_derive` test runs and fails as expected.
    *   Commit Message: `chore(former): Isolate failing generic enum test`

*   [✅] **Increment 2: Capture and Analyze Macro Output**
    *   Pre-Analysis: The compiler errors are cryptic because they are symptoms of malformed code. The root cause can only be found by inspecting the code the macro is generating.
    *   Detailed Plan Step 1: Modify the main `former_enum` handler in `module/core/former_meta/src/derive_former/former_enum.rs` to temporarily print the generated token stream to the console (e.g., using `println!("!{}", result);`).
    *   Verification Strategy: Execute the isolated failing test from Increment 1 (`cargo test --package former --test tests`). The test will still fail, but the output from `execute_command` will now contain the malformed generated code.
    *   Commit Message: `feat(former_meta): Add debug output to former_enum macro`

*   [✅] **Increment 3: Create a Manual, Working Implementation**
    *   Pre-Analysis: Based on the captured (and broken) macro output, it's clear what the macro is *trying* to do. We will now create a manual, correct version of this code to serve as a reference.
    *   Detailed Plan Step 1: Create a new test file `module/core/former/tests/inc/enum_unit_tests/generic_unit_variant_manual.rs`.
    *   Detailed Plan Step 2: In this new file, manually write the correct `Former` implementation for the generic enum, fixing the syntax errors observed in the macro's output from Increment 2.
    *   Detailed Plan Step 3: Modify `module/core/former/tests/inc/mod.rs` to include this new manual test instead of the failing derive test.
    *   Verification Strategy: Execute `cargo test --package former --test tests` via `execute_command`. Analyze the output to confirm the manual implementation compiles and passes.
    *   Commit Message: `test(former): Add manual implementation for generic enum`

*   [⚫] **Increment 4: Pinpoint Discrepancy and Fix the Handler**
    *   Pre-Analysis: We now have the broken output from the macro and a working manual implementation. By comparing them, we can find the exact source of the syntax error. The issue is likely in how the `where` clause or other generic parameters are being rendered.
    *   Detailed Plan Step 1: Compare the captured macro output with the working manual code.
    *   Detailed Plan Step 2: Identify the specific handler in `former_meta` responsible for generating the incorrect token sequence.
    *   Detailed Plan Step 3: Correct the logic in the identified handler to produce tokens that exactly match the working manual implementation.
    *   Detailed Plan Step 4: Remove the temporary debug `println!` from the macro.
    *   Detailed Plan Step 5: Modify `module/core/former/tests/inc/mod.rs` to run the original `generic_unit_variant_derive.rs` test.
    *   Verification Strategy: Execute `cargo test --package former --test tests` via `execute_command`. Analyze `execute_command` output to confirm the test now passes.
    *   Commit Message: `fix(former_meta): Correct token generation for generic enums`

*   [⚫] **Increment 5: Final Verification and Cleanup**
    *   Pre-Analysis: The specific fix is verified. Now, restore the original test configuration and ensure no regressions were introduced.
    *   Detailed Plan Step 1: Use `write_to_file` to restore the original content of `module/core/former/tests/inc/mod.rs`.
    *   Detailed Plan Step 2: Delete the temporary manual test file `module/core/former/tests/inc/enum_unit_tests/generic_unit_variant_manual.rs`.
    *   Verification Strategy: Execute the full test suite `cargo test --package former --test tests` via `execute_command`. Analyze the output to ensure all tests pass.
    *   Commit Message: `chore(former): Re-enable all tests and cleanup`

### Task Requirements
*   The fix must not introduce regressions in other parts of the `Former` macro.
*   The fix must be robust and handle all valid forms of generic enum definitions.
*   **No comments are to be added to the source code.**

### Project Requirements
*   All verification must be done on a per-crate basis.
*   Do not run workspace-level commands.

### Notes & Insights
*   The error `comparison operators cannot be chained` is a red herring from the compiler, indicating a subtle token stream corruption.
*   **Insight:** Debugging proc-macros is most effective when you can see the code they generate. Adding a temporary `println!` is a crucial first step.
*   **Insight:** Creating a parallel, manual implementation provides a "golden standard" to compare against, making it much easier to spot subtle syntax errors in the generated code.
*   **Insight:** The generated code for the generic enum has several syntax errors: missing `where` clauses on standalone functions, incorrect generic bounds on those functions, and improper concatenation of the `impl` block and the functions.