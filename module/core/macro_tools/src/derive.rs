//!
//! Macro helpers around derive macro and structure [`syn::DeriveInput`].
//!

/// Internal namespace.
mod private
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
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use private::
  {
    named_fields,
  };

}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::derive;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}
