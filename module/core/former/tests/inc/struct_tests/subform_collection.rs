#![allow(dead_code)]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Parent {
  #[ subform_collection( definition = former::VectorDefinition ) ]
  children: Vec<Child>,
}

// == begin of generated

// == end of generated

include!("./only_test/subform_collection.rs");
