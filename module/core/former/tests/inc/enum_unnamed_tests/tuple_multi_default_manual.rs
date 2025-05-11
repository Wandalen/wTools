//! Purpose: Provides a hand-written implementation of the `Former` pattern's static scalar constructor
//! for a multi-field tuple variant (`Variant(u32, String)`) within an enum, demonstrating the manual
//! implementation corresponding to the default behavior when no specific variant attribute is applied.
//!
//! Coverage:
//! - Rule 3f (Tuple + Multi-Field + Default): Manually implements the scalar constructor for a multi-field tuple variant, taking arguments for each field and returning the enum instance.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Provides a hand-written static method `TestEnum::variant(value1, value2)` that takes `u32` and `String` as arguments and returns `TestEnum::Variant(value1, value2)`.
//! - Includes shared test logic from `tuple_multi_default_only_test.rs`.
//! - The included test calls this manually implemented static method and asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies the manual implementation of the default scalar constructor for a multi-field tuple variant.

// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_default_manual.rs

// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum TestEnum
{
  Variant( u32, String ),
}

// Manually implement the static method for the variant
impl TestEnum
{
  /// Manually implemented constructor for the Variant variant (scalar style).
  #[ inline( always ) ]
  pub fn variant( value1 : u32, value2 : String ) -> Self
  {
    Self::Variant( value1, value2 )
  }
}

include!( "tuple_multi_default_only_test.rs" );