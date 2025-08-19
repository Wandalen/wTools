//! # Test Matrix for `Index` Manual Implementation
//!
//! This matrix outlines the test cases for the manual implementation of `Index`.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | I1.1  | Unit        | None   | Should not compile (Index requires a field)     |
//! | I1.2  | Tuple       | 1      | Should implement `Index` from the inner field      |
//! | I1.3  | Tuple       | >1     | Should not compile (Index requires one field)   |
//! | I1.4  | Named       | 1      | Should implement `Index` from the inner field      |
//! | I1.5  | Named       | >1     | Should not compile (Index requires one field)   |

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::*;
use core::ops::Index as _;

// I1.1: Unit struct - should not compile
// pub struct UnitStruct;

// I1.2: Tuple struct with one field
pub struct TupleStruct1( pub i32 );

impl core::ops::Index< usize > for TupleStruct1
{
  type Output = i32;
  fn index( &self, index : usize ) -> &Self::Output
  {
    match index
    {
      0 => &self.0,
      _ => panic!( "Index out of bounds" ),
    }
  }
}

// I1.3: Tuple struct with multiple fields - should not compile
// pub struct TupleStruct2( pub i32, pub i32 );

// I1.4: Named struct with one field
pub struct NamedStruct1
{
  pub field1 : i32,
}

impl core::ops::Index< &str > for NamedStruct1
{
  type Output = i32;
  fn index( &self, index : &str ) -> &Self::Output
  {
    match index
    {
      "field1" => &self.field1,
      _ => panic!( "Field not found" ),
    }
  }
}

// I1.5: Named struct with multiple fields - should not compile
// pub struct NamedStruct2
// {
//   pub field1 : i32,
//   pub field2 : i32,
// }

// Shared test logic
include!( "../index_only_test.rs" );
