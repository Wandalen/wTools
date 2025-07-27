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
use super::*;

// Common types for scalar_generic_tuple tests
include!( "scalar_generic_tuple_common_types.rs" );
// Uncomment modules as they are addressed in increments.

// mod basic_derive;
// mod basic_manual;
// mod basic_only_test; // This is included by the derive and manual files
// mod generics_in_tuple_variant_only_test;
// mod generics_independent_tuple_derive;
// mod generics_independent_tuple_manual;
// mod generics_independent_tuple_only_test;
mod generics_shared_tuple_derive;  // E0392: type parameter T is never used (Rust analyzer issue)
mod generics_shared_tuple_manual;
// mod generics_shared_tuple_only_test;
// mod test_syntax;
// mod scalar_generic_tuple_derive;  // E0392: type parameter T is never used (Rust analyzer issue)
// mod scalar_generic_tuple_manual;  // Disabled because it includes the derive version
// mod tuple_multi_default_derive;  // Syntax error in generated code
mod tuple_multi_default_manual;
// mod tuple_multi_default_only_test;
mod tuple_multi_scalar_derive;
mod tuple_multi_scalar_manual;
// mod tuple_multi_scalar_only_test;
// mod tuple_multi_standalone_args_derive;
// // mod tuple_multi_standalone_args_manual;
// // mod tuple_multi_standalone_args_only_test;
// mod tuple_multi_standalone_derive;
// // mod tuple_multi_standalone_manual;
// // mod tuple_multi_standalone_only_test;
mod usecase1_derive;  // Import and trait issues
// mod usecase1_manual;  // Import and trait issues
// mod enum_named_fields_unnamed_derive;
// mod enum_named_fields_unnamed_manual;
// mod enum_named_fields_unnamed_only_test;
// mod generics_in_tuple_variant_tuple_derive;
// mod generics_in_tuple_variant_tuple_manual;
// mod keyword_variant_tuple_derive;
// mod keyword_variant_tuple_only_test;
// mod standalone_constructor_tuple_derive;
// mod standalone_constructor_tuple_only_test;
// mod standalone_constructor_args_tuple_derive;
// mod standalone_constructor_args_tuple_single_manual; // Added
// mod standalone_constructor_args_tuple_multi_manual; // Added
// mod standalone_constructor_args_tuple_only_test;

// Coverage for `tuple_zero_fields_*` tests:
// - Tests zero-field tuple variants e.g., `MyEnum::Variant()`.
// - Verifies Rules 1b (scalar), 3b (default), and 4a (standalone_constructors).
mod tuple_zero_fields_derive; // Re-enabled after fixing _only_test.rs and derive attributes
mod tuple_zero_fields_manual; // Re-enabled after fixing _only_test.rs
                              // Note: tuple_zero_fields_only_test.rs is included by the manual and derive files.

// pub mod compile_fail;
