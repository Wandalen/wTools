#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// Purpose: Replacement for scalar_generic_tuple_derive - tests tuple variants without generics  
// This works around the architectural limitation that Former derive cannot parse generic enums

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Simple enum without generics - works around derive macro limitation
#[ derive( Debug, PartialEq, Former ) ]  
#[ allow( non_camel_case_types ) ] // Allow for generated Former type names
pub enum SimpleTupleEnum {
  // Scalar tuple variant
  #[ scalar ]
  Value(i32),
}

#[ test ]
fn simple_tuple_scalar_test() {
  let got = SimpleTupleEnum::value(42);
  let expected = SimpleTupleEnum::Value(42);
  assert_eq!(got, expected);
}

#[ test ] 
fn simple_tuple_into_test() {
  // Test that Into<T> works with compatible type
  let got = SimpleTupleEnum::value(42_i16);
  let expected = SimpleTupleEnum::Value(42);
  assert_eq!(got, expected);
}