# Task Plan: Fix tests and improve quality for diagnostics_tools

### Goal
*   Fix the failing doctest in `Readme.md`.
*   Refactor the `trybuild` test setup to be robust and idiomatic.
*   Increase test coverage by enabling existing compile-time tests and adding new `trybuild` tests to verify runtime assertion failure messages.
*   Ensure the crate adheres to standard Rust formatting and clippy lints.

### Ubiquitous Language (Vocabulary)
*   `cta`: Compile-Time Assertion
*   `rta`: Run-Time Assertion
*   `trybuild`: A test harness for testing compiler failures.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/diagnostics_tools`
*   **Overall Progress:** 2/6 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Fix failing doctest in `Readme.md`
    *   ✅ Increment 1.1: Diagnose and fix the Failing (Stuck) test: `module/core/diagnostics_tools/src/lib.rs - (line 18)`
    *   ✅ Increment 2: Refactor `trybuild` setup and enable CTA tests
    *   ⏳ Increment 3: Add `trybuild` tests for RTA failure messages
    *   ⚫ Increment 4: Apply code formatting
    *   ⚫ Increment 5: Fix clippy warnings
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   N/A

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/diagnostics_tools/Cargo.toml`
    *   `module/core/diagnostics_tools/Readme.md`
    *   `module/core/diagnostics_tools/tests/inc/cta_test.rs`
    *   `module/core/diagnostics_tools/tests/inc/layout_test.rs`
    *   `module/core/diagnostics_tools/tests/inc/rta_test.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   N/A
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   Rule 1: All tests, including doctests, must pass.
*   Rule 2: Code must be formatted with `rustfmt`.
*   Rule 3: Code must be free of `clippy` warnings.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `module/core/diagnostics_tools/src/lib.rs - (line 18)` | Fixed (Monitored) | Doctest marked `should_panic` was not panicking. Fixed by using `std::panic::catch_unwind` due to `should_panic` not working with `include_str!`. |

### Crate Conformance Check Procedure
*   Run `cargo test --workspace --all-features`.
*   Run `cargo clippy --workspace --all-features -- -D warnings`.
*   Run `cargo fmt --workspace --all -- --check`.

### Increments
##### Increment 1: Fix failing doctest in `Readme.md`
*   **Goal:** The doctest in `Readme.md` (which is included in `lib.rs`) is marked `should_panic` but succeeds. Fix the code snippet so it it panics as expected.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Use `read_file` to load `module/core/diagnostics_tools/Readme.md`.
    2.  The doctest for `a_id` is missing the necessary import to bring the macro into scope.
    3.  Use `search_and_replace` on `Readme.md` to add `use diagnostics_tools::a_id;` inside the `fn a_id_panic_test()` function in the example.
*   **Increment Verification:**
    1.  Execute `cargo test --doc --package diagnostics_tools` via `execute_command`.
    2.  Analyze the output to confirm all doctests now pass.
*   **Commit Message:** `fix(docs): Correct doctest in Readme.md to panic as expected`

##### Increment 1.1: Diagnose and fix the Failing (Stuck) test: `module/core/diagnostics_tools/src/lib.rs - (line 18)`
*   **Goal:** Diagnose and fix the `Failing (Stuck)` test: `module/core/diagnostics_tools/src/lib.rs - (line 18)`
*   **Specification Reference:** N/A
*   **Steps:**
    *   **Step A: Apply Problem Decomposition.** The plan must include an explicit step to analyze the failing test and determine if it can be broken down into smaller, more focused tests, or if its setup can be simplified. This is a mandatory first step in analysis.
    *   **Step B: Isolate the test case.**
        1.  Temporarily modify the `Readme.md` doctest to use a direct `panic!` call instead of `a_id!`. This will verify if the `should_panic` attribute itself is working.
        2.  Execute `cargo test --doc --package diagnostics_tools` via `execute_command`.
        3.  Analyze the output. If it panics, the `should_panic` attribute is working, and the issue is with `a_id!`. If it still doesn't panic, the issue is with the doctest environment or `should_panic` itself.
    *   **Step C: Add targeted debug logging.**
        1.  If `panic!` works, investigate `a_id!`. Add debug prints inside the `a_id!` macro (in `src/diag/rta.rs`) to see what `pretty_assertions::assert_eq!` is actually doing.
        2.  Execute `cargo test --doc --package diagnostics_tools` via `execute_command`.
        3.  Analyze the output for debug logs.
    *   **Step D: Review related code changes since the test last passed.** (N/A, this is a new task, test was failing from start)
    *   **Step E: Formulate and test a hypothesis.**
        1.  Based on debug logs, formulate a hypothesis about why `a_id!` is not panicking.
        2.  Propose a fix for `a_id!` or the doctest.
    *   Upon successful fix, document the root cause and solution in the `### Notes & Insights` section.
*   **Increment Verification:**
    *   Execute `cargo test --doc --package diagnostics_tools` via `execute_command`.
    *   Analyze the output to confirm the specific test ID now passes.
