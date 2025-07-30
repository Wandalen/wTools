//!
//! Functions and structures to handle and manipulate generic parameters using the `syn` crate. It's designed to support macro-driven code generation by simplifying, merging, extracting, and decomposing `syn::Generics`.
//!

/// Define a private namespace for all its items.
mod private {

  use crate::*;
  use crate::IterTrait;
  // use iter_tools::IterTrait;

  /// A `GenericsWithWhere` struct to handle the parsing of Rust generics with an explicit `where` clause.
  ///
  /// This wrapper addresses the limitation in the `syn` crate where parsing `Generics` directly from a `ParseStream`
  /// does not automatically handle associated `where` clauses. By integrating `where` clause parsing into the
  /// `GenericsWithWhere`, this struct provides a seamless way to capture both the generics and their constraints
  /// in scenarios where the `where` clause is crucial for type constraints and bounds in Rust macros and code generation.
  ///
  /// Usage:
  ///
  /// ```
  /// let parsed_generics : macro_tools::generic_params::GenericsWithWhere
  /// = syn::parse_str( "< T : Clone, U : Default = Default1 > where T : Default" ).unwrap();
  /// assert!( parsed_generics.generics.params.len() == 2 );
  /// assert!( parsed_generics.generics.where_clause.is_some() );
  /// ```
  ///

  #[derive(Debug)]
  pub struct GenericsWithWhere {
    /// Syn's generics parameters.
    pub generics: syn::Generics,
  }

  impl GenericsWithWhere {
    /// Unwraps the `GenericsWithWhere` to retrieve the inner `syn::Generics`.
    #[must_use]
    pub fn unwrap(self) -> syn::Generics {
      self.generics
    }

    /// Parses a string to a `GenericsWithWhere`, specifically designed to handle generics syntax with where clauses effectively.
    ///
    /// This function provides a convenient way to parse generic parameters and their associated
    /// `where` clauses from a string slice, returning a `GenericsWithWhere` instance.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice containing the generics and optional `where` clause (e.g., `"<T: Debug> where T: Default"`).
    ///
    /// # Returns
    ///
    /// Returns a `syn::Result` which is `Ok(GenericsWithWhere)` on successful parsing,
    /// or `Err(syn::Error)` if the input string does not conform to valid Rust generics syntax.
    ///
    /// # Errors
    ///
    /// Returns a `syn::Error` if the input string `s` cannot be parsed as valid Rust generics
    /// or a `where` clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use macro_tools::generic_params::GenericsWithWhere;
    ///
    /// let parsed = GenericsWithWhere::parse_from_str( "< T : Clone, U : Default = Default1 > where T : Default" ).unwrap();
    /// assert!( parsed.generics.params.len() == 2 );
    /// assert!( parsed.generics.where_clause.is_some() );
    ///
    /// let parsed_no_where = GenericsWithWhere::parse_from_str( "< T >" ).unwrap();
    /// assert!( parsed_no_where.generics.params.len() == 1 );
    /// assert!( parsed_no_where.generics.where_clause.is_none() );
    ///
    /// let parsed_only_where = GenericsWithWhere::parse_from_str( "where T : Debug" ).unwrap();
    /// assert!( parsed_only_where.generics.params.is_empty() );
    /// assert!( parsed_only_where.generics.where_clause.is_some() );
    /// ```
    pub fn parse_from_str(s: &str) -> syn::Result<GenericsWithWhere> {
      syn::parse_str::<GenericsWithWhere>(s)
    }
  }

  impl syn::parse::Parse for GenericsWithWhere {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
      let generics: syn::Generics = input.parse()?;
      let where_clause: Option<syn::WhereClause> = input.parse()?;

      let mut generics_clone = generics.clone();
      generics_clone.where_clause = where_clause;

