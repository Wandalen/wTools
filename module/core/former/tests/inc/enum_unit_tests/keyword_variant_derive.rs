//! Derive implementation for testing unit variants with keyword identifiers.

use super::*;

/// Enum with keyword identifiers for variants, using Former.
#[derive(Debug, PartialEq, former::Former)]
#[former(standalone_constructors)]
pub enum KeywordTest
{
  r#fn,
  r#struct,
}

include!("keyword_variant_only_test.rs");