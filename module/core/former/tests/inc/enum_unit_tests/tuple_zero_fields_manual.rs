#[ allow( unused_imports ) ]
use ::former::prelude::*;
use test_tools::exposed::*;
use core::fmt::Debug;
use core::marker::PhantomData;

// Helper struct used in tests
#[ derive( Debug, PartialEq, Default ) ]
pub struct InnerForSubform
{
  pub value : i32,
}

// qqq : ... implement ...

// Include the shared test logic
include!( "./tuple_zero_fields_only_test.rs" );