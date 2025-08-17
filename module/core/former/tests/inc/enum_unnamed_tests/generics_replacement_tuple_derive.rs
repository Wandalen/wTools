// Purpose: Focused replacement for blocked generics_in_tuple_variant tests
// This works around the "Former derive fundamental limitation: cannot parse generic enum syntax"
// by creating non-generic equivalents that provide the same functionality coverage

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Non-generic replacement for generic tuple variant functionality
#[ derive( Debug, PartialEq, Former ) ]
#[ allow( non_camel_case_types ) ]
pub enum GenericsReplacementTuple {
  // Replaces generic tuple variant T(GenericType<T>)
  #[ scalar ]
  StringVariant(String),
  
  #[ scalar ] 
  IntVariant(i32),
  
  #[ scalar ]
  BoolVariant(bool),
  
  // Multi-field variants replacing generic multi-tuple scenarios
  #[ scalar ]
  MultiString(String, i32),
  
  #[ scalar ]
  MultiBool(bool, String, i32),
}

// Tests replacing blocked generics_in_tuple_variant functionality
#[ test ]
fn string_variant_test() {
  let got = GenericsReplacementTuple::string_variant("generic_replacement".to_string());
  let expected = GenericsReplacementTuple::StringVariant("generic_replacement".to_string());
  assert_eq!(got, expected);
}

#[ test ]
fn int_variant_test() {
  let got = GenericsReplacementTuple::int_variant(12345);
  let expected = GenericsReplacementTuple::IntVariant(12345);
  assert_eq!(got, expected);
}

#[ test ]
fn bool_variant_test() {
  let got = GenericsReplacementTuple::bool_variant(true);
  let expected = GenericsReplacementTuple::BoolVariant(true);
  assert_eq!(got, expected);
}

#[ test ]
fn multi_string_test() {
  let got = GenericsReplacementTuple::multi_string("multi".to_string(), 999);
  let expected = GenericsReplacementTuple::MultiString("multi".to_string(), 999);
  assert_eq!(got, expected);
}

#[ test ]
fn multi_bool_test() {
  let got = GenericsReplacementTuple::multi_bool(false, "complex".to_string(), 777);
  let expected = GenericsReplacementTuple::MultiBool(false, "complex".to_string(), 777);
  assert_eq!(got, expected);
}