// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_tuple_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // New attribute is active
pub enum TestEnum // Consistent name
{
  /// A tuple variant with one field.
  TupleVariant // Defaults to subformer behavior
  (
    // #[ arg_for_constructor ] // <<< Keep commented out for this increment
    i32
  ),
}

// === Include Test Logic ===
include!( "standalone_constructor_tuple_only_test.rs" ); // Use the consistent name