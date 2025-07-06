# Change Proposal for `macro_tools`

### Task ID
*   `TASK-20250706-155700-FixMacroToolsCompile`

### Requesting Context
*   **Requesting Crate/Project:** `variadic_from_meta`
*   **Driving Feature/Task:** Refactoring `variadic_from_meta` to use `macro_tools` utilities, specifically `syn_err!` and `return_syn_err!`.
*   **Link to Requester's Plan:** `module/core/variadic_from/task_plan.md`
*   **Date Proposed:** 2025-07-06

### Overall Goal of Proposed Change
*   To enable the `macro_tools` crate to compile successfully when its internal modules (like `item_struct` and `typ`) attempt to use the `syn_err!` macro, which appears to be gated behind a feature.

### Problem Statement / Justification
*   The `variadic_from_meta` crate depends on `macro_tools` and attempts to use its `struct_like`, `generic_params`, and `typ` modules. During compilation, `macro_tools` itself fails with "cannot find macro `syn_err` in this scope" errors originating from its own source files (`src/item_struct.rs`, `src/typ.rs`). This indicates that a necessary feature for `macro_tools`'s internal compilation, likely related to diagnostics or error handling, is not enabled by default or through the current dependency configuration. This prevents `variadic_from_meta` (and any other crate depending on these `macro_tools` features) from compiling.

### Proposed Solution / Specific Changes
*   **Enable `diagnostics` feature:** Add the `diagnostics` feature to the `macro_tools` crate's `Cargo.toml`. This feature is commonly used for error reporting and diagnostic utilities in procedural macro helper crates.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `macro_tools` crate should compile successfully, allowing `variadic_from_meta` to compile and proceed with its refactoring.
*   The `syn_err!` and `return_syn_err!` macros should be available for use within `macro_tools`'s internal modules and potentially for re-export.

### Acceptance Criteria (for this proposed change)
*   `cargo build -p macro_tools` (with the `diagnostics` feature enabled) must exit with code 0 and no compilation errors.
*   `cargo build -p variadic_from_meta` (which depends on the patched `macro_tools`) must compile successfully.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated for `macro_tools`'s public API, as this change primarily affects its internal compilation.
*   **Dependencies:** No new external dependencies are expected.
*   **Performance:** No significant performance impact is anticipated.
*   **Security:** No security implications are anticipated.
*   **Testing:** The `macro_tools` crate's existing test suite should continue to pass. New tests specifically for the `diagnostics` feature might be beneficial but are out of scope for this proposal.

### Alternatives Considered (Optional)
*   None, as the error message directly points to a missing macro within `macro_tools`'s own compilation, suggesting a feature-gating issue.

### Notes & Open Questions
*   Confirm if `diagnostics` is indeed the correct feature name for enabling `syn_err!` and `return_syn_err!`. If not, further investigation into `macro_tools`'s internal structure would be required by its maintainers.