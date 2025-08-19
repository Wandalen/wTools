# Task Plan: Fix `test_tools` Test Compilation Failures

### Goal
*   Resolve the widespread compilation failures in the `test_tools` test suite by correcting the conditional compilation logic that is incorrectly hiding the public API from tests.

### Ubiquitous Language (Vocabulary)
*   **Aggregated Test:** A test suite imported from another crate (e.g., `error_tools`) to be run within the context of `test_tools` to ensure re-export consistency.
*   **`doctest` feature:** A cargo feature used to conditionally compile code, intended to manage specifics of documentation generation.
*   **`cfg` gate:** A `#[cfg(...)]` attribute used for conditional compilation.

### Progress
*   **Primary Editable Crate:** `module/core/test_tools`
*   **Overall Progress:** 0/2 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Remove restrictive `cfg` gates and verify compilation
    *   ⚫ Increment 2: Finalization

### Permissions & Boundaries
*   **Mode:** `code`
*   **Run workspace-wise commands:** `false`
*   **Add transient comments:** `true`
*   **Additional Editable Crates:**
    *   None

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/test_tools/src/lib.rs`
    *   `module/core/test_tools/.cargo/config.toml`
*   **Initial Analysis Summary:** The test suite fails with 147 `E0432` and `E0433` errors due to unresolved imports. The root cause is the `#[cfg(not(feature = "doctest"))]` attribute in `src/lib.rs` hiding the main API from the test runner. The test runner enables the `doctest` feature because of the `rustdocflags` in `.cargo/config.toml`, creating a conflict. The fix is to remove the problematic `cfg` gates to ensure the API is always visible to tests.

### Expected Behavior Rules / Specifications
*   The `test_tools` crate must successfully compile its entire test suite, including the aggregated tests from other modules.
*   The public API of `test_tools` must be visible to its own integration tests.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `tests::inc` | Failing (New) | Fails to compile due to unresolved imports. |

### Crate Conformance Check Procedure
1.  **Compile Check:** Run `timeout 90 cargo test -p test_tools --all-features --no-run`. Analyze output to ensure there are no compilation errors.
2.  **Test Execution:** If compilation succeeds, run `timeout 90 cargo test -p test_tools --all-features`. Analyze output to ensure all tests pass.

### Increments

##### Increment 1: Remove restrictive `cfg` gates and verify compilation
*   **Goal:** Surgically remove all instances of the `#[cfg(not(feature = "doctest"))]` attribute in `src/lib.rs` to make the public API unconditionally visible to the test suite, and then verify that all compilation errors are resolved.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read the file `module/core/test_tools/src/lib.rs` to ensure we have the latest content before modification.
        ```rust
        // Relevant snippets from module/core/test_tools/src/lib.rs that will be affected:
        // ...
        /// Namespace with dependencies.
        #[ allow( unused_imports ) ]
        #[ cfg( feature = "enabled" ) ]
        #[cfg(not(feature = "doctest"))] // <- This line will be removed
        pub mod dependency {
        // ...
        #[ cfg( feature = "enabled" ) ]
        #[cfg(not(feature = "doctest"))] // <- This line will be removed
        pub mod test;
        // ...
        #[ cfg( feature = "enabled" ) ]
        #[cfg(not(feature = "doctest"))] // <- This line will be removed
        #[cfg(all(feature = "standalone_build", not(feature = "normal_build")))]
        mod standalone;
        // ... and so on for all public modules and re-exports.
        ```
    *   Step 2: Execute a single `search_and_replace` operation to remove all occurrences of the `#[cfg(not(feature = "doctest"))]` line from `module/core/test_tools/src/lib.rs`. The search pattern will include the trailing newline to ensure the file is cleaned up correctly.
    *   Step 3: Perform Increment Verification. This is the crucial step to confirm the fix works.
    *   Step 4: If Increment Verification passes, perform the full Crate Conformance Check, which will run the actual tests.
*   **Increment Verification:**
    *   **Action:** Execute the following command: `timeout 90 cargo test -p test_tools --all-features --no-run`.
    *   **Success Criteria:**
        *   The command must exit with code `0`.
        *   The `stderr` output must **not** contain any lines starting with `error[E...`.
        *   The output should end with a success message like `Finished test [unoptimized + debuginfo] target(s) in ...s`.
    *   **Failure Action:** If the command fails or produces new compilation errors, initiate Critical Log Analysis.
