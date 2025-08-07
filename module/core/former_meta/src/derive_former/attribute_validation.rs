//! # Attribute Validation - Comprehensive Enum Variant Attribute Validation
//!
//! This module provides centralized validation for enum variant attributes to ensure
//! proper usage and prevent incompatible attribute combinations that would lead to
//! compilation errors or unexpected behavior.
//!
//! ## Core Functionality
//!
//! ### Validation Categories
//! - **Attribute Compatibility**: Prevent conflicting attribute combinations
//! - **Variant Type Appropriateness**: Ensure attributes are used on suitable variant types
//! - **Field Count Validation**: Verify attributes match the variant's field structure
//! - **Semantic Correctness**: Validate that attribute usage makes semantic sense
//!
//! ### Validation Rules Implemented
//!
//! #### Rule V-1: Scalar vs Subform Scalar Conflicts
//! - `#[scalar]` and `#[subform_scalar]` cannot be used together on the same variant
//! - Exception: Struct variants where both have identical behavior
//!
//! #### Rule V-2: Subform Scalar Appropriateness
//! - `#[subform_scalar]` cannot be used on unit variants (no fields to form)
//! - `#[subform_scalar]` cannot be used on zero-field variants (no fields to form)
//! - `#[subform_scalar]` cannot be used on multi-field tuple variants (ambiguous field selection)
//!
//! #### Rule V-3: Scalar Attribute Requirements
//! - Zero-field struct variants MUST have `#[scalar]` attribute (disambiguation requirement)
//! - Other variant types can use `#[scalar]` optionally
//!
//! #### Rule V-4: Field Count Consistency
//! - Single-field variants should use single-field appropriate attributes
//! - Multi-field variants should use multi-field appropriate attributes
//! - Zero-field variants should use zero-field appropriate attributes
//!
//! ## Architecture
//!
//! ### Validation Functions
//! - `validate_variant_attributes()`: Main validation entry point
//! - `validate_attribute_combinations()`: Check for conflicting attributes
//! - `validate_variant_type_compatibility()`: Ensure attributes match variant type
//! - `validate_field_count_requirements()`: Verify field count appropriateness
//!
//! ### Error Reporting
//! - Clear, actionable error messages
//! - Context-sensitive help suggestions
//! - Proper span information for IDE integration

use super::*;
use macro_tools::{Result, syn_err};
use super::field_attrs::FieldAttributes;

/// Validates all attributes on an enum variant for correctness and compatibility.
///
/// This function performs comprehensive validation of variant attributes to catch
/// common errors and provide helpful diagnostics at compile time.
///
/// # Arguments
/// * `variant` - The enum variant being validated
/// * `variant_attrs` - Parsed variant attributes
/// * `field_count` - Number of fields in the variant
/// * `variant_type` - Type of variant (Unit, Tuple, Struct)
///
/// # Returns
/// * `Ok(())` - All validation passed
/// * `Err(syn::Error)` - Validation failed with descriptive error
pub fn validate_variant_attributes(
  variant: &syn::Variant,
  variant_attrs: &FieldAttributes,
  field_count: usize,
  variant_type: VariantType,
) -> Result<()>
{
  validate_attribute_combinations(variant, variant_attrs)?;
  validate_variant_type_compatibility(variant, variant_attrs, variant_type)?;
  validate_field_count_requirements(variant, variant_attrs, field_count, variant_type)?;
  Ok(())
}

/// Represents the type of enum variant for validation purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantType
{
  /// Unit variant: `Variant`
  Unit,
  /// Tuple variant: `Variant(T1, T2, ...)`
  Tuple,
  /// Struct variant: `Variant { field1: T1, field2: T2, ... }`
  Struct,
}

