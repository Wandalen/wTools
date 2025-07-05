//! # Test Matrix for `IndexMut` Derive
//!
//! This matrix outlines the test cases for the `IndexMut` derive macro.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | IM1.1 | Unit        | None   | Should not compile (IndexMut requires a field)  |
//! | IM1.2 | Tuple       | 1      | Should derive `IndexMut` from the inner field   |
//! | IM1.3 | Tuple       | >1     | Should not compile (IndexMut requires one field)|
//! | IM1.4 | Named       | 1      | Should derive `IndexMut` from the inner field   |
//! | IM1.5 | Named       | >1     | Should not compile (IndexMut requires one field)|

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;
use core::ops::{ Index, IndexMut };
use derive_tools::IndexMut;

// IM1.1: Unit struct - should not compile
// #[ derive( IndexMut ) ]
// pub struct UnitStruct;

// IM1.2: Tuple struct with one field
#[ derive( IndexMut ) ]
pub struct TupleStruct1( #[ index_mut ] pub i32 );

// IM1.3: Tuple struct with multiple fields - should not compile
// #[ derive( IndexMut ) ]
// pub struct TupleStruct2( pub i32, pub i32 );

// IM1.4: Named struct with one field
#[ derive( IndexMut ) ]
pub struct NamedStruct1
{
  #[ index_mut ]
  pub field1 : i32,
}

// IM1.5: Named struct with multiple fields - should not compile
// #[ derive( IndexMut ) ]
// pub struct NamedStruct2
// {
//   pub field1 : i32,
//   pub field2 : i32,
// }

// Shared test logic
include!( "../index_mut_only_test.rs" );