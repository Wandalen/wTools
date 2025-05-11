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

// mod tuple_zero_fields_derive;
// mod tuple_zero_fields_manual;
// mod tuple_zero_fields_only_test;
// mod unit_variant_derive;
// mod unit_variant_manual;
// mod unit_variant_only_test; // This was for the previous tests, keyword tests will include their own _only_test

// mod keyword_variant_manual;
// mod keyword_variant_derive; // Will be enabled later
// mod keyword_variant_only_test; // Should only be included, not a module itself

// mod generic_unit_variant_manual;
mod generic_unit_variant_derive;

mod enum_named_fields_unit_derive;
// mod enum_named_fields_unit_manual;
// mod enum_named_fields_unit_only_test;
// mod generics_in_tuple_variant_unit_derive;
// mod generics_in_tuple_variant_unit_manual;
// mod keyword_variant_unit_derive;
// mod keyword_variant_unit_only_test;
// mod standalone_constructor_unit_derive;
// mod standalone_constructor_unit_only_test;
// mod standalone_constructor_args_unit_derive;
// mod standalone_constructor_args_unit_manual;
// mod standalone_constructor_args_unit_only_test;

// pub mod compile_fail;