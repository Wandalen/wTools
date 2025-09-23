#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// File: module/core/former/tests/inc/former_tests/keyword_field_derive.rs
use super::*;

#[ derive( Debug, PartialEq, Default, the_module::Former ) ]
pub struct KeywordFieldsStruct {
  r#if: bool,
  r#type: String,
  r#struct: i32,
}

include!("keyword_field_only_test.rs");
