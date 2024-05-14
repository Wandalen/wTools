//!
//! Parse structures, like `struct { a : i32 }`.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;
  // use interval_adapter::BoundExt;

  /// Represents various struct-like constructs.
  /// This enum can differentiate between unit types, structs, and unions,
  /// enabling detailed syntactic analysis and manipulation within macros.
  /// `StructLike` is particularly useful in scenarios where different behaviors
  /// are needed based on the type of struct-like data being parsed.
  #[ derive( Debug ) ]
  pub enum StructLike
  {
    /// Represents a unit type, which is a type without any fields or data.
    Unit,
    /// Represents a Rust struct, containing fields and potentially associated data.
    Struct( syn::ItemStruct ),
    /// Represents a Rust union, useful for when multiple types may occupy the same memory space.
    Union( syn::ItemUnion ),
  }

  impl syn::parse::Parse for StructLike
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let lookahead = input.lookahead1();

      if lookahead.peek( syn::Token![ struct ] )
      {
        let item_struct : syn::ItemStruct = input.parse()?;
        Ok( StructLike::Struct( item_struct ) )
      }
      else if lookahead.peek( syn::Token![ union ] )
      {
        let item_union : syn::ItemUnion = input.parse()?;
        Ok( StructLike::Union( item_union ) )
      }
      else
      {
        Ok( StructLike::Unit )
        // Err( lookahead.error() )
      }
    }
  }

  impl quote::ToTokens for StructLike
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      match self
      {
        StructLike::Unit =>
        {
          quote!( ; ).to_tokens( tokens );
        },
        StructLike::Struct( item ) =>
        {
          item.to_tokens( tokens );
        },
        StructLike::Union( item ) =>
        {
          item.to_tokens( tokens );
        },
      }
    }
  }

  impl StructLike
  {

    /// Returns an iterator over fields of the item.
    pub fn fields( &self ) -> Box< dyn Iterator< Item = &syn::Field > + '_ >
    {
      match self
      {
        StructLike::Unit =>
        {
          Box::new( std::iter::empty() )
        },
        StructLike::Struct( item ) =>
        {
          Box::new( item.fields.iter() )
        },
        StructLike::Union( item ) =>
        {
          Box::new( item.fields.named.iter() )
        },
      }
    }

    /// Extracts the types of each field.
    pub fn field_types( &self ) -> Box< dyn Iterator< Item = &syn::Type > + '_ >
    {
      Box::new( self.fields().map( | field | &field.ty ) )
    }

//     /// Returns a vector of the struct's fields for iteration.
//     pub fn fields_many( &self ) -> Vec< &syn::Field >
//     {
//       match &self.item.fields
//       {
//         syn::Fields::Unnamed( fields ) => fields.unnamed.iter().collect(),
//         syn::Fields::Named( fields ) => fields.named.iter().collect(),
//         syn::Fields::Unit => Vec::new(),
//       }
//     }
//
//     /// Extracts the types of each field into a vector.
//     pub fn field_types< 'a >( &'a self ) -> Vec< &'a syn::Type >
//     {
//       self.fields_many().iter().map( | field | &field.ty ).collect()
//     }
//
//     /// Retrieves the names of each field, if they exist.
//     pub fn field_names( &self ) -> Option< Vec< syn::Ident > >
//     {
//       let names: Vec< Option< syn::Ident > > = self.fields_many().iter().map( |field| field.ident.clone() ).collect();
//       if names.iter().any( Option::is_none )
//       {
//         None
//       }
//       else
//       {
//         Some( names.into_iter().filter_map( core::convert::identity ).collect() )
//       }
//     }
//
//     /// Retrieves the type of the first field of the struct.
//     ///
//     /// Returns the type if the struct has at least one field, otherwise returns an error.
//     pub fn first_field_type( &self ) -> Result< syn::Type >
//     {
//       let maybe_field = match self.item.fields
//       {
//         syn::Fields::Named( ref fields ) => fields.named.first(),
//         syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
//         _ => return Err( syn_err!( self.item.fields.span(), "Expects either named or unnamed field" ) ),
//       };
//
//       if let Some( field ) = maybe_field
//       {
//         return Ok( field.ty.clone() )
//       }
//
//       return Err( syn_err!( self.item.span(), "Expects at least one field" ) );
//     }
//
//     /// Retrieves the name of the first field of the struct, if available.
//     ///
//     /// Returns `Some` with the field identifier for named fields, or `None` for unnamed fields.
//     /// Returns an error if the struct has no fields
//     pub fn first_field_name( &self ) -> Result< Option< syn::Ident > >
//     {
//       let maybe_field = match self.item.fields
//       {
//         syn::Fields::Named( ref fields ) => fields.named.first(),
//         syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
//         _ => return Err( syn_err!( self.item.fields.span(), "Expects fields" ) ),
//       };
//
//       if let Some( field ) = maybe_field
//       {
//         return Ok( field.ident.clone() )
//       }
//
//       return Err( syn_err!( self.item.span(), "Expects type for fields" ) );
//     }

  }

  //

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
  pub use super::private::StructLike;
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
  pub use super::protected as struct_like;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