*   **Commit Message:** `fix(test): Resolve stuck test module/core/diagnostics_tools/src/lib.rs - (line 18)`

##### Increment 2: Refactor `trybuild` setup and enable CTA tests
*   **Goal:** Refactor the fragile, non-standard `trybuild` setup to be idiomatic and robust. Consolidate all compile-time assertion tests into this new setup.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Create a new test file: `module/core/diagnostics_tools/tests/trybuild.rs`.
    2.  Use `write_to_file` to add the standard `trybuild` test runner boilerplate to `tests/trybuild.rs`.
    3.  Use `insert_content` on `module/core/diagnostics_tools/Cargo.toml` to add `trybuild` to `[dev-dependencies]` and define the new test target: `[[test]]\nname = "trybuild"\nharness = false`.
    4.  In `tests/trybuild.rs`, add the test cases for all the existing `cta_*.rs` snippets from `tests/inc/snipet/`. The paths should be relative, e.g., `"inc/snipet/cta_type_same_size_fail.rs"`.
    5.  Use `search_and_replace` on `module/core/diagnostics_tools/tests/inc/cta_test.rs` and `module/core/diagnostics_tools/tests/inc/layout_test.rs` to remove the old, complex `cta_trybuild_tests` functions and their `tests_index!` entries.
*   **Increment Verification:**
    1.  Execute `cargo test --test trybuild` via `execute_command`.
    2.  Analyze the output to confirm all `trybuild` tests pass.
*   **Commit Message:** `refactor(test): Consolidate and simplify trybuild test setup`

##### Increment 3: Add `trybuild` tests for RTA failure messages
*   **Goal:** Use the new `trybuild` setup to verify the console output of `a_id!` and `a_not_id!` failures, replacing the old, brittle `*_run` tests.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Use `insert_content` on `module/core/diagnostics_tools/tests/trybuild.rs` to add test cases for `tests/inc/snipet/rta_id_fail.rs` and `tests/inc/snipet/rta_not_id_fail.rs`.
    2.  Use `search_and_replace` on `module/core/diagnostics_tools/tests/inc/rta_test.rs` to remove the `a_id_run` and `a_not_id_run` test functions and their corresponding entries from the `tests_index!` macro.
*   **Increment Verification:**
    1.  Execute `cargo test --test trybuild` via `execute_command`.
    2.  Analyze the output to confirm the new RTA failure tests pass.
*   **Commit Message:** `test(rta): Add trybuild tests for assertion failure messages`

##### Increment 4: Apply code formatting
*   **Goal:** Ensure consistent code formatting across the crate.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Execute `cargo fmt --package diagnostics_tools --all` via `execute_command`.
*   **Increment Verification:**
    1.  Execute `cargo fmt --package diagnostics_tools --all -- --check` via `execute_command` and confirm it passes.
    2.  Execute `cargo test --package diagnostics_tools --all-features` via `execute_command` to ensure no regressions.
*   **Commit Message:** `style: Apply rustfmt`

##### Increment 5: Fix clippy warnings
*   **Goal:** Eliminate all clippy warnings from the crate.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Run `cargo clippy --package diagnostics_tools --all-features -- -D warnings` to identify warnings.
    2.  The `any(...)` condition in `cta_test.rs` and `layout_test.rs` has a duplicate feature flag. Use `search_and_replace` to fix this in both files.
*   **Increment Verification:**
    1.  Execute `cargo clippy --package diagnostics_tools --all-features -- -D warnings` via `execute_command` and confirm no warnings are reported.
    2.  Execute `cargo test --package diagnostics_tools --all-features` via `execute_command` to ensure no regressions.
*   **Commit Message:** `style: Fix clippy lints`

##### Increment 6: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Critically review all changes against the `Goal` and `Expected Behavior Rules`.
    2.  Perform a final Crate Conformance Check.
*   **Increment Verification:**
    1.  Execute `cargo test --workspace --all-features` via `execute_command`.
    2.  Execute `cargo clippy --workspace --all-features -- -D warnings` via `execute_command`.
    3.  Execute `git status` via `execute_command` to ensure the working directory is clean.
*   **Commit Message:** `chore(diagnostics_tools): Complete test fixes and quality improvements`

### Task Requirements
*   N/A

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The `test_tools` dependency provides a `trybuild`-like testing framework.

### Out of Scope
*   Adding new features to the crate.
*   Refactoring core logic beyond what is necessary for fixes.

### External System Dependencies
*   N/A

### Notes & Insights
*   The failing doctest is due to a missing import, which prevents the macro from being resolved and thus from panicking.
*   Consolidating `trybuild` tests into a single, standard test target (`tests/trybuild.rs`) is more robust and maintainable than the previous scattered and brittle implementation.
*   **Root cause of doctest failure:** The `should_panic` attribute on doctests included via `include_str!` in `lib.rs` does not seem to function correctly. The fix involved explicitly catching the panic with `std::panic::catch_unwind` and asserting `is_err()`.

### Changelog
*
