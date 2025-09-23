//!
//! Functions and structures to handle and manipulate generic parameters using the `syn` crate. It's designed to support macro-driven code generation by simplifying, merging, extracting, and decomposing `syn ::Generics`.
//!

// Sub-modules
pub mod classification;
pub mod filter;
pub mod combine;

/// Define a private namespace for all its items.
mod private
{
  use crate :: *;

  /// A `GenericsWithWhere` struct to handle the parsing of Rust generics with an explicit `where` clause.
  ///
  /// This wrapper addresses the limitation in the `syn` crate where parsing `Generics` directly from a `ParseStream`
  /// does not automatically handle associated `where` clauses. By integrating `where` clause parsing into the
  /// `GenericsWithWhere`, this struct provides a seamless way to capture both the generics and their constraints
  /// in scenarios where the `where` clause is crucial for type constraints and bounds in Rust macros and code generation.
  ///
  /// Usage :
  ///
  /// ```
  /// let parsed_generics: macro_tools ::generic_params ::GenericsWithWhere
  /// = syn ::parse_str( "< T: Clone, U: Default = Default1 > where T: Default" ).unwrap();
  /// assert!( parsed_generics.generics.params.len() == 2 );
  /// assert!( parsed_generics.generics.where_clause.is_some() );
  /// ```
  ///
  #[ derive( Debug ) ]
  pub struct GenericsWithWhere
  {
  /// Syn's generics parameters.
  pub generics: syn ::Generics,
 }

  impl GenericsWithWhere 
  {
  /// Unwraps the `GenericsWithWhere` to retrieve the inner `syn ::Generics`.
  #[ must_use ]
  pub fn unwrap(self) -> syn ::Generics
  {
   self.generics
 }

  /// Parses a string to a `GenericsWithWhere`, specifically designed to handle generics syntax with where clauses effectively.
  ///
  /// This function provides a convenient way to parse generic parameters and their associated
  /// `where` clauses from a string slice, returning a `GenericsWithWhere` instance.
  ///
  /// # Arguments
  ///
  /// * `s` - The string slice containing the generics and optional `where` clause (e.g., `"< T: Debug > where T: Default"`).
  ///
  /// # Returns
  ///
  /// Returns a `syn ::Result` which is `Ok(GenericsWithWhere)` on successful parsing,
  /// or `Err(syn ::Error)` if the input string does not conform to valid Rust generics syntax.
  ///
  /// # Errors
  ///
  /// Returns a `syn ::Error` if the input string `s` cannot be parsed as valid Rust generics
  /// or a `where` clause.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use macro_tools ::generic_params ::GenericsWithWhere;
  ///
  /// let parsed = GenericsWithWhere ::parse_from_str( "< T: Clone, U: Default = Default1 > where T: Default" ).unwrap();
  /// assert!( parsed.generics.params.len() == 2 );
  /// assert!( parsed.generics.where_clause.is_some() );
  ///
  /// let parsed_no_where = GenericsWithWhere ::parse_from_str( "< T >" ).unwrap();
  /// assert!( parsed_no_where.generics.params.len() == 1 );
  /// assert!( parsed_no_where.generics.where_clause.is_none() );
  ///
  /// let parsed_only_where = GenericsWithWhere ::parse_from_str( "where T: Debug" ).unwrap();
  /// assert!( parsed_only_where.generics.params.is_empty() );
  /// assert!( parsed_only_where.generics.where_clause.is_some() );
  /// ```
  pub fn parse_from_str(s: &str) -> syn ::Result< GenericsWithWhere >
  {
   syn ::parse_str :: < GenericsWithWhere >(s)
 }
 }

  impl syn ::parse ::Parse for GenericsWithWhere 
  {
  fn parse(input: syn ::parse ::ParseStream< '_ >) -> syn ::Result< Self > 
  {
   let generics: syn ::Generics = input.parse()?;
   let where_clause: Option< syn ::WhereClause > = input.parse()?;

   let mut generics_clone = generics.clone();
   generics_clone.where_clause = where_clause;

   Ok(GenericsWithWhere {
  generics: generics_clone,
 })
 }
 }

