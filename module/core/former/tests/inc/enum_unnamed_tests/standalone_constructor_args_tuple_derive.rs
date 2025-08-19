#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_args_tuple_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone, Former ) ] // Removed debug attribute
#[ standalone_constructors ] // Enable standalone constructors
pub enum TestEnumArgs // Use the distinct name
{
  /// A tuple variant with one field marked as constructor arg.
  TupleVariantArgs // Use the distinct name
  (
    #[ arg_for_constructor ] // Mark field as constructor arg
    i32
  ),
  /// A tuple variant with multiple fields marked as constructor args.
  #[ scalar ] // <<< Keep scalar attribute
  MultiTupleArgs // Use the distinct name
  (
    // #[ arg_for_constructor ] // <<< REMOVED
    i32,
    // #[ arg_for_constructor ] // <<< REMOVED
    bool,
  ),
}

// === Include Test Logic ===
include!( "standalone_constructor_args_tuple_only_test.rs" ); // Include the specific test file