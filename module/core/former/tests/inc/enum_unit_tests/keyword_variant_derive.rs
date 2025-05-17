use former::Former; // Ensure derive is in scope
use super::*; // Needed for the include

#[derive(Debug, PartialEq, Former)]
#[former(standalone_constructors, debug)]
#[allow(non_camel_case_types)] // Explicitly allowing for testing keyword-like names
pub enum KeywordTest {
  r#fn,
  r#struct,
}

include!("keyword_variant_only_test.rs");