  impl quote ::ToTokens for GenericsWithWhere 
  {
  fn to_tokens(&self, tokens: &mut proc_macro2 ::TokenStream) 
  {
   self.generics.to_tokens(tokens);
 }
 }

  impl From< GenericsWithWhere > for syn ::Generics 
  {
  fn from(g: GenericsWithWhere) -> Self 
  {
   g.generics
 }
 }

  impl From< syn ::Generics > for GenericsWithWhere 
  {
  fn from(generics: syn ::Generics) -> Self 
  {
   GenericsWithWhere { generics }
 }
 }

  /// A wrapper around a reference to `syn ::Generics` to provide convenient helper methods
  /// for generating token streams related to generic parameters.
  ///
  /// This is particularly useful in procedural macros for constructing parts of function
  /// signatures, type paths, and where clauses that involve generics.
  #[ derive( Debug, Clone, Copy ) ]
  pub struct GenericsRef< 'a >
  {
  syn_generics: &'a syn ::Generics,
 }

  impl< 'a > GenericsRef< 'a >
  {
  /// Creates a new `GenericsRef` from a reference to `syn ::Generics`.
  #[ must_use ]
  pub fn new_borrowed(syn_generics: &'a syn ::Generics) -> Self
  {
   Self { syn_generics }
 }

  /// Creates a new `GenericsRef` from a reference to `syn ::Generics`. Alias for `new_borrowed`.
  #[ must_use ]
  pub fn new(syn_generics: &'a syn ::Generics) -> Self
  {
   Self ::new_borrowed(syn_generics)
 }

  /// Returns the `impl_generics` part (e.g., `< T: Trait, 'b, const C: usize >`)
  /// as a `TokenStream` if generics are present, otherwise an empty `TokenStream`.
  ///
  /// This is suitable for use in `impl < #impl_generics > Struct ...` contexts.
  /// It includes bounds and lifetimes.
  #[ must_use ]
  pub fn impl_generics_tokens_if_any( &self ) -> proc_macro2 ::TokenStream
  {
   if self.syn_generics.params.is_empty() 
   {
  return quote ::quote! {};
 }
   let (impl_g, _, _) = self.syn_generics.split_for_impl();
   quote ::quote! { #impl_g }
 }

  /// Returns the `ty_generics` part (e.g., `< T, 'b, C >`) as a `TokenStream`
  /// if generics are present, otherwise an empty `TokenStream`.
  ///
  /// This is suitable for use in type paths like `Struct :: < #ty_generics >`.
  /// It includes only the identifiers of the generic parameters (types, lifetimes, consts).
  #[ must_use ]
  pub fn ty_generics_tokens_if_any( &self ) -> proc_macro2 ::TokenStream
  {
   if self.syn_generics.params.is_empty() 
   {
  return quote ::quote! {};
 }
   let (_, ty_g, _) = self.syn_generics.split_for_impl();
   quote ::quote! { #ty_g }
 }

  /// Returns the `where_clause` (e.g., `where T: Trait`) as a `TokenStream`
  /// if a where clause is present in the original generics, otherwise an empty `TokenStream`.
  #[ must_use ]
  pub fn where_clause_tokens_if_any( &self ) -> proc_macro2 ::TokenStream
  {
   let (_, _, where_clause) = self.syn_generics.split_for_impl();
   quote ::quote! { #where_clause }
 }

  /// Returns a token stream representing a path to a type, including its generic arguments
  /// if present (e.g., `MyType :: < T, U >`). If no generics are present, it returns
  /// just the `base_ident`.
  ///
  /// # Arguments
  ///
  /// * `base_ident` : The identifier of the base type (e.g., `MyType`).
  #[ must_use ]
  pub fn type_path_tokens_if_any(&self, base_ident: &syn ::Ident) -> proc_macro2 ::TokenStream
  {
   if self.syn_generics.params.is_empty() 
   {
  quote ::quote! { #base_ident }
 }
  else
  {
  let (_, ty_g, _) = self.syn_generics.split_for_impl();
  quote ::quote! { #base_ident #ty_g }
 }
  }

  /// Get classification of the generics.
  ///
  /// This method analyzes the generic parameters and returns a classification
  /// containing information about the types of parameters present.
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params :: { GenericsRef, classify_generics };
  /// use syn ::parse_quote;
  ///
  /// let generics: syn ::Generics = parse_quote! { < 'a, T, const N: usize > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// let classification = generics_ref.classification();
  ///
  /// assert!(classification.has_mixed);
  /// assert_eq!(classification.lifetimes.len(), 1);
  /// assert_eq!(classification.types.len(), 1);
  /// assert_eq!(classification.consts.len(), 1);
  /// ```
  #[ must_use ]
  pub fn classification( &self ) -> super ::classification ::GenericsClassification< 'a >
  {
   super ::classification ::classify_generics(self.syn_generics)
 }
  
  /// Get impl generics without lifetimes.
  ///
  /// This method returns the impl generics token stream with lifetime parameters filtered out,
  /// keeping only type and const parameters.
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params ::GenericsRef;
  /// use syn ::parse_quote;
  ///
  /// let generics: syn ::Generics = parse_quote! { < 'a, T: Clone, const N: usize > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// let impl_no_lifetimes = generics_ref.impl_generics_no_lifetimes();
  ///
  /// // Result will be: < T: Clone, const N: usize >
  /// ```
  #[ must_use ]
  pub fn impl_generics_no_lifetimes( &self ) -> proc_macro2 ::TokenStream
  {
   let filtered = super ::filter ::filter_params(&self.syn_generics.params, super ::filter ::filter_non_lifetimes);
   if filtered.is_empty() 
   {
  quote ::quote! {}
 } else {
  quote ::quote! { < #filtered > }
 }
 }
  
  /// Get type generics without lifetimes.
  ///
  /// This method returns the type generics token stream with lifetime parameters filtered out,
  /// keeping only type and const parameters (simplified for type usage).
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params ::GenericsRef;
  /// use syn ::parse_quote;
  ///
  /// let generics: syn ::Generics = parse_quote! { < 'a, T, const N: usize > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// let ty_no_lifetimes = generics_ref.ty_generics_no_lifetimes();
  ///
  /// // Result will be: < T, N >
  /// ```
  #[ must_use ]
  pub fn ty_generics_no_lifetimes( &self ) -> proc_macro2 ::TokenStream
  {
   let (_, _, ty_params, _) = crate::generic_params::decompose(self.syn_generics);
   let filtered = super ::filter ::filter_params(&ty_params, super ::filter ::filter_non_lifetimes);
   if filtered.is_empty() 
   {
  quote ::quote! {}
 } else {
  quote ::quote! { < #filtered > }
 }
 }
  
  /// Check if generics contain only lifetime parameters.
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params ::GenericsRef;
  /// use syn ::parse_quote;
  ///
  /// let generics: syn ::Generics = parse_quote! { < 'a, 'b > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// assert!(generics_ref.has_only_lifetimes());
  ///
  /// let generics2: syn ::Generics = parse_quote! { < 'a, T > };
  /// let generics_ref2 = GenericsRef ::new(&generics2);
  /// assert!(!generics_ref2.has_only_lifetimes());
  /// ```
  #[ must_use ]
  pub fn has_only_lifetimes( &self ) -> bool
  {
   self.classification().has_only_lifetimes
 }
  
  /// Check if generics contain only type parameters.
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params ::GenericsRef;
  /// use syn ::parse_quote;
  ///
  /// let generics: syn ::Generics = parse_quote! { < T, U > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// assert!(generics_ref.has_only_types());
  ///
  /// let generics2: syn ::Generics = parse_quote! { < T, const N: usize > };
  /// let generics_ref2 = GenericsRef ::new(&generics2);
  /// assert!(!generics_ref2.has_only_types());
  /// ```
  #[ must_use ]
  pub fn has_only_types( &self ) -> bool
  {
   self.classification().has_only_types
 }
  
  /// Check if generics contain only const parameters.
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params ::GenericsRef;
  /// use syn ::parse_quote;
  ///
  /// let generics: syn ::Generics = parse_quote! { < const N: usize, const M: i32 > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// assert!(generics_ref.has_only_consts());
  /// ```
  #[ must_use ]
  pub fn has_only_consts( &self ) -> bool
  {
   self.classification().has_only_consts
 }
  
  /// Get type path without lifetime parameters.
  ///
  /// This method returns a token stream representing a path to a type with
  /// lifetime parameters filtered out from the generic arguments.
  ///
  /// # Arguments
  ///
  /// * `base_ident` - The identifier of the base type
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools ::generic_params ::GenericsRef;
  /// use syn :: { parse_quote, Ident };
  /// use quote ::format_ident;
  ///
  /// let generics: syn ::Generics = parse_quote! { < 'a, T, const N: usize > };
  /// let generics_ref = GenericsRef ::new(&generics);
  /// let base = format_ident!("MyType");
  /// let path = generics_ref.type_path_no_lifetimes(&base);
  ///
  /// // Result will be: MyType :: < T, N >
  /// ```
  #[ must_use ]
  pub fn type_path_no_lifetimes(&self, base_ident: &syn ::Ident) -> proc_macro2 ::TokenStream
  {
   let ty_no_lifetimes = self.ty_generics_no_lifetimes();
   if self.syn_generics.params.is_empty() || 
  self.syn_generics.params.iter().all(|p| matches!(p, syn ::GenericParam ::Lifetime(_))) {
  quote ::quote! { #base_ident }
 } else {
  quote ::quote! { #base_ident #ty_no_lifetimes }
 }
 }
}
}




// Function implementations moved outside private module
/// Merges two `syn ::Generics` instances into a new one.
///
/// This function takes two references to `syn ::Generics` and combines their
/// type parameters and where clauses into a new `syn ::Generics` instance. If
/// both instances have where clauses, the predicates of these clauses are merged
/// into a single where clause.
///
/// # Arguments
///
/// * `a` - A reference to the first `syn ::Generics` instance.
/// * `b` - A reference to the second `syn ::Generics` instance.
///
/// # Returns
///
/// Returns a new `syn ::Generics` instance containing the merged type parameters
/// and where clauses from `a` and `b`.
#[ must_use ]
#[ allow( clippy ::default_trait_access ) ]
pub fn merge(a: &syn ::Generics, b: &syn ::Generics) -> syn ::Generics
{
  let mut result = syn ::Generics {
    params: Default ::default(),
    where_clause: None,
    lt_token: Some(syn ::token ::Lt ::default()),
    gt_token: Some(syn ::token ::Gt ::default()),
  };

  // Merge params
  for param in &a.params
  {
    result.params.push(param.clone());
  }
  for param in &b.params
  {
    result.params.push(param.clone());
  }

  // Merge where clauses
  result.where_clause =  match (&a.where_clause, &b.where_clause)
  {
    (Some(a_clause), Some(b_clause)) =>
    {
      let mut merged_where_clause = syn ::WhereClause {
        where_token: a_clause.where_token,
        predicates: a_clause.predicates.clone(),
      };
      for predicate in &b_clause.predicates
      {
        merged_where_clause.predicates.push(predicate.clone());
      }
      Some(merged_where_clause)
    }
    (Some(a_clause), None) => Some(a_clause.clone()),
    (None, Some(b_clause)) => Some(b_clause.clone()),
    _ => None,
  };

  result
}

/// Extracts parameter names from the given `Generics`,
/// dropping bounds, defaults, and the where clause.
#[ allow( clippy ::default_trait_access ) ]
#[ must_use ]
pub fn only_names(generics: &syn ::Generics) -> syn ::Generics
{
  use syn :: { Generics, GenericParam, LifetimeParam, TypeParam, ConstParam };

  let params = generics
    .params
    .iter()
    .map(|param| match param {
      GenericParam::Type(TypeParam { ident, .. }) => GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: ident.clone(),
        colon_token: None,
        bounds: Default::default(),
        eq_token: None,
        default: None,
      }),
      GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => GenericParam::Lifetime(LifetimeParam {
        attrs: Vec::new(),
        lifetime: lifetime.clone(),
        colon_token: None,
        bounds: Default::default(),
      }),
      GenericParam::Const(ConstParam { ident, ty, .. }) => GenericParam::Const(ConstParam {
        attrs: Vec::new(),
        const_token: Default::default(),
        ident: ident.clone(),
        colon_token: Default::default(),
        ty: ty.clone(),
        eq_token: Default::default(),
        default: None,
      }),
    })
    .collect();

  Generics {
    params,
    where_clause: None,
    lt_token: generics.lt_token,
    gt_token: generics.gt_token,
  }
}

/// Extracts the names of type parameters, lifetimes, and const parameters from the given `Generics`.
pub fn names(generics: &syn ::Generics) -> impl Iterator< Item = &syn ::Ident >
{
  generics.params.iter().map( |param| match param
  {
    syn ::GenericParam ::Type(type_param) => &type_param.ident,
    syn ::GenericParam ::Lifetime(lifetime_def) => &lifetime_def.lifetime.ident,
    syn ::GenericParam ::Const(const_param) => &const_param.ident,
  } )
}

/// Decomposes `syn ::Generics` into components suitable for different usage contexts in Rust implementations.
#[ allow( clippy ::type_complexity ) ]
#[ allow( clippy ::too_many_lines ) ]
#[ must_use ]
pub fn decompose(
  generics: &syn ::Generics,
) -> (
  syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  syn ::punctuated ::Punctuated< syn ::WherePredicate, syn ::token ::Comma >,
) {
  let mut generics_with_defaults = generics.params.clone();
  crate::punctuated ::ensure_trailing_comma(&mut generics_with_defaults);

  let mut generics_for_impl = syn ::punctuated ::Punctuated ::new();
  let mut generics_for_ty = syn ::punctuated ::Punctuated ::new();

  // Process each generic parameter
  let params_count = generics.params.len();
  for (idx, param) in generics.params.iter().enumerate()
  {
    let is_last = idx == params_count - 1;
    match param
    {
      syn ::GenericParam ::Type(type_param) =>
      {
        // Retain bounds for generics_for_impl, remove defaults
        let impl_param = syn ::GenericParam ::Type(syn ::TypeParam {
          attrs: vec![],
          ident: type_param.ident.clone(),
          colon_token: type_param.colon_token,
          bounds: type_param.bounds.clone(),
          eq_token: None, // Remove default token
          default: None,  // Remove default value
        });
        generics_for_impl.push_value(impl_param);
        if !is_last
        {
          generics_for_impl.push_punct(syn ::token ::Comma ::default());
        }

        // Simplify for generics_for_ty by removing all except identifiers
        let ty_param = syn ::GenericParam ::Type(syn ::TypeParam {
          attrs: vec![],
          ident: type_param.ident.clone(),
          colon_token: None,
          bounds: syn ::punctuated ::Punctuated ::new(),
          eq_token: None,
          default: None,
        });
        generics_for_ty.push_value(ty_param);
        if !is_last
        {
          generics_for_ty.push_punct(syn ::token ::Comma ::default());
        }
      }
      syn ::GenericParam ::Const(const_param) =>
      {
        // Simplify const parameters by removing all details except the identifier
        let impl_param = syn ::GenericParam ::Const(syn ::ConstParam {
          attrs: vec![],
          const_token: const_param.const_token,
          ident: const_param.ident.clone(),
          colon_token: const_param.colon_token,
          ty: const_param.ty.clone(),
          eq_token: None,
          default: None,
        });
        generics_for_impl.push_value(impl_param);
        if !is_last
        {
          generics_for_impl.push_punct(syn ::token ::Comma ::default());
        }

        let ty_param = syn ::GenericParam ::Const(syn ::ConstParam {
          attrs: vec![],
          const_token: const_param.const_token,
          ident: const_param.ident.clone(),
          colon_token: const_param.colon_token,
          ty: const_param.ty.clone(),
          eq_token: None,
          default: None,
        });
        generics_for_ty.push_value(ty_param);
        if !is_last
        {
          generics_for_ty.push_punct(syn ::token ::Comma ::default());
        }
      }
      syn ::GenericParam ::Lifetime(lifetime_param) =>
      {
        // Lifetimes are added as-is to generics_for_impl and without bounds to generics_for_ty
        generics_for_impl.push_value(syn ::GenericParam ::Lifetime(lifetime_param.clone()));
        if !is_last
        {
          generics_for_impl.push_punct(syn ::token ::Comma ::default());
        }

        let ty_param = syn ::GenericParam ::Lifetime(syn ::LifetimeParam {
          attrs: vec![],
          lifetime: lifetime_param.lifetime.clone(),
          colon_token: None,
          bounds: syn ::punctuated ::Punctuated ::new(),
        });
        generics_for_ty.push_value(ty_param);
        if !is_last
        {
          generics_for_ty.push_punct(syn ::token ::Comma ::default());
        }
      }
    }
  }

  // Remove any trailing punctuation from impl and ty generics to prevent trailing commas
  while generics_for_impl.trailing_punct()
  {
    generics_for_impl.pop_punct();
  }
  while generics_for_ty.trailing_punct()
  {
    generics_for_ty.pop_punct();
  }

  // Clone where predicates if present, ensuring they end with a comma
  let generics_where =  if let Some(where_clause) = &generics.where_clause
  {
    let mut predicates = where_clause.predicates.clone();
    crate::punctuated ::ensure_trailing_comma(&mut predicates);
    predicates
  } else {
    syn ::punctuated ::Punctuated ::new()
  };

  (generics_with_defaults, generics_for_impl, generics_for_ty, generics_where)
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

#[ allow( unused_imports ) ]
/// Own namespace of the module.
pub mod own 
{

  use super :: *;

  #[ doc( inline ) ]
  pub use orphan :: *;
  #[ doc( inline ) ]
  pub use crate::generic_params::private :: {
  GenericsRef, GenericsWithWhere,
 };

  // Re-export the moved functions
  pub use super::{ merge, only_names, names, decompose };


  // Classification utilities
  #[ doc( inline ) ]
  pub use crate::generic_params::classification :: {
  GenericsClassification, classify_generics,
  DecomposedClassified, decompose_classified,
 };

  // Filter utilities
  #[ doc( inline ) ]
  pub use crate::generic_params::filter :: {
  filter_params,
  filter_lifetimes, filter_types, filter_consts, filter_non_lifetimes,
 };

  // Combination utilities
  #[ doc( inline ) ]
  pub use crate::generic_params::combine :: {
  merge_params_ordered, params_with_additional, params_from_components,
 };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan 
{

  use super :: *;
  #[ doc( inline ) ]
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed 
{

  use super :: *;
  pub use crate::generic_params;

  #[ doc( inline ) ]
  pub use prelude :: *;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super :: *;
}
