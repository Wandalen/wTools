# Project Plan: Refactor Enum Unit Variant Handling in `former`

### Goal
*   Refactor the implementation of `#[derive(Former)]` for **enum unit variants** within the `former_meta` crate.
*   This refactoring will focus on:
    1.  Intensively analyzing and integrating reusable components from the `macro_tools` crate into the enum unit variant handling logic (`former_meta/src/derive_former/former_enum/unit_variant_handler.rs`).
    2.  Analyzing the existing enum unit variant handling logic in `former_meta` to identify and potentially extract generalizable, well-tested utilities into the `macro_tools` crate.
*   The process will include proposing an initial detailed refactoring solution, critiquing it, and then implementing an improved version.
*   All changes must strictly adhere to `code/gen` instructions, Design Rules, and Codestyle Rules.

### Progress
*   Milestones Achieved:
    *   ✅ Increment 1: Analyze `macro_tools` for `former_meta` (Enum Unit Variants)
    *   ✅ Increment 2: Analyze `former_meta` (Enum Unit Variants) for `macro_tools` Generalizations
    *   ✅ Increment 3: Propose Initial Detailed Refactoring Solution for Enum Unit Variants
    *   ✅ Increment 4: Critique and Improve Refactoring Solution
    *   ✅ Increment 5: Implement Generalizations (New Utilities in `macro_tools`)
    *   ✅ Increment 6: Implement Improved Refactoring (Enum Unit Variants in `former_meta`)
*   Currently Working On:
    *   ⏳ Increment 7: Final Verification and Documentation Update
*   Up Next:
    *   ⚫ Increment 7: Final Verification and Documentation Update

### Target Crate
*   `module/core/former`
*   **Note:** While this plan file resides in the `former` crate, the primary modifications will occur in `module/core/former_meta` and `module/core/macro_tools`. The `former` crate will be used for verification, as it contains the tests for the `Former` derive macro.

### Relevant Context
*   **Primary Crates for Modification:**
    *   `module/core/former_meta` (specifically `src/derive_former/former_enum/unit_variant_handler.rs` and potentially `src/derive_former/former_enum.rs`)
    *   `module/core/macro_tools` (for potential additions and modifications)
*   **Key `macro_tools` Files for Analysis (Full Analysis in Increment 1):**
    *   All files within `module/core/macro_tools/src/` including `attr.rs`, `attr_prop.rs`, `diag.rs`, `ident.rs`, `kw.rs`, `generic_params.rs`, `typ.rs`, `item.rs`, `name.rs`, `punctuated.rs`, `quantifier.rs`, `tokens.rs`, etc.
*   **Key `former_meta` Files for Analysis:**
    *   `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (for context, dispatch, and `EnumVariantHandlerContext`)
    *   `module/core/former_meta/src/derive_former/field_attrs.rs`
    *   `module/core/former_meta/src/derive_former/struct_attrs.rs` (for `ItemAttributes` like `standalone_constructors`, `debug`)
*   **Key Documentation for Reference:**
    *   `module/core/former/Readme.md`
    *   `module/core/former/advanced.md`
    *   Existing `plan.md` files for "Expected Enum Former Behavior" rules.
*   **Workspace:** Yes, this is part of a Cargo workspace.
*   **Other Active Plans:** The refactoring plan for `former_meta` (`former_meta/plan.md`) should be considered, as changes here might affect its assumptions.

### Expected Behavior Rules (Enum Unit Variants)
*   **Rule 1a (Unit + `#[scalar]`):** Generates `Enum::variant() -> Enum`. (Handled by: `unit_variant_handler.rs`)
*   **Rule 2a (Unit + `#[subform_scalar]`):** Error. (Checked in: `unit_variant_handler.rs`)
*   **Rule 3a (Unit + Default):** Generates `Enum::variant() -> Enum`. (Handled by: `unit_variant_handler.rs`)
*   **Rule 4a (`#[standalone_constructors]` on Enum):**
    *   For unit variants, generates top-level `fn variant_name() -> EnumName` (or `fn enum_name_variant_name() -> EnumName` depending on naming convention for standalone, to be confirmed from existing behavior). The name should be snake_case.

### Increments

