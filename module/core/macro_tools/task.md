# Change Proposal for macro_tools

### Task ID
*   TASK-20250628-081959-MacroToolsGenericEnumFix

### Requesting Context
*   **Requesting Crate/Project:** `module/core/former_meta` (from `module/core/former/plan.md`)
*   **Driving Feature/Task:** Unblocking `former_meta`'s `Former` derive macro for generic enums and refining `macro_tools` utilities.
*   **Link to Requester's Plan:** `module/core/former/plan.md`
*   **Date Proposed:** 2025-06-28

### Overall Goal of Proposed Change
*   Investigate and robustly fix the "comparison operators cannot be chained" error that occurs when `Former` is derived on generic enums, which is currently blocking `former_meta`'s full functionality.
*   Review, refine, and potentially extend the `macro_tools` utilities (specifically `ident::cased_ident_from_ident` and `generic_params::GenericsRef`) that were generalized from `former_meta` in Increment 5 of the `former_meta` refactoring task.

### Problem Statement / Justification
*   The `former_meta` refactoring task is blocked by a persistent and difficult-to-debug macro expansion error (`comparison operators cannot be chained`) when `Former` is derived on generic enums. This error appears to be a red herring, and attempts to fix it within `former_meta` have failed. A dedicated investigation within `macro_tools` (or a related utility crate) is needed to provide a robust solution.
*   The `macro_tools` utilities generalized from `former_meta` (Increment 5) were a first pass. A dedicated review is needed to ensure they are optimally designed, cover all necessary edge cases, and are fully ergonomic for future use.

### Proposed Solution / Specific Changes
*   **Phase 1: Generic Enum Derivation Fix**
    *   **Investigation:** Thoroughly investigate the root cause of the "comparison operators cannot be chained" error when `Former` is derived on generic enums. This may involve:
        *   Analyzing the `syn` and `quote` output for problematic generic enum derivations.
        *   Comparing the generated code for generic vs. non-generic enums.
        *   Debugging the macro expansion process.
        *   Consulting Rust compiler diagnostics or `proc_macro` best practices.
    *   **Solution Development:** Implement a robust solution within `macro_tools` (or propose changes to `former_meta` if the issue is specific to its usage of `macro_tools`) that correctly handles generic parameters in derive macros, preventing the observed error. This might involve new helper functions for generic parameter manipulation or quoting.
*   **Phase 2: `macro_tools` Utilities Review and Refinement**
    *   **Review:** Conduct a comprehensive review of `ident::cased_ident_from_ident` and `generic_params::GenericsRef` (and related utilities).
    *   **Refinement:** Identify areas for improvement in terms of API ergonomics, robustness, and coverage of edge cases. Implement any necessary changes.
    *   **Extension:** Consider if additional general-purpose utilities related to identifier manipulation or generic parameter handling could be extracted from `former_meta` or other crates into `macro_tools`.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `Former` derive macro in `former_meta` should successfully compile and function correctly when applied to generic enums, without the "comparison operators cannot be chained" error.
*   The `macro_tools` utilities should be well-documented, robust, and easy to use for future macro development.

### Acceptance Criteria (for this proposed change)
*   The `former` crate's tests related to generic enum derivation (currently disabled) pass when re-enabled after the fix.
*   The `macro_tools` utilities are reviewed, refined, and documented as per best practices.

### Potential Impact & Considerations
*   **Breaking Changes:** Potential for minor breaking changes in `macro_tools` if API refinement is significant, but efforts should be made to minimize this.
*   **Dependencies:** None.
*   **Performance:** Should not negatively impact macro expansion performance.
*   **Security:** No impact.
*   **Testing:** Requires new and updated tests in `macro_tools` and `former` to verify the fix and utility refinements.

### Alternatives Considered (Optional)
*   Continuing to debug the generic enum issue within `former_meta` (rejected due to repeated failures and blocking current task).

### Notes & Open Questions
*   The exact nature of the generic enum derivation bug is still unknown, requiring dedicated investigation.
*   Coordination with the `former_meta` team might be needed if the fix requires changes in `former_meta`'s usage of `macro_tools`.