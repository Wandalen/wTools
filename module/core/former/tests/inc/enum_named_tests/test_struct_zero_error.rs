//! Quick test to verify struct_zero_fields_handler error validation
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub enum TestZeroErrorEnum
{
  // This should cause a compilation error: zero-field struct variants require #[ scalar ]
  ZeroFieldNoScalar {},
}

#[ test ]
fn test_would_fail_to_compile()
{
  // This test should not actually run if the validation works
  // let _got = TestZeroErrorEnum::zero_field_no_scalar();
}