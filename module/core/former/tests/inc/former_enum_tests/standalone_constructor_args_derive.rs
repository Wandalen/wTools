// module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs
//!
//! Derive-based tests for standalone constructors for enums with arguments.
//! Uses distinct names matching the manual version for testing.
//!

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // Enable standalone constructors
pub enum TestEnumArgs // Use the distinct name
{
  /// A unit variant.
  UnitVariantArgs, // Use the distinct name
  /// A tuple variant with one field marked as constructor arg.
  TupleVariantArgs // Use the distinct name
  (
    #[ arg_for_constructor ] // Mark field as constructor arg
    i32
  ),
  /// A struct variant with one field marked as constructor arg.
  StructVariantArgs // Use the distinct name
  {
    #[ arg_for_constructor ] // Mark field as constructor arg
    field : String,
  },
}

// === Include Test Logic ===
include!( "standalone_constructor_args_only_test.rs" ); // Include the specific test file