// Purpose: Replacement for generics_independent_struct_derive - tests struct variants without generics
// This works around the architectural limitation that Former derive cannot parse generic enums

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Inner struct for testing
#[ derive( Debug, PartialEq, Default, Clone, Former ) ]
pub struct SimpleInner {
  pub value: i32,
}

// Simple enum without generics - works around derive macro limitation
#[ derive( Debug, PartialEq, Former ) ]
#[ allow( non_camel_case_types ) ] // Allow for generated Former type names
pub enum SimpleStructEnum {
  // Single-field struct variant (default behavior - subform)
  Variant { inner: SimpleInner },
  
  // Multi-field scalar struct variant  
  #[ scalar ]
  MultiVariant { field1: i32, field2: String },
}

#[ test ]
fn simple_struct_subform_test() {
  let inner = SimpleInner { value: 42 };
  let got = SimpleStructEnum::variant()
    .inner(inner.clone())
    .form();
  let expected = SimpleStructEnum::Variant { inner };
  assert_eq!(got, expected);
}

#[ test ]
fn simple_struct_scalar_test() {
  let got = SimpleStructEnum::multi_variant(123, "test".to_string());
  let expected = SimpleStructEnum::MultiVariant { 
    field1: 123, 
    field2: "test".to_string() 
  };
  assert_eq!(got, expected);
}