#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
use super::*; // Needed for the include
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

#[ derive( Debug, PartialEq, Former ) ]
#[ standalone_constructors ]
#[ allow( non_camel_case_types ) ] // Explicitly allowing for testing keyword-like names
pub enum KeywordTest {
  r#fn,
  r#struct,
}

include!("keyword_variant_only_test.rs");
