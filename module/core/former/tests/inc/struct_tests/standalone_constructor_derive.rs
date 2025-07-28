//!
//! Derive-based tests for standalone constructors for structs.
//! Uses consistent names matching the manual version for testing.
//!

#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Struct Definition: No Args ===

/// Struct using derive for standalone constructors without arguments.
// Attributes to be implemented by the derive macro
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, Default, Clone, Former)]
#[derive(Debug, PartialEq, Default, Clone)]
#[standalone_constructors] // New attribute
pub struct TestStructNoArgs
// Consistent name
{
  /// A simple field.
  pub field1: i32,
}

// === Struct Definition: With Args ===

/// Struct using derive for standalone constructors with arguments.
// Attributes to be implemented by the derive macro
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, Default, Clone, Former)]
#[derive(Debug, PartialEq, Default, Clone)]
#[standalone_constructors] // New attribute
pub struct TestStructWithArgs
// Consistent name
{
  /// Field A (constructor arg - attribute removed for now).
  #[arg_for_constructor] // <<< Uncommented
  pub a: String,
  /// Field B (constructor arg - attribute removed for now).
  #[arg_for_constructor] // <<< Uncommented
  pub b: bool,
  /// Field C (optional, not constructor arg).
  pub c: Option<f32>,
}

// === Include Test Logic ===
include!("standalone_constructor_only_test.rs"); // Include the single test file
