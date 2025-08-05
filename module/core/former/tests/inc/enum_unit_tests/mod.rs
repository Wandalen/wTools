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

// Uncomment modules as they are addressed in increments.

// Coverage for `unit_variant_*` tests is described in the Test Matrix at the top of this file.
mod unit_variant_derive; // Enabled - enum Former derive now implemented
mod unit_variant_manual;

// Coverage for `keyword_variant_*` tests:
// - Tests unit variants with keyword identifiers e.g., `MyEnum::r#fn`.
// - Verifies Rules 1a, 3a, and 4a.
mod keyword_variant_derive; // Enabled - testing keyword variant derive
mod keyword_variant_manual; // Known broken - let's try to fix it

// Coverage for `generic_unit_variant_*` tests:
// - Tests unit variants within generic enums e.g., `Enum<T>::UnitVariant`.
// - Verifies Rules 1a, 3a, and 4a in a generic context.
mod generic_unit_variant_derive; // Re-enabled to debug generic parsing issues

// Coverage for `mixed_enum_unit_*` tests:
// - Tests unit variants in enums that also contain non-unit (e.g., struct/tuple) variants.
// - Verifies Rules 1a, 3a, and 4a for the unit variants in such mixed enums.
mod mixed_enum_unit_derive; // Enabled - testing mixed enum unit derive
mod mixed_enum_unit_manual; // Configured to test only static method for SimpleUnit

// Coverage for `enum_named_fields_unit_*` tests:
// - Tests unit variants within an enum where other variants use named field syntax.
// - Verifies Rules 1a, 3a, and 4a.
mod enum_named_fields_unit_derive; // Enabled - testing unit variants in named fields enum
mod enum_named_fields_unit_manual;

// Coverage for `generic_enum_simple_unit_*` tests:
// - Tests a simple unit variant within a generic enum e.g., `EnumOuter<X>::OtherVariant`.
// - Verifies Rules 1a, 3a, and 4a.
// Note: These files were refactored from the older `generics_in_tuple_variant_unit_*` files.
// mod generic_enum_simple_unit_derive; // Temporarily disabled - working on generic parsing
mod generic_enum_simple_unit_manual;
// Note: keyword_variant_unit_derive was removed as redundant (Increment 11)
// Note: standalone_constructor_unit_derive was removed as redundant (Increment 12)
// Note: standalone_constructor_args_unit_derive and _manual were removed as redundant (Increment 13)

// Coverage for `compile_fail` module:
// - Tests scenarios expected to fail compilation for unit variants.
// - Currently verifies Rule 2a (`#[subform_scalar]` on a unit variant is an error).
pub mod compile_fail;
