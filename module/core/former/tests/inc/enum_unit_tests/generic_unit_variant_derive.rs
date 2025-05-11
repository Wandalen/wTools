//! Derive implementation for testing unit variants in generic enums.

use super::*;
use former_types::EntityToFormer; // Keep for potential internal use by macro

/// Generic enum with a unit variant, using Former.
#[derive(Debug, PartialEq, former::Former)]
#[former(standalone_constructors)]
pub enum GenericOption<T: std::fmt::Debug + PartialEq + Clone> // Revert to minimal bounds
{
  Value(T),
  UnitNone,
}

include!("generic_unit_variant_only_test.rs");