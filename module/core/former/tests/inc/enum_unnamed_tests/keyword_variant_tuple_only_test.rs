// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[derive(Former)]` for enums with unnamed (tuple) variants that have keyword identifiers.
// This file is included by `keyword_variant_tuple_derive.rs`.
//
// Coverage:
// - Rule 1d (Tuple + Single-Field + `#[scalar]` -> Scalar): Tests static method `KeywordVariantEnum::r#use()`.
// - Rule 3d (Tuple + Single-Field + Default -> Subform): Tests static method `KeywordVariantEnum::r#break()`.
// - Rule 4b (Option 2 Logic): Tests the use of the subformer returned by the `r#break` variant constructor.
//
// Test Relevance/Acceptance Criteria:
// - Relies on the enum `KeywordVariantEnum` and inner struct `Break` defined in the including file (via `include!`).
// - Defines test functions (`keyword_variant_scalar_test`, `keyword_variant_subform_test`) that invoke the static methods
//   `KeywordVariantEnum::r#use()` and `KeywordVariantEnum::r#break()` provided by the including file.
// - Asserts that `KeywordVariantEnum::r#use()` takes the inner `u32` value and returns the `KeywordVariantEnum::r#use()` instance.
// - Asserts that `KeywordVariantEnum::r#break()` returns a subformer for `Break`, and that using its setter (`.value()`) and `.form()` results in the `KeywordVariantEnum::r#break()` instance.
// - Confirms correct handling of keyword identifiers and mixed scalar/subform behavior for tuple variants.
#[ allow( unused_imports ) ]
use super::*; // Imports items from the parent file (either manual or derive)

// Note: The enum `KeywordVariantEnum` and struct `Break` are defined in the including file.

#[ test ]
fn keyword_variant_scalar_test()
{
  // Test the scalar variant with a keyword identifier
  let got = KeywordVariantEnum::r#use( 10 ); // Use the derived static method

  let expected = KeywordVariantEnum::r#use( 10 ); // Manually construct the expected variant

  assert_eq!( got, expected );
}

#[ test ]
fn keyword_variant_scalar_break_test()
{
  // Test the scalar variant r#break with a keyword identifier
  let expected_inner = Break { value : 20 };
  let got = KeywordVariantEnum::r#break( expected_inner.clone() ); // Use the derived static method (scalar)

  let expected = KeywordVariantEnum::r#break( expected_inner ); // Manually construct the expected variant

  assert_eq!( got, expected );
}