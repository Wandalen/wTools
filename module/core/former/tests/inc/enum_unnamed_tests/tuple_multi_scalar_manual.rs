//! Purpose: Provides a hand-written implementation of the `Former` pattern's static scalar constructor
//! for a multi-field tuple variant (`Variant(u32, String)`) within an enum, demonstrating the manual
//! implementation corresponding to the behavior when the variant is explicitly marked with the
//! `#[scalar]` attribute.
//!
//! Coverage:
//! - Rule 1f (Tuple + Multi-Field + `#[scalar]`): Manually implements the scalar constructor for a multi-field tuple variant, taking arguments for each field and returning the enum instance.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Provides a hand-written static method `TestEnum::variant(value1, value2)` that takes `u32` and `String` as arguments and returns `TestEnum::Variant(value1, value2)`. This mimics the behavior expected when `#[scalar]` is applied.
//! - Includes shared test logic from `tuple_multi_scalar_only_test.rs`.
//! - The included test calls this manually implemented static method and asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies the manual implementation of the scalar constructor for a multi-field tuple variant when `#[scalar]` is intended.

// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_manual.rs

// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum TestEnum
{
  Variant( u32, String ),
}

// Manually implement the static method for the variant, mimicking #[scalar] behavior
impl TestEnum
{
  /// Manually implemented constructor for the Variant variant (scalar style, mimicking #[scalar]).
  #[ inline( always ) ]
  pub fn variant( value1 : u32, value2 : String ) -> Self
  {
    Self::Variant( value1, value2 )
  }
}

include!( "tuple_multi_scalar_only_test.rs" );