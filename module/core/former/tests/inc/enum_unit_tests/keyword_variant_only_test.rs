/// Shared test logic for unit variants with keyword identifiers.
use super::*;

#[ test ]
fn keyword_static_constructors()
{
  // Expect original names (for derive macro)
  assert_eq!(KeywordTest::r#fn, KeywordTest::r#fn);
  assert_eq!(KeywordTest::r#struct, KeywordTest::r#struct);
}

#[ test ]
fn keyword_standalone_constructors()
{
  // Expect original names (for derive macro)
  assert_eq!(r#fn(), KeywordTest::r#fn);
  assert_eq!(r#struct(), KeywordTest::r#struct);
}