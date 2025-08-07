//! # Test Matrix for `InnerFrom` Manual Implementation
//!
//! This matrix outlines the test cases for the manual implementation of `InnerFrom`.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | IF1.1 | Unit        | None   | Should not compile (InnerFrom requires a field) |
//! | IF1.2 | Tuple       | 1      | Should implement `InnerFrom` from the inner field  |
//! | IF1.3 | Tuple       | >1     | Should not compile (InnerFrom requires one field) |
//! | IF1.4 | Named       | 1      | Should implement `InnerFrom` from the inner field  |
//! | IF1.5 | Named       | >1     | Should not compile (InnerFrom requires one field) |

#![allow(unused_imports)]
#![allow(dead_code)]

use test_tools::prelude::*;

// IF1.1: Unit struct - should not compile
// pub struct UnitStruct;

// IF1.2: Tuple struct with one field
pub struct TupleStruct1(pub i32);

impl From<i32> for TupleStruct1 {
  fn from(src: i32) -> Self {
    Self(src)
  }
}

// IF1.3: Tuple struct with multiple fields - should not compile
// pub struct TupleStruct2( pub i32, pub i32 );

// IF1.4: Named struct with one field
pub struct NamedStruct1 {
  pub field1: i32,
}

impl From<i32> for NamedStruct1 {
  fn from(src: i32) -> Self {
    Self { field1: src }
  }
}

// IF1.5: Named struct with multiple fields - should not compile
// pub struct NamedStruct2
// {
//   pub field1 : i32,
//   pub field2 : i32,
// }

// Shared test logic
include!("../inner_from_only_test.rs");
