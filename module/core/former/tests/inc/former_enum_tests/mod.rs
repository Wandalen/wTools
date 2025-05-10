//! ## Test Matrix Coverage (Unit Variants)
//!
//! This plan focuses on verifying the behavior for **Unit Variants**. The relevant factors and combinations tested by the `unit_variant_*` files are:
//!
//! *   **Factors:**
//!     1.  Variant Type: Unit (Implicitly selected)
//!     2.  Variant-Level Attribute: None (Default), `#[scalar]`
//!     3.  Enum-Level Attribute: None, `#[standalone_constructors]`
//!
//! *   **Combinations Covered by `unit_variant_only_test.rs`:**
//!     *   Unit + Default + None (Rule 3a) -> Tested via `Status::pending()` / `Status::complete()` in `unit_variant_constructors()` test.
//!     *   Unit + `#[scalar]` + None (Rule 1a) -> Tested via `Status::pending()` / `Status::complete()` in `unit_variant_constructors()` test (as default is scalar).
//!     *   Unit + Default + `#[standalone_constructors]` (Rule 3a, 4) -> Tested via `pending()` / `complete()` in `unit_variant_standalone_constructors()` test.
//!     *   Unit + `#[scalar]` + `#[standalone_constructors]` (Rule 1a, 4) -> Tested via `pending()` / `complete()` in `unit_variant_standalone_constructors()` test.
//!
//! ---
//!
//! ## Test Matrix Coverage (Tuple Variants)
//!
//! This plan focuses on verifying the behavior for **Tuple Variants**. The relevant factors and combinations tested by the relevant files are:
//!
//! *   **Factors:**
//!     1.  Variant Type: Tuple (Implicitly selected)
//!     2.  Number of Fields: Zero (`V()`), One (`V(T1)`), Multiple (`V(T1, T2, ...)`)
//!     3.  Field Type `T1` (for Single-Field): Derives `Former`, Does NOT derive `Former`
//!     4.  Variant-Level Attribute: None (Default), `#[scalar]`, `#[subform_scalar]`
//!     5.  Enum-Level Attribute: None, `#[standalone_constructors]`
//!     6.  Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context): N/A, On single field, On all/some/no fields (multi)
//!
//! *   **Combinations Covered (Mapped to Rules & Test Files):**
//!     *   **Zero-Field (`V()`):**
//!         *   T0.1 (Default): Rule 3b (`enum_named_fields_*`)
//!         *   T0.2 (`#[scalar]`): Rule 1b (`enum_named_fields_*`)
//!         *   T0.3 (Default + Standalone): Rule 3b, 4 (`enum_named_fields_*`)
//!         *   T0.4 (`#[scalar]` + Standalone): Rule 1b, 4 (`enum_named_fields_*`)
//!         *   T0.5 (`#[subform_scalar]`): Rule 2b (Error - `compile_fail/tuple_zero_subform_scalar_error.rs`)
//!     *   **Single-Field (`V(T1)`):**
//!         *   T1.1 (Default, T1 derives Former): Rule 3d.i (`basic_*`, `generics_in_tuple_variant_*`, `generics_shared_tuple_*`, `usecase1.rs`)
//!         *   T1.2 (Default, T1 not Former): Rule 3d.ii (Needs specific test file if not covered implicitly)
//!         *   T1.3 (`#[scalar]`): Rule 1d (`generics_independent_tuple_*`, `scalar_generic_tuple_*`, `keyword_variant_*`)
//!         *   T1.4 (`#[subform_scalar]`, T1 derives Former): Rule 2d (Needs specific test file if not covered implicitly)
//!         *   T1.5 (`#[subform_scalar]`, T1 not Former): Rule 2d (Error - `compile_fail/tuple_single_subform_non_former_error.rs`)
//!         *   T1.6 (Default, T1 derives Former + Standalone): Rule 3d.i, 4 (`standalone_constructor_*`)
//!         *   T1.7 (Default, T1 not Former + Standalone): Rule 3d.ii, 4 (Needs specific test file if not covered implicitly)
//!         *   T1.8 (`#[scalar]` + Standalone): Rule 1d, 4 (`standalone_constructor_args_*`)
//!         *   T1.9 (`#[subform_scalar]`, T1 derives Former + Standalone): Rule 2d, 4 (Needs specific test file if not covered implicitly)
//!         *   T1.10 (`#[subform_scalar]`, T1 not Former + Standalone): Rule 2d (Error - Covered by T1.5)
//!     *   **Multi-Field (`V(T1, T2, ...)`):**
//!         *   TN.1 (Default): Rule 3f (Needs specific test file if not covered implicitly by TN.4)
//!         *   TN.2 (`#[scalar]`): Rule 1f (`keyword_variant_*`, `standalone_constructor_args_*`)
//!         *   TN.3 (`#[subform_scalar]`): Rule 2f (Error - `compile_fail/tuple_multi_subform_scalar_error.rs`)
//!         *   TN.4 (Default + Standalone): Rule 3f, 4 (Needs specific test file, potentially `standalone_constructor_args_*` if adapted)
//!         *   TN.5 (`#[scalar]` + Standalone): Rule 1f, 4 (`standalone_constructor_args_*`)
//!
//! Note: The effect of `#[arg_for_constructor]` is covered by Rule 4 in conjunction with the base behavior.
//!
//! ---
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
// Uncomment modules as they are addressed in increments.

pub mod unit_tests;
pub mod unnamed_tests;
pub mod named_tests;
pub mod compile_fail;
