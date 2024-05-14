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

  // impl quote::spanned::Spanned for StructLike
  // {
  //   fn span( &self ) -> proc_macro2::Span
  //   {
  //     match self
  //     {
  //       StructLike::Unit =>
  //       {
  //         // You might want to return a default or dummy span since Unit types generally don't have associated spans.
  //         proc_macro2::Span::call_site()
  //       },
  //       StructLike::Struct( item_struct ) =>
  //       {
  //         // Delegate to the span of the `ItemStruct`
  //         quote::spanned::Spanned::span( item_struct )
  //       },
  //       StructLike::Union( item_union ) =>
  //       {
  //         // Delegate to the span of the `ItemUnion`
  //         quote::spanned::Spanned::span( item_union )
  //       },
  //     }
  //   }
  // }

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

    /// Extracts the type of each field.
    pub fn field_types( &self ) -> Box< dyn Iterator< Item = &syn::Type > + '_ >
    {
      Box::new( self.fields().map( | field | &field.ty ) )
    }

    /// Extracts the name of each field.
    pub fn field_names( &self ) -> Box< dyn Iterator< Item = Option< &syn::Ident > > + '_ >
    {
      Box::new( self.fields().map( | field | field.ident.as_ref() ) )
    }

    /// Extract the first field.
    pub fn first_field( &self ) -> Option< &syn::Field >
    {
      self.fields().next()
      // .ok_or( syn_err!( self.span(), "Expects at least one field" ) )
    }

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
