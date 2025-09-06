#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]

use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;

/// Child
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  #[ subform_scalar ]
  child: Child,
}

// == begin of generated

// == end of generated

include!("./only_test/subform_scalar.rs");
