// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone, Former, debug ) ] // Added debug attribute
#[ standalone_constructors ] // Enable standalone constructors
pub enum TestEnumArgs // Use the distinct name
{
  /// A struct variant with one field marked as constructor arg.
  StructVariantArgs // Use the distinct name
  {
    #[ arg_for_constructor ] // Mark field as constructor arg
    field : String,
  },
  /// A struct variant with multiple fields marked as constructor args.
  // #[ scalar ] // <<< Keep scalar attribute
  MultiStructArgs // Use the distinct name
  {
    #[ arg_for_constructor ]
    a : i32,
    #[ arg_for_constructor ]
    b : bool,
  },
}

// === Include Test Logic ===
include!( "standalone_constructor_args_named_only_test.rs" ); // Include the specific test file