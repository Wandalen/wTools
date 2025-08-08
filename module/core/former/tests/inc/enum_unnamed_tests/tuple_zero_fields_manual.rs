//! Purpose: Provides a hand-written implementation of the `Former` pattern's static constructors
//! for zero-field tuple variants, demonstrating the manual implementation corresponding to both
//! default behavior and the effect of the `#[ scalar ]` attribute.
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Manually implements the static method `EnumWithZeroFieldTuple::variant_zero_default()` to return the enum instance.
//! - Rule 1b (Tuple + Zero-Field + `#[ scalar ]`): Manually implements the static method `EnumWithZeroFieldTuple::variant_zero_scalar()` to return the enum instance.
//! - Rule 4a (`#[ standalone_constructors ]`): Manually implements standalone constructor functions (`enum_with_zero_field_tuple_variant_zero_default`, `enum_with_zero_field_tuple_variant_zero_scalar`) to return the enum instance, corresponding to the tests in `_only_test.rs`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithZeroFieldTuple` with zero-field tuple variants `VariantZeroDefault` and `VariantZeroScalar`.
//! - Provides hand-written static methods (`variant_zero_default`, `variant_zero_scalar`) and standalone functions (`enum_with_zero_field_tuple_variant_zero_default`, `enum_with_zero_field_tuple_variant_zero_scalar`) that mimic the behavior expected from the `#[ derive( Former ) ]` macro for zero-field tuple variants.
//! - Includes shared test logic from `tuple_zero_fields_only_test.rs`.
//! - The included tests call these manually implemented methods/functions and assert that the returned enum instances match the direct enum variants. This verifies the manual implementation of constructors for zero-field tuple variants.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use test_tools::exposed::*;
use core::fmt::Debug;
use core::marker::PhantomData;

// Helper struct used in tests (though not directly by this enum's variants)
#[ derive( Debug, PartialEq, Default ) ]
#[ allow( dead_code ) ]
pub struct InnerForSubform {
  pub value: i32,
}

// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithZeroFieldTuple {
  VariantZeroDefault(), // Zero-field tuple variant
  VariantZeroScalar(),  // Conceptually, this is the one that would have #[ scalar ] in derive
}

impl EnumWithZeroFieldTuple {
  #[ inline( always ) ]
  pub fn variant_zero_default() -> Self {
    Self::VariantZeroDefault()
  }

  #[ inline( always ) ]
  pub fn variant_zero_scalar() -> Self {
    // Manual equivalent of scalar behavior
    Self::VariantZeroScalar()
  }
}

// Standalone constructors (matching derive macro output)
#[ inline( always ) ]
#[ allow( dead_code ) ] // Suppress unused warning for demonstration function
pub fn variant_zero_default() -> EnumWithZeroFieldTuple {
  // Name matches derive output
  EnumWithZeroFieldTuple::VariantZeroDefault()
}

#[ inline( always ) ]
#[ allow( dead_code ) ] // Suppress unused warning for demonstration function
pub fn variant_zero_scalar() -> EnumWithZeroFieldTuple {
  // Name matches derive output
  EnumWithZeroFieldTuple::VariantZeroScalar()
}

// Include the shared test logic
include!("./tuple_zero_fields_only_test.rs");
