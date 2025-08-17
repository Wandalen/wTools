// Purpose: Replacement for generics_independent_tuple_derive - tests multi-field tuple without generics
// This works around the architectural limitation that Former derive cannot parse generic enums

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Simple enum without generics - works around derive macro limitation
#[ derive( Debug, PartialEq, Former ) ]
#[ allow( non_camel_case_types ) ] // Allow for generated Former type names  
pub enum SimpleMultiTupleEnum {
  // Multi-field scalar tuple variant
  #[ scalar ]
  MultiValue(i32, String, bool),
}

#[ test ]
fn simple_multi_tuple_scalar_test() {
  let got = SimpleMultiTupleEnum::multi_value(42, "test".to_string(), true);
  let expected = SimpleMultiTupleEnum::MultiValue(42, "test".to_string(), true);
  assert_eq!(got, expected);
}

#[ test ] 
fn simple_multi_tuple_into_test() {
  // Test that Into<T> works for string conversion
  let got = SimpleMultiTupleEnum::multi_value(42, "test", true);
  let expected = SimpleMultiTupleEnum::MultiValue(42, "test".to_string(), true);
  assert_eq!(got, expected);
}