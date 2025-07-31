#![allow(dead_code)]
#[allow(unused_imports)]
use super::*;

/// Parameter description.
#[allow(explicit_outlives_requirements)]
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, the_module::Former)]
#[derive(Debug, PartialEq)]
#[ debug ]
pub struct Child<'child, T: ?Sized + 'child> {
  name: String,
  arg: &'child T,
}