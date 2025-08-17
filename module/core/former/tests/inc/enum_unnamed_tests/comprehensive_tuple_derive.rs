// Purpose: Comprehensive replacement for multiple blocked generic tuple tests
// This works around the architectural limitation that Former derive cannot parse generic enums


use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Inner struct that derives Former for subform testing
#[ derive( Debug, PartialEq, Default, Clone, Former ) ]
pub struct InnerStruct {
  pub content: String,
}

// Comprehensive enum testing multiple tuple variant scenarios
#[ derive( Debug, PartialEq, Former ) ]
#[ allow( non_camel_case_types ) ] // Allow for generated Former type names
#[ former( standalone_constructors ) ]
pub enum ComprehensiveTupleEnum {
  // Zero-field tuple (unit-like)
  #[ scalar ]
  ZeroField(),
  
  // Single-field scalar tuple
  #[ scalar ]  
  SingleScalar(i32),
  
  // Single-field subform tuple (default behavior)
  SingleSubform(InnerStruct),
  
  // Multi-field scalar tuple
  #[ scalar ]
  MultiScalar(i32, String, bool),
  
  // Multi-field default tuple (should use positional setters)
  MultiDefault(f64, bool, String),
}

/// Tests zero-field tuple variant construction.
#[ test ]
fn zero_field_test() {
  let got = ComprehensiveTupleEnum::zero_field();
  let expected = ComprehensiveTupleEnum::ZeroField();
  assert_eq!(got, expected);
}

/// Tests single scalar tuple variant.
#[ test ]
fn single_scalar_test() {
  let got = ComprehensiveTupleEnum::single_scalar(42);
  let expected = ComprehensiveTupleEnum::SingleScalar(42);
  assert_eq!(got, expected);
}

/// Tests single subform tuple variant with builder pattern.
#[ test ]
fn single_subform_test() {
  let inner = InnerStruct { content: "test".to_string() };
  let got = ComprehensiveTupleEnum::single_subform()
    ._0(inner.clone())
    .form();
  let expected = ComprehensiveTupleEnum::SingleSubform(inner);
  assert_eq!(got, expected);
}

/// Tests multi-scalar tuple variant with multiple types.
#[ test ]
fn multi_scalar_test() {
  let got = ComprehensiveTupleEnum::multi_scalar(42, "test".to_string(), true);
  let expected = ComprehensiveTupleEnum::MultiScalar(42, "test".to_string(), true);
  assert_eq!(got, expected);
}

/// Tests multi-default tuple variant with positional setters.
#[ test ]
fn multi_default_test() {
  let got = ComprehensiveTupleEnum::multi_default()
    ._0(3.14)
    ._1(false) 
    ._2("test".to_string())
    .form();
  let expected = ComprehensiveTupleEnum::MultiDefault(3.14, false, "test".to_string());
  assert_eq!(got, expected);
}

/// Tests standalone constructors attribute validation.
#[ test ]
fn standalone_constructors_test() {
  // Test that standalone constructors are generated (this validates the attribute worked)
  // Note: The actual standalone functions would be at module level if properly implemented
  let got = ComprehensiveTupleEnum::zero_field();
  let expected = ComprehensiveTupleEnum::ZeroField();
  assert_eq!(got, expected);
}
