// // File: module/core/former/tests/inc/former_enum_tests/multi_field_derive.rs
// #![ allow( dead_code ) ] // Allow unused variants/methods during macro phase
// use super::*; // Assuming it's in a module within `former_enum_tests`
//
// /// Enum with different variant types for testing.
// /// NOTE: Uses the derive macro here!
// #[ derive( Debug, PartialEq, the_module::Former ) ]
// enum EnumWithMultiField
// {
//   /// A simple variant with one field (would use standard Former logic).
//   Simple( String ),
//   /// A variant with multiple unnamed fields.
//   MultiTuple( i32, String, bool ),
//   /// A variant with no fields.
//   Empty,
// }
//
// // Include the actual test logic from the separate file
// include!( "multi_field_only_test.rs" ); // Include the same test logic