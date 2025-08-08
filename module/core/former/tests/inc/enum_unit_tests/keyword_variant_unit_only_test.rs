// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[ derive( Former ) ]` for enums with unit variants that use keyword identifiers.
// This file is included by `keyword_variant_unit_derive.rs`.
//
// Coverage:
// - Rule 3a (Unit + Default): Tests static method `KeywordVariantEnum::r#loop()`.
// - Rule 1a (Unit + `#[ scalar ]`): Tests static method (as default for unit is scalar).
//
// Test Relevance/Acceptance Criteria:
// - Defines a test function (`keyword_variant_constructors`) that invokes the static method
//   `KeywordVariantEnum::r#loop()` provided by the including file (derived).
// - Asserts that the instance created by this constructor is equal to the expected
//   enum variant (`KeywordVariantEnum::r#Loop`).
//
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/keyword_variant_unit_only_test.rs
use super::*;

#[ test ]
fn keyword_variant_constructors()
{
  // Test unit variant - Expects direct constructor
  let got_loop = KeywordVariantEnum::r#loop();
  let exp_loop = KeywordVariantEnum::r#Loop;
  assert_eq!( got_loop, exp_loop );
}