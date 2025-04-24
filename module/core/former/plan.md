# Project Plan: Consistent Enum Variant Handling in Former Derive

## Progress

*   ✅ Increment 1: Analyze `former_meta/src` & current enum macro logic (`former_enum.rs`)
*   ⏳ **Increment 2: Refactor `former_enum.rs` for consistent unit/single-field scalar/subform behavior** <-- Current
*   ⚫ Increment 3: Refactor `former_enum.rs` for consistent multi-field/struct variant behavior
*   ⚫ Increment 4: Update tests for unit/single-field variants
*   ⚫ Increment 5: Update tests for multi-field/struct variants & remove qqq comment
*   ⚫ Increment 6: Update documentation (`Readme.md`, `advanced.md`)
*   ⚫ Increment 7: Final verification

## Increments

*   ✅ Increment 1: Analyze `former_meta/src` & current enum macro logic (`former_enum.rs`)
    *   Goal: Thoroughly analyze all source files within `module/core/former_meta/src` to understand the overall macro structure, helpers, and potential interactions. Then, specifically analyze the existing implementation in `former_enum.rs` for handling different variant kinds and attributes (`#[scalar]`, `#[subform_scalar]`). Identify discrepancies with the target consistent behavior rules.
    *   Detailed Plan Step 1: Read `former_meta/src/lib.rs` to understand the overall structure, features, and entry points for the different derive macros.
    *   Detailed Plan Step 2: Read `former_meta/src/derive_former.rs` and its submodules (`field_attrs.rs`, `field.rs`, `struct_attrs.rs`) to understand the main dispatch logic, attribute parsing, and helper structures used by the `Former` derive.
    *   Detailed Plan Step 3: Read `former_meta/src/derive_former/former_struct.rs` to understand how struct formers are generated (as a comparison point for consistency).
    *   Detailed Plan Step 4: Read `former_meta/src/derive_former/former_enum.rs` carefully. Map the existing code paths to different variant kinds (Unit, Tuple(1), Tuple(N), Struct(0), Struct(1), Struct(N)) and attribute combinations (`#[scalar]`, `#[subform_scalar]`, none). Pay close attention to how generics and bounds are handled.
    *   Detailed Plan Step 5: Document the findings from Step 4, specifically noting:
        *   How unit variants are handled (constructor generation).
        *   How single-field tuple variants are handled (default behavior, `#[scalar]`, `#[subform_scalar]`). Note the heuristic check for `inner_former_exists`.
        *   How multi-field tuple variants are handled (default behavior, `#[scalar]`, `#[subform_scalar]`). Note the error for the default case.
        *   How struct-like variants (0, 1, N fields) are handled (default behavior, `#[scalar]`, `#[subform_scalar]`). Note the implicit former generation logic.
        *   How generics and bounds are currently propagated in the enum logic (e.g., use of `generic_params::merge`, phantom data).
    *   Detailed Plan Step 6: Compare the documented current behavior (from Step 5) against the **Proposed Consistent Behavior Rules** defined during planning. Identify specific areas in `former_enum.rs` that need refactoring in subsequent increments (e.g., error handling for multi-field defaults, struct(0) handling, consistency of `#[scalar]` behavior).
    *   Detailed Plan Step 7: Record findings, identified discrepancies, and areas needing refactoring in the `## Notes & Insights` section of `plan.md`.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Comments and Documentation](#comments-and-documentation).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during the analysis and documentation phase.
    *   Verification Strategy: Manual review of the analysis findings against the codebase (`former_meta/src`) and the target rules. Confirm understanding of the current state and required changes. No code changes in this increment.
*   ⏳ Increment 2: Refactor `former_enum.rs` for consistent unit/single-field scalar/subform behavior
    *   Goal: Modify the macro code to correctly generate direct constructors or subformer starters for unit and single-field variants according to the defined rules (considering `#[scalar]`, `#[subform_scalar]`, and default behavior).
    *   Detailed Plan Step 1: Locate the code block in `former_enum.rs` handling `syn::Fields::Unit`. Verify it correctly generates a direct constructor static method (`Enum::variant() -> Enum`). Ensure `#[scalar]` or `#[subform_scalar]` on unit variants are ignored or produce a reasonable error.
    *   Detailed Plan Step 2: Locate the code block handling `syn::Fields::Unnamed` with `fields.unnamed.len() == 1`.
    *   Detailed Plan Step 3: Remove the `inner_former_exists` heuristic check.
    *   Detailed Plan Step 4: Implement logic for `#[scalar]`: Ensure it *always* generates a direct constructor static method (`Enum::variant(InnerType) -> Enum`).
    *   Detailed Plan Step 5: Implement logic for `#[subform_scalar]`: Ensure it *always* generates a subformer starter static method (`Enum::variant() -> InnerFormer<...>`) and the corresponding `End` struct/impl. Add a check to error if the attribute is applied to a non-path inner type (e.g., `Variant( (i32) )`). Rely on the compiler for errors if `InnerFormer` doesn't exist.
    *   Detailed Plan Step 6: Implement logic for the **Default (No Attribute)** case: Ensure it *always* generates a subformer starter static method (`Enum::variant() -> InnerFormer<...>`) and the corresponding `End` struct/impl (as per the **Revised Rule**). Rely on the compiler for errors if `InnerFormer` doesn't exist.
    *   Detailed Plan Step 7: Apply the same logic (Steps 4-6) to single-field *struct* variants (`syn::Fields::Named` with `fields.named.len() == 1`).
    *   Detailed Plan Step 8: Ensure consistent handling of generics and bounds propagation in all generated code paths within this increment.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily](#code-style-do-not-reformat-arbitrarily) (apply mandated style), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during refactoring.
    *   Verification Strategy: Compile checks (`cargo check --package former_meta`), review generated code snippets mentally or using `#[debug]` for specific test cases (e.g., `basic_*`, `scalar_generic_tuple_*` variant 1), ensure no regressions in related tests yet (though full testing is in later increments).
*   ⚫ Increment 3: Refactor `former_enum.rs` for consistent multi-field/struct variant behavior
    *   Goal: Modify the macro code to generate implicit former builders for multi-field/struct variants *only* when `#[scalar]` is present. Generate compile-time errors for multi-field/struct variants without `#[scalar]`. Ensure struct(0) variants behave like multi-field.
*   ⚫ Increment 4: Update tests for unit/single-field variants
    *   Goal: Review and update tests in `former/tests/inc/former_enum_tests/` related to unit and single-field variants (e.g., `unit_*`, `basic_*`, `scalar_generic_tuple_*`, `generics_shared_tuple_*`, `generics_independent_tuple_*`) to ensure they align with the refactored, consistent logic. Update corresponding `*_manual.rs` files.
*   ⚫ Increment 5: Update tests for multi-field/struct variants & remove qqq comment
    *   Goal: Review and update tests related to multi-field/struct variants (e.g., `multi_field_*`, `enum_named_fields_*`, `generics_shared_struct_*`, `generics_independent_struct_*`). Ensure they test the implicit former builder generation with `#[scalar]` and potentially add tests for the error case without `#[scalar]`. Remove the misleading `qqq` comment from `scalar_generic_tuple_derive.rs`. Update corresponding `*_manual.rs` files.
*   ⚫ Increment 6: Update documentation (`Readme.md`, `advanced.md`)
    *   Goal: Clearly document the consistent rules for how `#[derive(Former)]` handles different enum variants and the effects of `#[scalar]` and `#[subform_scalar]` attributes in the main `former` crate documentation.
*   ⚫ Increment 7: Final verification
    *   Goal: Run the entire test suite for the `former` crate (`cargo test`) to ensure all tests pass and there are no regressions.

## Notes & Insights

*   [2025-04-24/Init] Plan created to enforce consistent behavior for enum variant handling in `#[derive(Former)]` based on variant structure and attributes (`#[scalar]`, `#[subform_scalar]`).
*   [2025-04-24/Init] Analysis of all `former_meta/src` files added as the first step of Increment 1.
*   [2025-04-24/Init] Explicit mention of rule adherence (code/gen, design, codestyle) will be added to the detailed plan for each increment.
*   [2025-04-24/Inc 1] Analysis revealed inconsistencies in handling struct-like variants (0, 1, N fields) compared to tuple variants, especially regarding default behavior (should error) and `#[scalar]` (should only generate implicit former starter). Unit and single-field tuple handling mostly aligns with target rules. Multi-field tuple handling aligns. Error handling for invalid attribute combinations (e.g., `#[subform_scalar]` on multi-field) is missing.
*   [2025-04-24/Inc 2] **Rule Adjustment:** Default behavior for Single-Field variants (Tuple or Struct) will now *always* generate a Subformer Starter, relying on the compiler to error if the inner type does not derive `Former`. This removes the unreliable heuristic check.