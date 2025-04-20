// // module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs
// use super::*; // Imports testing infrastructure and potentially other common items
// use std::fmt::Debug; // Import Debug trait for bounds
//
// // --- Inner Struct Definition with Bounds ---
// // Needs to derive Former for the enum's derive to work correctly for subforming.
// #[derive(Debug, PartialEq, Clone, Copy, former::Former)] // Added Former derive
// pub struct InnerGeneric< T : Debug + Copy > // Added Copy bound here too
// {
//   pub inner_field : T,
// }
//
// // --- Enum Definition with Bounds ---
// // Apply Former derive here. This is what we are testing.
// #[derive(Debug, PartialEq, former::Former)]
// // #[derive(Debug, PartialEq)]
// #[debug]
// pub enum EnumOuter< X : Copy > // Enum bound: Copy
// {
//   Variant( InnerGeneric< X > ), // Inner type uses X, which must satisfy InnerGeneric's bounds (Debug + Copy)
//   OtherVariant,
// }
//
// // --- Include the Test Logic ---
// // This file contains the actual #[ test ] functions.
// include!( "generics_in_tuple_variant_only_test.rs" );