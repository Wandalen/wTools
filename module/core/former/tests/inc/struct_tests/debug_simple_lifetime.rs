#![allow(dead_code)]
use super::*;

// Minimal test with single lifetime, no complex bounds
#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ] // Commented out - debug attribute only for temporary debugging
pub struct SimpleLifetime<'a> {
  data: &'a str,
}

// == begin of generated
// == end of generated