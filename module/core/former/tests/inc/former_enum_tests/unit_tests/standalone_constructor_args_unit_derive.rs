// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone, Former, debug ) ] // Added debug attribute
#[ standalone_constructors ] // Enable standalone constructors
pub enum TestEnumArgs // Use the distinct name
{
  /// A unit variant.
  UnitVariantArgs, // Use the distinct name
}

// === Include Test Logic ===
include!( "standalone_constructor_args_unit_only_test.rs" ); // Include the specific test file