      Ok(GenericsWithWhere {
        generics: generics_clone,
      })
    }
  }

  impl quote::ToTokens for GenericsWithWhere {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      self.generics.to_tokens(tokens);
    }
  }

  impl From<GenericsWithWhere> for syn::Generics {
    fn from(g: GenericsWithWhere) -> Self {
      g.generics
    }
  }

  impl From<syn::Generics> for GenericsWithWhere {
    fn from(generics: syn::Generics) -> Self {
      GenericsWithWhere { generics }
    }
  }

  /// A wrapper around a reference to `syn::Generics` to provide convenient helper methods
  /// for generating token streams related to generic parameters.
  ///
  /// This is particularly useful in procedural macros for constructing parts of function
  /// signatures, type paths, and where clauses that involve generics.
  #[derive(Debug, Clone, Copy)]
  pub struct GenericsRef<'a> {
    syn_generics: &'a syn::Generics,
  }

  impl<'a> GenericsRef<'a> {
    /// Creates a new `GenericsRef` from a reference to `syn::Generics`.
    #[must_use]
    pub fn new_borrowed(syn_generics: &'a syn::Generics) -> Self {
      Self { syn_generics }
    }

    /// Creates a new `GenericsRef` from a reference to `syn::Generics`. Alias for `new_borrowed`.
    #[must_use]
    pub fn new(syn_generics: &'a syn::Generics) -> Self {
      Self::new_borrowed(syn_generics)
    }

    /// Returns the `impl_generics` part (e.g., `<T: Trait, 'b, const C: usize>`)
    /// as a `TokenStream` if generics are present, otherwise an empty `TokenStream`.
    ///
    /// This is suitable for use in `impl <#impl_generics> Struct ...` contexts.
    /// It includes bounds and lifetimes.
    #[must_use]
    pub fn impl_generics_tokens_if_any(&self) -> proc_macro2::TokenStream {
      if self.syn_generics.params.is_empty() {
        return quote::quote! {};
      }
      let (impl_g, _, _) = self.syn_generics.split_for_impl();
      quote::quote! { #impl_g }
    }

    /// Returns the `ty_generics` part (e.g., `<T, 'b, C>`) as a `TokenStream`
    /// if generics are present, otherwise an empty `TokenStream`.
    ///
    /// This is suitable for use in type paths like `Struct::<#ty_generics>`.
    /// It includes only the identifiers of the generic parameters (types, lifetimes, consts).
    #[must_use]
    pub fn ty_generics_tokens_if_any(&self) -> proc_macro2::TokenStream {
      if self.syn_generics.params.is_empty() {
        return quote::quote! {};
      }
      let (_, ty_g, _) = self.syn_generics.split_for_impl();
      quote::quote! { #ty_g }
    }

    /// Returns the `where_clause` (e.g., `where T: Trait`) as a `TokenStream`
    /// if a where clause is present in the original generics, otherwise an empty `TokenStream`.
    #[must_use]
    pub fn where_clause_tokens_if_any(&self) -> proc_macro2::TokenStream {
      let (_, _, where_clause) = self.syn_generics.split_for_impl();
      quote::quote! { #where_clause }
    }

    /// Returns a token stream representing a path to a type, including its generic arguments
    /// if present (e.g., `MyType::<T, U>`). If no generics are present, it returns
    /// just the `base_ident`.
    ///
    /// # Arguments
    ///
    /// * `base_ident`: The identifier of the base type (e.g., `MyType`).
    #[must_use]
    pub fn type_path_tokens_if_any(&self, base_ident: &syn::Ident) -> proc_macro2::TokenStream {
      if self.syn_generics.params.is_empty() {
        quote::quote! { #base_ident }
      } else {
        let (_, ty_g, _) = self.syn_generics.split_for_impl();
        quote::quote! { #base_ident #ty_g }
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
    /// use macro_tools::generic_params::{GenericsRef, classify_generics};
    /// use syn::parse_quote;
    ///
    /// let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// let classification = generics_ref.classification();
    ///
    /// assert!(classification.has_mixed);
    /// assert_eq!(classification.lifetimes.len(), 1);
    /// assert_eq!(classification.types.len(), 1);
    /// assert_eq!(classification.consts.len(), 1);
    /// ```
    #[must_use]
    pub fn classification(&self) -> GenericsClassification<'a> {
      classify_generics(self.syn_generics)
    }
    
    /// Get impl generics without lifetimes.
    ///
    /// This method returns the impl generics token stream with lifetime parameters filtered out,
    /// keeping only type and const parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use macro_tools::generic_params::GenericsRef;
    /// use syn::parse_quote;
    ///
    /// let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// let impl_no_lifetimes = generics_ref.impl_generics_no_lifetimes();
    ///
    /// // Result will be: <T: Clone, const N: usize>
    /// ```
    #[must_use]
    pub fn impl_generics_no_lifetimes(&self) -> proc_macro2::TokenStream {
      let filtered = filter_params(&self.syn_generics.params, filter_non_lifetimes);
      if filtered.is_empty() {
        quote::quote! {}
      } else {
        quote::quote! { < #filtered > }
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
    /// use macro_tools::generic_params::GenericsRef;
    /// use syn::parse_quote;
    ///
    /// let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// let ty_no_lifetimes = generics_ref.ty_generics_no_lifetimes();
    ///
    /// // Result will be: <T, N>
    /// ```
    #[must_use]
    pub fn ty_generics_no_lifetimes(&self) -> proc_macro2::TokenStream {
      let (_, _, ty_params, _) = decompose(self.syn_generics);
      let filtered = filter_params(&ty_params, filter_non_lifetimes);
      if filtered.is_empty() {
        quote::quote! {}
      } else {
        quote::quote! { < #filtered > }
      }
    }
    
    /// Check if generics contain only lifetime parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use macro_tools::generic_params::GenericsRef;
    /// use syn::parse_quote;
    ///
    /// let generics: syn::Generics = parse_quote! { <'a, 'b> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// assert!(generics_ref.has_only_lifetimes());
    ///
    /// let generics2: syn::Generics = parse_quote! { <'a, T> };
    /// let generics_ref2 = GenericsRef::new(&generics2);
    /// assert!(!generics_ref2.has_only_lifetimes());
    /// ```
    #[must_use]
    pub fn has_only_lifetimes(&self) -> bool {
      self.classification().has_only_lifetimes
    }
    
    /// Check if generics contain only type parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use macro_tools::generic_params::GenericsRef;
    /// use syn::parse_quote;
    ///
    /// let generics: syn::Generics = parse_quote! { <T, U> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// assert!(generics_ref.has_only_types());
    ///
    /// let generics2: syn::Generics = parse_quote! { <T, const N: usize> };
    /// let generics_ref2 = GenericsRef::new(&generics2);
    /// assert!(!generics_ref2.has_only_types());
    /// ```
    #[must_use]
    pub fn has_only_types(&self) -> bool {
      self.classification().has_only_types
    }
    
    /// Check if generics contain only const parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use macro_tools::generic_params::GenericsRef;
    /// use syn::parse_quote;
    ///
    /// let generics: syn::Generics = parse_quote! { <const N: usize, const M: i32> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// assert!(generics_ref.has_only_consts());
    /// ```
    #[must_use]
    pub fn has_only_consts(&self) -> bool {
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
    /// use macro_tools::generic_params::GenericsRef;
    /// use syn::{parse_quote, Ident};
    /// use quote::format_ident;
    ///
    /// let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    /// let generics_ref = GenericsRef::new(&generics);
    /// let base = format_ident!("MyType");
    /// let path = generics_ref.type_path_no_lifetimes(&base);
    ///
    /// // Result will be: MyType::<T, N>
    /// ```
    #[must_use]
    pub fn type_path_no_lifetimes(&self, base_ident: &syn::Ident) -> proc_macro2::TokenStream {
      let ty_no_lifetimes = self.ty_generics_no_lifetimes();
      if self.syn_generics.params.is_empty() || 
         self.syn_generics.params.iter().all(|p| matches!(p, syn::GenericParam::Lifetime(_))) {
        quote::quote! { #base_ident }
      } else {
        quote::quote! { #base_ident #ty_no_lifetimes }
      }
    }
  }

  /// Merges two `syn::Generics` instances into a new one.
  ///
  /// This function takes two references to `syn::Generics` and combines their
  /// type parameters and where clauses into a new `syn::Generics` instance. If
  /// both instances have where clauses, the predicates of these clauses are merged
  /// into a single where clause.
  ///
  /// # Arguments
  ///
  /// * `a` - A reference to the first `syn::Generics` instance.
  /// * `b` - A reference to the second `syn::Generics` instance.
  ///
  /// # Returns
  ///
  /// Returns a new `syn::Generics` instance containing the merged type parameters
  /// and where clauses from `a` and `b`.
  ///
  /// # Examples
  ///
  ///
  /// # use `syn::{Generics`, `parse_quote`};
  ///
  /// let mut `generics_a` : `syn::Generics` = `parse_quote`!{ < T : Clone, U : Default > };
  /// `generics_a.where_clause` = `parse_quote`!{ where T : Default };
  /// let mut `generics_b` : `syn::Generics` = `parse_quote`!{ < V : `core::fmt::Debug` > };
  /// `generics_b.where_clause` = `parse_quote`!{ where V : Sized };
  /// let got = `generic_params::merge`( &`generics_a`, &`generics_b` );
  ///
  /// let mut exp : `syn::Generics` = `parse_quote`!
  /// {
  ///   < T : Clone, U : Default, V : `core::fmt::Debug` >
  /// };
  /// `exp.where_clause` = `parse_quote`!
  /// {
  ///   where
  ///     T : Default,
  ///     V : Sized
  /// };
  ///
  /// `assert_eq`!( got, exp );
  #[must_use]
  #[allow(clippy::default_trait_access)]
  pub fn merge(a: &syn::Generics, b: &syn::Generics) -> syn::Generics {
    let mut result = syn::Generics {
      params: Default::default(),
      where_clause: None,
      lt_token: Some(syn::token::Lt::default()),
      gt_token: Some(syn::token::Gt::default()),
    };

    // Merge params
    for param in &a.params {
      result.params.push(param.clone());
    }
    for param in &b.params {
      result.params.push(param.clone());
    }

    // Merge where clauses
    result.where_clause = match (&a.where_clause, &b.where_clause) {
      (Some(a_clause), Some(b_clause)) => {
        let mut merged_where_clause = syn::WhereClause {
          where_token: a_clause.where_token,
          predicates: a_clause.predicates.clone(),
        };
        for predicate in &b_clause.predicates {
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
  ///
  /// This function simplifies the generics to include only the names of the type parameters,
  /// lifetimes, and const parameters, without any of their associated bounds or default values.
  /// The resulting `Generics` will have an empty where clause.
  ///
  /// # Arguments
  ///
  /// * `generics` - The `Generics` instance from which to extract parameter names.
  ///
  /// # Returns
  ///
  /// Returns a new `syn::Generics` instance containing only the names of the parameters.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use macro_tools::syn::parse_quote;
  ///
  /// let mut generics : syn::Generics = parse_quote!{ < T : Clone + Default, U, 'a, const N : usize > };
  /// generics.where_clause = parse_quote!{ where T: core::fmt::Debug };
  /// // let generics : Generics = parse_quote!{ < T : Clone + Default, U, 'a, const N : usize > where T: core::fmt::Debug };
  /// let simplified_generics = macro_tools::generic_params::only_names( &generics );
  ///
  /// assert_eq!( simplified_generics.params.len(), 4 ); // Contains T, U, 'a, and N
  /// assert!( simplified_generics.where_clause.is_none() ); // Where clause is removed
  /// ```
  #[allow(clippy::default_trait_access)]
  #[must_use]
  pub fn only_names(generics: &syn::Generics) -> syn::Generics {
    use syn::{Generics, GenericParam, LifetimeParam, TypeParam, ConstParam};

    let result = Generics {
      params: generics
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
        .collect(),
      where_clause: None,
      lt_token: generics.lt_token,
      gt_token: generics.gt_token,
    };

    result
  }

  /// Extracts the names of type parameters, lifetimes, and const parameters from the given `Generics`.
  ///
  /// This function returns an iterator over the names of the parameters in the `Generics`,
  /// which can be useful for generating code that requires just the names of the parameters
  /// without their associated bounds or default values.
  ///
  /// # Arguments
  ///
  /// * `generics` - The `Generics` instance from which to extract parameter names.
  ///
  /// # Returns
  ///
  /// Returns an iterator over the names of the parameters.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use macro_tools::syn::parse_quote;
  ///
  /// let generics : syn::Generics = parse_quote!
  /// {
  ///   < T : Clone + Default, U, 'a, const N : usize >
  /// };
  /// let names : Vec< _ > = macro_tools::generic_params::names( &generics ).collect();
  ///
  /// assert_eq!( names, vec!
  /// [
  ///   &syn::Ident::new( "T", proc_macro2::Span::call_site() ),
  ///   &syn::Ident::new( "U", proc_macro2::Span::call_site() ),
  ///   &syn::Ident::new( "a", proc_macro2::Span::call_site() ),
  ///   &syn::Ident::new( "N", proc_macro2::Span::call_site() )
  /// ]);
  /// ```
  #[must_use]
  pub fn names(generics: &syn::Generics) -> impl IterTrait<'_, &syn::Ident> {
    generics.params.iter().map(|param| match param {
      syn::GenericParam::Type(type_param) => &type_param.ident,
      syn::GenericParam::Lifetime(lifetime_def) => &lifetime_def.lifetime.ident,
      syn::GenericParam::Const(const_param) => &const_param.ident,
    })
  }

  /// Decomposes `syn::Generics` into components suitable for different usage contexts in Rust implementations,
  /// specifically focusing on different requirements for `impl` blocks and type definitions.
  ///
  /// This function prepares three versions of the generics:
  /// - One preserving the full structure for `impl` declarations.
  /// - One simplified for type definitions, removing bounds and defaults from type and const parameters, retaining only identifiers.
  /// - One for the where clauses, if present, ensuring they are correctly punctuated.
  ///
  /// This helps in situations where you need different representations of generics for implementing traits,
  /// defining types, or specifying trait bounds and conditions.
  ///
  /// This function is similar to `syn::Generics::split_for_impl`, which also splits generics into components
  /// suitable for `impl` blocks and type definitions. However, `split_for_impl` wraps the tokens in `<>`, which
  /// can reduce the flexibility of the results. The `decompose` function provides more control over the output
  /// by not wrapping the tokens, allowing for more precise usage in macros and other contexts.
  /// Additionally, `decompose` returns an extra component with the generics including defaults, which is often
  /// in demand for certain macro or code generation tasks.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let code : syn::Generics = syn::parse_quote!{ <'a, T, const N : usize, U : Trait1> };
  /// let ( generics_with_defaults, generics_for_impl, generics_for_ty, generics_where ) = macro_tools::generic_params::decompose( &code );
  ///
  /// // Use in a macro for generating code
  /// macro_tools::qt!
  /// {
  ///   impl < #generics_for_impl > MyTrait for Struct1 < #generics_for_ty >
  ///   where
  ///     #generics_where
  ///   {
  ///     // implementation details...
  ///   }
  /// };
  /// ```
  ///
  /// # Arguments
  ///
  /// * `generics` - A reference to the `syn::Generics` to be decomposed.
  ///
  /// # Returns
  ///
  /// Returns a tuple containing:
  /// - `syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>`: Original generics with defaults, used where full specification is needed.
  /// - `syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>`: Generics for `impl` blocks, retaining bounds but no defaults.
  /// - `syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>`: Simplified generics for type definitions, only identifiers.
  /// - `syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>`: Where clauses, properly punctuated for use in where conditions.
  ///
  /// # Differences from `syn::Generics::split_for_impl`
  ///
  /// While both `decompose` and `split_for_impl` functions split generics into components for `impl` blocks, type definitions, and where clauses,
  /// there are key differences:
  /// - `split_for_impl` wraps the generics in `<>`, which can be limiting when you need to use the generics in a different context or format.
  /// - `decompose` provides raw punctuated generic parameters, offering greater flexibility and control over the output format.
  /// - `decompose` returns an extra component with the generics including defaults, which is often needed for certain macro or code generation tasks.
  ///
  /// # Example of function signature using `decompose`
  ///
  /// ```rust
  /// use macro_tools::{ syn, proc_macro2, qt };
  ///
  /// fn generate_unit
  /// (
  ///   item_name : &syn::Ident,
  ///   generics_with_defaults : syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  ///   generics_impl : syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  ///   generics_ty : syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  ///   generics_where: syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  /// )
  /// -> proc_macro2::TokenStream
  /// {
  ///   qt!
  ///   {
  ///     #[ automatically_derived ]
  ///     impl< #generics_impl > From< i32 > for #item_name< #generics_ty >
  ///     where
  ///       #generics_where
  ///     {
  ///       #[ inline ]
  ///       fn from( src : i32 ) -> Self
  ///       {
  ///         Wrap( src )
  ///       }
  ///     }
  ///   }
  /// }
  /// ```
  ///
  #[allow(clippy::type_complexity)]
  #[must_use]
  pub fn decompose(
    generics: &syn::Generics,
  ) -> (
    syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
  ) {
    let mut generics_with_defaults = generics.params.clone();
    punctuated::ensure_trailing_comma(&mut generics_with_defaults);

    let mut generics_for_impl = syn::punctuated::Punctuated::new();
    let mut generics_for_ty = syn::punctuated::Punctuated::new();

    // Process each generic parameter
    let params_count = generics.params.len();
    for (idx, param) in generics.params.iter().enumerate() {
      let is_last = idx == params_count - 1;
      match param {
        syn::GenericParam::Type(type_param) => {
          // Retain bounds for generics_for_impl, remove defaults
          let impl_param = syn::GenericParam::Type(syn::TypeParam {
            attrs: vec![],
            ident: type_param.ident.clone(),
            colon_token: type_param.colon_token,
            bounds: type_param.bounds.clone(),
            eq_token: None, // Remove default token
            default: None,  // Remove default value
          });
          generics_for_impl.push_value(impl_param);
          if !is_last {
            generics_for_impl.push_punct(syn::token::Comma::default());
          }

          // Simplify for generics_for_ty by removing all except identifiers
          let ty_param = syn::GenericParam::Type(syn::TypeParam {
            attrs: vec![],
            ident: type_param.ident.clone(),
            colon_token: None,
            bounds: syn::punctuated::Punctuated::new(),
            eq_token: None,
            default: None,
          });
          generics_for_ty.push_value(ty_param);
          if !is_last {
            generics_for_ty.push_punct(syn::token::Comma::default());
          }
        }
        syn::GenericParam::Const(const_param) => {
          // Simplify const parameters by removing all details except the identifier
          let impl_param = syn::GenericParam::Const(syn::ConstParam {
            attrs: vec![],
            const_token: const_param.const_token,
            ident: const_param.ident.clone(),
            colon_token: const_param.colon_token,
            ty: const_param.ty.clone(),
            eq_token: None,
            default: None,
          });
          generics_for_impl.push_value(impl_param);
          if !is_last {
            generics_for_impl.push_punct(syn::token::Comma::default());
          }

          let ty_param = syn::GenericParam::Const(syn::ConstParam {
            attrs: vec![],
            const_token: const_param.const_token,
            ident: const_param.ident.clone(),
            colon_token: const_param.colon_token,
            ty: const_param.ty.clone(),
            eq_token: None,
            default: None,
          });
          generics_for_ty.push_value(ty_param);
          if !is_last {
            generics_for_ty.push_punct(syn::token::Comma::default());
          }
        }
        syn::GenericParam::Lifetime(lifetime_param) => {
          // Lifetimes are added as-is to generics_for_impl and without bounds to generics_for_ty
          generics_for_impl.push_value(syn::GenericParam::Lifetime(lifetime_param.clone()));
          if !is_last {
            generics_for_impl.push_punct(syn::token::Comma::default());
          }

          let ty_param = syn::GenericParam::Lifetime(syn::LifetimeParam {
            attrs: vec![],
            lifetime: lifetime_param.lifetime.clone(),
            colon_token: None,
            bounds: syn::punctuated::Punctuated::new(),
          });
          generics_for_ty.push_value(ty_param);
          if !is_last {
            generics_for_ty.push_punct(syn::token::Comma::default());
          }
        }
      }
    }

    // Clone where predicates if present, ensuring they end with a comma
    let generics_where = if let Some(where_clause) = &generics.where_clause {
      let mut predicates = where_clause.predicates.clone();
      punctuated::ensure_trailing_comma(&mut predicates);
      predicates
    } else {
      syn::punctuated::Punctuated::new()
    };

    (generics_with_defaults, generics_for_impl, generics_for_ty, generics_where)
  }

  /// Classification of generic parameters by their type.
  ///
  /// This struct provides a detailed breakdown of generic parameters into their constituent types
  /// (lifetimes, type parameters, and const parameters) and includes convenience flags for common queries.
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools::generic_params;
  /// use syn::parse_quote;
  ///
  /// let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
  /// let classification = generic_params::classify_generics(&generics);
  ///
  /// assert_eq!(classification.lifetimes.len(), 1);
  /// assert_eq!(classification.types.len(), 1);
  /// assert_eq!(classification.consts.len(), 1);
  /// assert!(classification.has_mixed);
  /// ```
  #[derive(Debug, Clone)]
  pub struct GenericsClassification<'a> {
    /// Vector of references to lifetime parameters
    pub lifetimes: Vec<&'a syn::LifetimeParam>,
    /// Vector of references to type parameters
    pub types: Vec<&'a syn::TypeParam>,
    /// Vector of references to const parameters
    pub consts: Vec<&'a syn::ConstParam>,
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
  /// This function analyzes a `syn::Generics` struct and categorizes its parameters
  /// into lifetimes, types, and const parameters, providing useful metadata about
  /// the composition of the generics.
  ///
  /// # Arguments
  ///
  /// * `generics` - A reference to the `syn::Generics` to classify
  ///
  /// # Returns
  ///
  /// A `GenericsClassification` struct containing the categorized parameters and metadata
  ///
  /// # Example
  ///
  /// ```
  /// use macro_tools::generic_params;
  /// use syn::parse_quote;
  ///
  /// let generics: syn::Generics = parse_quote! { <'a, 'b, T> };
  /// let classification = generic_params::classify_generics(&generics);
  ///
  /// assert_eq!(classification.lifetimes.len(), 2);
  /// assert_eq!(classification.types.len(), 1);
  /// assert!(!classification.has_only_lifetimes);
  /// assert!(classification.has_mixed);
  /// ```
  #[must_use]
  pub fn classify_generics(generics: &syn::Generics) -> GenericsClassification<'_> {
    let mut lifetimes = Vec::new();
    let mut types = Vec::new();
    let mut consts = Vec::new();

    for param in &generics.params {
      match param {
        syn::GenericParam::Lifetime(lt) => lifetimes.push(lt),
        syn::GenericParam::Type(ty) => types.push(ty),
        syn::GenericParam::Const(ct) => consts.push(ct),
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
  pub fn filter_lifetimes(param: &syn::GenericParam) -> bool {
    matches!(param, syn::GenericParam::Lifetime(_))
  }

  /// Predicate to filter only type parameters.
  pub fn filter_types(param: &syn::GenericParam) -> bool {
    matches!(param, syn::GenericParam::Type(_))
  }

  /// Predicate to filter only const parameters.
  pub fn filter_consts(param: &syn::GenericParam) -> bool {
    matches!(param, syn::GenericParam::Const(_))
  }

  /// Predicate to filter out lifetime parameters (keeping types and consts).
  pub fn filter_non_lifetimes(param: &syn::GenericParam) -> bool {
    !matches!(param, syn::GenericParam::Lifetime(_))
  }

  /// Extended decomposition result that includes classification and pre-filtered common cases.
  ///
  /// This struct builds upon the basic `decompose` function by providing additional
  /// classification information and pre-computed filtered parameter lists for common use cases.
  #[derive(Debug, Clone)]
  pub struct DecomposedClassified {
    /// Original fields from decompose - generics with defaults preserved and trailing comma
    pub generics_with_defaults: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    /// Original fields from decompose - generics for impl without defaults
    pub generics_impl: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    /// Original fields from decompose - generics for type usage (simplified)
    pub generics_ty: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    /// Original fields from decompose - where clause predicates
    pub generics_where: syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    
    /// Classification information about the original generics
    pub classification: GenericsClassification<'static>,
    
    /// Pre-filtered common cases for convenience
    /// Impl generics containing only type parameters
    pub generics_impl_only_types: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    /// Impl generics with lifetime parameters filtered out
    pub generics_impl_no_lifetimes: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    /// Type generics containing only type parameters
    pub generics_ty_only_types: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    /// Type generics with lifetime parameters filtered out
    pub generics_ty_no_lifetimes: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
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
  /// use macro_tools::generic_params;
  /// use syn::parse_quote;
  ///
  /// let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
  /// let decomposed = generic_params::decompose_classified(&generics);
  ///
  /// assert!(decomposed.classification.has_mixed);
  /// assert_eq!(decomposed.generics_impl_only_types.len(), 1);
  /// assert_eq!(decomposed.generics_impl_no_lifetimes.len(), 2); // T and const N
  /// ```
  #[must_use]
  pub fn decompose_classified(generics: &syn::Generics) -> DecomposedClassified {
    let (with_defaults, impl_params, ty_params, where_clause) = decompose(generics);
    
    // Create an owned classification for the original generics
    // We need to leak the memory to get 'static lifetime, but this is acceptable
    // for the classification use case as these are typically used in proc macros
    let generics_leaked = Box::leak(Box::new(generics.clone()));
    let classification = classify_generics(generics_leaked);
    
    // Pre-compute common filtered cases
    let generics_impl_only_types = filter_params(&impl_params, filter_types);
    let generics_impl_no_lifetimes = filter_params(&impl_params, filter_non_lifetimes);
    let generics_ty_only_types = filter_params(&ty_params, filter_types);
    let generics_ty_no_lifetimes = filter_params(&ty_params, filter_non_lifetimes);
    
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
      for param in params.iter() {
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
      .chain(types.into_iter())
      .chain(consts.into_iter())
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
}

#[doc(inline)]
#[allow(unused_imports)]
pub use own::*;

#[allow(unused_imports)]
/// Own namespace of the module.
pub mod own {

  use super::*;

  #[doc(inline)]
  pub use orphan::*;
  #[doc(inline)]
  pub use private::{
    merge, only_names, names, decompose, GenericsRef, GenericsWithWhere,
    // New utilities
    GenericsClassification, classify_generics, filter_params,
    filter_lifetimes, filter_types, filter_consts, filter_non_lifetimes,
    DecomposedClassified, decompose_classified,
    merge_params_ordered, params_with_additional, params_from_components,
  };
}

/// Orphan namespace of the module.
#[allow(unused_imports)]
pub mod orphan {

  use super::*;
  #[doc(inline)]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[allow(unused_imports)]
pub mod exposed {

  use super::*;
  pub use super::super::generic_params;

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super::{prelude::*};
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[allow(unused_imports)]
pub mod prelude {
  use super::*;
}
