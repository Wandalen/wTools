#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;

#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1<'a> {
  pub string_slice_1: &'a str,
}

// === begin_coercing of generated

// === end of generated

include!("./only_test/string_slice.rs");