*   [✅] **Increment 1: Analyze `macro_tools` for `former_meta` (Enum Unit Variants)**
    *   Commit Message: `docs(former_meta): Analyze macro_tools for refactoring unit variant handling`

*   [✅] **Increment 2: Analyze `former_meta` (Enum Unit Variants) for `macro_tools` Generalizations**
    *   Commit Message: `docs(macro_tools): Analyze former_meta unit variant logic for potential generalizations`

*   [✅] **Increment 3: Propose Initial Detailed Refactoring Solution for Enum Unit Variants**
    *   Commit Message: `docs(former_meta): Propose initial detailed refactoring for unit variant handling`

*   [✅] **Increment 4: Critique and Improve Refactoring Solution**
    *   Commit Message: `docs(former_meta): Critique and improve refactoring plan for unit variants`

*   [✅] **Increment 5: Implement Generalizations (New Utilities in `macro_tools`)**
    *   Commit Message: `feat(macro_tools): Add new utilities generalized from former_meta enum handling`

*   [✅] **Increment 6: Implement Improved Refactoring (Enum Unit Variants in `former_meta`)**
    *   Target Crate(s): `former_meta`
    *   Pre-Analysis: Review the approved improved refactoring solution from Increment 4. The `macro_tools` utilities are now implemented and tested from Increment 5.
    *   Detailed Plan Step 1: Modify `former_meta/src/derive_former/former_enum/unit_variant_handler.rs` according to the approved plan from Increment 4. This involves:
        *   Replacing the `#[subform_scalar]` error handling with `macro_tools::diag::return_syn_err!`.
        *   Replacing the manual identifier creation for `method_ident` with a single call to `macro_tools::ident::cased_ident_from_ident`.
        *   Replacing manual generic quoting with calls to the `macro_tools::generic_params::GenericsRef` helper methods.
    *   Detailed Plan Step 2: Ensure all existing tests in `former` crate for enum unit variants continue to pass with identical behavior.
    *   Crucial Design Rules: [Prioritize Reuse and Minimal Change], [Proc Macro: Development Workflow].
    *   Relevant Behavior Rules: Rules 1a, 2a, 3a, 4a.
    *   Verification Strategy:
        *   User applies changes to `former_meta/src/derive_former/former_enum/unit_variant_handler.rs`.
        *   `cargo check --package former_meta` must pass.
        *   `cargo test --package former_meta` must pass.
        *   `cargo clippy --package former_meta --all-targets -- -D warnings` should pass.
    *   Test Matrix: Not applicable for this refactoring increment directly, but existing tests cover behavior.
    *   Commit Message: `refactor(former_meta): Improve unit variant handling using macro_tools`

*   [❌] **Increment 7: Final Verification and Documentation Update**
    *   Target Crate(s): `former_meta`, `macro_tools`, `former`
    *   Detailed Plan Step 1: Run `cargo clippy --package macro_tools --all-targets -- -D warnings` and address any new lints.
    *   Detailed Plan Step 2: Run `cargo clippy --package former_meta --all-targets -- -D warnings` and address any new lints.
    *   Detailed Plan Step 3: Run `cargo clippy --package former --all-targets -- -D warnings` and address any new lints.
    *   Detailed Plan Step 4: Run `cargo test --package macro_tools` to ensure no regressions.
    *   Detailed Plan Step 5: Run `cargo test --package former_meta` to ensure no regressions.
    *   Detailed Plan Step 6: Run `cargo test --package former` to ensure no regressions.
    *   Detailed Plan Step 7: Update any relevant internal documentation or comments in `former_meta` (especially `unit_variant_handler.rs`) and `macro_tools` to reflect the refactoring and new utilities.
    *   Detailed Plan Step 8: Review if the `former_meta/plan.md` (for splitting large files) needs adjustment based on changes to `unit_variant_handler.rs` or `former_enum.rs`. Propose updates if necessary.
    *   Verification Strategy: User confirms all checks pass and reviews documentation updates and any proposed changes to other plans.
    *   Commit Message: `chore(former): Final verification and docs update after unit variant refactor`

