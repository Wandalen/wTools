/// Internal namespace.
pub( crate ) mod private
{
  use crate::exposed::*;
  use type_constructor::prelude::*;

  //

  types!
  {

    ///
    /// Attribute which is inner.
    ///
    /// For example: `#![ warn( missing_docs ) ]`.
    ///

    #[ derive( Debug, PartialEq, Eq, Clone ) ]
    pub single AttributeInner : syn::Attribute;

  }

  impl syn::parse::Parse
  for AttributeInner
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let input2;
      Ok( Self( syn::Attribute
      {
        pound_token : input.parse()?,
        style : syn::AttrStyle::Inner( input.parse()? ),
        bracket_token : bracketed!( input2 in input ),
        path : input2.call( syn::Path::parse_mod_style )?,
        tokens : input2.parse()?,
      }))
      // Ok( ( input.call( syn::Attribute::parse_inner )? ).into() )
    }
  }

  impl quote::ToTokens
  for AttributeInner
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.0.to_tokens( tokens );
    }
  }

  //

  types!
  {

    ///
    /// Attribute which is outer.
    ///
    /// For example: `#[ derive( Copy ) ]`.
    ///

    #[ derive( Debug, PartialEq, Eq, Clone ) ]
    pub single AttributeOuter : syn::Attribute;

  }

  impl syn::parse::Parse
  for AttributeOuter
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let input2;
      // println!( "AttributeOuter::parse::a" );
      Ok( Self( syn::Attribute
      {
        pound_token : input.parse()?,
        style : syn::AttrStyle::Outer,
        bracket_token : bracketed!( input2 in input ),
        path : input2.call( syn::Path::parse_mod_style )?,
        tokens : input2.parse()?,
      }))
      // Ok( ( input.call( syn::Attribute::parse_inner )? ).into() )
    }
  }

  impl quote::ToTokens
  for AttributeOuter
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.0.to_tokens( tokens );
    }
  }

  ///
  /// Pair of syntax elements.
  ///

  // zzz : use pair
  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Pair< T1, T2 >
  ( pub T1, pub T2 )
  where
    T1 : syn::parse::Parse + quote::ToTokens,
    T2 : syn::parse::Parse + quote::ToTokens,
  ;

  impl< T1, T2 > Pair< T1, T2 >
  where
    T1 : syn::parse::Parse + quote::ToTokens,
    T2 : syn::parse::Parse + quote::ToTokens,
  {
    /// Constructor.
    pub fn new( src1 : T1, src2 : T2 ) -> Self
    {
      Self( src1, src2 )
    }
  }

  impl< T1, T2 > syn::parse::Parse for Pair< T1, T2 >
  where
    T1 : syn::parse::Parse + quote::ToTokens,
    T2 : syn::parse::Parse + quote::ToTokens,
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Self( input.parse()?, input.parse()? ) )
    }
  }

  impl< T1, T2 > quote::ToTokens for Pair< T1, T2 >
  where
    T1 : syn::parse::Parse + quote::ToTokens,
    T2 : syn::parse::Parse + quote::ToTokens,
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.0.to_tokens( tokens );
      self.1.to_tokens( tokens );
    }
  }

  // zzz : publish module cotainer with good prelude

  ///
  /// Parse as much elements as possible.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Many< T > ( Vec< T > )
  where
    T : quote::ToTokens,
  ;

  impl< T > Many< T >
  where
    T : quote::ToTokens,
  {
    /// Constructor.
    pub fn new() -> Self
    {
      Self( Vec::new() )
    }
  }

  impl< T > quote::ToTokens
  for Many< T >
  where
    T : quote::ToTokens,
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
      // self.0.to_tokens( tokens );
    }
  }

  // xxx : qqq : cover by test
  impl syn::parse::Parse
  for Many< AttributeInner >
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let mut result = Self::new();
      loop
      {
        let lookahead = input.lookahead1();
        if !lookahead.peek( Token![ # ] )
        {
          break;
        }
        result.0.push( input.parse()? );
      }
      Ok( result )
    }
  }

  // xxx : qqq : cover by test
  impl syn::parse::Parse
  for Many< AttributeOuter >
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      // println!( "Many< AttributeOuter >::parse::a" );
      let mut result = Self::new();
      loop
      {
        let lookahead = input.lookahead1();
        if !lookahead.peek( Token![ # ] )
        {
          break;
        }
        // println!( "Many< AttributeOuter >::parse::b" );
        result.0.push( input.parse()? );
      }
      Ok( result )
    }
  }

  // xxx : qqq : cover by test
  impl syn::parse::Parse
  for Many< syn::Item >
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
    {

      let mut items = vec![];
      while !input.is_empty()
      {
        let item : syn::Item = input.parse()?;
        items.push( item );
      }

      Ok( Self( items ) )
    }
  }

//   impl< T > syn::parse::Parse
//   for Many< T >
//   where
//     T : syn::parse::Parse,
//   {
//     fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
//     {
//
//       let mut items = vec![];
//       while !input.is_empty()
//       {
//         let item : T = input.parse()?;
//         items.push( item );
//       }
//
//       Ok( Self( items ) )
//     }
//   }

  // zzz : macro?
  impl< T > core::ops::Deref
  for Many< T >
  where
    T : quote::ToTokens,
  {
    type Target = Vec< T >;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  // zzz : macro?
  impl< T > From< Vec< T > > for Many< T >
  where
    T : quote::ToTokens,
  {
    fn from( src : Vec< T > ) -> Self
    {
      Self( src )
    }
  }

  impl< T > From< Many< T > > for Vec< T >
  where
    T : quote::ToTokens,
  {
    fn from( src : Many< T > ) -> Self
    {
      src.0
    }
  }

  ///
  /// Attribute and ident.
  ///

  pub type AttributedIdent = Pair< Many< AttributeInner >, syn::Ident >;

  impl From< syn::Ident > for AttributedIdent
  {
    fn from( src : syn::Ident ) -> Self
    {
      Self( Vec::new().into(), src )
    }
  }

  impl From< AttributedIdent > for syn::Ident
  {
    fn from( src : AttributedIdent ) -> Self
    {
      src.1
    }
  }

//   impl syn::parse::Parse for AttributedIdent
//   {
//     fn parse( input : ParseStream< '_ > ) -> Result< Self >
//     {
//       Ok( Self( input.parse()?, input.parse()? ) )
//     }
//   }
//
//   impl quote::ToTokens for AttributedIdent
//   {
//     fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
//     {
//       self.0.to_tokens( tokens );
//       self.1.to_tokens( tokens );
//     }
//   }

  ///
  /// Many items.
  ///

  // xxx : use Many instead
  // #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  pub struct Items
  (
    pub Vec< syn::Item >,
  );

  impl syn::parse::Parse for Items
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
    {

      let mut items = vec![];
      while !input.is_empty()
      {
        let item : syn::Item = input.parse()?;
        items.push( item );
      }

      Ok( Self( items ) )
    }
  }

  impl quote::ToTokens for Items
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      // use quote::ToTokens;
      self.0.iter().for_each( | item | item.to_tokens( tokens ) );
    }
  }

}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::private::
  {
    AttributeInner,
    AttributeOuter,
    Pair,
    Many,
    AttributedIdent,
    Items,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
