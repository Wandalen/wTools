# Change Proposal for macro_tools

### Task ID
*   TASK-20250705-110800-MacroToolsFixes

### Requesting Context
*   **Requesting Crate/Project:** derive_tools
*   **Driving Feature/Task:** Restoration and validation of derive_tools test suite (V4 plan)
*   **Link to Requester's Plan:** ../derive_tools/task_plan.md
*   **Date Proposed:** 2025-07-05

### Overall Goal of Proposed Change
*   To resolve compilation errors and ambiguous name conflicts within the `macro_tools` crate, specifically related to module imports and `derive` attribute usage, and to properly expose necessary types for external consumption.

### Problem Statement / Justification
*   During the restoration and validation of the `derive_tools` test suite, `macro_tools` (a dependency) failed to compile due to several issues:
    *   `E0432: unresolved import prelude` in `src/lib.rs` because `pub use prelude::*;` was attempting to import `prelude` from the current crate's root, not `std::prelude`.
    *   `E0659: derive is ambiguous` errors across multiple files (e.g., `src/attr.rs`, `src/attr_prop/singletone.rs`, `src/generic_params.rs`). This occurs because `use crate::*;` glob imports conflict with the `derive` attribute macro from the standard prelude.
    *   `E0412: cannot find type GenericsWithWhere` in `src/generic_params.rs` tests, indicating that `GenericsWithWhere` was not properly exposed for use in tests or by dependent crates.
    *   A stray doc comment in `src/generic_params.rs` caused a "expected item after doc comment" error.
    *   **NEW:** `mismatched closing delimiter: `]` in `src/lib.rs` at line 24, indicating a syntax error in a `#[cfg]` attribute.
*   These issues prevent `derive_tools` from compiling and testing successfully, as `macro_tools` is a core dependency. Temporary workarounds were applied in `derive_tools`'s context (e.g., `#[allow(ambiguous_glob_reexports)]`), but these are not sustainable or proper fixes for an external crate.

### Proposed Solution / Specific Changes
*   **API Changes:**
    *   **`src/lib.rs`:** Change `pub use prelude::*;` to `pub use crate::prelude::*;` to correctly reference the crate's own prelude module.
    *   **`src/generic_params.rs`:** Ensure `GenericsWithWhere` is publicly exposed (e.g., `pub use own::GenericsWithWhere;` in `src/generic_params/mod.rs` or similar mechanism if `mod_interface!` is used).
*   **Behavioral Changes:**
    *   The `derive` ambiguity issue (E0659) should be addressed by refactoring the `use crate::*;` glob imports in affected files (e.g., `src/attr.rs`, `src/attr_prop/singletone.rs`, etc.) to be more specific, or by explicitly importing `derive` where needed (e.g., `use proc_macro::TokenStream; use syn::DeriveInput;` and then `#[proc_macro_derive(...)]` or `#[derive(...)]`). The current `#[allow(ambiguous_glob_reexports)]` is a temporary workaround and should be removed.
*   **Internal Changes:**
    *   **`src/generic_params.rs`:** Remove the stray doc comment that caused compilation errors.
    *   **`src/lib.rs`:** Correct the mismatched closing delimiter in the `#[cfg]` attribute at line 24.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `macro_tools` crate should compile without errors or warnings.
*   `derive_tools` should be able to compile and run its tests successfully without needing `#[allow(ambiguous_glob_reexports)]` or other workarounds related to `macro_tools`.
*   `GenericsWithWhere` should be accessible from `derive_tools_meta` for its internal logic and tests.

### Acceptance Criteria (for this proposed change)
*   `macro_tools` compiles successfully with `cargo build -p macro_tools --all-targets` and `cargo clippy -p macro_tools -- -D warnings`.
*   `derive_tools` compiles and passes all its tests (`cargo test -p derive_tools --all-targets`) without any temporary `#[allow]` attributes related to `macro_tools` issues.

### Potential Impact & Considerations
*   **Breaking Changes:** The proposed changes are primarily fixes and clarifications; they should not introduce breaking changes to `macro_tools`'s public API.
*   **Dependencies:** No new dependencies are introduced.
*   **Performance:** No significant performance implications are expected.
*   **Testing:** Existing tests in `macro_tools` should continue to pass. New tests might be beneficial to cover the `GenericsWithWhere` exposure.

### Notes & Open Questions
*   The `derive` ambiguity is a common issue with glob imports and attribute macros. A systematic review of `use crate::*;` in `macro_tools` might be beneficial.