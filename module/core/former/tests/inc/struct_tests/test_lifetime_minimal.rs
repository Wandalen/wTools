#![allow(dead_code)]
use super::*;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, PartialEq, the_module::Former)]

#[derive(Debug, PartialEq, the_module::Former)]
pub struct LifetimeStruct<'a> {
  data: &'a str,
}

#[test]
fn can_construct() {
  let s = "test";
  let instance = LifetimeStruct::former().data(s).form();
  assert_eq!(instance.data, "test");
}