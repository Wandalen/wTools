#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![deny(missing_docs)]
#![allow(dead_code)]

use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Parent {
  // #[ subform_collection( definition = former::VectorDefinition ) ]
  #[ subform_collection ]
  children: Vec<Child>,
}

// == begin of generated

// == end of generated

include!("./only_test/subform_collection.rs");
