/// Internal namespace.
pub( crate ) mod private
{
  // xxx : replace all `use crate::*;`
  use type_constructor::prelude::*;
  use crate::exposed::*;
  use crate::exposed::{ Pair, Many };

  // =

  types!
  {

    ///
    /// Attribute which is inner.
    ///
    /// For example: `#![ warn( missing_docs ) ]`.
    ///

    #[ derive( Debug, PartialEq, Eq, Clone ) ]
    pub many AttributesInner : syn::Attribute;

  }

  impl syn::parse::Parse
  for AttributesInner
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let input2;
      Ok( Self( vec![ syn::Attribute
      {
        pound_token : input.parse()?,
        style : syn::AttrStyle::Inner( input.parse()? ),
        bracket_token : bracketed!( input2 in input ),
        path : input2.call( syn::Path::parse_mod_style )?,
        tokens : input2.parse()?,
      }]))
    }
  }

  impl quote::ToTokens
  for AttributesInner
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
      // self.0.to_tokens( tokens );
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
    pub many AttributesOuter : syn::Attribute;

  }

  impl syn::parse::Parse
  for AttributesOuter
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let mut result : Self = make!();
      loop
      {
        // let lookahead = input.lookahead1();
        if !input.peek( Token![ # ] )
        {
          break;
        }
        let input2;
        let element = syn::Attribute
        {
          pound_token : input.parse()?,
          style : syn::AttrStyle::Outer,
          bracket_token : bracketed!( input2 in input ),
          path : input2.call( syn::Path::parse_mod_style )?,
          tokens : input2.parse()?,
        };
        result.0.push( element );
      }
      Ok( result )
    }
  }

  impl quote::ToTokens
  for AttributesOuter
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
    }
  }

  ///
  /// Attribute and ident.
  ///

  pub type AttributedIdent = Pair< Many< AttributesInner >, syn::Ident >;

  impl From< syn::Ident > for AttributedIdent
  {
    fn from( src : syn::Ident ) -> Self
    {
      Self( Vec::< AttributesInner >::new().into(), src )
    }
  }

  impl From< AttributedIdent > for syn::Ident
  {
    fn from( src : AttributedIdent ) -> Self
    {
      src.1
    }
  }

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
    AttributesInner,
    AttributesOuter,
    AttributedIdent,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

