//!
//! Attributes analyzys and manipulation.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  ///
  /// For attribute like `#[former( default = 31 ) ]` return key `default` and value `31`,
  /// as well as syn::Meta as the last element of result tuple.
  ///
  /// ### Basic use-case.
  /// ```rust
  /// use macro_tools::*;
  /// let attr : syn::Attribute = syn::parse_quote!( #[ former( default = 31 ) ] );
  /// // tree_print!( attr );
  /// let got = equation( &attr ).unwrap();
  /// assert_eq!( code_to_str!( got ), "default = 31".to_string() );
  /// ```

  pub fn equation( attr : &syn::Attribute ) -> Result< tokens::Equation >
  {
    let meta = &attr.meta;
    return match meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        let eq : tokens::Equation = syn::parse2( meta_list.tokens.clone() )?;
        Ok( eq )
      }
      _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
    };
  }

  ///
  /// Attribute which is inner.
  ///
  /// For example: `// #![ deny( missing_docs ) ]`.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
  pub struct AttributesInner( pub Vec< syn::Attribute > );

  impl From< Vec< syn::Attribute > > for AttributesInner
  {
    #[ inline( always ) ]
    fn from( src : Vec< syn::Attribute > ) -> Self
    {
      Self( src )
    }
  }

  impl From< AttributesInner > for Vec< syn::Attribute >
  {
    #[ inline( always ) ]
    fn from( src : AttributesInner ) -> Self
    {
      src.0
    }
  }

  impl AttributesInner
  {
    /// Iterator
    pub fn iter( &self ) -> core::slice::Iter< '_, syn::Attribute >
    {
      self.0.iter()
    }
  }

  impl syn::parse::Parse
  for AttributesInner
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      // let mut result : Self = from!();
      let mut result : Self = Default::default();
      loop
      {
        if !input.peek( Token![ # ] ) || !input.peek2( Token![ ! ] )
        {
          break;
        }
        let input2;
        let element = syn::Attribute
        {
          pound_token : input.parse()?,
          style : syn::AttrStyle::Inner( input.parse()? ),
          bracket_token : bracketed!( input2 in input ),
          // path : input2.call( syn::Path::parse_mod_style )?,
          // tokens : input2.parse()?,
          meta : input2.parse()?,
        };
        result.0.push( element );
      }
      Ok( result )
    }
  }

  impl quote::ToTokens
  for AttributesInner
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
    }
  }

  //

  // types!
  // {

    ///
    /// Attribute which is outer.
    ///
    /// For example: `#[ derive( Copy ) ]`.
    ///

    // #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
    // pub many AttributesOuter : syn::Attribute;
    // xxx : apply maybe collection of derives for TDD

    #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
    pub struct AttributesOuter( pub Vec< syn::Attribute > );

  // }

  impl From< Vec< syn::Attribute > > for AttributesOuter
  {
    #[ inline( always ) ]
    fn from( src : Vec< syn::Attribute > ) -> Self
    {
      Self( src )
    }
  }

  impl From< AttributesOuter > for Vec< syn::Attribute >
  {
    #[ inline( always ) ]
    fn from( src : AttributesOuter ) -> Self
    {
      src.0
    }
  }

  impl AttributesOuter
  {
    /// Iterator
    pub fn iter( &self ) -> core::slice::Iter< '_, syn::Attribute >
    {
      self.0.iter()
    }
  }

  impl syn::parse::Parse
  for AttributesOuter
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let mut result : Self = Default::default();
      loop
      {
        if !input.peek( Token![ # ] ) || input.peek2( Token![ ! ] )
        {
          break;
        }
        let input2;
        let element = syn::Attribute
        {
          pound_token : input.parse()?,
          style : syn::AttrStyle::Outer,
          bracket_token : bracketed!( input2 in input ),
          // path : input2.call( syn::Path::parse_mod_style )?,
          // tokens : input2.parse()?,
          meta : input2.parse()?,
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

  // qqq : example?

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
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    equation,
    AttributesInner,
    AttributesOuter,
    AttributedIdent,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

