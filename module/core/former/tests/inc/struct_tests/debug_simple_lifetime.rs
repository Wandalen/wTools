#![allow(dead_code)]
use super::*;

// Minimal test with single lifetime, no complex bounds
#[derive(Debug, PartialEq, the_module::Former)]
#[debug]
pub struct SimpleLifetime<'a> {
  data: &'a str,
}

// == begin of generated
// == end of generated