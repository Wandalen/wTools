//!
//! Parse structures, like `struct { a : i32 }`.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  // use iter_tools::{ IterTrait, BoxedIter };

  /// Extracts the types of each field into a vector.
  pub fn field_types< 'a >( t : &'a syn::ItemStruct )
  ->
  impl IterTrait< 'a, &'a syn::Type >
  // -> std::iter::Map
  // <
  //   syn::punctuated::Iter< 'a, syn::Field >,
  //   impl FnMut( &'a syn::Field ) -> &'a syn::Type + 'a,
  // >
  {
    t.fields.iter().map( | field | &field.ty )
  }

  /// Retrieves the names of each field, if they exist.
  pub fn field_names< 'a >( t : &'a syn::ItemStruct ) -> Option< BoxedIter< 'a, &'a syn::Ident > >
  {
    match &t.fields
    {
      syn::Fields::Named( fields ) => Some( Box::new( fields.named.iter().map( | field | field.ident.as_ref().unwrap() ) ) ),
      syn::Fields::Unit => Some( Box::new( core::iter::empty() ) ),
      _ => None,
    }
  }

  /// Retrieves the type of the first field of the struct.
  ///
  /// Returns the type if the struct has at least one field, otherwise returns an error.
  pub fn first_field_type( t : &syn::ItemStruct ) -> Result< syn::Type >
  {
    let maybe_field = match t.fields
    {
      syn::Fields::Named( ref fields ) => fields.named.first(),
      syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
      _ => return Err( syn_err!( t.fields.span(), "Expects either named or unnamed field" ) ),
    };

    if let Some( field ) = maybe_field
    {
      return Ok( field.ty.clone() )
    }

    return Err( syn_err!( t.span(), "Expects at least one field" ) );
  }

  /// Retrieves the name of the first field of the struct, if available.
  ///
  /// Returns `Some` with the field identifier for named fields, or `None` for unnamed fields.
  /// Returns an error if the struct has no fields
  pub fn first_field_name( t : &syn::ItemStruct ) -> Result< Option< syn::Ident > >
  {
    let maybe_field = match t.fields
    {
      syn::Fields::Named( ref fields ) => fields.named.first(),
      syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
      _ => return Err( syn_err!( t.fields.span(), "Expects fields" ) ),
    };

    if let Some( field ) = maybe_field
    {
      return Ok( field.ident.clone() )
    }

    return Err( syn_err!( t.span(), "Expects type for fields" ) );
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
    field_types,
    field_names,
    first_field_type,
    first_field_name,
  };
}

/// Orphan namespace of the module.
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
  pub use super::super::item_struct;

  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
