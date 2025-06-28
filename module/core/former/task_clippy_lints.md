# Change Proposal for former

### Task ID
*   TASK-20250628-081929-FormerClippyLints

### Requesting Context
*   **Requesting Crate/Project** : `module/core/former_meta` (from `module/core/former/plan.md`)
*   **Driving Feature/Task** : Completion of `former_meta` refactoring and unblocking final verification.
*   **Link to Requester's Plan** : `module/core/former/plan.md`
*   **Date Proposed** : 2025-06-28

### Overall Goal of Proposed Change
*   Run `cargo clippy --package former --all-targets -- -D warnings` and address any persistent Clippy lints in the `former` crate.

### Problem Statement / Justification
*   During the `former_meta` refactoring task, a significant number of Clippy lints in the `former` crate's test files could not be resolved within the current task's scope, despite multiple attempts. This is due to unexpected behavior of `#[allow]` attributes or the Clippy setup. To allow the current task to proceed and be verified, these lints need to be addressed in a separate, dedicated future task.

### Proposed Solution / Specific Changes
*   **Action** : Execute `cargo clippy --package former --all-targets -- -D warnings` in the `module/core/former` directory.
*   **Action** : Analyze the output and systematically address all reported Clippy lints. This may involve:
    *   Adjusting code to conform to lint suggestions.
    *   Adding appropriate `#[ allow( ... ) ]` attributes if a lint is deemed acceptable for a specific case, with clear justification.
    *   Investigating and resolving any issues with `#[ allow ]` attributes not functioning as expected.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `former` crate should pass `cargo clippy --package former --all-targets -- -D warnings` with no warnings or errors.

### Acceptance Criteria (for this proposed change)
*   `cargo clippy --package former --all-targets -- -D warnings` exits with code 0 and no warnings in its output.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes to public API are expected, but code style or minor refactorings might occur.
*   **Dependencies** : None.
*   **Performance** : No impact.
*   **Security** : No impact.
*   **Testing** : This task is focused on code quality and linting. Existing tests should continue to pass.

### Alternatives Considered (Optional)
*   Attempting to resolve all Clippy lints within the current `former_meta` task. This was attempted and failed, blocking progress. Delegating to a separate task allows the current task to complete.

### Notes & Open Questions
*   The specific lints and affected files will be identified by the executor of this `task.md` based on the `cargo clippy` output.