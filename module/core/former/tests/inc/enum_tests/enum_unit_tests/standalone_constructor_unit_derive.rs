// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_unit_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // New attribute is active
pub enum TestEnum // Consistent name
{
  /// A unit variant.
  UnitVariant,
}

// === Include Test Logic ===
include!( "standalone_constructor_unit_only_test.rs" ); // Use the consistent name