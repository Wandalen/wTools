#![allow(dead_code)]
use super::*;

// Minimal test with single lifetime, no complex bounds
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, the_module::Former)]
#[derive(Debug, PartialEq)]
#[debug]
pub struct SimpleLifetime<'a> {
  data: &'a str,
}

// == begin of generated
// == end of generated