//! Utilities for handling raw identifiers in method name generation
//!
//! This module provides functions to properly handle raw identifiers (like `r#break`, `r#move`)
//! when generating method names from enum variant names or struct field names.
//! 
//! ## Key Functions
//! - `variant_to_method_name`: Converts variant names to method names with raw identifier support
//! - `strip_raw_prefix`: Safely strips the `r#` prefix when it's safe to do so
//! - `preserve_raw_identifier`: Preserves raw identifiers when necessary
//! - `strip_raw_prefix_for_compound_ident`: **CRITICAL** - Strips r# for use in compound identifiers
//!
//! ## Critical Bug ⚠️
//! 
//! **Issue**: Enum variant handlers concatenate raw identifiers without stripping `r#` prefix
//! - **Symptom**: Panic with error like `"KeywordVariantEnumr#breakFormerStorage"` is not a valid identifier
//! - **Root Cause**: Direct string concatenation of raw identifiers in type name generation
//! - **Affected**: All enum variant handlers processing keyword identifiers
//! - **Workaround**: Use `strip_raw_prefix_for_compound_ident()` before concatenation
//! - **Status**: Utility implemented but needs integration across all enum handlers

use macro_tools::{ syn, quote::format_ident, ident };
use convert_case::{Case, Casing};

/// Converts a variant name to a method name, properly handling raw identifiers.
/// 
/// This function takes an enum variant identifier and converts it to an appropriate
/// method name, handling raw identifiers correctly.
/// 
/// ## Examples
/// - `Break` -> `r#break` (preserves raw when needed)
/// - `Move` -> `r#move` (preserves raw when needed)  
/// - `Value` -> `value` (normal identifier)
/// - `MyVariant` -> `my_variant` (normal `snake_case` conversion)
pub fn variant_to_method_name(variant_ident: &syn::Ident) -> syn::Ident {
    let variant_str = variant_ident.to_string();
    
    // Check if this is a raw identifier
    if let Some(actual_name) = variant_str.strip_prefix("r#") {
        // Extract the actual identifier without the r# prefix
        // Convert to snake_case
        let snake_case_name = actual_name.to_case(Case::Snake);
        
        // Check if the snake_case version is a Rust keyword that needs raw identifier
        if is_rust_keyword(&snake_case_name) {
            // Create raw identifier
            format_ident!("r#{}", snake_case_name, span = variant_ident.span())
        } else {
            // Safe to use without raw prefix
            format_ident!("{}", snake_case_name, span = variant_ident.span())
        }
    } else {
        // Normal identifier, convert to snake_case
        let snake_case_name = variant_str.to_case(Case::Snake);
        
        // Check if result would be a keyword
        if is_rust_keyword(&snake_case_name) {
            // Make it a raw identifier
            format_ident!("r#{}", snake_case_name, span = variant_ident.span())
        } else {
            // Normal identifier
            format_ident!("{}", snake_case_name, span = variant_ident.span())
        }
    }
}

/// Checks if a string is a Rust keyword that would require raw identifier syntax.
fn is_rust_keyword(s: &str) -> bool {
    matches!(s, 
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" |
        "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" |
        "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self" |
        "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" |
        "use" | "where" | "while" | "async" | "await" | "dyn" | "abstract" |
        "become" | "box" | "do" | "final" | "macro" | "override" | "priv" |
        "typeof" | "unsized" | "virtual" | "yield" | "try"
    )
}

/// Converts a field identifier to a parameter name, handling raw identifiers.
/// 
/// This is similar to `ident::ident_maybe_raw` but specifically designed for
/// parameter name generation in constructor contexts.
#[ allow( dead_code ) ]
pub fn field_to_param_name(field_ident: &syn::Ident) -> syn::Ident {
    ident::ident_maybe_raw(field_ident)
}

/// Strips the raw identifier prefix for safe use in compound identifiers.
///
/// When building compound identifiers like `EnumVariantFormerStorage`, we need to strip
/// the `r#` prefix from variant names to avoid invalid identifiers like `EnumR#BreakFormerStorage`.
///
/// # Examples
/// - `r#break` -> `break` 
/// - `r#use` -> `use`
/// - `MyVariant` -> `MyVariant` (unchanged)
pub fn strip_raw_prefix_for_compound_ident(ident: &syn::Ident) -> String {
    let ident_str = ident.to_string();
    if let Some(stripped) = ident_str.strip_prefix("r#") {
        stripped.to_string()
    } else {
        ident_str
    }
}

/// Creates a constructor name from a struct/enum name, handling raw identifiers.
#[ allow( dead_code ) ]
pub fn type_to_constructor_name(type_ident: &syn::Ident) -> syn::Ident {
    let type_str = type_ident.to_string();
    
    // Handle raw identifier types
    if let Some(actual_name) = type_str.strip_prefix("r#") {
        let snake_case_name = actual_name.to_case(Case::Snake);
        
        if is_rust_keyword(&snake_case_name) {
            format_ident!("r#{}", snake_case_name, span = type_ident.span())
        } else {
            format_ident!("{}", snake_case_name, span = type_ident.span())
        }
    } else {
        let snake_case_name = type_str.to_case(Case::Snake);
        
        if is_rust_keyword(&snake_case_name) {
            format_ident!("r#{}", snake_case_name, span = type_ident.span())
        } else {
            format_ident!("{}", snake_case_name, span = type_ident.span())
        }
    }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use macro_tools::quote::format_ident;

  #[ test ]
  fn test_variant_to_method_name_normal()
  {
    let variant = format_ident!( "MyVariant" );
    let method = variant_to_method_name( &variant );
    assert_eq!( method.to_string(), "my_variant" );
  }

  #[ test ]
  fn test_variant_to_method_name_keyword()
  {
    let variant = format_ident!( "Break" );
    let method = variant_to_method_name( &variant );
    // Should become raw identifier since "break" is a keyword
    assert_eq!( method.to_string(), "r#break" );
  }

  #[ test ]
  fn test_is_rust_keyword()
  {
    assert!( is_rust_keyword( "break" ) );
    assert!( is_rust_keyword( "move" ) );
    assert!( is_rust_keyword( "async" ) );
    assert!( !is_rust_keyword( "normal" ) );
    assert!( !is_rust_keyword( "value" ) );
  }

  #[ test ]
  fn test_type_to_constructor_name()
  {
    let type_name = format_ident!( "MyStruct" );
    let constructor = type_to_constructor_name( &type_name );
    assert_eq!( constructor.to_string(), "my_struct" );
  }
}

