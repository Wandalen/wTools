use super::*;
use test_tools::exposed::*;

//!
//! # Former Test Suite Organization
//!
//! This module contains comprehensive tests for the Former derive macro, organized by functionality.
//!
//! ## Test Architecture
//!
//! Tests follow a three-file pattern for verification:
//! - `*_manual.rs`: Hand-written implementation that macro should generate
//! - `*_derive.rs`: Uses `#[derive(Former)]` on identical structure
//! - `*_only_test.rs`: Shared test logic included by both manual and derive files
//!
//! ## Disabled Test Categories
//!
//! When tests are disabled, they typically fall into these systematic categories:
//!
//! **CATEGORY 1 - Missing Former types (Easy Fix)**
//! - Symptom: `BreakFormer not found`, `RunFormerDefinition not found`
//! - Cause: Commented-out `#[derive(Former)]` attributes
//! - Solution: Re-enable derives (historical "trailing comma issue" resolved)
//! - Files: basic_manual.rs, usecase1_derive.rs, etc.
//!
//! **CATEGORY 2 - Generic parsing issues (Hard)**
//! - Symptom: Complex generic parameter compilation errors
//! - Cause: Macro limitations with generic bounds/lifetimes
//! - Solution: Requires macro architecture improvements
//! - Files: All generics_* tests
//!
//! **CATEGORY 3 - Import/scope issues (Easy Fix)**
//! - Symptom: `TestEnum not found`, type resolution errors
//! - Cause: Incorrect import paths or module structure
//! - Solution: Fix imports, understand include vs module patterns
//! - Files: Most *_only_test.rs files
//!
//! **CATEGORY 4 - Trait conflicts (Medium)**
//! - Symptom: Conflicting trait implementations
//! - Cause: Multiple trait impls or missing trait bounds
//! - Solution: Resolve trait conflicts, add bounds
//! - Files: Manual implementations with trait issues
//!
//! **CATEGORY 5 - Unimplemented attributes (Hard)**
//! - Symptom: Attribute not recognized or not working
//! - Cause: Attribute parsing/handling not implemented
//! - Solution: Implement attribute support in macro
//! - Files: Tests using #[arg_for_constructor], etc.
//!
//! **CATEGORY 6 - Lifetime issues (Hard)**
//! - Symptom: Borrowed data escapes, undeclared lifetime
//! - Cause: Complex lifetime parameter interactions
//! - Solution: Requires careful lifetime analysis
//! - Files: parametrized_* tests with lifetimes
//!
//! **CATEGORY 7 - Infrastructure gaps (Medium)**
//! - Symptom: Missing methods, trait implementations
//! - Cause: Supporting infrastructure not implemented
//! - Solution: Implement missing supporting code
//! - Files: subform_collection_*, validation tests
//!
//! ## Critical Issues
//!
//! **Raw Identifier Bug**: Enum variants with raw identifiers (r#break) cause macro panics
//! **Inner Doc Comments**: Files with //! cannot be safely included with include!()
//! **Enum Former Delegation**: Current implementation uses positional setters, not field delegation
//!

#[cfg(feature = "derive_former")]
mod struct_tests;

// Tests for enum variants.
// These are categorized by the kind of variant fields.

#[cfg(feature = "derive_former")]
/// Tests for true unit variants (e.g., `Variant`).
pub mod enum_unit_tests;

#[cfg(feature = "derive_former")]
/// Tests for enum variants with unnamed (tuple) fields (e.g., `Variant(i32)`, `Variant()`).
/// Includes zero-field tuple variants.
pub mod enum_unnamed_tests;

#[cfg(feature = "derive_former")]
/// Tests for enum variants with named (struct-like) fields (e.g., `Variant { val: i32 }`).
/// Includes zero-field struct variants.
pub mod enum_named_tests;

#[cfg(feature = "derive_former")]
/// Tests for complex enum scenarios, combinations of features, or advanced use cases
/// not fitting neatly into unit/unnamed/named categories.
pub mod enum_complex_tests;