/// Validates that attribute combinations are compatible.
///
/// Prevents conflicting attributes from being used together on the same variant.
fn validate_attribute_combinations(
  variant: &syn::Variant,
  variant_attrs: &FieldAttributes,
) -> Result<()>
{
  // Rule V-1: #[scalar] and #[subform_scalar] conflict (except for struct variants)
  if variant_attrs.scalar.is_some() && variant_attrs.subform_scalar.is_some() {
    // For struct variants, both attributes have the same behavior, so allow it
    if matches!(variant.fields, syn::Fields::Named(_)) {
      // This is acceptable - both attributes produce the same result for struct variants
    } else {
      return Err(syn_err!(
        variant,
        "Cannot use both #[scalar] and #[subform_scalar] on the same variant. \
         These attributes have conflicting behaviors for tuple variants. \
         Choose either #[scalar] for direct construction or #[subform_scalar] for subform construction."
      ));
    }
  }

  Ok(())
}

/// Validates that attributes are appropriate for the variant type.
///
/// Ensures attributes are only used on variant types where they make semantic sense.
fn validate_variant_type_compatibility(
  variant: &syn::Variant,
  variant_attrs: &FieldAttributes,
  variant_type: VariantType,
) -> Result<()>
{
  // Rule V-2: #[subform_scalar] appropriateness
  if variant_attrs.subform_scalar.is_some() {
    match variant_type {
      VariantType::Unit => {
        return Err(syn_err!(
          variant,
          "#[subform_scalar] cannot be used on unit variants. \
           Unit variants have no fields to form. \
           Consider removing the #[subform_scalar] attribute."
        ));
      }
      VariantType::Tuple | VariantType::Struct => {
        // Will be validated by field count requirements
      }
    }
  }

  Ok(())
}

/// Validates that attributes are appropriate for the variant's field count.
///
/// Ensures attributes match the structural requirements of the variant.
fn validate_field_count_requirements(
  variant: &syn::Variant,
  variant_attrs: &FieldAttributes,
  field_count: usize,
  variant_type: VariantType,
) -> Result<()>
{
  // Rule V-2 continued: #[subform_scalar] field count requirements
  if variant_attrs.subform_scalar.is_some() {
    match (variant_type, field_count) {
      (VariantType::Tuple | VariantType::Struct, 0) => {
        return Err(syn_err!(
          variant,
          "#[subform_scalar] cannot be used on zero-field variants. \
           Zero-field variants have no fields to form. \
           Consider using #[scalar] attribute instead for direct construction."
        ));
      }
      (VariantType::Tuple, count) if count > 1 => {
        return Err(syn_err!(
          variant,
          "#[subform_scalar] cannot be used on multi-field tuple variants. \
           Multi-field tuple variants have ambiguous field selection for subform construction. \
           Consider using #[scalar] for direct construction with all fields as parameters, \
           or restructure as a struct variant for field-specific subform construction."
        ));
      }
      _ => {
        // Single-field variants are OK for subform_scalar
      }
    }
  }

  // Rule V-3: Zero-field struct variants require #[scalar]
  if variant_type == VariantType::Struct && field_count == 0
    && variant_attrs.scalar.is_none() && variant_attrs.subform_scalar.is_none() {
      return Err(syn_err!(
        variant,
        "Zero-field struct variants require explicit #[scalar] attribute for disambiguation. \
         Add #[scalar] to generate a direct constructor for this variant."
      ));
    }

  Ok(())
}

/// Helper function to get validation-friendly field count from `syn::Fields`.
pub fn get_field_count(fields: &syn::Fields) -> usize
{
  match fields {
    syn::Fields::Unit => 0,
    syn::Fields::Unnamed(fields) => fields.unnamed.len(),
    syn::Fields::Named(fields) => fields.named.len(),
  }
}

/// Helper function to get variant type from `syn::Fields`.
pub fn get_variant_type(fields: &syn::Fields) -> VariantType
{
  match fields {
    syn::Fields::Unit => VariantType::Unit,
    syn::Fields::Unnamed(_) => VariantType::Tuple,
    syn::Fields::Named(_) => VariantType::Struct,
  }
}