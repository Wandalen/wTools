#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]
#[ allow( unused_imports ) ]
use super::*;

/// Parameter description.
#[ allow( explicit_outlives_requirements ) ]
#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ] // disabled
pub struct Child<'child, T: ?Sized + 'child> {
  name: String,
  arg: &'child T,
}