#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Derive implementation for testing unit variants in enums with mixed variant kinds.

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

/// Enum with a unit variant and a struct-like variant, using Former.
#[ derive( Debug, PartialEq, Former ) ]
#[ former( standalone_constructors ) ] // Enable standalone constructors
pub enum MixedEnum {
  SimpleUnit,
  #[ allow( dead_code ) ] // This variant is not constructed by these specific unit tests
  Complex {
    data: i32,
  }, // Complex variant present
}

include!("mixed_enum_unit_only_test.rs");
