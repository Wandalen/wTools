# Task Plan: Fix Iterator implementation for SplitOptions with Vec<&str> delimiter

### Goal
*   To fix the `Iterator` trait implementation for `strs_tools::split::SplitOptions` when the delimiter type `D` is `Vec<&str>`, ensuring it can be consumed by iterator methods like `collect()` without compilation errors. This will unblock the `unilang_instruction_parser` crate.

### Ubiquitous Language (Vocabulary)
*   **SplitOptions:** The struct in `strs_tools` that configures and represents a string splitting operation.
*   **MRE:** Minimal Reproducible Example, a specific test case that demonstrates the bug.
*   **Downstream Crate:** A crate that depends on another, in this case `unilang_instruction_parser` is downstream of `strs_tools`.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/strs_tools`
*   **Overall Progress:** 2/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Replicate the Failure Locally
    *   ✅ Increment 2: Investigate and Fix the Iterator Implementation
    *   ⏳ Increment 3: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   None

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/strs_tools/task.md` (Original proposal)
    *   `module/move/unilang_instruction_parser/tests/strs_tools_mre.rs` (MRE test file to be copied)
    *   `module/core/strs_tools/src/string/split.rs` (Likely location of the bug)
    *   `module/core/strs_tools/Cargo.toml`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications
*   Rule 1: `strs_tools::split::SplitOptions< '_, Vec<&str> >` must implement the `Iterator` trait.
*   Rule 2: Code like `strs_tools::split().delimeter(vec![" "]).form().iter().collect::<Vec<_>>()` must compile and run without errors.

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 180 cargo test -p strs_tools --all-targets`. Analyze output for failures. If fails, initiate Critical Log Analysis.
*   Step 2: If tests pass, execute `timeout 180 cargo clippy -p strs_tools -- -D warnings`. Analyze output for failures. If fails, initiate Linter Fix & Regression Check Procedure.
*   Step 3: Execute `cargo clean -p strs_tools` followed by `timeout 180 cargo build -p strs_tools`. Analyze for unexpected debug output. If found, initiate Critical Log Analysis.

### Increments
##### Increment 1: Replicate the Failure Locally
*   **Goal:** To copy the MRE from `unilang_instruction_parser` into a new test file within `strs_tools` to reliably reproduce the compilation failure. This test will be preserved to prevent regressions.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `list_files` to inspect the `module/core/strs_tools/tests/` directory to find the main test entry point file (e.g., `tests.rs` or `all.rs`).
    *   Step 2: Use `read_file` to get the content of the MRE from `module/move/unilang_instruction_parser/tests/strs_tools_mre.rs`.
    *   Step 3: Use `write_to_file` to create a new test file at `module/core/strs_tools/tests/inc/iterator_vec_delimiter_test.rs` with the content from the MRE. The content will be adjusted to work within the `strs_tools` crate's test environment (e.g., using `use strs_tools::...`).
    *   Step 4: Use `insert_content` to add `mod iterator_vec_delimiter_test;` to the main test entry point file found in Step 1.
    *   Step 5: Perform Increment Verification.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 180 cargo build -p strs_tools` via `execute_command`.
    *   Step 2: Analyze the `stderr` from the command output. It **must** contain a compilation error related to unsatisfied trait bounds for `Iterator`, similar to `error[E0599]: the method \`iter\` exists for struct \`split::private::SplitOptions\`, but its trait bounds were not satisfied`.
    *   Step 3: If the expected error is not present, the verification fails.
*   **Commit Message:** "test(strs_tools): Add failing test for Iterator on SplitOptions<Vec<&str>>"

##### Increment 2: Investigate and Fix the Iterator Implementation
*   **Goal:** To analyze the `Iterator` implementation for `SplitOptions` and correct the trait bounds or implementation logic to properly handle cases where the delimiter `D` is of type `Vec<&str>`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read the content of `module/core/strs_tools/src/string/split.rs` to understand the current implementation of `SplitOptions` and its `Iterator` trait.
    *   Step 2: Analyze the `E0599` error message and the code in `split.rs` to identify why `SplitOptions<'_, Dlist<&str>>` does not satisfy the `Iterator` trait. This likely involves examining the `Dlist` type and how it interacts with the `Delimiter` trait.
    *   Step 3: Based on the analysis, modify `module/core/strs_tools/src/string/split.rs` to adjust the `Iterator` implementation or related trait bounds to correctly handle `Vec<&str>` as a delimiter. This might involve adding a new `impl` block or modifying an existing one.
    *   Step 4: Perform Increment Verification.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 180 cargo test -p strs_tools --test strs_tools_tests -- --nocapture` via `execute_command`.
    *   Step 2: Analyze the `stdout` and `stderr` from the command output. The test `test_split_with_vec_delimiter_iterator` must pass, and there should be no compilation errors.
    *   Step 3: Perform Crate Conformance Check.
*   **Commit Message:** "fix(strs_tools): Correct Iterator impl for SplitOptions with Vec<&str> delimiter"

##### Increment 3: Finalization
*   **Goal:** To perform a final, holistic review and verification of the task's output, ensuring all requirements have been met and the `strs_tools` crate is in a clean, compliant state.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique against the `Goal`, `Task Requirements`, and `Project Requirements` in the plan file.
    *   Step 2: Execute the full `Crate Conformance Check Procedure` for `strs_tools`.
    *   Step 3: Execute `cargo clean -p strs_tools` followed by `timeout 180 cargo build -p strs_tools`. Analyze the output for any unexpected debug prints from procedural macros.
    *   Step 4: Execute `git status` to ensure the working directory is clean.
    *   Step 5: If all checks pass, the task is complete.
*   **Increment Verification:**
    *   Execute the full `Finalization Increment Verification` procedure as defined in the design rules.
*   **Commit Message:** "chore(strs_tools): Finalize iterator fix task"

### Task Requirements
*   The fix must not introduce any breaking changes to the public API of `strs_tools`.
*   The fix must be covered by a new regression test in the `strs_tools` crate.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The MRE provided in `unilang_instruction_parser` accurately reflects the bug.
*   The bug is located within the `strs_tools` crate's `Iterator` implementation or related trait bounds.

### Out of Scope
*   Making any changes to `unilang_instruction_parser`.
*   Refactoring parts of `strs_tools` not directly related to the `Iterator` implementation for `SplitOptions`.

### External System Dependencies
*   None

### Notes & Insights
*   The core of the issue seems to be how the generic `OpType<T>` used within `SplitOptions` handles being converted from a `Vec<&str>`. This is a good place to start the investigation.

### Changelog
*   [Initial] Task plan created from change proposal `module/core/strs_tools/task.md`.
*   [Feedback] Updated plan to copy MRE instead of editing downstream crate. Disabled workspace commands.
*   [Increment 1] Added a failing test case to `strs_tools` to reproduce the iterator compilation error.
*   [Increment 2] Corrected the `IntoIterator` implementation for `SplitOptions` and fixed the test case.