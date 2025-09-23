//!
//! Generic parameter classification utilities.
//!

use crate :: *;

/// Classification of generic parameters by their type.
///
/// This struct provides a detailed breakdown of generic parameters into their constituent types
/// (lifetimes, type parameters, and const parameters) and includes convenience flags for common queries.
///
/// # Example
///
/// ```
/// use macro_tools ::generic_params;
/// use syn ::parse_quote;
///
/// let generics: syn ::Generics = parse_quote! { < 'a, T: Clone, const N: usize > };
/// let classification = generic_params ::classify_generics(&generics);
///
/// assert_eq!(classification.lifetimes.len(), 1);
/// assert_eq!(classification.types.len(), 1);
/// assert_eq!(classification.consts.len(), 1);
/// assert!(classification.has_mixed);
/// ```
#[ allow( clippy ::struct_excessive_bools ) ]
#[ derive( Debug, Clone ) ]
pub struct GenericsClassification< 'a > 
{
  /// Vector of references to lifetime parameters
  pub lifetimes: Vec< &'a syn ::LifetimeParam >,
  /// Vector of references to type parameters
  pub types: Vec< &'a syn ::TypeParam >,
  /// Vector of references to const parameters
  pub consts: Vec< &'a syn ::ConstParam >,
  /// True if generics contain only lifetime parameters
  pub has_only_lifetimes: bool,
  /// True if generics contain only type parameters
  pub has_only_types: bool,
  /// True if generics contain only const parameters
  pub has_only_consts: bool,
  /// True if generics contain a mix of parameter types
  pub has_mixed: bool,
  /// True if generics are empty
  pub is_empty: bool,
}

/// Classify generic parameters by their type.
///
/// This function analyzes a `syn ::Generics` struct and categorizes its parameters
/// into lifetimes, types, and const parameters, providing useful metadata about
/// the composition of the generics.
///
/// # Arguments
///
/// * `generics` - A reference to the `syn ::Generics` to classify
///
/// # Returns
///
/// A `GenericsClassification` struct containing the categorized parameters and metadata
///
/// # Example
///
/// ```
/// use macro_tools ::generic_params;
/// use syn ::parse_quote;
///
/// let generics: syn ::Generics = parse_quote! { < 'a, 'b, T > };
/// let classification = generic_params ::classify_generics(&generics);
///
/// assert_eq!(classification.lifetimes.len(), 2);
/// assert_eq!(classification.types.len(), 1);
/// assert!(!classification.has_only_lifetimes);
/// assert!(classification.has_mixed);
/// ```
#[ must_use ]
pub fn classify_generics(generics: &syn ::Generics) -> GenericsClassification< '_ > 
{
  let mut lifetimes = Vec ::new();
  let mut types = Vec ::new();
  let mut consts = Vec ::new();

  for param in &generics.params 
  {
  match param 
  {
   syn ::GenericParam ::Lifetime(lt) => lifetimes.push(lt),
   syn ::GenericParam ::Type(ty) => types.push(ty),
   syn ::GenericParam ::Const(ct) => consts.push(ct),
 }
 }

  let total = lifetimes.len() + types.len() + consts.len();
  let is_empty = total == 0;
  let has_only_lifetimes = !is_empty && lifetimes.len() == total;
  let has_only_types = !is_empty && types.len() == total;
  let has_only_consts = !is_empty && consts.len() == total;
  let has_mixed = !is_empty && !has_only_lifetimes && !has_only_types && !has_only_consts;

  GenericsClassification {
  lifetimes,
  types,
  consts,
  has_only_lifetimes,
  has_only_types,
  has_only_consts,
  has_mixed,
  is_empty,
 }
}

/// Extended decomposition result that includes classification and pre-filtered common cases.
///
/// This struct builds upon the basic `decompose` function by providing additional
/// classification information and pre-computed filtered parameter lists for common use cases.
#[ derive( Debug, Clone ) ]
pub struct DecomposedClassified 
{
  /// Original fields from decompose - generics with defaults preserved and trailing comma
  pub generics_with_defaults: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  /// Original fields from decompose - generics for impl without defaults
  pub generics_impl: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  /// Original fields from decompose - generics for type usage (simplified)
  pub generics_ty: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  /// Original fields from decompose - where clause predicates
  pub generics_where: syn ::punctuated ::Punctuated< syn ::WherePredicate, syn ::token ::Comma >,
  
  /// Classification information about the original generics
  pub classification: GenericsClassification< 'static >,
  
  /// Pre-filtered common cases for convenience
  /// Impl generics containing only type parameters
  pub generics_impl_only_types: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  /// Impl generics with lifetime parameters filtered out
  pub generics_impl_no_lifetimes: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  /// Type generics containing only type parameters
  pub generics_ty_only_types: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  /// Type generics with lifetime parameters filtered out
  pub generics_ty_no_lifetimes: syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
}

/// Extended decompose that provides classified parameters.
///
/// This function combines the functionality of `decompose` with `classify_generics`
/// and provides pre-filtered parameter lists for common use cases.
///
/// # Arguments
///
/// * `generics` - The generics to decompose and classify
///
/// # Returns
///
/// A `DecomposedClassified` struct containing all decomposed forms, classification,
/// and pre-filtered common cases.
///
/// # Example
///
/// ```
/// use macro_tools ::generic_params;
/// use syn ::parse_quote;
///
/// let generics: syn ::Generics = parse_quote! { < 'a, T: Clone, const N: usize > };
/// let decomposed = generic_params ::decompose_classified(&generics);
///
/// assert!(decomposed.classification.has_mixed);
/// assert_eq!(decomposed.generics_impl_only_types.len(), 1);
/// assert_eq!(decomposed.generics_impl_no_lifetimes.len(), 2); // T and const N
/// ```
#[ must_use ]
pub fn decompose_classified(generics: &syn ::Generics) -> DecomposedClassified 
{
  // use super :: { decompose, filter };
  use super ::filter;
  
  let (with_defaults, impl_params, ty_params, where_clause) = crate::generic_params::decompose(generics);
  
  // Create an owned classification for the original generics
  // We need to leak the memory to get 'static lifetime, but this is acceptable
  // for the classification use case as these are typically used in proc macros
  let generics_leaked = Box ::leak(Box ::new(generics.clone()));
  let classification = classify_generics(generics_leaked);
  
  // Pre-compute common filtered cases
  let generics_impl_only_types = filter ::filter_params(&impl_params, filter ::filter_types);
  let generics_impl_no_lifetimes = filter ::filter_params(&impl_params, filter ::filter_non_lifetimes);
  let generics_ty_only_types = filter ::filter_params(&ty_params, filter ::filter_types);
  let generics_ty_no_lifetimes = filter ::filter_params(&ty_params, filter ::filter_non_lifetimes);
  
  DecomposedClassified {
  generics_with_defaults: with_defaults,
  generics_impl: impl_params,
  generics_ty: ty_params,
  generics_where: where_clause,
  classification,
  generics_impl_only_types,
  generics_impl_no_lifetimes,
  generics_ty_only_types,
  generics_ty_no_lifetimes,
 }
}