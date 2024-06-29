//!
//! Macro helpers around derive macro and structure [`syn::DeriveInput`].
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use syn::punctuated::Punctuated;

  ///
  /// Extracts the named fields from a struct defined in a `syn::DeriveInput`.
  ///
  /// This function specifically handles `syn::DeriveInput` that represent structs
  /// with named fields. It will return an error if the provided AST does not conform to these expectations.
  ///
  /// # Example
  ///
  /// ```rust, ignore
  /// let ast = match syn::parse::< syn::DeriveInput >( input )
  /// {
  ///   Ok( syntax_tree ) => syntax_tree,
  ///   Err( err ) => return Err( err ),
  /// };
  /// let fields = derive.named_fields( &ast );
  /// ```

  pub fn named_fields< 'a >( ast : &'a syn::DeriveInput ) -> crate::Result< &'a Punctuated< syn::Field, syn::token::Comma > >
  {

    let fields = match ast.data
    {
      syn::Data::Struct( ref data_struct ) => match data_struct.fields
      {
        syn::Fields::Named( ref fields_named ) =>
        {
          &fields_named.named
        },
        _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Fields::Named( ref fields_named )\n  {}", qt!{ #ast } ) ),
      },
      _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Data::Struct( ref data_struct )\n  {}", qt!{ #ast } ) ),
    };

    Ok( fields )
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
    named_fields,
  };

}

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::derive;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };

}
