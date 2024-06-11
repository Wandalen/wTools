//!
//! Functions and structures to handle and manipulate generic parameters using the `syn` crate. It's designed to support macro-driven code generation by simplifying, merging, extracting, and decomposing `syn::Generics`.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

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
  /// let parsed_generics : macro_tools::GenericsWithWhere
  /// = syn::parse_str( "< T : Clone, U : Default = Default1 > where T : Default" ).unwrap();
  /// assert!( parsed_generics.generics.params.len() == 2 );
  /// assert!( parsed_generics.generics.where_clause.is_some() );
  /// ```
  ///

  #[ derive( Debug ) ]
  pub struct GenericsWithWhere
  {
    /// Syn's generics parameters.
    pub generics : syn::Generics,
  }

  impl GenericsWithWhere
  {
    /// Unwraps the `GenericsWithWhere` to retrieve the inner `syn::Generics`.
    pub fn unwrap( self ) -> syn::Generics
    {
      self.generics
    }

    /// Parses a string to a `GenericsWithWhere`, specifically designed to handle generics syntax with where clauses effectively.
    pub fn parse_from_str( s : &str ) -> syn::Result< GenericsWithWhere >
    {
      syn::parse_str::< GenericsWithWhere >( s )
    }
  }

  impl syn::parse::Parse for GenericsWithWhere
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let generics : syn::Generics = input.parse()?;
      let where_clause : Option< syn::WhereClause > = input.parse()?;

      let mut generics_clone = generics.clone();
      generics_clone.where_clause = where_clause;

      Ok( GenericsWithWhere
      {
        generics : generics_clone,
      })
    }
  }

  impl quote::ToTokens for GenericsWithWhere
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.generics.to_tokens( tokens );
    }
  }

  impl From<GenericsWithWhere> for syn::Generics
  {
    fn from( g : GenericsWithWhere ) -> Self
    {
      g.generics
    }
  }

  impl From<syn::Generics> for GenericsWithWhere
  {
    fn from( generics : syn::Generics ) -> Self
    {
      GenericsWithWhere { generics }
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
  /// # use syn::{Generics, parse_quote};
  ///
  /// let mut generics_a : syn::Generics = parse_quote!{ < T : Clone, U : Default > };
  /// generics_a.where_clause = parse_quote!{ where T : Default };
  /// let mut generics_b : syn::Generics = parse_quote!{ < V : core::fmt::Debug > };
  /// generics_b.where_clause = parse_quote!{ where V : Sized };
  /// let got = generic_params::merge( &generics_a, &generics_b );
  ///
  /// let mut exp : syn::Generics = parse_quote!
  /// {
  ///   < T : Clone, U : Default, V : core::fmt::Debug >
  /// };
  /// exp.where_clause = parse_quote!
  /// {
  ///   where
  ///     T : Default,
  ///     V : Sized
  /// };
  ///
  /// assert_eq!( got, exp );

  pub fn merge( a : &syn::Generics, b : &syn::Generics ) -> syn::Generics
  {

    let mut result = syn::Generics
    {
      params : Default::default(),
      where_clause : None,
      lt_token : Some( syn::token::Lt::default() ),
      gt_token : Some( syn::token::Gt::default() ),
    };

    // Merge params
    // result.params.extend( a.params.iter().chain( b.params.iter() ) );
    for param in &a.params
    {
      result.params.push( param.clone() );
    }
    for param in &b.params
    {
      result.params.push( param.clone() );
    }

    // Merge where clauses
    result.where_clause = match( &a.where_clause, &b.where_clause )
    {
      ( Some( a_clause ), Some( b_clause ) ) =>
      {
        let mut merged_where_clause = syn::WhereClause
        {
          where_token: a_clause.where_token,
          predicates: a_clause.predicates.clone(),
        };
        for predicate in &b_clause.predicates
        {
          merged_where_clause.predicates.push( predicate.clone() );
        }
        Some( merged_where_clause )
      },
      ( Some( a_clause ), None ) => Some( a_clause.clone() ),
      ( None, Some( b_clause ) ) => Some( b_clause.clone() ),
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
  /// Returns a new `Generics` instance containing only the names of the parameters.
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

  pub fn only_names( generics : &syn::Generics ) -> syn::Generics
  {
    // use syn::{ Generics, GenericParam, LifetimeDef, TypeParam, ConstParam };
    use syn::{ Generics, GenericParam, LifetimeParam, TypeParam, ConstParam };

    let result = Generics
    {
      params : generics.params.iter().map( | param | match param
      {
        GenericParam::Type( TypeParam { ident, .. } ) => GenericParam::Type( TypeParam
        {
          attrs : Vec::new(),
          ident : ident.clone(),
          colon_token : None,
          bounds : Default::default(),
          eq_token : None,
          default : None,
        }),
        GenericParam::Lifetime( LifetimeParam { lifetime, .. } ) => GenericParam::Lifetime( LifetimeParam
        {
          attrs : Vec::new(),
          lifetime : lifetime.clone(),
          colon_token : None,
          bounds : Default::default(),
        }),
        GenericParam::Const( ConstParam { ident, ty, .. } ) => GenericParam::Const( ConstParam
        {
          attrs : Vec::new(),
          const_token : Default::default(),
          ident : ident.clone(),
          colon_token : Default::default(),
          ty : ty.clone(),
          eq_token : Default::default(),
          default : None,
        }),
      }).collect(),
      where_clause : None,
      lt_token : generics.lt_token,
      gt_token : generics.gt_token,
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

  pub fn names< 'a >( generics : &'a syn::Generics )
  -> impl IterTrait< 'a, &'a syn::Ident >
  // -> std::iter::Map
  // <
  //   syn::punctuated::Iter< 'a, syn::GenericParam >,
  //   impl FnMut( &'a syn::GenericParam ) -> &'a syn::Ident + 'a,
  // >
  {
    generics.params.iter().map( | param | match param
    {
      syn::GenericParam::Type( type_param ) => &type_param.ident,
      syn::GenericParam::Lifetime( lifetime_def ) => &lifetime_def.lifetime.ident,
      syn::GenericParam::Const( const_param ) => &const_param.ident,
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

  pub fn decompose
  (
    generics : &syn::Generics,
  )
  ->
  (
    syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  )
  {

    let mut generics_with_defaults = generics.params.clone();
    punctuated::ensure_trailing_comma( &mut generics_with_defaults );

    let mut generics_for_impl = syn::punctuated::Punctuated::new();
    let mut generics_for_ty = syn::punctuated::Punctuated::new();

    // Process each generic parameter
    for param in &generics.params
    {
      match param
      {
        syn::GenericParam::Type( type_param ) =>
        {
          // Retain bounds for generics_for_impl, remove defaults
          let impl_param = syn::GenericParam::Type( syn::TypeParam
          {
            attrs : vec![],
            ident : type_param.ident.clone(),
            colon_token : type_param.colon_token,
            bounds : type_param.bounds.clone(),
            eq_token : None, // Remove default token
            default : None, // Remove default value
          } );
          generics_for_impl.push_value( impl_param );
          generics_for_impl.push_punct( syn::token::Comma::default() );

          // Simplify for generics_for_ty by removing all except identifiers
          let ty_param = syn::GenericParam::Type( syn::TypeParam
          {
            attrs : vec![],
            ident : type_param.ident.clone(),
            colon_token : None,
            bounds : syn::punctuated::Punctuated::new(),
            eq_token : None,
            default : None,
          } );
          generics_for_ty.push_value( ty_param );
          generics_for_ty.push_punct( syn::token::Comma::default() );
        },
        syn::GenericParam::Const( const_param ) =>
        {
          // Simplify const parameters by removing all details except the identifier
          let impl_param = syn::GenericParam::Const( syn::ConstParam
          {
            attrs : vec![],
            const_token : const_param.const_token,
            ident : const_param.ident.clone(),
            colon_token : const_param.colon_token,
            ty : const_param.ty.clone(),
            eq_token : None,
            default : None,
          } );
          generics_for_impl.push_value( impl_param );
          generics_for_impl.push_punct( syn::token::Comma::default() );

          let ty_param = syn::GenericParam::Type( syn::TypeParam
          {
            attrs : vec![],
            ident : const_param.ident.clone(),
            colon_token : None,
            bounds : syn::punctuated::Punctuated::new(),
            eq_token : None,
            default : None,
          });
          generics_for_ty.push_value( ty_param );
          generics_for_ty.push_punct( syn::token::Comma::default() );
        },
        syn::GenericParam::Lifetime( lifetime_param ) =>
        {
          // Lifetimes are added as-is to generics_for_impl and without bounds to generics_for_ty
          generics_for_impl.push_value( syn::GenericParam::Lifetime( lifetime_param.clone() ) );
          generics_for_impl.push_punct( syn::token::Comma::default() );

          let ty_param = syn::GenericParam::Lifetime( syn::LifetimeParam
          {
            attrs : vec![],
            lifetime : lifetime_param.lifetime.clone(),
            colon_token : None,
            bounds : syn::punctuated::Punctuated::new(),
          });
          generics_for_ty.push_value( ty_param );
          generics_for_ty.push_punct( syn::token::Comma::default() );
        }
      }
    }

    // Clone where predicates if present, ensuring they end with a comma
    let generics_where = if let Some( where_clause ) = &generics.where_clause
    {
      let mut predicates = where_clause.predicates.clone();
      punctuated::ensure_trailing_comma( &mut predicates );
      predicates
    }
    else
    {
      syn::punctuated::Punctuated::new()
    };

    ( generics_with_defaults, generics_for_impl, generics_for_ty, generics_where )
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

pub mod protected
{

  //!
  //! Functions and structures to handle and manipulate generic parameters using the `syn` crate. It's designed to support macro-driven code generation by simplifying, merging, extracting, and decomposing `syn::Generics`.
  //!

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    merge,
    only_names,
    names,
    decompose,
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    GenericsWithWhere,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as generic_params;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
