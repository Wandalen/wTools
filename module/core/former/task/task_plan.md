# Task Plan: Fix all tests in `former` crate

### Goal
*   To identify and fix all failing tests within the `former` crate to establish a stable baseline before proceeding with new feature development.

### Ubiquitous Language (Vocabulary)
*   N/A for this task.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/former`
*   **Overall Progress:** 1/6 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Initial Test Run and Failure Analysis
    *   ❌ Increment 2: Fix Macro-Generated Generic Parameters (Attempt 1)
    *   ⏳ Increment 3: Focused Debugging for `scalar_generic_tuple_derive`
    *   ⚫ Increment 4: Debug Macro Entry Point and Parsing
    *   ⚫ Increment 5: Create Minimal Reproducible Example (MRE)
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/former_meta` (Reason: Test failures may originate from the proc-macro implementation)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./task/enum_feature_plan_paused.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/former/src/lib.rs`
    *   `module/core/former_meta/src/lib.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `former`
    *   `former_meta`

### Expected Behavior Rules / Specifications
*   Rule 1: All tests in the `former` crate must pass successfully when run with `cargo test -p former`.
*   Rule 2: All tests in the `former_meta` crate must pass successfully when run with `cargo test -p former_meta`.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `tests::inc::enum_unnamed_tests::scalar_generic_tuple_derive` | Failing (Stuck) | Macro expansion fails due to unparsable tokens. `error: comparison operators cannot be chained`. Multiple fix attempts failed. Root cause seems to be in initial parsing of generic enums. |
| `tests::inc::enum_unnamed_tests::scalar_generic_tuple_only_test` | Failing (New) | Fails due to missing methods, a symptom of the macro failure. |

### Crate Conformance Check Procedure
*   **Step 1: Run Build.** Execute `timeout 300 cargo build -p former -p former_meta`. If this fails, fix all compilation errors before proceeding.
*   **Step 2: Run Tests (Conditional).** Only if Step 1 passes, execute `timeout 300 cargo test -p former` and `timeout 300 cargo test -p former_meta`.
*   **Step 3: Run Linter (Conditional).** Only if Step 2 passes, execute `timeout 300 cargo clippy -p former -- -D warnings` and `timeout 300 cargo clippy -p former_meta -- -D warnings`.

### Increments
(Note: The status of each increment is tracked in the `### Progress` section.)
##### Increment 1: Initial Test Run and Failure Analysis
*   **Goal:** Execute the test suite for the `former` and `former_meta` crates to identify all failing tests and populate the `### Tests` tracking table.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute `timeout 300 cargo test -p former`.
    *   Step 2: Analyze the output. For each failing test, add an entry to the `### Tests` table with the status `Failing (New)`.
    *   Step 3: Execute `timeout 300 cargo test -p former_meta`.
    *   Step 4: Analyze the output. For each failing test, add an entry to the `### Tests` table with the status `Failing (New)`.
    *   Step 5: If there are failing tests, subsequent increments will be dynamically added to fix them one by one. If all tests pass, this increment is complete.
*   **Increment Verification:**
    *   The `### Tests` table in this plan is accurately populated with all failing tests from the `former` and `former_meta` crates.
*   **Commit Message:** "chore(testing): Identify failing tests in former crates"

