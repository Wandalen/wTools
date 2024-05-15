//!
//! Parse structures, like `struct { a : i32 }`.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;
  // use interval_adapter::BoundExt;

  #[ derive( Debug, PartialEq ) ]
  pub enum FieldOrVariant
  {
    /// Represents a field within a struct or union.
    Field( syn::Field ),
    /// Represents a variant within an enum.
    Variant( syn::Variant ),
  }

  impl From< syn::Field > for FieldOrVariant
  {
    fn from( field : syn::Field ) -> Self
    {
      FieldOrVariant::Field( field )
    }
  }

  impl From< syn::Variant > for FieldOrVariant
  {
    fn from( variant : syn::Variant ) -> Self
    {
      FieldOrVariant::Variant( variant )
    }
  }

//   impl syn::parse::Parse for FieldOrVariant
//   {
//     fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
//     {
//       let lookahead = input.lookahead1();
//
//       if lookahead.peek( syn::Token![ struct ] ) || lookahead.peek( syn::Token![ union ] )
//       {
//         let field : syn::Field = input.parse()?;
//         Ok( FieldOrVariant::Field( field ) )
//       }
//       else if lookahead.peek( syn::Token![ enum ] )
//       {
//         let variant : syn::Variant = input.parse()?;
//         Ok( FieldOrVariant::Variant( variant ) )
//       }
//       else
//       {
//         Err( lookahead.error() )
//       }
//     }
//   }

  impl quote::ToTokens for FieldOrVariant
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      match self
      {
        FieldOrVariant::Field( item ) =>
        {
          item.to_tokens( tokens );
        },
        FieldOrVariant::Variant( item ) =>
        {
          item.to_tokens( tokens );
        },
      }
    }
  }

  /// Represents various struct-like constructs in Rust code.
  ///
  /// This enum enables differentiation among unit types, structs, and enums, allowing
  /// for syntactic analysis and manipulation within macros. `StructLike` is designed to be
  /// used in macro contexts where behaviors may vary based on the struct-like type being processed.
  ///
  /// Variants:
  /// - `Unit`: Represents unit structs, which are types without any fields or data. Useful in scenarios where
  ///   a type needs to exist but does not hold any data itself, typically used for type-safe markers.
  /// - `Struct`: Represents regular Rust structs that contain fields. This variant is used to handle data structures
  ///   that hold multiple related data pieces together in a named format.
  /// - `Enum`: Represents enums in Rust, which are types that can hold one of multiple possible variants. This is particularly
  ///   useful for type-safe state or option handling without the use of external discriminators.
  ///
  #[ derive( Debug, PartialEq ) ]
  pub enum StructLike
  {
    /// A unit struct with no fields.
    Unit( syn::ItemStruct ),
    /// A typical Rust struct with named fields.
    Struct( syn::ItemStruct ),
    /// A Rust enum, which can be one of several defined variants.
    Enum( syn::ItemEnum ),
  }

  impl From< syn::ItemStruct > for StructLike
  {
    fn from( item_struct : syn::ItemStruct ) -> Self
    {
      if item_struct.fields.is_empty()
      {
        StructLike::Unit( item_struct )
      }
      else
      {
        StructLike::Struct( item_struct )
      }
    }
  }

  impl From< syn::ItemEnum > for StructLike
  {
    fn from( item_enum : syn::ItemEnum ) -> Self
    {
      StructLike::Enum( item_enum )
    }
  }

  impl syn::parse::Parse for StructLike
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let ahead = input.fork();
      let _visibility : Option< syn::Visibility > = ahead.parse().ok(); // Skip visibility

      let lookahead = ahead.lookahead1();
      if lookahead.peek( syn::Token![ struct ] )
      {
        let item_struct : syn::ItemStruct = input.parse()?;
        if item_struct.fields.is_empty()
        {
          Ok( StructLike::Unit( item_struct ) )
        }
        else
        {
          Ok( StructLike::Struct( item_struct ) )
        }
      }
      else if lookahead.peek( syn::Token![ enum ] )
      {
        let item_enum : syn::ItemEnum = input.parse()?;
        Ok( StructLike::Enum( item_enum ) )
      }
      else
      {
        Err( lookahead.error() )
      }
    }
  }

  impl quote::ToTokens for StructLike
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      match self
      {
        StructLike::Unit( item ) | StructLike::Struct( item ) =>
        {
          item.to_tokens( tokens );
        },
        StructLike::Enum( item ) =>
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
        StructLike::Unit( _item ) =>
        {
          Box::new( std::iter::empty() )
        },
        StructLike::Struct( item ) =>
        {
          Box::new( item.fields.iter() )
        },
        StructLike::Enum( _item ) =>
        {
          Box::new( std::iter::empty() )
          // Box::new( item.variants.iter() )
        },
      }
    }

    // xxx
    // /// Returns an iterator over elements of the item.
    // pub fn elements( &self ) -> Box< dyn Iterator< Item = &FieldOrVariant > + '_ >
    // {
    //   match self
    //   {
    //     StructLike::Unit( item ) =>
    //     {
    //       Box::new( std::iter::empty() )
    //     },
    //     StructLike::Struct( item ) =>
    //     {
    //       Box::new( item.fields.iter() )
    //     },
    //     StructLike::Enum( item ) =>
    //     {
    //       Box::new( item.variants.iter() )
    //     },
    //   }
    // }

    /// Extracts the name of each field.
    pub fn field_names( &self ) -> Box< dyn Iterator< Item = Option< &syn::Ident > > + '_ >
    {
      Box::new( self.fields().map( | field | field.ident.as_ref() ) )
    }

    /// Extracts the type of each field.
    pub fn field_types( &self ) -> Box< dyn Iterator< Item = &syn::Type > + '_ >
    {
      Box::new( self.fields().map( | field | &field.ty ) )
    }

    /// Extracts the name of each field.
    pub fn field_attrs( &self ) -> Box< dyn Iterator< Item = &Vec< syn::Attribute > > + '_ >
    {
      Box::new( self.fields().map( | field | &field.attrs ) )
    }

    /// Extract the first field.
    pub fn first_field( &self ) -> Option< &syn::Field >
    {
      self.fields().next()
      // .ok_or( syn_err!( self.span(), "Expects at least one field" ) )
    }

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
