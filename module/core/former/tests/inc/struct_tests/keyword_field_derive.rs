// File: module/core/former/tests/inc/former_tests/keyword_field_derive.rs
use super::*;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, PartialEq, Default, the_module::Former)]

#[derive(Debug, PartialEq, Default)]
pub struct KeywordFieldsStruct {
  r#if: bool,
  r#type: String,
  r#struct: i32,
}

include!("keyword_field_only_test.rs");
