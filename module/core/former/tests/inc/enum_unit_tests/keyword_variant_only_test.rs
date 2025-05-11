/// Shared test logic for unit variants with keyword identifiers.

use super::*;

#[test]
fn keyword_static_constructors()
{
  // Test Matrix Row: T3.1 / T3.3
  assert_eq!(<KeywordTest>::r#fn(), KeywordTest::r#fn); // Expect original name from derive
  // Test Matrix Row: T3.2 / T3.4
  assert_eq!(<KeywordTest>::r#struct(), KeywordTest::r#struct); // Expect original name from derive
}

#[test]
fn keyword_standalone_constructors()
{
  // Test Matrix Row: T3.3
  assert_eq!(r#fn(), KeywordTest::r#fn); // Expect original name from derive
  // Test Matrix Row: T3.4
  assert_eq!(r#struct(), KeywordTest::r#struct); // Expect original name from derive
}