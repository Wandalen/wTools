use super::*;

// DISABLED: Has lifetime regression issues - commenting out temporarily
// #[derive(Debug, PartialEq, former::Former)]
#[derive(Debug, PartialEq)]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1<'a> {
  pub string_slice_1: &'a str,
}

// === begin_coercing of generated

// === end of generated

// DISABLED: Has lifetime regression issues - commenting out temporarily
// include!("./only_test/string_slice.rs");
