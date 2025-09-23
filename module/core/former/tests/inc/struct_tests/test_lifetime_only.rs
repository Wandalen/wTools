#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]
#[ allow( unused_imports ) ]
use super::*;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[ derive( Debug, PartialEq, the_module::Former ) ]

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ] // Commented out - debug attribute only for temporary debugging
pub struct WithLifetime<'a> {
  name: &'a str,
}

// == begin of generated (expected)
// This is what we expect the macro to generate

// Storage struct
// pub struct WithLifetimeFormerStorage<'a> {
//   pub name: ::core::option::Option<&'a str>,
// }

// == end of generated

#[ test ]
fn basic() {
  let data = "test";
  let instance = WithLifetime::former().name(data).form();
  assert_eq!(instance.name, "test");
}