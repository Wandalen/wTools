#![allow(dead_code)]

use super::*;

/// Parameter description.
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, Default, PartialEq, the_module::Former)]
#[derive(Debug, Default, PartialEq)]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent required for the template.
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, Default, PartialEq, the_module::Former)]
#[derive(Debug, Default, PartialEq)]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  #[ subform_collection( definition = former::VectorDefinition ) ]
  children: Vec<Child>,
}

// == begin of generated

// == end of generated

include!("./only_test/subform_collection.rs");
