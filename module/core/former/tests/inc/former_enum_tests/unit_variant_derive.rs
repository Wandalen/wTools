// File: module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs
use super::*;

/// Enum with only unit variants for testing.
#[ derive( Debug, PartialEq, the_module::Former ) ]
enum Status
{
  Pending,
  Complete,
}

// Include the test logic
include!( "unit_variant_only_test.rs" );