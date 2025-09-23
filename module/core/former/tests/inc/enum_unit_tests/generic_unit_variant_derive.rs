#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Derive implementation for testing unit variants in generic enums.

use super::*;
use former::Former;
// use former_types::{EntityToFormer, FormerDefinition}; // Not needed if Value(T) is scalar

/// Generic enum with a unit variant, using Former.
// Temporarily making this non-generic to test basic functionality
#[ derive( Debug, PartialEq, Former ) ]
#[ former( standalone_constructors ) ] // debug disabled
pub enum GenericOption
{
  #[ scalar ] // Treat Value as a scalar constructor for the enum
  #[ allow( dead_code ) ] // This variant is not constructed by these specific unit tests
  Value(i32),
  NoValue, // Unit variant
}

include!("generic_unit_variant_only_test.rs");
