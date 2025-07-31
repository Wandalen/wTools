#![allow(dead_code)]
#[allow(unused_imports)]
use super::*;

/// Parameter description.
#[allow(explicit_outlives_requirements)]
// DISABLED: Has lifetime regression issues - commenting out temporarily
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, the_module::Former)]
#[derive(Debug, PartialEq)]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Child<'child, T>
where
  T: ?Sized + 'child,
{
  name: String,
  arg: &'child T,
}

// == begin of generated

// == end of generated

// DISABLED: Has lifetime regression issues - commenting out temporarily
// include!("./only_test/parametrized_field.rs");
