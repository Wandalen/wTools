use former::Former; // Ensure derive is in scope
use super::*; // Needed for the include

#[derive(Debug, PartialEq, Former)]
#[former(standalone_constructors, debug)]
pub enum KeywordTest {
  r#fn,
  r#struct,
}

include!("keyword_variant_only_test.rs");