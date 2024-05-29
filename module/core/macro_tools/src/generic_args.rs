//!
//! This module provides utilities to handle and manipulate generic arguments using the `syn` crate. It includes traits and functions for transforming, merging, and managing generic parameters within procedural macros, enabling seamless syntactic analysis and code generation.
//!

/// Internal namespace.
pub( crate ) mod private
{

  /// A trait for converting a reference to an existing type into a `syn::AngleBracketedGenericArguments`.
  ///
  /// This trait provides a mechanism to transform various types that represent generic parameters,
  /// such as `syn::Generics`, into a uniform `syn::AngleBracketedGenericArguments`. This is particularly
  /// useful when working with Rust syntax trees in procedural macros, allowing for the manipulation
  /// and merging of generic parameters from different syntactic elements.
  pub trait IntoGenericArgs
  {
    /// Converts a reference of the implementing type into `syn::AngleBracketedGenericArguments`.
    ///
    /// This method should handle the conversion logic necessary to transform the implementing
    /// type's generic parameter representations into the structured format required by
    /// `syn::AngleBracketedGenericArguments`, which is commonly used to represent generic parameters
    /// enclosed in angle brackets.
    ///
    /// # Returns
    /// A new instance of `syn::AngleBracketedGenericArguments` representing the generic parameters
    /// of the original type.
    fn into_generic_args( &self ) -> syn::AngleBracketedGenericArguments;
  }

  impl IntoGenericArgs for syn::Generics
  {
    fn into_generic_args( &self ) -> syn::AngleBracketedGenericArguments
    {
      let args = self.params.iter().map( | param |
      {
        match param
        {
          syn::GenericParam::Type( ty ) => syn::GenericArgument::Type( syn::Type::Path( syn::TypePath
          {
            qself: None,
            path: ty.ident.clone().into(),
          })),
          syn::GenericParam::Lifetime( lifetime ) => syn::GenericArgument::Lifetime( lifetime.lifetime.clone() ),
          syn::GenericParam::Const( const_param ) => syn::GenericArgument::Const( syn::Expr::Path( syn::ExprPath
          {
            attrs: vec![],
            qself: None,
            path: const_param.ident.clone().into(),
          })),
        }
      }).collect();

      syn::AngleBracketedGenericArguments
      {
        colon2_token: None,
        lt_token: syn::token::Lt::default(),
        args,
        gt_token: syn::token::Gt::default(),
      }
    }
  }

  /// Merges two `syn::AngleBracketedGenericArguments` instances into a new one,
  /// prioritizing lifetime parameters before other types of generic arguments.
  ///
  /// This function takes two references to `syn::AngleBracketedGenericArguments` and
  /// categorizes their arguments into lifetimes and other types. It then combines
  /// them such that all lifetimes from both instances precede any other arguments in the
  /// resulting `syn::AngleBracketedGenericArguments` instance. This is particularly useful
  /// for ensuring that the merged generics conform to typical Rust syntax requirements where
  /// lifetimes are declared before other generic parameters.
  ///
  /// # Arguments
  ///
  /// * `a` - A reference to the first `syn::AngleBracketedGenericArguments` instance, containing one or more generic arguments.
  /// * `b` - A reference to the second `syn::AngleBracketedGenericArguments` instance, containing one or more generic arguments.
  ///
  /// # Returns
  ///
  /// Returns a new `syn::AngleBracketedGenericArguments` instance containing the merged
  /// arguments from both `a` and `b`, with lifetimes appearing first.
  ///
  /// # Examples
  ///
  /// ```
  /// use macro_tools::{
  ///   generic_args,
  ///   syn::{parse_quote, AngleBracketedGenericArguments},
  /// };
  ///
  /// let a: AngleBracketedGenericArguments = parse_quote! { <'a, T: Clone, U: Default> };
  /// let b: AngleBracketedGenericArguments = parse_quote! { <'b, V: core::fmt::Debug> };
  /// let merged = generic_args::merge(&a, &b);
  ///
  /// let expected: AngleBracketedGenericArguments = parse_quote! { <'a, 'b, T: Clone, U: Default, V: core::fmt::Debug> };
  /// assert_eq!(merged, expected);
  /// ```
  ///
  /// This example demonstrates how lifetimes `'a` and `'b` are placed before other generic parameters
  /// like `T`, `U`, and `V` in the merged result, adhering to the expected syntax order in Rust generics.
  pub fn merge
  (
    a : &syn::AngleBracketedGenericArguments,
    b : &syn::AngleBracketedGenericArguments
  ) -> syn::AngleBracketedGenericArguments
  {
    let mut lifetimes : syn::punctuated::Punctuated< syn::GenericArgument, syn::token::Comma > = syn::punctuated::Punctuated::new();
    let mut others : syn::punctuated::Punctuated< syn::GenericArgument, syn::token::Comma > = syn::punctuated::Punctuated::new();

    // Function to categorize and collect arguments into lifetimes and others
    let mut categorize_and_collect = |args : &syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma>|
    {
      for arg in args.iter()
      {
        match arg
        {
          syn::GenericArgument::Lifetime( _ ) => lifetimes.push( arg.clone() ),
          _ => others.push( arg.clone() ),
        }
      }
    };

    // Categorize and collect from both input arguments
    categorize_and_collect( &a.args );
    categorize_and_collect( &b.args );

    // Combine lifetimes and other arguments into final merged arguments
    let mut args = syn::punctuated::Punctuated::new();
    args.extend( lifetimes );
    args.extend( others );

    syn::AngleBracketedGenericArguments
    {
      colon2_token: None, // Adjust if needed based on context
      lt_token: syn::token::Lt::default(),
      args,
      gt_token: syn::token::Gt::default(),
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{

  //!
  //! This module provides utilities to handle and manipulate generic arguments using the `syn` crate. It includes traits and functions for transforming, merging, and managing generic parameters within procedural macros, enabling seamless syntactic analysis and code generation.
  //!

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    merge,
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
    IntoGenericArgs,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as generic_args;
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
