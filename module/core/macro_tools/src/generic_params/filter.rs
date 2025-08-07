//!
//! Generic parameter filtering utilities.
//!

use crate::*;

/// Filter generic parameters based on a predicate.
///
/// This function creates a new `Punctuated` list containing only the parameters
/// that match the given predicate, maintaining proper comma punctuation between elements.
///
/// # Arguments
///
/// * `params` - The punctuated list of generic parameters to filter
/// * `predicate` - A function that returns true for parameters to include
///
/// # Returns
///
/// A new `Punctuated` list containing only the filtered parameters
///
/// # Example
///
/// ```
/// use macro_tools::generic_params;
/// use syn::parse_quote;
///
/// let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
/// let only_types = generic_params::filter_params(
///     &generics.params,
///     |p| matches!(p, syn::GenericParam::Type(_))
/// );
///
/// assert_eq!(only_types.len(), 1);
/// ```
#[must_use]
pub fn filter_params<F>(
  params: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  predicate: F,
) -> syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>
where
  F: Fn(&syn::GenericParam) -> bool,
{
  let mut filtered = syn::punctuated::Punctuated::new();
  let matching_params: Vec<_> = params.iter().filter(|p| predicate(p)).cloned().collect();
  
  for (idx, param) in matching_params.iter().enumerate() {
    filtered.push_value(param.clone());
    if idx < matching_params.len() - 1 {
      filtered.push_punct(syn::token::Comma::default());
    }
  }

  filtered
}

/// Predicate to filter only lifetime parameters.
#[must_use] pub fn filter_lifetimes(param: &syn::GenericParam) -> bool {
  matches!(param, syn::GenericParam::Lifetime(_))
}

/// Predicate to filter only type parameters.
#[must_use] pub fn filter_types(param: &syn::GenericParam) -> bool {
  matches!(param, syn::GenericParam::Type(_))
}

/// Predicate to filter only const parameters.
#[must_use] pub fn filter_consts(param: &syn::GenericParam) -> bool {
  matches!(param, syn::GenericParam::Const(_))
}

/// Predicate to filter out lifetime parameters (keeping types and consts).
#[must_use] pub fn filter_non_lifetimes(param: &syn::GenericParam) -> bool {
  !matches!(param, syn::GenericParam::Lifetime(_))
}