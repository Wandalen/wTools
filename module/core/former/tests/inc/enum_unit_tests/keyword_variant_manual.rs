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
  pub fn construct_fn() -> Self // Renamed
  {
    Self::r#fn
  }

  #[inline(always)]
  pub fn construct_struct() -> Self // Renamed
  {
    Self::r#struct
  }
}

// Standalone constructors
#[inline(always)]
pub fn standalone_construct_fn() -> KeywordTest // Renamed
{
  KeywordTest::r#fn
}

#[inline(always)]
pub fn standalone_construct_struct() -> KeywordTest // Renamed
{
  KeywordTest::r#struct
}

include!("keyword_variant_only_test.rs");