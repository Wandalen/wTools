// // File: module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs
// use super::*; // Imports testing infrastructure and potentially other common items
//
// // --- Dummy Bounds and Concrete Types ---
// // Are defined in the included _only_test.rs file
//
// // --- Inner Struct Definition ---
// // Also defined in the included _only_test.rs file,
// // but conceptually needed here for the enum definition.
// // #[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
// // pub struct InnerG6< U : BoundB > { pub inner_field : U }
//
// // --- Enum Definition with Bounds ---
// // Apply Former derive here. This is what we are testing.
// #[ derive( Debug, PartialEq, Clone, former::Former ) ]
// // #[ debug ] // Uncomment to see generated code later
// pub enum EnumG6< T : BoundA > // BoundA required by enum
// {
//   V1 // Struct-like variant
//   {
//     // Field holding the inner struct instantiated with a *concrete* type
//     inner : InnerG6< TypeForU >, // TypeForU satisfies BoundB implicitly via _only_test.rs
//     // A non-generic field for testing
//     flag : bool,
//   },
// }
//
// // --- Include the Test Logic ---
// // This file contains the actual #[test] functions.
// include!( "generics_independent_struct_only_test.rs" );