### Task Requirements
*   The refactoring should prioritize clarity, maintainability, and testability of `unit_variant_handler.rs`.
*   Any utilities moved to or created in `macro_tools` must be genuinely reusable, well-documented with examples (if applicable for complex utilities), and not overly specific to `former_meta`'s internal logic.
*   The "Expected Enum Former Behavior" for unit variants must be strictly preserved or corrected if bugs are found and approved as part of the plan.
*   Naming conventions for standalone constructors (e.g., `variant_name()` vs `enum_name_variant_name()`) should be consistent with the established patterns in `former_meta` or clarified if ambiguous.
*   Consider the impact on generic enums: ensure refactoring correctly handles generics in unit variant constructors (both static and standalone).

### Project Requirements
*   (This section should be cumulative. Assuming previous project requirements like Rust edition 2021, documentation for public APIs, etc., are still in effect. New project-level requirements identified will be added here.)
*   **Behavioral Equivalence:** Refactoring must not change the externally observable behavior or the generated code structure of the `Former` macro for enum unit variants, unless explicitly justified by a bug fix or alignment with documented "Expected Enum Former Behavior". Existing tests in the `former` crate for unit variants serve as the primary regression guard.
*   **`macro_tools` Generalization:** All new or modified code in `macro_tools` must be general-purpose, well-documented, and include unit tests. Utilities should not be overly specific to `former_meta`'s internal implementation details.
*   **Code Quality:** Code changes should demonstrably improve clarity, maintainability, and reduce redundancy in `unit_variant_handler.rs`.
*   **Error Reporting:** If `macro_tools` utilities are used for error handling, the quality (clarity, span accuracy) of compiler error messages generated by `former_meta` must be maintained or improved.
*   **Performance:** The refactoring should not introduce measurable performance regressions in macro expansion time. (Primarily a consideration for complex macros, but good to keep in mind).
*   **Rule Adherence:** All new and modified code must strictly adhere to the system prompt's Design Rules and Codestyle Rules, overriding existing styles in the repository if they conflict.
*   **Proc Macro Workflow:** While this is primarily a refactoring task, if any part of the core macro logic generation for unit variants is significantly altered (beyond just using helper functions), the principles of the "Proc Macro: Development Workflow" (e.g., clear separation of concerns, testability) should be respected.
*   **Verification Scope:** All `cargo` commands for verification (check, test, clippy) **must be scoped to individual packages** (e.g., `cargo test --package former_meta`) unless an increment explicitly plans for workspace-level integration testing as a final step.
*   **No Workspace Commands:** Do not run workspace-level commands like `cargo test --workspace` as the workspace is currently broken. All verification must be done on a per-crate basis.

### Notes & Insights
*   (This section will be populated as the plan progresses)
*   **[2025-06-22/Blocker]** The final verification step is blocked by a persistent and difficult-to-debug macro expansion error in the `former` crate. The error `comparison operators cannot be chained` occurs when deriving `Former` on a generic enum. All attempts to fix this by refactoring the code generation logic in `former_meta` have failed. The error message appears to be a red herring, as the generated code looks syntactically correct. I have exhausted all standard debugging and isolation strategies. I am reverting all changes to the last known good state and escalating to the user for guidance.
*   **[2025-05-24/Critique]** The original plan to implement changes in `former_meta` before `macro_tools` was impractical as it would leave the `former_meta` crate in a non-compilable state. The plan has been reordered to implement the `macro_tools` utilities first, ensuring each increment is verifiable.
*   **[2025-05-24/Critique-2]** The proposed `macro_tools` utilities have been refined for better ergonomics. `new_ident_from_cased_str` is replaced by `cased_ident_from_ident` to encapsulate more logic. The implementation plan for `GenericsRef` is clarified to be more efficient. The test matrix is updated accordingly.
*   **[2025-05-24/Hypothesis-H1]** Creating a higher-level utility `ident::cased_ident_from_ident` will be more ergonomic and reduce boilerplate in `former_meta`. **Result:** Confirmed.
*   **[2025-05-24/Hypothesis-H2]** Implementing `GenericsRef` to work on a borrowed `&syn::Generics` will be more efficient. **Result:** Confirmed.
*   **[2025-05-24/Hypothesis-H3]** The existing `kw::is()` function is sufficient for robust keyword detection. **Result:** Partially Rejected. The keyword list needs to be updated to include reserved keywords for full correctness. This is now part of the detailed plan for Increment 5.
