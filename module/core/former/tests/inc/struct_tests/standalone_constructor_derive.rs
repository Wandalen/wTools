//!
//! Derive-based tests for standalone constructors for structs.
//! Uses consistent names matching the manual version for testing.
//!

#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Struct Definition: No Args ===

/// Struct using derive for standalone constructors without arguments.
// All fields are constructor args, so constructor returns Self directly
#[derive(Debug, PartialEq, Default, Clone, Former)]
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
#[derive(Debug, PartialEq, Default, Clone, Former)]
#[standalone_constructors] // New attribute
pub struct TestStructWithArgs
// Consistent name
{
  /// Field A (constructor arg - no attribute needed).
  pub a: String,
  /// Field B (constructor arg - no attribute needed).
  pub b: bool,
  /// Field C (optional, not constructor arg).
  #[former_ignore] // <<< New attribute with inverted logic
  pub c: Option<f32>,
}

// === Include Test Logic ===
include!("standalone_constructor_only_test.rs"); // Include the single test file
