# Task Plan: Remove `bitflags` dependency from `strs_tools`

### Goal
*   To eliminate the `bitflags` crate dependency from `module/core/strs_tools` by replacing its functionality with a custom implementation, ensuring all existing features and tests continue to pass without regression.

### Ubiquitous Language (Vocabulary)
*   **Bitflags:** The `bitflags` crate, used for creating a type-safe way to work with bitmasks.
*   **Custom Flag Type:** A new enum or struct that will replace the functionality provided by `bitflags`.
*   **StrsTools:** The `module/core/strs_tools` crate, the primary target for this task.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/strs_tools`
*   **Overall Progress:** 1/5 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Analyze `bitflags` usage and prepare for replacement.
    *   ⏳ Increment 2: Implement custom flag type.
    *   ⚫ Increment 3: Replace `bitflags` usage in `src/string/split.rs`.
    *   ⚫ Increment 4: Remove `bitflags` dependency from `Cargo.toml`.
    *   ⚫ Increment 5: Finalization.

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   N/A

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/strs_tools/Cargo.toml`
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   The `strs_tools` crate must compile successfully after the changes.
*   All existing tests for `strs_tools` must pass after the changes.
*   The functionality of string splitting, quoting, and unescaping must remain identical to its current behavior.
*   The `bitflags` dependency must be completely removed from `strs_tools/Cargo.toml`.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `split_behavior_tests::test_bitor_operator` | Fixed (Attempt 1) | Corrected expected value. |

### Crate Conformance Check Procedure
*   1. Run Tests: For the `Primary Editable Crate` (`strs_tools`), execute `timeout 90 cargo test -p strs_tools --all-targets`.
*   2. Analyze Test Output: If any test command fails, initiate the `Critical Log Analysis` procedure and resolve all test failures before proceeding.
*   3. Run Linter: Only if all tests in the previous step pass, for the `Primary Editable Crate`, execute `timeout 90 cargo clippy -p strs_tools -- -D warnings`.
*   4. Analyze Linter Output: If any linter command fails, initiate the `Linter Fix & Regression Check Procedure`.
*   5. Perform Output Cleanliness Check: Execute `cargo clean -p strs_tools` followed by `timeout 90 cargo build -p strs_tools`. Critically analyze the build output for any unexpected debug prints from procedural macros. If any are found, the check fails; initiate the `Critical Log Analysis` procedure.

### Increments
##### Increment 1: Analyze `bitflags` usage and prepare for replacement.
*   **Goal:** Understand the current usage of `bitflags` within `strs_tools` and identify the specific flags and their contexts to inform the custom implementation.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `read_file` to load `module/core/strs_tools/Cargo.toml`, `module/core/strs_tools/src/string/split.rs`, and `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`.
    *   Step 2: Analyze the content of `Cargo.toml` to confirm the `bitflags` dependency.
    *   Step 3: Analyze `src/string/split.rs` to identify the `bitflags!` macro usage for `SplitBehavior` and how its flags (`AllowEmpty`, `AllowEmptyWithQuotes`, `KeepQuotes`, `KeepOuterQuotes`) are used in the `split` function.
    *   Step 4: Analyze `quoting_and_unescaping_tests.rs` to see how `SplitBehavior` flags are combined and used in test cases.
    *   Step 5: Based on the analysis, document in `### Notes & Insights` that a struct with consts and bitwise operations (`|`, `&`) will be the most direct replacement for the `bitflags!` macro. The struct will need to implement `BitOr`, `BitAnd`, `Not`, `From<i32>`, and a `contains` method.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Confirm that the analysis of `bitflags` usage is complete and the chosen replacement strategy is documented in the `### Notes & Insights` section of the plan.
    *   Step 2: Run `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command` to ensure the current state is clean before making changes. Analyze the output.
*   **Commit Message:** `chore(strs_tools): Analyze bitflags usage and plan replacement`

##### Increment 2: Implement custom flag type.
*   **Goal:** Create a new module and define a custom flag type that replicates the necessary functionality of `bitflags::bitflags!` for `SplitBehavior`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Add `mod split_behavior;` to `module/core/strs_tools/src/string/mod.rs`.
    *   Step 2: Create the file `module/core/strs_tools/src/string/split/split_behavior.rs`.
    *   Step 3: Implement the custom flag type (e.g., an enum with `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` and `From` implementations for conversions, or a struct with bitwise operations) in `module/core/strs_tools/src/string/split/split_behavior.rs` to mimic the behavior of `SplitBehavior` from `bitflags`.
    *   Step 4: Add basic unit tests for the new custom flag type in `module/core/strs_tools/tests/inc/split_test/split_behavior_tests.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Confirm `split_behavior.rs` exists and contains the custom flag type.
    *   Confirm `split_behavior_tests.rs` exists and contains tests for the new type.
    *   Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests` via `execute_command` and analyze output to ensure new tests pass.
