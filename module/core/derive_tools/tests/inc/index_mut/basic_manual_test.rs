//! # Test Matrix for `IndexMut` Manual Implementation
//!
//! This matrix outlines the test cases for the manual implementation of `IndexMut`.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | IM1.1 | Unit        | None   | Should not compile (IndexMut requires a field)  |
//! | IM1.2 | Tuple       | 1      | Should implement `IndexMut` from the inner field   |
//! | IM1.3 | Tuple       | >1     | Should not compile (IndexMut requires one field)|
//! | IM1.4 | Named       | 1      | Should implement `IndexMut` from the inner field   |
//! | IM1.5 | Named       | >1     | Should not compile (IndexMut requires one field)|

#![allow(unused_imports)]
#![allow(dead_code)]

use test_tools::prelude::*;
use core::ops::IndexMut as _;
use core::ops::Index as _;

// IM1.1: Unit struct - should not compile
// pub struct UnitStruct;

// IM1.2: Tuple struct with one field
pub struct TupleStruct1(pub i32);

impl core::ops::Index<usize> for TupleStruct1 {
  type Output = i32;
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.0,
      _ => panic!("Index out of bounds"),
    }
  }
}

impl core::ops::IndexMut<usize> for TupleStruct1 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.0,
      _ => panic!("Index out of bounds"),
    }
  }
}

// IM1.3: Tuple struct with multiple fields - should not compile
// pub struct TupleStruct2( pub i32, pub i32 );

// IM1.4: Named struct with one field
pub struct NamedStruct1 {
  pub field1: i32,
}

impl core::ops::Index<&str> for NamedStruct1 {
  type Output = i32;
  fn index(&self, index: &str) -> &Self::Output {
    match index {
      "field1" => &self.field1,
      _ => panic!("Field not found"),
    }
  }
}

impl core::ops::IndexMut<&str> for NamedStruct1 {
  fn index_mut(&mut self, index: &str) -> &mut Self::Output {
    match index {
      "field1" => &mut self.field1,
      _ => panic!("Field not found"),
    }
  }
}

// IM1.5: Named struct with multiple fields - should not compile
// pub struct NamedStruct2
// {
//   pub field1 : i32,
//   pub field2 : i32,
// }

// Shared test logic
include!("../index_mut_only_test.rs");
