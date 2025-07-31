#![allow(dead_code)]
#[allow(unused_imports)]
use super::*;

// Test the simplest case with lifetime only
#[derive(Debug, PartialEq)]
pub struct Basic<'a> {
  val: &'a str,
}

// Manual implementation to test
impl<'a> Basic<'a> {
  pub fn former() -> BasicFormer<'a> {
    BasicFormer { storage: BasicFormerStorage { val: None } }
  }
}

pub struct BasicFormerStorage<'a> {
  val: Option<&'a str>,
}

pub struct BasicFormer<'a> {
  storage: BasicFormerStorage<'a>,
}

impl<'a> BasicFormer<'a> {
  pub fn val(mut self, val: &'a str) -> Self {
    self.storage.val = Some(val);
    self
  }
  
  pub fn form(self) -> Basic<'a> {
    Basic {
      val: self.storage.val.unwrap(),
    }
  }
}

#[test]
fn manual_works() {
  let data = "test";
  let result = Basic::former().val(data).form();
  assert_eq!(result.val, "test");
}