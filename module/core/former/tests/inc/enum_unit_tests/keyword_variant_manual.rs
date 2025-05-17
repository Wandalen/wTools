//! Manual implementation for testing unit variants with keyword identifiers.

use super::*;

/// Enum with keyword identifiers for variants.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)] // Explicitly allowing for testing keyword-like names
pub enum KeywordTest
{
  r#fn,
  r#struct,
}

#[allow(dead_code)] // Functions are used by included _only_test.rs
impl KeywordTest
{
  #[inline(always)]
  pub fn r#fn() -> Self
  {
    Self::r#fn
  }

  #[inline(always)]
  pub fn r#struct() -> Self
  {
    Self::r#struct
  }
}

// Standalone constructors
#[inline(always)]
pub fn r#fn() -> KeywordTest
{
  KeywordTest::r#fn
}

#[inline(always)]
pub fn r#struct() -> KeywordTest
{
  KeywordTest::r#struct
}

include!("keyword_variant_only_test.rs");