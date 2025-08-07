//!
//! Generic parameter combination and merging utilities.
//!

use crate::*;

/// Merge multiple parameter lists maintaining proper order (lifetimes, types, consts).
///
/// This function combines multiple generic parameter lists while ensuring that
/// parameters are ordered correctly: lifetime parameters first, then type parameters,
/// then const parameters.
///
/// # Arguments
///
/// * `param_lists` - Slice of references to punctuated parameter lists to merge
///
/// # Returns
///
/// A new punctuated list containing all parameters in the correct order
///
/// # Example
///
/// ```
/// use macro_tools::generic_params;
/// use syn::parse_quote;
///
/// let list1: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
///     parse_quote! { T, const N: usize };
/// let list2: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
///     parse_quote! { 'a, U };
///
/// let merged = generic_params::merge_params_ordered(&[&list1, &list2]);
/// // Result will be ordered as: 'a, T, U, const N: usize
/// ```
#[must_use]
pub fn merge_params_ordered(
  param_lists: &[&syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>],
) -> syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> {
  let mut lifetimes = Vec::new();
  let mut types = Vec::new();
  let mut consts = Vec::new();

  // Collect all parameters by type
  for params in param_lists {
    for param in *params {
      match param {
        syn::GenericParam::Lifetime(lt) => lifetimes.push(syn::GenericParam::Lifetime(lt.clone())),
        syn::GenericParam::Type(ty) => types.push(syn::GenericParam::Type(ty.clone())),
        syn::GenericParam::Const(ct) => consts.push(syn::GenericParam::Const(ct.clone())),
      }
    }
  }

  // Build the result in the correct order
  let mut result = syn::punctuated::Punctuated::new();
  let all_params: Vec<_> = lifetimes.into_iter()
    .chain(types)
    .chain(consts)
    .collect();

  for (idx, param) in all_params.iter().enumerate() {
    result.push_value(param.clone());
    if idx < all_params.len() - 1 {
      result.push_punct(syn::token::Comma::default());
    }
  }

  result
}

/// Add parameters to existing list with smart comma handling.
///
/// This function appends additional parameters to an existing parameter list,
/// handling comma punctuation correctly to avoid trailing commas.
///
/// # Arguments
///
/// * `base` - The base parameter list to extend
/// * `additional` - Slice of additional parameters to add
///
/// # Returns
///
/// A new punctuated list containing all parameters
///
/// # Example
///
/// ```
/// use macro_tools::generic_params;
/// use syn::parse_quote;
///
/// let base: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
///     parse_quote! { T, U };
/// let additional = vec![parse_quote! { V }];
///
/// let extended = generic_params::params_with_additional(&base, &additional);
/// // Result: T, U, V
/// ```
#[must_use]
pub fn params_with_additional(
  base: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  additional: &[syn::GenericParam],
) -> syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> {
  let mut result = base.clone();
  
  // Remove trailing punctuation if present
  while result.trailing_punct() {
    result.pop_punct();
  }

  // Add additional parameters
  for param in additional {
    if !result.is_empty() {
      result.push_punct(syn::token::Comma::default());
    }
    result.push_value(param.clone());
  }

  result
}

/// Create a new parameter list from individual components.
///
/// This function builds a properly ordered and punctuated generic parameter list
/// from separate lifetime, type, and const parameter components.
///
/// # Arguments
///
/// * `lifetimes` - Slice of lifetime parameters
/// * `types` - Slice of type parameters
/// * `consts` - Slice of const parameters
///
/// # Returns
///
/// A punctuated list containing all parameters in the correct order
///
/// # Example
///
/// ```
/// use macro_tools::generic_params;
/// use syn::parse_quote;
///
/// let lifetimes = vec![parse_quote! { 'a }, parse_quote! { 'b }];
/// let types = vec![parse_quote! { T: Clone }];
/// let consts = vec![parse_quote! { const N: usize }];
///
/// let params = generic_params::params_from_components(&lifetimes, &types, &consts);
/// // Result: 'a, 'b, T: Clone, const N: usize
/// ```
#[must_use]
pub fn params_from_components(
  lifetimes: &[syn::LifetimeParam],
  types: &[syn::TypeParam],
  consts: &[syn::ConstParam],
) -> syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> {
  let mut result = syn::punctuated::Punctuated::new();
  
  let all_params: Vec<syn::GenericParam> = lifetimes.iter()
    .map(|lt| syn::GenericParam::Lifetime(lt.clone()))
    .chain(types.iter().map(|ty| syn::GenericParam::Type(ty.clone())))
    .chain(consts.iter().map(|ct| syn::GenericParam::Const(ct.clone())))
    .collect();

  for (idx, param) in all_params.iter().enumerate() {
    result.push_value(param.clone());
    if idx < all_params.len() - 1 {
      result.push_punct(syn::token::Comma::default());
    }
  }

  result
}