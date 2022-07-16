/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use derive_tools::IsVariant;
  use proc_macro_tools::exposed::*;

  /// Custom keywords.
  pub mod kw
  {
    super::syn::custom_keyword!( layer );
  }

  /// Kind of element.
  #[ derive( IsVariant, Debug, PartialEq, Eq, Clone, Copy ) ]
  pub enum ElementType
  {
    MicroModule( syn::token::Mod ),
    Layer( kw::layer ),
    Use( syn::token::Use ),
  }

  //

  impl syn::parse::Parse for ElementType
  {

    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let lookahead = input.lookahead1();
      let element_type = match()
      {
        _case if lookahead.peek( syn::token::Mod ) =>
        {
          ElementType::MicroModule( input.parse()? )
        },
        _case if lookahead.peek( syn::token::Use ) =>
        {
          ElementType::Use( input.parse()? )
        },
        _case if lookahead.peek( kw::layer ) =>
        {
          ElementType::Layer( input.parse()? )
        },
        _default =>
        {
          return Err( lookahead.error() )
        },
      };

      Ok( element_type )
    }

  }

  //

  impl quote::ToTokens for ElementType
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use ElementType::*;
      match self
      {
        MicroModule( e ) => e.to_tokens( tokens ),
        Use( e ) => e.to_tokens( tokens ),
        Layer( e ) => e.to_tokens( tokens ),
      }
    }
  }

  ///
  /// Record.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Record
  {
    pub attrs : AttributesOuter,
    pub vis : Visibility,
    pub element_type : ElementType,
    pub elements : syn::punctuated::Punctuated< Pair< AttributesOuter, syn::Path >, syn::token::Comma >,
    pub use_elements : Option< crate::UseTree >,
    pub semi : Option< syn::token::Semi >,
  }

  //

  impl syn::parse::Parse for Record
  {

    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {

      let attrs = input.parse()?;
      let vis = input.parse()?;
      let element_type = input.parse()?;
      let mut elements;
      let mut use_elements = None;

      match element_type
      {
        ElementType::Use( _ ) =>
        {
          use_elements = Some( input.parse()? );
          // println!( "{}", qt!{ #use_elements } );
          elements = syn::punctuated::Punctuated::new();
        },
        _ =>
        {
          let lookahead = input.lookahead1();
          if lookahead.peek( syn::token::Brace )
          {
            let input2;
            let _brace_token = syn::braced!( input2 in input );
            elements = syn::punctuated::Punctuated::parse_terminated( &input2 )?;
          }
          else
          {
            let ident = input.parse()?;
            elements = syn::punctuated::Punctuated::new();
            elements.push( Pair::new( make!(), ident ) );
          }
        },
      }

      let lookahead = input.lookahead1();
      if !lookahead.peek( Token![ ; ] )
      {
        return Err( lookahead.error() );
      }

      let semi = Some( input.parse()? );
      return Ok( Record
      {
        attrs,
        vis,
        element_type,
        elements,
        use_elements,
        semi,
      })

    }

  }

  //

  impl quote::ToTokens for Record
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.attrs.to_tokens( tokens );
      self.vis.to_tokens( tokens );
      self.element_type.to_tokens( tokens );
      self.elements.to_tokens( tokens );
      self.semi.to_tokens( tokens );
    }
  }

  ///
  /// Module-specific item.
  ///

  #[ derive( Debug ) ]
  pub struct Records
  (
    pub Vec< Record >,
  );

  //

  impl syn::parse::Parse for Records
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
    {
      let mut items = vec![];
      while !input.is_empty()
      {
        let item : Record = input.parse()?;
        items.push( item );
      }
      Ok( Self( items ) )
    }
  }

  //

  impl quote::ToTokens for Records
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use proc_macro_tools::quote::TokenStreamExt;
      tokens.append_all( &self.0 )
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::private::
  {
    Record,
    Records,
    ElementType,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
