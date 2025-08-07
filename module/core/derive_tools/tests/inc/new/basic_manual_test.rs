//! # Test Matrix for `New` Manual Implementation
//!
//! This matrix outlines the test cases for the manual implementation of `New`.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | N1.1  | Unit        | None   | Should have `new()` constructor               |
//! | N1.2  | Tuple       | 1      | Should have `new()` constructor with one arg  |
//! | N1.3  | Tuple       | >1     | Should have `new()` constructor with multiple args |
//! | N1.4  | Named       | 1      | Should have `new()` constructor with one arg  |
//! | N1.5  | Named       | >1     | Should have `new()` constructor with multiple args |

#![allow(unused_imports)]
#![allow(dead_code)]

use test_tools::prelude::*;

// N1.1: Unit struct
pub struct UnitStruct;

impl UnitStruct {
  pub fn new() -> Self {
    Self {}
  }
}

// N1.2: Tuple struct with one field
pub struct TupleStruct1(pub i32);

impl TupleStruct1 {
  pub fn new(field0: i32) -> Self {
    Self(field0)
  }
}

// N1.3: Tuple struct with multiple fields
pub struct TupleStruct2(pub i32, pub i32);

impl TupleStruct2 {
  pub fn new(field0: i32, field1: i32) -> Self {
    Self(field0, field1)
  }
}

// N1.4: Named struct with one field
pub struct NamedStruct1 {
  pub field1: i32,
}

impl NamedStruct1 {
  pub fn new(field1: i32) -> Self {
    Self { field1 }
  }
}

// N1.5: Named struct with multiple fields
pub struct NamedStruct2 {
  pub field1: i32,
  pub field2: i32,
}

impl NamedStruct2 {
  pub fn new(field1: i32, field2: i32) -> Self {
    Self { field1, field2 }
  }
}

// Shared test logic
include!("../new_only_test.rs");
