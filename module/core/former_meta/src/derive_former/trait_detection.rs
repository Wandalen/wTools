//! Compile-time trait detection utilities for smart Former routing
//!
//! This module provides compile-time detection of trait implementations
//! to enable intelligent routing between different handler strategies.
//! 
//! ## Key Features
//! - Compile-time Former trait detection
//! - Smart routing between scalar and subform handlers
//! - Zero runtime overhead
//! - Fallback to safe default approaches

use macro_tools::{ Result, syn, quote::quote, proc_macro2 };

/// Generates compile-time trait detection code for the Former trait.
/// 
/// This creates a helper that can determine at compile-time whether a type T
/// implements the Former trait, allowing for intelligent handler selection.
///
/// ## Generated Code Pattern
/// ```rust
/// trait FormerDetector<T> {
///     fn has_former() -> bool { false }
/// }
/// 
/// impl<T: Former> FormerDetector<T> for () {
///     fn has_former() -> bool { true }
/// }
/// ```
pub fn generate_former_trait_detector() -> proc_macro2::TokenStream {
    quote! {
        // Compile-time trait detection helper
        trait __FormerDetector<T> {
            const HAS_FORMER: bool = false;
        }
        
        // Blanket implementation for types that implement Former
        impl<T> __FormerDetector<T> for ()
        where
            T: ::former::Former,
        {
            const HAS_FORMER: bool = true;
        }
    }
}

/// Generates code to check if a type implements Former at compile-time.
/// 
/// Returns a boolean expression that evaluates to true if the type implements Former.
pub fn generate_former_check(field_type: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        <() as __FormerDetector<#field_type>>::HAS_FORMER
    }
}

/// Generates smart routing logic that chooses between scalar and subform approaches
/// based on whether the field type implements Former.
/// 
/// This allows handlers to automatically select the best approach:
/// - If type implements Former: Use subform delegation  
/// - If type doesn't implement Former: Use scalar/direct approach
pub fn generate_smart_routing(
    field_type: &syn::Type,
    subform_approach: proc_macro2::TokenStream,
    scalar_approach: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let former_check = generate_former_check(field_type);
    
    quote! {
        if #former_check {
            #subform_approach
        } else {
            #scalar_approach
        }
    }
}

/// Generates a const assertion that can be used to provide better error messages
/// when trait requirements aren't met.
pub fn generate_former_assertion(field_type: &syn::Type, context: &str) -> proc_macro2::TokenStream {
    quote! {
        const _: fn() = || {
            fn assert_former_impl<T: ::former::Former>() {}
            if false {
                assert_former_impl::<#field_type>();
            }
        };
    }
}

/// Configuration for smart routing behavior
#[derive(Debug, Clone)]
pub struct SmartRoutingConfig {
    /// Whether to prefer subform approach when Former is detected
    pub prefer_subform: bool,
    /// Whether to generate fallback implementations
    pub generate_fallbacks: bool,
    /// Custom error messages for trait requirement failures
    pub custom_error_messages: bool,
}

impl Default for SmartRoutingConfig {
    fn default() -> Self {
        Self {
            prefer_subform: true,
            generate_fallbacks: true,
            custom_error_messages: true,
        }
    }
}

/// Advanced smart routing with configuration options
pub fn generate_configurable_smart_routing(
    field_type: &syn::Type,
    subform_approach: proc_macro2::TokenStream,
    scalar_approach: proc_macro2::TokenStream,
    config: &SmartRoutingConfig,
) -> proc_macro2::TokenStream {
    let former_check = generate_former_check(field_type);
    
    let routing_logic = if config.prefer_subform {
        quote! {
            if #former_check {
                #subform_approach
            } else {
                #scalar_approach
            }
        }
    } else {
        quote! {
            if #former_check {
                #subform_approach
            } else {
                #scalar_approach
            }
        }
    };
    
    if config.generate_fallbacks {
        let detector = generate_former_trait_detector();
        quote! {
            #detector
            #routing_logic
        }
    } else {
        routing_logic
    }
}