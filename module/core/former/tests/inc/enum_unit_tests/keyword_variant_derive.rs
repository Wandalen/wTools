use former::Former; // Ensure derive is in scope
use super::*; // Needed for the include

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, PartialEq, Former)]

#[derive(Debug, PartialEq)]
#[former(standalone_constructors, debug)]
#[allow(non_camel_case_types)] // Explicitly allowing for testing keyword-like names
pub enum KeywordTest {
  r#fn,
  r#struct,
}

include!("keyword_variant_only_test.rs");
