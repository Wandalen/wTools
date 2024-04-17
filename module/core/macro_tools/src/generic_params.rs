//!
//! Manipulations on generic parameters.
//!
//! # Example of generic parameters
//!
//!```rust
//!
//! pub struct CommandFormer< K, Context = () >
//! where
//!   K : core::hash::Hash + std::cmp::Eq,
//! {
//!   properties : core::option::Option< std::collections::HashMap< K, String > >,
//!   _phantom : core::marker::PhantomData< Context >,
//! }
//!
//! impl< K, Context >
//! CommandFormer< K, Context >
//! where
//!   K : core::hash::Hash + std::cmp::Eq,
//! {}
//!```

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  /// A `GenericsWithWhere` struct to handle the parsing of Rust generics with an explicit `where` clause.
  ///
  /// This wrapper addresses the limitation in the `syn` crate where parsing `Generics` directly from a `ParseStream`
  /// does not automatically handle associated `where` clauses. By integrating `where` clause parsing into the
  /// `GenericsWithWhere`, this struct provides a seamless way to capture both the generics and their constraints
  /// in scenarios where the `where` clause is crucial for type constraints and bounds in Rust macros and code generation.
  ///
  /// Usage:
  /// ```
  /// let parsed_generics : macro_tools::GenericsWithWhere = syn::parse_str( "< T : Clone, U : Default = Default1 > where T : Default").unwrap();
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
  /// let mut generics_b : syn::Generics = parse_quote!{ < V : std::fmt::Debug > };
  /// generics_b.where_clause = parse_quote!{ where V : Sized };
  /// let got = generic_params::merge( &generics_a, &generics_b );
  ///
  /// let mut exp : syn::Generics = parse_quote!
  /// {
  ///   < T : Clone, U : Default, V : std::fmt::Debug >
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

//   // add embedded generic parameters
//   let mut extra_generics : syn::Generics = parse_quote!
//   {
//     < Definition = #former_definition < #generics_ty (), #struct_name, former::ReturnPreformed > >
//     // Definition = Struct1FormerDefinition< (), Struct1, former::ReturnPreformed >,
//     // xxx
//   };
//
//   extra_generics.where_clause = parse_quote!
//   {
//     where
//       Definition : former::FormerDefinition,
//       Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage #generics_ty >,
//       // < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
//   };
//
//   // zzz : write helper to fix bug with where
//   let generics_of_former = generic_params::merge( &generics, &extra_generics );

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
  /// generics.where_clause = parse_quote!{ where T: std::fmt::Debug };
  /// // let generics : Generics = parse_quote!{ < T : Clone + Default, U, 'a, const N : usize > where T: std::fmt::Debug };
  /// let simplified_generics = macro_tools::generic_params::names( &generics );
  ///
  /// assert_eq!( simplified_generics.params.len(), 4 ); // Contains T, U, 'a, and N
  /// assert!( simplified_generics.where_clause.is_none() ); // Where clause is removed
  /// ```

  pub fn names( generics : &syn::Generics ) -> syn::Generics
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

  /// Splits generics into three parts suitable for use in impls, converting to `syn::punctuated::Punctuated` types.
  ///
  /// This function utilizes `syn::Generics::split_for_impl` from the `syn` crate and adapts
  /// the results to return simple `syn::punctuated::Punctuated` structures for generic parameters and
  /// where predicates.
  ///
  /// Split a type’s generics into the pieces required for impl’ing a trait for that type.
  ///
  /// ```rust
  /// let code : syn::Generics = syn::parse_quote!{ < T1, T2 > };
  /// let ( generics_impl, generics_ty, generics_where ) = macro_tools::generic_params::decompose( &code );
  ///
  /// macro_tools::qt!
  /// {
  ///   impl < #generics_impl > MyTrait for Struct1 < #generics_ty >
  ///   where
  ///     #generics_where
  ///   {
  ///     // ...
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
  /// - `syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>` for use with `impl`
  /// - `syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>` for use with type definition
  /// - `syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>` for the where clause
  ///
  pub fn decompose
  (
    generics : &syn::Generics
  )
  ->
  (
    syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
  )
  {
    let mut generics_impl = generics.params.clone();
    punctuated::ensure_trailing_comma( &mut generics_impl );

    let mut generics_ty = generics.params.clone();
    punctuated::ensure_trailing_comma( &mut generics_ty );

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

    ( generics_impl, generics_ty, generics_where )
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    merge,
    names,
    decompose,
  };
}

// xxx : external attr instead of internal?
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
