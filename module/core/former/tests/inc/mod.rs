use super::*;
use test_tools::exposed::*;

#[cfg(feature = "derive_former")]
mod struct_tests;

// Tests for enum variants.
// These are categorized by the kind of variant fields.

#[cfg(feature = "derive_former")]
/// Tests for true unit variants (e.g., `Variant`).
pub mod enum_unit_tests;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[cfg(feature = "derive_former")]
// /// Tests for enum variants with unnamed (tuple) fields (e.g., `Variant(i32)`, `Variant()`).
// /// Includes zero-field tuple variants.
// pub mod enum_unnamed_tests;

#[cfg(feature = "derive_former")]
/// Tests for enum variants with named (struct-like) fields (e.g., `Variant { val: i32 }`).
/// Includes zero-field struct variants.
pub mod enum_named_tests;

// #[cfg(feature = "derive_former")]
// /// Tests for complex enum scenarios, combinations of features, or advanced use cases
// /// not fitting neatly into unit/unnamed/named categories.
// pub mod enum_complex_tests;
