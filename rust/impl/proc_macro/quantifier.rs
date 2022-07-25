//!
//! Quantifiers like Pair and Many.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::exposed::*;
  use type_constructor::prelude::*;

  ///
  /// Marker saying how to parse several elements of such type in a row.
  ///

  pub trait AsMuchAsPossibleNoDelimiter {}
  // pub trait WhileDelimiter
  // {
  //   type Peek : syn::parse::Peek;
  //   type Delimiter : syn::token::Token + Default + Copy + Into< Self::Peek >;
  // }

//   ///
//   /// Pair of syntax elements.
//   ///
//
//   #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
//   pub struct Pair< T1, T2 >
//   ( pub T1, pub T2 )
//   where
//     T1 : syn::parse::Parse + quote::ToTokens,
//     T2 : syn::parse::Parse + quote::ToTokens,
//   ;

  types!
  {
    ///
    /// Parse as much elements as possible.
    ///

    #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
    pub pair Pair : < T1 : syn::parse::Parse + quote::ToTokens, T2 : syn::parse::Parse + quote::ToTokens >
  }

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

  //

  types!
  {
    ///
    /// Parse as much elements as possible.
    ///

    #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
    pub many Many : < T : quote::ToTokens >
  }

  impl< T > Many< T >
  where
    T : quote::ToTokens + syn::parse::Parse,
  {
    /// Constructor.
    pub fn new() -> Self
    {
      Self( Vec::new() )
    }
    /// Constructor.
    pub fn new_with( src : Vec< T > ) -> Self
    {
      Self( src )
    }
  }

  impl< T > From< Many< T > > for Vec< T >
  where
    T : quote::ToTokens + syn::parse::Parse,
  {
    fn from( src : Many< T > ) -> Self
    {
      src.0
    }
  }

  impl< T > quote::ToTokens
  for Many< T >
  where
    T : quote::ToTokens + syn::parse::Parse,
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
    }
  }

  impl< T > syn::parse::Parse
  for Many< T >
  where
    T : quote::ToTokens + syn::parse::Parse + AsMuchAsPossibleNoDelimiter,
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
    {
      let mut items = vec![];
      while !input.is_empty()
      {
        let item : T = input.parse()?;
        items.push( item );
      }
      Ok( Self( items ) )
    }
  }

// zzz : make that working
//
//   impl< T > syn::parse::Parse
//   for Many< T >
//   where
//     T : quote::ToTokens + syn::parse::Parse + WhileDelimiter,
//   {
//     fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
//     {
//       let mut result = Self::new();
//       loop
//       {
//         let lookahead = input.lookahead1();
//         let token = < T as WhileDelimiter >::Delimiter::default().into();
//         if !lookahead.peek( token )
//         {
//           break;
//         }
//         result.0.push( input.parse()? );
//       }
//       Ok( result )
//     }
//   }
//
//   impl WhileDelimiter for AttributesInner
//   {
//     type Peek = syn::token::Pound;
//     type Delimiter = syn::token::Pound;
//   }
//   impl WhileDelimiter for AttributesOuter
//   {
//     type Peek = syn::token::Pound;
//     type Delimiter = syn::token::Pound;
//   }

  impl syn::parse::Parse
  for Many< AttributesInner >
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let mut result = Self::new();
      loop
      {
        // let lookahead = input.lookahead1();
        if !input.peek( Token![ # ] )
        {
          break;
        }
        result.0.push( input.parse()? );
      }
      Ok( result )
    }
  }

  impl syn::parse::Parse
  for Many< AttributesOuter >
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let mut result = Self::new();
      loop
      {
        // let lookahead = input.lookahead1();
        if !input.peek( Token![ # ] )
        {
          break;
        }
        result.0.push( input.parse()? );
      }
      Ok( result )
    }
  }

  impl AsMuchAsPossibleNoDelimiter for syn::Item {}

  // impl syn::parse::Parse
  // for Many< syn::Item >
  // {
  //   fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  //   {
  //     let mut items = vec![];
  //     while !input.is_empty()
  //     {
  //       let item : syn::Item = input.parse()?;
  //       items.push( item );
  //     }
  //     Ok( Self( items ) )
  //   }
  // }

}

#[ doc( inline ) ]
pub use exposed::*;

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::private::
  {
    Pair,
    Many,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::
  {
    AsMuchAsPossibleNoDelimiter,
  };
}
