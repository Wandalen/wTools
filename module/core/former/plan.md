
# Project Plan: Refactor Enum Variant Handling in Former Derive

## Initial Task

// ==================================
// Refactoring Plan Documentation - UPDATED
// ==================================
//
//! # Refactoring Plan for `former_for_enum`
//!
//! The main `former_for_enum` function has become complex due to handling
//! multiple enum variant structures (Unit, Tuple, Struct) and field counts (0, 1, N)
//! within nested match statements.
//!
//! **Goal:** Improve readability, maintainability, and testability by extracting
//! the logic for handling each distinct variant case into its own dedicated function
//! located in a separate file within a new submodule.
//!
//! **Extraction Cases & Logic Handoff:**
//!
//! The main `former_for_enum` function dispatches control to specific handlers based on
//! the variant's field kind (`Unit`, `Unnamed`, `Named`) and field count. Each handler
//! then implements the logic based on the presence of `#[scalar]` or `#[subform_scalar]`
//! attributes, according to the rules defined below the documentation comment.
//!

// ==================================
//      Enum Variant Handling Rules (Consistent Logic) - UPDATED
// ==================================
//
// This macro implements the `Former` derive for enums based on the following consistent rules.
// Each case is handled by a specific function as noted:
//
// 1.  **`#[scalar]` Attribute:**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
//     *   **Zero-Field Variant (Struct):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`) // <<< CORRECTED Handler
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `handle_struct_non_zero_variant`) // <<< CORRECTED Handler
//     *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
//     *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.
//
// 2.  **`#[subform_scalar]` Attribute:**
//     *   **Unit Variant:** Error. (Checked in: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple or Struct):** Error. (Checked in: `handle_tuple_zero_variant`, `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//     *   **Multi-Field Variant (Tuple):** Error. Cannot use `subform_scalar` on multi-field tuple variants. (Checked in: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//
// 3.  **Default Behavior (No Attribute):**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
//     *   **Zero-Field Variant (Struct):** Error. Requires `#[scalar]`. (Checked in: `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//     *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//
// Body attribute `standalone_constructors` creates stand-alone, top-level constructors for struct/enum. for struct it's always single function, for enum it's as many functions as enum has vartianys.
//
// ==================================


## Increments

## Notes & Insights

*   *(No notes yet)*
*   **[2025-04-29] Skipped Increment:** Increment 5 (Extract handler for Tuple variants with non-zero fields) was skipped due to persistent issues with applying automated changes to `module/core/former_meta/src/derive_former/former_enum.rs`. Manual intervention is required to complete this increment.
*   **[2025-04-29] Stuck in Increment 6:** Encountered persistent compilation errors after moving code into `handle_struct_non_zero_variant`. Initiating Stuck Resolution Process.
*   **[2025-04-29] Hypotheses for Increment 6:**
    *   Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error.
    *   Hypothesis 5: The issue arises from the combination or interaction of the individually generated components, not the components themselves.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `Storage` struct and its `impl` blocks (`storage_def`, `storage_default_impl`, `storage_trait_impl`, `storage_preform_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `DefinitionTypes` struct and its `impl` blocks (`def_types_struct`, `def_types_default_impl`, `def_types_former_impl`, `def_types_mutator_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `Definition` struct and its `impl` blocks (`def_struct`, `def_default_impl`, `def_former_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` block generating the `Former` struct definition (`former_struct_def`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2024-04-30/Increment 6] Fix:** Resolved E0599 compilation errors by changing how `merged_where_clause` is passed to handler functions (passing `Option<&WhereClause>` instead of `Option<&Punctuated<...>>`).
