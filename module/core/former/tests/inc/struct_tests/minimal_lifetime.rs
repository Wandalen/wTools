#![allow(dead_code)]
use super::*;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, PartialEq, the_module::Former)]

#[derive(Debug, PartialEq, the_module::Former)]
pub struct Minimal<'a> {
  value: &'a str,
}

#[test]
fn basic() {
  let data = "test";
  let instance = Minimal::former().value(data).form();
  assert_eq!(instance.value, "test");
}