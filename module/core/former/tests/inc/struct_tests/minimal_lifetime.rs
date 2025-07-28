#![allow(dead_code)]
use super::*;

#[derive(Debug, PartialEq, the_module::Former)]
#[debug]
pub struct Minimal<'a> {
  value: &'a str,
}

#[test]
fn basic() {
  let data = "test";
  let _instance = Minimal { value: data };
}