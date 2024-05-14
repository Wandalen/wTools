//!
//! Parse structures, like `struct { a : i32 }`.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;
  // use interval_adapter::BoundExt;

  // xxx : raname to Parsed

  /// Represents the outcome of parsing a Rust `struct` definition.
  ///
  /// This structure encapsulates details extracted from a structure definition,
  /// such as the structure itself, its name, and its fields. It provides a comprehensive
  /// view of a parsed structure, facilitating further processing or analysis of its components.
  #[ derive( Debug ) ]
  pub struct TypeStructParsed
  {
    /// The parsed structure item, encompassing the entire `struct`.
    pub item : syn::ItemStruct,
    // /// Identifier of the struct, useful for referencing in generated code.
    // pub item_name : syn::Ident,
  }

  impl TypeStructParsed
  {

    /// Returns a vector of the struct's fields for iteration.
    pub fn fields_many( &self ) -> Vec< &syn::Field >
    {
      match &self.item.fields
      {
        syn::Fields::Unnamed( fields ) => fields.unnamed.iter().collect(),
        syn::Fields::Named( fields ) => fields.named.iter().collect(),
        syn::Fields::Unit => Vec::new(),
      }
    }

    /// Extracts the types of each field into a vector.
    pub fn field_types< 'a >( &'a self ) -> Vec< &'a syn::Type >
    {
      self.fields_many().iter().map( | field | &field.ty ).collect()
    }

    /// Retrieves the names of each field, if they exist.
    pub fn field_names( &self ) -> Option< Vec< syn::Ident > >
    {
      let names: Vec< Option< syn::Ident > > = self.fields_many().iter().map( |field| field.ident.clone() ).collect();
      if names.iter().any( Option::is_none )
      {
        None
      }
      else
      {
        Some( names.into_iter().filter_map( core::convert::identity ).collect() )
      }
    }

    /// Retrieves the type of the first field of the struct.
    ///
    /// Returns the type if the struct has at least one field, otherwise returns an error.
    pub fn first_field_type( &self ) -> Result< syn::Type >
    {
      let maybe_field = match self.item.fields
      {
        syn::Fields::Named( ref fields ) => fields.named.first(),
        syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
        _ => return Err( syn_err!( self.item.fields.span(), "Expects either named or unnamed field" ) ),
      };

      if let Some( field ) = maybe_field
      {
        return Ok( field.ty.clone() )
      }

      return Err( syn_err!( self.item.span(), "Expects at least one field" ) );
    }

    /// Retrieves the name of the first field of the struct, if available.
    ///
    /// Returns `Some` with the field identifier for named fields, or `None` for unnamed fields.
    /// Returns an error if the struct has no fields
    pub fn first_field_name( &self ) -> Result< Option< syn::Ident > >
    {
      let maybe_field = match self.item.fields
      {
        syn::Fields::Named( ref fields ) => fields.named.first(),
        syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
        _ => return Err( syn_err!( self.item.fields.span(), "Expects fields" ) ),
      };

      if let Some( field ) = maybe_field
      {
        return Ok( field.ident.clone() )
      }

      return Err( syn_err!( self.item.span(), "Expects type for fields" ) );
    }
  }

  //

  impl syn::parse::Parse for TypeStructParsed
  {
    // qqq : write proper documentation with examples of input

    // # example of input
    //
    // pub struct IsTransparent( bool );
    //
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let item : syn::ItemStruct = input.parse()?;
      // let item_name = item.ident.clone();
      Ok( Self { item } )
    }
  }

  //

  impl quote::ToTokens for TypeStructParsed
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.item.to_tokens( tokens );
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
  pub use super::private::TypeStructParsed;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as type_struct;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
