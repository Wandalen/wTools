# Task Plan: Fix all tests in `former` crate

### Goal
*   To identify and fix all failing tests within the `former` crate to establish a stable baseline before proceeding with new feature development.

### Ubiquitous Language (Vocabulary)
*   N/A for this task.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/former`
*   **Overall Progress:** 0/2 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Initial Test Run and Failure Analysis
    *   ⚫ Increment 2: Finalization

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
| | | |

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

##### Increment 2: Finalization
*   **Goal:** Perform a final verification of the `former` and `former_meta` crates.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a final Crate Conformance Check.
    *   Step 2: Self-critique against all requirements and rules.
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
*   This task is a prerequisite for resuming feature development.

### Changelog
*   [Initial] Task created to fix all tests in `former` and `former_meta`.