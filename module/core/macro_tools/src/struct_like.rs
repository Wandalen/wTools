//!
//! Parse structures, like `struct { a : i32 }`.
//!

/// Define a private namespace for all its items.
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  /// Enum to encapsulate either a field from a struct or a variant from an enum.
  #[ derive( Debug, PartialEq, Clone ) ]
  pub enum FieldOrVariant< 'a >
  {
    /// Represents a field within a struct or union.
    Field( &'a syn::Field ),
    /// Represents a variant within an enum.
    Variant( &'a syn::Variant ),
  }

  impl< 'a > Copy for FieldOrVariant< 'a >
  {
  }

  impl< 'a > From< &'a syn::Field > for FieldOrVariant< 'a >
  {
    fn from( field : &'a syn::Field ) -> Self
    {
      FieldOrVariant::Field( field )
    }
  }

  impl< 'a > From< &'a syn::Variant > for FieldOrVariant< 'a >
  {
    fn from( variant : &'a syn::Variant ) -> Self
    {
      FieldOrVariant::Variant( variant )
    }
  }

  impl quote::ToTokens for FieldOrVariant< '_ >
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

  impl< 'a > FieldOrVariant< 'a >
  {

    /// Returns a reference to the attributes of the item.
    #[ must_use ]
    pub fn attrs( &self ) -> &Vec< syn::Attribute >
    {
      match self
      {
        FieldOrVariant::Field( e ) => &e.attrs,
        FieldOrVariant::Variant( e ) => &e.attrs,
      }
    }

    /// Returns a reference to the visibility of the item.
    #[ must_use ]
    pub fn vis( &self ) -> Option< &syn::Visibility >
    {
      match self
      {
        FieldOrVariant::Field( e ) => Some( &e.vis ),
        FieldOrVariant::Variant( _ ) => None,
      }
    }

    /// Returns a reference to the mutability of the item.
    #[ must_use ]
    pub fn mutability( &self ) -> Option< &syn::FieldMutability >
    {
      match self
      {
        FieldOrVariant::Field( e ) => Some( &e.mutability ),
        FieldOrVariant::Variant( _ ) => None,
      }
    }

    /// Returns a reference to the identifier of the item.
    #[ must_use]
    pub fn ident( &self ) -> Option< &syn::Ident >
    {
      match self
      {
        FieldOrVariant::Field( e ) => e.ident.as_ref(),
        FieldOrVariant::Variant( e ) => Some( &e.ident ),
      }
    }

    /// Returns an iterator over elements of the item.
    #[ must_use ]
    pub fn typ( &self ) -> Option< &syn::Type >
    {
      match self
      {
        FieldOrVariant::Field( e ) =>
        {
          Some( &e.ty )
        },
        FieldOrVariant::Variant( _e ) =>
        {
          None
        },
      }
    }

    /// Returns a reference to the fields of the item.
    #[ must_use ]
    pub fn fields( &self ) -> Option< &syn::Fields >
    {
      match self
      {
        FieldOrVariant::Field( _ ) => None,
        FieldOrVariant::Variant( e ) => Some( &e.fields ),
      }
    }

    /// Returns a reference to the discriminant of the item.
    #[ must_use ]
    pub fn discriminant( &self ) -> Option< &( syn::token::Eq, syn::Expr ) >
    {
      match self
      {
        FieldOrVariant::Field( _ ) => None,
        FieldOrVariant::Variant( e ) => e.discriminant.as_ref(),
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
      use syn::{ ItemStruct, ItemEnum, Visibility, Attribute };

      // Parse attributes
      let attributes : Vec< Attribute > = input.call( Attribute::parse_outer )?;
      // Parse visibility
      let visibility : Visibility = input.parse().unwrap_or( syn::Visibility::Inherited );

      // Fork input stream to handle struct/enum keyword without consuming
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Token![ struct ] )
      {
        // Parse ItemStruct
        let mut item_struct : ItemStruct = input.parse()?;
        item_struct.vis = visibility;
        item_struct.attrs = attributes;
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
        // Parse ItemEnum
        let mut item_enum : ItemEnum = input.parse()?;
        item_enum.vis = visibility;
        item_enum.attrs = attributes;
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


    /// Returns an iterator over elements of the item.
    // pub fn elements< 'a >( &'a self ) -> impl IterTrait< 'a, FieldOrVariant< 'a > > + 'a
    pub fn elements< 'a >( &'a self ) -> BoxedIter< 'a, FieldOrVariant< 'a > >
    {
      match self
      {
        StructLike::Unit( _ ) =>
        {
          let empty : Vec< FieldOrVariant< 'a > > = vec![];
          Box::new( empty.into_iter() )
        },
        StructLike::Struct( item ) =>
        {
          let fields = item.fields.iter().map( FieldOrVariant::from );
          Box::new( fields )
        },
        StructLike::Enum( item ) =>
        {
          let variants = item.variants.iter().map( FieldOrVariant::from );
          Box::new( variants )
        },
      }
    }

    /// Returns an iterator over elements of the item.
    #[ must_use ]
    pub fn attrs( &self ) -> &Vec< syn::Attribute >
    {
      match self
      {
        StructLike::Unit( item ) |
        StructLike::Struct( item ) =>
        {
          &item.attrs
        },
        StructLike::Enum( item ) =>
        {
          &item.attrs
        },
      }
    }

    /// Returns an iterator over elements of the item.
    #[ must_use ]
    pub fn vis( &self ) -> &syn::Visibility
    {
      match self
      {
        StructLike::Unit( item ) |
        StructLike::Struct( item ) =>
        {
          &item.vis
        },
        StructLike::Enum( item ) =>
        {
          &item.vis
        },
      }
    }

    /// Returns an iterator over elements of the item.
    #[ must_use ]
    pub fn ident( &self ) -> &syn::Ident
    {
      match self
      {
        StructLike::Unit( item ) |
        StructLike::Struct( item ) =>
        {
          &item.ident
        },
        StructLike::Enum( item ) =>
        {
          &item.ident
        },
      }
    }

    /// Returns an iterator over elements of the item.
    #[ must_use ]
    pub fn generics( &self ) -> &syn::Generics
    {
      match self
      {
        StructLike::Unit( item ) |
        StructLike::Struct( item ) =>
        {
          &item.generics
        },
        StructLike::Enum( item ) =>
        {
          &item.generics
        },
      }
    }

    /// Returns an iterator over fields of the item.
    // pub fn fields< 'a >( &'a self ) -> impl IterTrait< 'a, &'a syn::Field >
    #[ must_use ]
    pub fn fields< 'a >( &'a self ) -> BoxedIter< 'a, &'a syn::Field >
    {
      let result : BoxedIter< 'a, &'a syn::Field > = match self
      {
        StructLike::Unit( _item ) =>
        {
          Box::new( core::iter::empty() )
        },
        StructLike::Struct( item ) =>
        {
          Box::new( item.fields.iter() )
        },
        StructLike::Enum( _item ) =>
        {
          Box::new( core::iter::empty() )
        },
      };
      result
    }

    /// Extracts the name of each field.
    /// # Panics
    /// qqq: docs
    // pub fn field_names< 'a >( &'a self ) -> Option< impl IterTrait< 'a, &'a syn::Ident > + '_ >
    #[ must_use ]
    pub fn field_names( &self ) -> Option< BoxedIter< '_, &syn::Ident >>
    {
      match self
      {
        StructLike::Unit( item ) |
        StructLike::Struct( item ) =>
        {
          item_struct::field_names( item )
        },
        StructLike::Enum( _item ) =>
        {
          let iter = Box::new( self.fields().map( | field | field.ident.as_ref().unwrap() ) );
          Some( iter )
        },
      }
    }

    /// Extracts the type of each field.
    #[ must_use ]
    pub fn field_types( & self )
    -> BoxedIter< '_, & syn::Type >
    // -> std::iter::Map
    // <
    //   std::boxed::Box< dyn _IterTrait< '_, &syn::Field > + 'a >,
    //   impl FnMut( &'a syn::Field ) -> &'a syn::Type + 'a,
    // >
    {
      Box::new( self.fields().map( move | field | &field.ty ) )
    }

    /// Extracts the name of each field.
    // pub fn field_attrs< 'a >( &'a self ) -> impl IterTrait< 'a, &'a Vec< syn::Attribute > >
    #[ must_use ]
    pub fn field_attrs( & self )
    -> BoxedIter< '_, &Vec< syn::Attribute > >
    // -> std::iter::Map
    // <
    //   std::boxed::Box< dyn _IterTrait< '_, &syn::Field > + 'a >,
    //   impl FnMut( &'a syn::Field ) -> &'a Vec< syn::Attribute > + 'a,
    // >
    {
      Box::new( self.fields().map( | field | &field.attrs ) )
    }

    /// Extract the first field.
    #[ must_use ]
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
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::
  {
    StructLike,
    FieldOrVariant,
  };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use super::super::struct_like;

  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
