#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![deny(missing_docs)]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;

// Test re-enabled to verify proper fix
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct Struct1 {
  pub int_1: i32,
}

// Test with a struct that has lifetime parameters
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct TestLifetime<'a> {
  value: &'a str,
}

// == begin of generated

// == end of generated

include!("./only_test/basic.rs");
