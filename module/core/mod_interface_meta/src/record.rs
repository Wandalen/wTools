/// Define a private namespace for all its items.
mod private
{
  use crate::*;
  use macro_tools::exposed::*;

  ///
  /// Custom keywords.
  ///

  pub mod kw
  {
    super::syn::custom_keyword!( layer );
    super::syn::custom_keyword!( reuse );
  }

  ///
  /// Kind of element.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone, Copy ) ]
  pub enum ElementType
  {
    MicroModule( syn::token::Mod ),
    Layer( kw::layer ),
    Use( syn::token::Use ),
    Reuse( kw::reuse ),
  }

  //

  impl syn::parse::Parse for ElementType
  {

    fn parse( input : ParseStream< '_ > ) -> syn::Result< Self >
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
        _case if lookahead.peek( kw::reuse ) =>
        {
          ElementType::Reuse( input.parse()? )
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
        Reuse( e ) => e.to_tokens( tokens ),
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

    fn parse( input : ParseStream< '_ > ) -> syn::Result< Self >
    {

      let attrs = input.parse()?;
      let vis = input.parse()?;
      let element_type = input.parse()?;
      let mut elements;
      let mut use_elements = None;

      match element_type
      {
        ElementType::Use( _ ) | ElementType::Reuse( _ ) =>
        {
          use_elements = Some( input.parse()? );
          elements = syn::punctuated::Punctuated::new();
        },
        _ =>
        {
          if input.peek( syn::token::Brace )
          {
            let input2;
            let _brace_token = syn::braced!( input2 in input );
            elements = syn::punctuated::Punctuated::parse_terminated( &input2 )?;
          }
          else
          {
            let ident = input.parse()?;
            elements = syn::punctuated::Punctuated::new();
            elements.push( Pair::new( Default::default(), ident ) );
          }
        },
      }

      let lookahead = input.lookahead1();
      if !lookahead.peek( Token![ ; ] )
      {
        return Err( lookahead.error() );
      }

      let semi = Some( input.parse()? );
      Ok( Record
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
  /// Many records.
  ///

  pub type Records = Many< Record >;

  impl AsMuchAsPossibleNoDelimiter for Record {}

  ///
  /// Thesis.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Thesis
  {
    pub head : AttributesInner,
    pub records : Records,
  }

  //

  impl Thesis
  {
    /// Validate each inner attribute of the thesis.
    #[ allow ( dead_code ) ]
    pub fn inner_attributes_validate( &self ) -> syn::Result< () >
    {
      self.head.iter().try_for_each( | attr |
      {
        // code_print!( attr );
        // code_print!( attr.path() );
        // code_print!( attr.meta );

        let good = true
          && code_to_str!( attr.path() ) == "debug"
          // && code_to_str!( attr.meta ).is_empty()
        ;

        if !good
        {
          return Err( syn_err!
          (
            attr,
            "Unknown inner attribute:\n{}",
            tree_diagnostics_str!( attr ),
          ));
        }

        syn::Result::Ok( () )
      })?;
      Ok( () )
    }
    /// Does the thesis has debug inner attribute.
    #[ allow ( dead_code ) ]
    pub fn has_debug( &self ) -> bool
    {
      self.head.iter().any( | attr |
      {
        code_to_str!( attr.path() ) == "debug"
      })
    }
  }

  //

  impl syn::parse::Parse for Thesis
  {
    fn parse( input : ParseStream< '_ > ) -> syn::Result< Self >
    {
      let head = input.parse()?;
      // let head = Default::default();
      let records = input.parse()?;
      Ok( Thesis
      {
        head,
        records,
      })
    }
  }

  //

  impl quote::ToTokens for Thesis
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.head.to_tokens( tokens );
      self.records.to_tokens( tokens );
    }
  }

}

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use prelude::*;
  pub use private::
  {
    ElementType,
    Record,
    Records,
    Thesis,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use private::
  {
  };
}
