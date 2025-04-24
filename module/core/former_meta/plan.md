# Project Plan: Refactor Large Files in `former_meta`

## Progress

*   [⏳] **Increment 1: Plan Splitting `src/derive_former/field.rs`** <-- Current
*   [⚫] Increment 2: Implement Splitting `src/derive_former/field.rs`
*   [⚫] Increment 3: Plan Splitting `src/derive_former/former_enum.rs`
*   [⚫] Increment 4: Implement Splitting `src/derive_former/former_enum.rs`

## Increments

*   [⏳] **Increment 1: Plan Splitting `src/derive_former/field.rs`**
    *   **Analysis:**
        *   Current File: `src/derive_former/field.rs` (1440 lines)
        *   Purpose: Defines `FormerField` struct and associated methods for generating code related to individual struct fields (storage representation, preform logic, various setters).
        *   Key Items:
            *   `FormerField` struct definition.
            *   `impl FormerField`:
                *   `from_syn`: Constructor.
                *   `storage_fields_none`: Generates `field: None`.
                *   `storage_field_optional`: Generates `pub field: Option<T>`.
                *   `storage_field_preform`: Generates complex logic for unwrapping/defaulting fields in the `form()` method. (Large)
                *   `storage_field_name`: Generates `field,` for struct construction.
                *   `former_field_setter`: Main dispatcher calling specific setter generation methods.
                *   `scalar_setter`: Generates simple scalar setter.
                *   `subform_scalar_setter`: Generates complex scalar subformer setter, including `End` struct. (Very Large)
                *   `subform_collection_setter`: Generates complex collection subformer setter, including `End` struct. (Very Large)
                *   `subform_entry_setter`: Generates complex entry subformer setter, including `End` struct. (Very Large)
                *   Helper methods: `scalar_setter_name`, `subform_scalar_setter_name`, `subform_collection_setter_name`, `subform_entry_setter_name`, `scalar_setter_required`.
    *   **Proposed Splitting Strategy:**
        *   Create a new sub-module: `src/derive_former/field/`.
        *   Move the `FormerField` struct definition and the `impl FormerField` block containing the *simpler* methods (`from_syn`, `storage_fields_none`, `storage_field_optional`, `storage_field_name`, `former_field_setter`, `scalar_setter`, name helpers, `scalar_setter_required`) into `src/derive_former/field/mod.rs`.
        *   Extract the complex `storage_field_preform` logic into its own file: `src/derive_former/field/preform.rs`. Make the function public within the `field` module.
        *   Extract the `subform_scalar_setter` logic (including its `End` struct generation) into `src/derive_former/field/setter_subform_scalar.rs`. Make the function public within the `field` module.
        *   Extract the `subform_collection_setter` logic (including its `End` struct generation) into `src/derive_former/field/setter_subform_collection.rs`. Make the function public within the `field` module.
        *   Extract the `subform_entry_setter` logic (including its `End` struct generation) into `src/derive_former/field/setter_subform_entry.rs`. Make the function public within the `field` module.
        *   Update `src/derive_former/mod.rs` to declare `pub mod field;`.
        *   Ensure all extracted functions are correctly called from `former_field_setter` in `field/mod.rs`.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Verification Strategy:** Ensure the code compiles successfully after refactoring. Review diffs to confirm only code movement occurred. Run existing tests (`cargo test`) to confirm semantic equivalence.

