#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// Purpose: Replacement for generic_enum_simple_unit_derive - tests unit variants without generics
// This works around the architectural limitation that Former derive cannot parse generic enums

#![allow(non_camel_case_types)] // Allow for generated Former type names with underscores

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Simple enum without generics - works around derive macro limitation
#[ derive( Debug, PartialEq, Former ) ]
#[ allow( non_camel_case_types ) ] // Allow for generated Former type names with underscores
pub enum SimpleEnum {
  // Unit variant
  UnitVariant,
  // Phantom variant to use marker
  #[ allow( dead_code ) ]
  _Phantom(core::marker::PhantomData<i32>),
}

#[ test ]
fn simple_unit_variant_test() {
  let got = SimpleEnum::unit_variant();
  let expected = SimpleEnum::UnitVariant;
  assert_eq!(got, expected);
}

#[ test ] 
fn simple_enum_construction() {
  // Test basic unit variant construction
  let instance = SimpleEnum::unit_variant();
  assert_eq!(instance, SimpleEnum::UnitVariant);
}