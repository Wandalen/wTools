use former::Former;
use test_tools::exposed::*;
use core::fmt::Debug;
use core::marker::PhantomData;

// Helper struct used in tests (inferred from previous manual file)
#[ derive( Debug, PartialEq, Default ) ]
pub struct InnerForSubform
{
  pub value : i32,
}

// The enum under test for zero-field tuple variants with #[derive(Former)]
#[ derive( Debug, PartialEq, Former ) ]
// #[ derive( Default ) ] // Do not derive Default here, it caused issues before.
pub enum EnumWithZeroFieldTuple
{
  VariantZeroDefault, // Default behavior (Rule 3b)
  #[ scalar ]
  VariantZeroScalar, // #[scalar] attribute (Rule 1b)
}

// Include the shared test logic
include!( "./tuple_zero_fields_only_test.rs" );