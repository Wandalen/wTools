//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for zero-field tuple variants, covering both default behavior and the effect of the `#[scalar]` attribute. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Verifies the derived static method `EnumWithZeroFieldTuple::variant_zero_default()` returns the enum instance.
//! - Rule 1b (Tuple + Zero-Field + `#[scalar]`): Verifies the derived static method `EnumWithZeroFieldTuple::variant_zero_scalar()` returns the enum instance.
//! - Rule 4a (`#[standalone_constructors]`): Implicitly covered by the tests in `_only_test.rs` which include standalone constructor tests, although the `#[standalone_constructors]` attribute is not currently on the enum in this file.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithZeroFieldTuple` with zero-field tuple variants `VariantZeroDefault` and `VariantZeroScalar`.
//! - Applies `#[derive(Former)]` to the enum.
//! - Applies `#[scalar]` to `VariantZeroScalar`.
//! - Includes shared test logic from `tuple_zero_fields_only_test.rs`.
//! - The included tests call the derived static methods (`variant_zero_default`, `variant_zero_scalar`) and standalone constructors (if enabled on the enum) and assert that the returned enum instances match the direct enum variants. This verifies the constructor generation for zero-field tuple variants.

use former::Former;
use test_tools::exposed::*;
use core::fmt::Debug;
use core::marker::PhantomData;

// Helper struct used in tests (inferred from previous manual file)
#[ derive( Debug, PartialEq, Default ) ]
#[ allow( dead_code ) ]
pub struct InnerForSubform
{
  pub value : i32,
}

// The enum under test for zero-field tuple variants with #[derive(Former)]
#[ derive( Debug, PartialEq, Former ) ]
#[former(standalone_constructors, debug)] // Added standalone_constructors and debug
// #[ derive( Default ) ] // Do not derive Default here, it caused issues before.
pub enum EnumWithZeroFieldTuple
{
  VariantZeroDefault, // Default behavior (Rule 3b)
  #[ scalar ]
  VariantZeroScalar, // #[scalar] attribute (Rule 1b)
}

// Include the shared test logic
include!( "./tuple_zero_fields_only_test.rs" );