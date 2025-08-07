# Change Proposal for clone_dyn_meta

### Task ID
*   TASK-20250701-211117-FixGenericsWithWhere

### Requesting Context
*   **Requesting Crate/Project:** `derive_tools`
*   **Driving Feature/Task:** Fixing `Deref` derive tests (Increment 3)
*   **Link to Requester's Plan:** `../derive_tools/task_plan.md`
*   **Date Proposed:** 2025-07-01

### Overall Goal of Proposed Change
*   Update `clone_dyn_meta` to correctly import `GenericsWithWhere` from `macro_tools` to resolve compilation errors.

### Problem Statement / Justification
*   The `clone_dyn_meta` crate fails to compile because it attempts to import `GenericsWithWhere` directly from the `macro_tools` crate root (`use macro_tools::GenericsWithWhere;`). However, `GenericsWithWhere` is located within the `generic_params` module of `macro_tools` (`macro_tools::generic_params::GenericsWithWhere`). This incorrect import path leads to compilation errors.

### Proposed Solution / Specific Changes
*   **File:** `module/core/clone_dyn_meta/src/clone_dyn.rs`
*   **Change:** Modify the import statement for `GenericsWithWhere`.
    ```diff
    - use macro_tools::GenericsWithWhere;
    + use macro_tools::generic_params::GenericsWithWhere;
    ```

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `clone_dyn_meta` crate should compile successfully without errors related to `GenericsWithWhere`.

### Acceptance Criteria (for this proposed change)
*   The `clone_dyn_meta` crate compiles successfully.
*   `cargo test -p clone_dyn_meta` passes.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated as this is a correction of an internal import path.
*   **Dependencies:** No new dependencies are introduced.
*   **Performance:** No performance impact.
*   **Security:** No security implications.
*   **Testing:** Existing tests for `clone_dyn_meta` should continue to pass, and the crate should compile.

### Alternatives Considered (Optional)
*   None. The issue is a direct result of an incorrect import path.

### Notes & Open Questions
*   This change is necessary to unblock the `derive_tools` task, which depends on a compilable `clone_dyn_meta`.