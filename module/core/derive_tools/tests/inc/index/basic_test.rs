//! # Test Matrix for `Index` Derive
//!
//! This matrix outlines the test cases for the `Index` derive macro.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | I1.1  | Unit        | None   | Should not compile (Index requires a field)     |
//! | I1.2  | Tuple       | 1      | Should derive `Index` from the inner field      |
//! | I1.3  | Tuple       | >1     | Should not compile (Index requires one field)   |
//! | I1.4  | Named       | 1      | Should derive `Index` from the inner field      |
//! | I1.5  | Named       | >1     | Should not compile (Index requires one field)   |

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;
use the_module::Index;
use core::ops::Index as _;

// I1.1: Unit struct - should not compile
// #[ derive( Index ) ]
// pub struct UnitStruct;

// I1.2: Tuple struct with one field
#[ derive( Index ) ]
pub struct TupleStruct1( pub i32 );

// I1.3: Tuple struct with multiple fields - should not compile
// #[ derive( Index ) ]
// pub struct TupleStruct2( pub i32, pub i32 );

// I1.4: Named struct with one field
#[ derive( Index ) ]
pub struct NamedStruct1
{
  pub field1 : i32,
}

// I1.5: Named struct with multiple fields - should not compile
// #[ derive( Index ) ]
// pub struct NamedStruct2
// {
//   pub field1 : i32,
//   pub field2 : i32,
// }

// Shared test logic
include!( "../index_only_test.rs" );