# Change Proposal for macro_tools

### Task ID
*   TASK-20250625-FORMER-GENERIC-ENUM-FIX

### Requesting Context
*   **Requesting Crate/Project:** `module/core/former`
*   **Driving Feature/Task:** Unblocking `#[derive(Former)]` for generic enums and refining `macro_tools` utilities.
*   **Link to Requester's Plan:** `module/core/former/plan.md`
*   **Date Proposed:** 2025-06-25

### Overall Goal of Proposed Change
*   To thoroughly investigate and resolve the "comparison operators cannot be chained" error that occurs when `#[derive(Former)]` is used on generic enums.
*   To review, refine, and potentially extend the `macro_tools` utilities (e.g., `ident::cased_ident_from_ident`, `generic_params::GenericsRef`) that were generalized from `former_meta` in a previous task.

### Problem Statement / Justification
*   The `former` crate's `#[derive(Former)]` macro currently fails to compile when applied to generic enums, producing a misleading "comparison operators cannot be chained" error. This issue is a critical blocker for full generic enum support in `former`.
*   The `macro_tools` crate was recently updated with utilities generalized from `former_meta`. A dedicated review is needed to ensure these utilities are robust, truly general-purpose, well-documented, and have comprehensive tests, as direct modification of `macro_tools` was outside the scope of the `former` refactoring task.

### Proposed Solution / Specific Changes
*   **Generic Enum Fix:**
    *   Investigate the root cause of the "comparison operators cannot be chained" error when `#[derive(Former)]` is applied to generic enums. This may involve deep diving into `syn` and `quote` interactions with generic parameters and bounds.
    *   Implement a robust fix within `former_meta` (potentially requiring new or modified utilities in `macro_tools`) that correctly handles generic parameters, lifetimes, and where clauses for derived constructors on generic enums.
    *   Ensure the fix supports various generic scenarios (e.g., multiple generic parameters, complex bounds, associated types).
*   **`macro_tools` Utilities Review/Refinement:**
    *   Review existing `macro_tools` utilities (e.g., `ident::cased_ident_from_ident`, `generic_params::GenericsRef`) for completeness, edge cases, and adherence to best practices.
    *   Add comprehensive unit tests for all `macro_tools` utilities to ensure their correctness and robustness.
    *   Improve documentation for `macro_tools` utilities, including usage examples where appropriate.
    *   Consider if any other common procedural macro patterns from `former_meta` could be further generalized and moved to `macro_tools`.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `#[derive(Former)]` macro should successfully compile and generate correct code for generic enums, including those with complex generic parameters and bounds.
*   Example (from `former` crate, currently disabled):
    ```rust
    #[derive(Debug, PartialEq, former::Former)]
    pub enum EnumOuter< X : Copy + Debug + PartialEq >
    {
      OtherVariant,
      _Phantom(core::marker::PhantomData::<X>),
    }
    // Should compile and allow usage like:
    // let _ = EnumOuter::<i32>::other_variant();
    ```
*   `macro_tools` utilities should be stable, well-tested, and clearly documented for future use.

### Acceptance Criteria (for this proposed change)
*   The `former` crate's `cargo test --package former` (with the generic enum tests re-enabled) passes without compilation errors or test failures related to generic enums.
*   All new and modified `macro_tools` utilities have comprehensive unit tests that pass.
*   `cargo clippy --package macro_tools --all-targets -- -D warnings` passes without warnings or errors.
*   `macro_tools` documentation is updated and clear.

### Potential Impact & Considerations
*   **Breaking Changes:** Unlikely, as this is primarily a bug fix and internal refinement.
*   **Dependencies:** No new external dependencies are anticipated.
*   **Performance:** The fix should not introduce significant performance regressions in macro expansion.
*   **Testing:** Requires new and updated tests in `former` (re-enabling existing ones) and `macro_tools`.

### Notes & Open Questions
*   The "comparison operators cannot be chained" error message is highly misleading; the actual issue is likely deeper in generic handling.
*   The `default-features` warning for `convert_case` in `macro_tools/Cargo.toml` should also be addressed as part of this task.