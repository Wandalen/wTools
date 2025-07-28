#![allow(dead_code)]

use super::*;

/// Child
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, Default, PartialEq, the_module::Former)]
#[derive(Debug, Default, PartialEq)]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, Default, PartialEq, the_module::Former)]

#[derive(Debug, Default, PartialEq)]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  #[subform_scalar]
  child: Child,
}

// == begin of generated

// == end of generated

include!("./only_test/subform_scalar.rs");
