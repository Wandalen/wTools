#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// Purpose: Tests the `#[ derive( Former ) ]` macro's generation of constructors for unnamed (tuple)
// variants with keyword identifiers, specifically when the variant is marked with `#[ scalar ]`
// or uses the default subform behavior. This file focuses on verifying the derive-based implementation.
//
// Coverage:
// - Rule 1d (Tuple + Single-Field + `#[ scalar ]` -> Scalar): Verifies `KeywordVariantEnum::r#use() -> KeywordVariantEnum`.
// - Rule 3d (Tuple + Single-Field + Default -> Subform): Verifies `KeywordVariantEnum::r#break() -> BreakFormer`.
// - Rule 4b (Option 2 Logic): Verifies the use of the subformer returned by the `r#break` variant constructor.
//
// Test Relevance/Acceptance Criteria:
// - Defines an enum `KeywordVariantEnum` with tuple variants using keyword identifiers (`r#use(u32)`, `r#break(Break)`).
// - The `r#use` variant is marked `#[ scalar ]`, and `r#break` uses default behavior (which results in a subformer).
// - The enum has `#[ derive( Former ) ]`.
// - Relies on the derived static methods `KeywordVariantEnum::r#use()` and `KeywordVariantEnum::r#break()` provided by this file (via `include!`).
// - Asserts that `KeywordVariantEnum::r#use()` takes the inner `u32` value and returns the `KeywordVariantEnum` instance.
// - Asserts that `KeywordVariantEnum::r#break()` returns a subformer for `Break`, and that using its setter (`.value()`) and `.form()` results in the `KeywordVariantEnum` instance.
// - Confirms correct handling of keyword identifiers and mixed scalar/subform behavior for tuple variants.
#[ allow( unused_imports ) ]
use super::*; // Imports testing infrastructure and potentially other common items
use former::Former;

// --- Dummy Struct ---
// Used in the `r#break` variant. Needs to derive Former for the enum's derive to work correctly for subforming.
#[ derive( Debug, Clone, Default, PartialEq, Former ) ]
pub struct Break
{
  pub value : u32,
}

// --- Enum Definition ---
// Apply Former derive here. This is what we are testing.
#[ allow( non_camel_case_types ) ] // Allow raw identifiers like r#use, r#break for keyword testing
#[ derive( Debug, PartialEq, Clone, Former ) ]
// #[ debug ] // Debug the macro to see what's being generated
pub enum KeywordVariantEnum
{
  // --- Tuple Variants with Keyword Identifiers ---
  #[ scalar ] // Explicitly scalar
  r#use( u32 ),
  // Also use scalar for r#break to test raw identifier handling  
  #[ scalar ]
  r#break( Break ),
}

// --- Test what methods are available ---
#[ test ]
fn test_what_methods_exist() {
  // Test the scalar constructor (should work)
  let scalar_result = KeywordVariantEnum::r#use(10u32);
  assert_eq!(scalar_result, KeywordVariantEnum::r#use(10u32));
  
  // Test Break Former works independently
  let break_instance = Break::former()
    .value(42u32)
    .form();
  assert_eq!(break_instance.value, 42);
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "keyword_variant_tuple_only_test.rs" );