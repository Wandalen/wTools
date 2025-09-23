#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of constructors for unit variants
//! with keyword identifiers. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Verifies `KeywordVariantEnum::r#loop() -> KeywordVariantEnum` for a unit variant with a keyword identifier.
//! - Rule 1a (Unit + `#[ scalar ]`): Verifies `KeywordVariantEnum::r#loop() -> KeywordVariantEnum` (as default for unit is scalar) for a unit variant with a keyword identifier.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `KeywordVariantEnum` with a unit variant `r#Loop` using a raw identifier.
//! - Relies on the derived static method `KeywordVariantEnum::r#loop()` defined in `keyword_variant_unit_only_test.rs`.
//! - Asserts that the `got` instance is equal to an `expected` instance, which is manually
//!   constructed as `KeywordVariantEnum::r#Loop`. This confirms the constructor handles keyword identifiers correctly.
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/keyword_variant_unit_derive.rs
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
enum KeywordVariantEnum
{
  /// Unit: Expects r#loop()
  r#Loop,
}

// Include the test logic
include!( "keyword_variant_unit_only_test.rs" );