*   [⚫] Increment 2: Implement Splitting `src/derive_former/field.rs`
    *   **Goal:** Refactor `src/derive_former/field.rs` into the `src/derive_former/field/` module as planned in Increment 1. **This refactoring must be purely structural, ensuring the code remains semantically identical to the original.**
    *   Detailed Plan Step 1: Create directory `src/derive_former/field/`.
    *   Detailed Plan Step 2: Create `src/derive_former/field/mod.rs`. Move `FormerField` struct and simpler methods from `src/derive_former/field.rs` into it. Add necessary `pub use` or `mod` statements for the files to be created.
    *   Detailed Plan Step 3: Create `src/derive_former/field/preform.rs` and move the `storage_field_preform` function logic into it. Adjust visibility.
    *   Detailed Plan Step 4: Create `src/derive_former/field/setter_subform_scalar.rs` and move the `subform_scalar_setter` function logic (including `End` struct) into it. Adjust visibility.
    *   Detailed Plan Step 5: Create `src/derive_former/field/setter_subform_collection.rs` and move the `subform_collection_setter` function logic (including `End` struct) into it. Adjust visibility.
    *   Detailed Plan Step 6: Create `src/derive_former/field/setter_subform_entry.rs` and move the `subform_entry_setter` function logic (including `End` struct) into it. Adjust visibility.
    *   Detailed Plan Step 7: Delete the original `src/derive_former/field.rs`.
    *   Detailed Plan Step 8: Update `src/derive_former/mod.rs` to declare `pub mod field;`.
    *   Detailed Plan Step 9: Run `cargo check` or `cargo build` to ensure compilation. Fix any path or visibility errors.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private), [Structuring: Add Module Declaration Before Content](#structuring-add-module-declaration-before-content).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Confirm no semantic changes were introduced.
    *   **Verification Strategy:** Compilation success (`cargo check`), review code diffs to confirm only code movement, **run tests (`cargo test`) to verify semantic equivalence.**

*   [⚫] Increment 3: Plan Splitting `src/derive_former/former_enum.rs`
    *   Detailed Plan Step 1: Analyze `src/derive_former/former_enum.rs` (items, complexity).
    *   Detailed Plan Step 2: Propose a new module structure (e.g., `src/derive_former/enum/mod.rs`, `src/derive_former/enum/variant_former.rs`).
    *   Detailed Plan Step 3: Define which items go into which new file. Focus on extracting the large `generate_implicit_former_for_variant` helper.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Verification Strategy:** Ensure the plan logically separates concerns and reduces file size effectively.

*   [⚫] Increment 4: Implement Splitting `src/derive_former/former_enum.rs`
    *   **Goal:** Refactor `src/derive_former/former_enum.rs` into the `src/derive_former/enum/` module as planned in Increment 3. **This refactoring must be purely structural, ensuring the code remains semantically identical to the original.**
    *   Detailed Plan Step 1: Create directory `src/derive_former/enum/`.
    *   Detailed Plan Step 2: Create `src/derive_former/enum/mod.rs`. Move `former_for_enum` and smaller helpers into it.
    *   Detailed Plan Step 3: Create `src/derive_former/enum/variant_former.rs`. Move `generate_implicit_former_for_variant` into it. Adjust visibility.
    *   Detailed Plan Step 4: Delete the original `src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 5: Update `src/derive_former/mod.rs` to declare `pub mod r#enum;` (using raw identifier for `enum`).
    *   Detailed Plan Step 6: Run `cargo check` or `cargo build` to ensure compilation. Fix any path or visibility errors.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private), [Structuring: Add Module Declaration Before Content](#structuring-add-module-declaration-before-content).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Confirm no semantic changes were introduced.
    *   **Verification Strategy:** Compilation success (`cargo check`), review code diffs to confirm only code movement, **run tests (`cargo test`) to verify semantic equivalence.**

## Notes & Insights

*   **[Date/Increment 1] Insight:** Splitting `field.rs` focuses on isolating the complex setter generation logic (`subform_*`) and the `preform` logic into separate files within a `field` submodule. This maintains the core `FormerField` definition and simpler methods together while improving maintainability of the complex parts.
*   **[Date/Increment 1] Insight:** Splitting `former_enum.rs` primarily involves extracting the large `generate_implicit_former_for_variant` helper function into its own file within an `enum` submodule. This isolates the most complex part of the enum derivation logic.
*   **[Date/Increment 1] Requirement:** All file splitting operations (Increments 2 and 4) must maintain semantic equivalence with the original code. The primary verification for this will be running `cargo test` after each split.