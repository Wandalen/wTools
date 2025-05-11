//! Manual implementation for testing unit variants with keyword identifiers.

use super::*;

/// Enum with keyword identifiers for variants.
#[derive(Debug, PartialEq)]
pub enum KeywordTest
{
  r#fn,
  r#struct,
}

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