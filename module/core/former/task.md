# Change Proposal for former

### Task ID
*   TASK-20250625-FORMER-CLIPPY-LINT-FIX

### Requesting Context
*   **Requesting Crate/Project:** `module/core/former` (Self-request)
*   **Driving Feature/Task:** Resolving persistent Clippy lints in `former`'s test suite.
*   **Link to Requester's Plan:** `module/core/former/plan.md`
*   **Date Proposed:** 2025-06-25

### Overall Goal of Proposed Change
*   To systematically address and resolve all Clippy lints in the `module/core/former` crate's test suite that were not resolvable during the `former` enum unit variant refactoring task.

### Problem Statement / Justification
*   During the recent refactoring of `former`'s enum unit variant handling, a significant number of Clippy lints were encountered in the `former` crate's test files. Despite multiple attempts, these lints could not be resolved directly within the scope of that task, partly due to unexpected behavior of `#[allow]` attributes or the Clippy setup.
*   These unresolved lints hinder code quality, maintainability, and adherence to project standards. A dedicated effort is required to clean up the test suite.

### Proposed Solution / Specific Changes
*   **Systematic Lint Resolution:**
    *   Review all remaining Clippy lints reported by `cargo clippy --package former --all-targets -- -D warnings`.
    *   For each lint, determine the most appropriate fix:
        *   Apply suggested code changes (e.g., `unwrap_or_default`, `String::new`, direct format args, removing useless conversions, removing unneeded returns, fixing needless borrows).
        *   For `struct_field_names` (postfix `_1`), evaluate if renaming fields is feasible without breaking external contracts or if `#[allow(clippy::struct_field_names)]` is acceptable with a clear justification.
        *   For `unused_self` and `used_underscore_items`, re-evaluate if the suggested refactoring to associated functions is appropriate, or if `#[allow(...)]` attributes are justified with clear comments explaining why the lint is being suppressed (e.g., due to macro-generated API patterns).
        *   For `#[should_panic]` without a reason, add an `expect = "reason"` argument.
        *   For `empty_line_after_doc_comments`, remove the empty lines.
        *   For `needless_raw_string_hashes`, remove the extra hashes.
        *   For `std_instead_of_core`, replace `std::` with `core::` where applicable.
    *   Ensure all fixes adhere to the project's Codestyle Rules.
*   **Verification:**
    *   After each batch of fixes, run `cargo clippy --package former --all-targets -- -D warnings` to confirm resolution.
    *   Ensure `cargo test --package former` continues to pass.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   `cargo clippy --package former --all-targets -- -D warnings` should pass with zero warnings or errors.
*   The `former` crate's test suite should continue to pass all tests.

### Acceptance Criteria (for this proposed change)
*   `cargo clippy --package former --all-targets -- -D warnings` exits with code 0 and no output (other than the `macro_tools` warning, which is out of scope for this task).
*   `cargo test --package former` passes.

### Potential Impact & Considerations
*   **Breaking Changes:** Unlikely, as this focuses on lint resolution within tests.
*   **Dependencies:** No new dependencies.
*   **Performance:** No significant performance impact.
*   **Testing:** Requires careful application of fixes and re-verification of tests.

### Notes & Open Questions
*   The `default-features` warning for `convert_case` in `macro_tools/Cargo.toml` is a separate issue to be addressed in the `macro_tools` task.
*   Some lints (e.g., `very_complex_type`) were intentionally ignored in the previous task; this task should decide whether to address them or continue to suppress them with justification.