*   **Commit Message:** `fix(test_tools): Remove doctest cfg gates to resolve test compilation errors`

##### Increment 2: Finalization
*   **Goal:** Perform a final, comprehensive verification of the changes to ensure the project is in a clean, correct, and passing state before completing the task.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute the full `Crate Conformance Check Procedure` one last time. This ensures that not only does the code compile, but all tests now pass as expected.
    *   Step 2: Execute `git status` via `execute_command` to confirm that the working directory is clean and all changes have been staged for the commit.
*   **Increment Verification:**
    *   **Action 1 (Conformance):** Execute `timeout 90 cargo test -p test_tools --all-features --no-run` followed by `timeout 90 cargo test -p test_tools --all-features`.
    *   **Success Criteria 1:** Both commands must exit with code `0` and produce no warnings or errors.
    *   **Action 2 (Git Status):** Execute `git status`.
    *   **Success Criteria 2:** The output must contain the message `nothing to commit, working tree clean`.
*   **Commit Message:** `chore(test_tools): Finalize compilation fix`

### Task Requirements
*   The fix must resolve all 147 compilation errors.
*   The fix should not break the intended behavior of the `doctest` feature if possible, but making the tests compile is the primary priority. Removing the `cfg` gates is the most direct way to achieve this.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook.
*   All changes must be verified by running the test suite.

### Assumptions
*   The `doctest` feature was intended for documentation generation and its removal for regular test builds is the correct approach.
*   The aggregated tests are a critical part of the crate's quality assurance and must be made to pass.

### Out of Scope
*   Refactoring the module visibility system (`own`, `orphan`, etc.).
*   Addressing any other `// qqq` or `// xxx` comments not directly related to the compilation failure.

### External System Dependencies
*   None.

### Notes & Insights
*   The core issue is a conflict between a documentation-oriented feature (`doctest`) and the integration testing strategy. The solution is to prioritize the correctness of the integration tests by ensuring a consistent API surface.

### Analysis

Here is a breakdown of how the errors are connected and why the plan addresses them all:

    The Core Problem: The #[cfg(not(feature = "doctest"))] attribute is removing almost the entire public API of test_tools when tests are run.

    Category 1: Unresolved Top-Level Imports:

        Errors like: unresolved imports test_tools::tests_impls, test_tools::tests_index, test_tools::a_id

        Explanation: These macros and functions are directly exposed in test_tools/src/test/mod.rs and re-exported to the top level. Because the test module itself is gated by #[cfg(not(feature = "doctest"))], these items cease to exist during compilation, causing the initial import failures in the aggregated tests.

    Category 2: Unresolved mod_interface Modules:

        Errors like: could not find exposed in test_tools

        Explanation: The own, orphan, exposed, and prelude modules are the fundamental structure created by the mod_interface pattern. All of these modules in src/lib.rs are gated by #[cfg(not(feature = "doctest"))]. When the doctest feature is on, these modules are compiled out, making any path like the_module::exposed::* invalid.

    Category 3: Unresolved Re-exported Crates:

        Errors like: could not find impls_index in test_tools or could not find error in the_module

        Explanation: test_tools re-exports other crates like error_tools and impls_index. These pub use statements are also gated by #[cfg(not(feature = "doctest"))]. When the gate is active, the re-exports are removed, and the symbols are not found.

    Category 4: Consequential Failures (Domino Effect):

        Errors like: could not find bmap in the_module or cannot find macro f1 in this scope

        Explanation: These are secondary failures.

            The compiler can't find bmap because it first couldn't find the_module::collection_tools or the_module::exposed where bmap is re-exported.

            The compiler can't find the macro f1 because it's defined inside a tests_impls! block. But the compiler never found tests_impls! in the first place (Category 1), so the macro f1 was never defined.

Conclusion:

All 147 errors are a direct consequence of the API being hidden. The elaborated plan to surgically remove all instances of #[cfg(not(feature = "doctest"))] in src/lib.rs is designed to fix this single point of failure. By making the entire API surface of test_tools visible to the test compilation context, it will resolve all categories of errors, from the initial unresolved imports down to the final consequential failures.

The plan is comprehensive and addresses the entirety of the problem space presented in the error log. I am ready to proceed with the first increment.

### Changelog
*   [Log will be populated as increments are completed]
