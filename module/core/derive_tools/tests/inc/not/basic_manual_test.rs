//! # Test Matrix for `Not` Manual Implementation
//!
//! This matrix outlines the test cases for the manual implementation of `Not`.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | N1.1  | Unit        | None   | Should implement `Not` for unit structs            |
//! | N1.2  | Tuple       | 1      | Should implement `Not` for tuple structs with one field |
//! | N1.3  | Tuple       | >1     | Should not compile (Not requires one field)     |
//! | N1.4  | Named       | 1      | Should implement `Not` for named structs with one field |
//! | N1.5  | Named       | >1     | Should not compile (Not requires one field)     |

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;

// N1.1: Unit struct
pub struct UnitStruct;

impl core::ops::Not for UnitStruct
{
  type Output = Self;
  fn not( self ) -> Self::Output
  {
    self
  }
}

// N1.2: Tuple struct with one field
pub struct TupleStruct1( pub bool );

impl core::ops::Not for TupleStruct1
{
  type Output = Self;
  fn not( self ) -> Self::Output
  {
    Self( !self.0 )
  }
}

// N1.3: Tuple struct with multiple fields - should not compile
// pub struct TupleStruct2( pub bool, pub bool );

// N1.4: Named struct with one field
pub struct NamedStruct1
{
  pub field1 : bool,
}

impl core::ops::Not for NamedStruct1
{
  type Output = Self;
  fn not( self ) -> Self::Output
  {
    Self { field1 : !self.field1 }
  }
}

// N1.5: Named struct with multiple fields - should not compile
// pub struct NamedStruct2
// {
//   pub field1 : bool,
//   pub field2 : bool,
// }

// Shared test logic
include!( "../not_only_test.rs" );
