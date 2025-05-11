/// Shared test logic for unit variants with keyword identifiers.

use super::*;

#[test]
fn keyword_static_constructors()
{
  // Calling renamed manual methods for Increment 7 verification
  assert_eq!(KeywordTest::construct_fn(), KeywordTest::r#fn);
  assert_eq!(KeywordTest::construct_struct(), KeywordTest::r#struct);
}

#[test]
fn keyword_standalone_constructors()
{
  // Calling renamed manual methods for Increment 7 verification
  assert_eq!(standalone_construct_fn(), KeywordTest::r#fn);
  assert_eq!(standalone_construct_struct(), KeywordTest::r#struct);
}