*   **Commit Message:** `feat(strs_tools): Implement custom flag type for SplitBehavior`

##### Increment 3: Replace `bitflags` usage in `src/string/split.rs`.
*   **Goal:** Modify `src/string/split.rs` to use the newly created custom flag type instead of the `bitflags` version of `SplitBehavior`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Modify `module/core/strs_tools/src/string/split.rs` to import and use the new custom `SplitBehavior` type from `split_behavior.rs`.
    *   Step 2: Replace all instances of `bitflags!` macro usage and `SplitBehavior` flag access (e.g., `SplitBehavior::AllowEmpty`) with the corresponding new custom flag type and its API.
    *   Step 3: Adjust any logic in `split.rs` that relied on `bitflags` specific methods (e.g., `contains`, `insert`, `remove`) to use the equivalent functionality provided by the custom flag type.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p strs_tools` via `execute_command` and analyze output to ensure the crate compiles without errors.
    *   Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command` and analyze output to ensure all existing tests pass.
*   **Commit Message:** `refactor(strs_tools): Replace bitflags usage in split.rs`

##### Increment 4: Remove `bitflags` dependency from `Cargo.toml`.
*   **Goal:** Remove the `bitflags` entry from `strs_tools/Cargo.toml` and verify that the crate still compiles and all tests pass.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/strs_tools/Cargo.toml`.
    *   Step 2: Remove the `bitflags` entry from the `[dependencies]` section of `module/core/strs_tools/Cargo.toml`.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p strs_tools` via `execute_command` and analyze output to ensure the crate compiles without errors.
    *   Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command` and analyze output to ensure all existing tests pass.
    *   Confirm that `bitflags` is no longer listed in `module/core/strs_tools/Cargo.toml`.
*   **Commit Message:** `chore(strs_tools): Remove bitflags dependency`

##### Increment 5: Finalization.
*   **Goal:** Perform a final, holistic review and verification of the entire task's output, ensuring all requirements are met and the codebase is clean.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Self-Critique: Review all changes made during the task against the `Goal`, `Task Requirements`, and `Project Requirements` in the plan file.
    *   Step 2: Execute Test Quality and Coverage Evaluation.
    *   Step 3: Full Conformance Check: Execute the full `Crate Conformance Check Procedure` on `strs_tools`.
    *   Step 4: Final Output Cleanliness Check: Execute `cargo clean -p strs_tools` followed by `timeout 90 cargo build -p strs_tools`. Critically analyze the build output for any unexpected debug prints.
    *   Step 5: Final Status Check: Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    *   Confirm all checks in the steps pass.
*   **Commit Message:** `chore(strs_tools): Finalize bitflags removal task`

### Task Requirements
*   The `bitflags` crate must be completely removed from the `strs_tools` crate.
*   All existing tests must pass after the removal.
*   The functionality of string splitting must remain unchanged.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.
*   All new APIs must be async. (N/A for this task as it's a refactor)

### Assumptions
*   The `bitflags` usage in `strs_tools` is limited to `src/string/split.rs` and its associated tests.
*   The functionality provided by `bitflags` can be adequately replicated with a custom Rust enum/struct and bitwise operations.

### Out of Scope
*   Refactoring or optimizing the string splitting logic beyond replacing the `bitflags` dependency.
*   Adding new features to the `strs_tools` crate.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The `bitflags!` macro is used to define `SplitFlags` with a `u8` representation.
*   The flags are: `PRESERVING_EMPTY`, `PRESERVING_DELIMITERS`, `PRESERVING_QUOTING`, `STRIPPING`, `QUOTING`.
*   The replacement will be a newtype struct `SplitFlags(u8)`.
*   It will have `const` associated items for each flag.
*   It will implement `BitOr`, `BitAnd`, `Not` for combining flags.
*   It will have methods `contains`, `insert`, and `remove` to mimic the `bitflags` API used in the code.

### Changelog
*   [Increment 1 | 2025-07-13 12:07 UTC] Analyzed `bitflags` usage and documented replacement strategy.
*   [Increment 2 | 2025-07-13 12:18 UTC] Implemented custom flag type for `SplitBehavior` and added tests.
