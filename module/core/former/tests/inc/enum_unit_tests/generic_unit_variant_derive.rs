//! Derive implementation for testing unit variants in generic enums.

use super::*;
use former::Former;
// use former_types::{EntityToFormer, FormerDefinition}; // Not needed if Value(T) is scalar

/// Generic enum with a unit variant, using Former.
#[derive(Debug, PartialEq, Former)]
#[former(standalone_constructors, debug)]
pub enum GenericOption<T: core::fmt::Debug + PartialEq + Clone>
// Minimal bounds for T
{
  #[scalar] // Treat Value(T) as a scalar constructor for the enum
  #[allow(dead_code)] // This variant is not constructed by these specific unit tests
  Value(T),
  NoValue, // Unit variant
}

include!("generic_unit_variant_only_test.rs");
