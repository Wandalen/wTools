#![allow(unexpected_cfgs)]

use super::*;

#[derive(Debug, PartialEq)]
pub struct BaseCase {
  #[cfg(feature = "enabled")]
  enabled: i32,
  #[cfg(feature = "disabled")]
  disabled: i32,
}

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, PartialEq, former::Former)]

#[derive(Debug, PartialEq)]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Foo {
  #[cfg(feature = "enabled")]
  #[allow(dead_code)]
  enabled: i32,
  #[cfg(feature = "disabled")]
  disabled: i32,
}

// == begin of generated

// == end of generated

#[test]
fn basecase() {
  let got = BaseCase { enabled: 13 };
  let exp = BaseCase { enabled: 13 };
  a_id!(got, exp);
}

#[test]
fn basic() {
  let got = Foo::former().enabled(13).form();
  let exp = Foo { enabled: 13 };
  a_id!(got, exp);
}