##### Increment 2: Fix Macro-Generated Generic Parameters (Attempt 1)
*   **Goal:** Fix the `former_meta` proc-macro to generate correct generic parameter syntax, resolving the `unparsable tokens` error.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read the file `module/core/former_meta/src/derive_former/former_enum.rs` to locate the code generation logic for enum formers.
    *   Step 2: Identify the part of the code that generates the `FormerDefinitionTypes` struct and its `impl` block. The error `EnumScalarGenericVariant2FormerDefinitionTypes < < T : Bound >` suggests the issue is with how the generic parameters are being combined.
    *   Step 3: Propose a fix to correct the generation of generic parameters, likely involving how `qt!` is used with `macro_tools::GenericsRef`. The goal is to produce `< T: Bound >` instead of `< < T : Bound > >`.
    *   Step 4: Apply the fix using `search_and_replace`.
    *   Step 5: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 300 cargo test -p former`.
    *   Analyze the output to confirm that the `unparsable tokens` error is resolved and that the `scalar_generic_tuple_derive` test now passes.
*   **Commit Message:** "fix(former_meta): Correct generic parameter generation in enum former"

##### Increment 3: Focused Debugging for `scalar_generic_tuple_derive`
*   **Goal:** Isolate the failure in the `scalar_generic_tuple_derive` test by simplifying the test case.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Apply Problem Reduction.** Read the file `module/core/former/tests/inc/enum_unnamed_tests/scalar_generic_tuple_derive.rs`.
    *   Step 2: Temporarily remove the `Variant2(InnerScalar<T>, bool)` from the `EnumScalarGeneric` enum in the test file. This will isolate the single-field scalar variant (`Variant1`).
    *   Step 3: Read the file `module/core/former/tests/inc/enum_unnamed_tests/scalar_generic_tuple_only_test.rs`.
    *   Step 4: Temporarily comment out the tests that use `variant_2`.
    *   Step 5: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 300 cargo test -p former`.
    *   Analyze the output. If the test now passes, the problem is confirmed to be in the multi-field variant handling. If it still fails, the problem is more general.
*   **Commit Message:** "chore(testing): Isolate failure in scalar_generic_tuple test"

##### Increment 4: Debug Macro Entry Point and Parsing
*   **Goal:** Investigate and fix the initial parsing of generic enums at the derive macro's entry point.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/former_meta/src/lib.rs` to find the `#[proc_macro_derive(Former)]` function.
    *   Step 2: Read `module/core/former_meta/src/derive_former.rs` which likely contains the main dispatch logic.
    *   Step 3: Analyze the parsing logic. The problem is likely in how `syn::parse(input).unwrap()` or similar is called, and how the result is processed before being passed to `former_for_enum`.
    *   Step 4: Propose a fix to ensure generic parameters and their bounds are correctly parsed and maintained.
    *   Step 5: Apply the fix.
    *   Step 6: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 300 cargo test -p former` with the simplified test case.
    *   Confirm the `comparison operators cannot be chained` error is gone.
*   **Commit Message:** "fix(former_meta): Correct parsing of generic enums in derive macro"

##### Increment 5: Create Minimal Reproducible Example (MRE)
*   **Goal:** Create a new, minimal crate to isolate the failing test case and determine if the issue is environmental.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create a new library crate: `cargo new --lib module/core/former_mre`.
    *   Step 2: Add `former` as a dependency to `module/core/former_mre/Cargo.toml`.
    *   Step 3: Copy the minimal failing code (the `Bound` trait, `InnerScalar` struct, and `EnumScalarGeneric` enum) into `module/core/former_mre/src/lib.rs`.
    *   Step 4: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 300 cargo build -p former_mre`.
    *   Analyze the output. If it fails with the same error, the bug is confirmed in the macro. If it passes, the issue is with the `former` crate's test setup.
*   **Commit Message:** "chore(testing): Create MRE for generic enum parsing issue"

##### Increment 6: Finalization
*   **Goal:** Perform a final verification of the `former` and `former_meta` crates.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Revert any temporary changes made during debugging.
    *   Step 2: Remove the temporary `former_mre` crate.
    *   Step 3: Perform a final Crate Conformance Check.
    *   Step 4: Self-critique against all requirements and rules.
*   **Increment Verification:**
    *   All checks pass.
*   **Commit Message:** "chore(former): Finalize test fixes"

### Task Requirements
*   Do not run commands for the whole workspace. All `cargo` commands must be targeted at a specific crate using the `-p` flag.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The `former` crate and its dependencies are expected to build, even if tests fail.

### Out of Scope
*   Implementing new features.
*   Fixing tests in any crate other than `former` and `former_meta`.

### External System Dependencies
*   None.

### Notes & Insights
*   The test `scalar_generic_tuple_derive` is stuck. The root cause is a fundamental parsing issue with `syn` and generic enums when the derive macro is present. An MRE is needed to isolate the problem from the current test environment.

### Changelog
*   [Initial] Task created to fix all tests in `former` and `former_meta`.
*   [Increment 2] Attempted to fix generic parameter generation, but this repeatedly introduced syntax errors. The test remains stuck, indicating a deeper issue. Moving to focused debugging.