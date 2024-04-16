//!
//! Manipulations on generic arguments.
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
