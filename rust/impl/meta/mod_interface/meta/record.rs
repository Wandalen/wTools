/// Internal namespace.
mod internal
{
  // use crate::exposed::*;
  // use super::*;
  use crate::*;
  use proc_macro_tools::exposed::*;

  // use core::hash::{ Hash, Hasher };

  // #[ allow( unused_imports ) ]
  // use proc_macro_tools::prelude::*;
//   #[ allow( unused_imports ) ]
//   use proc_macro_tools::{ Result };
//
//   use proc_macro_tools::syn::
//   {
//     ext::IdentExt,
//     parse::discouraged::Speculative,
//   };
//   use core::hash::{ Hash, Hasher };

  ///
  /// Record.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Record
  {
    pub attrs : Vec< syn::Attribute >,
    pub vis : Visibility,
    pub mod_token : Option< syn::token::Mod >,
    pub elements : Many< AttributedIdent >,
    // pub ident : syn::Ident,
    // pub content : Option< ( syn::token::Brace, Vec< Record > ) >,
    pub semi : Option< syn::token::Semi >,
  }

  //
//
//   pub fn attrs_parse_inner_single( input : ParseStream< '_ > ) -> Result< syn::Attribute >
//   {
//     let input2;
//     Ok( syn::Attribute
//     {
//       pound_token : input.parse()?,
//       style : syn::AttrStyle::Inner( input.parse()? ),
//       bracket_token : bracketed!( input2 in input ),
//       path : input2.call( syn::Path::parse_mod_style )?,
//       tokens : input2.parse()?,
//     })
//   }
//
//   //
//
//   pub fn attrs_parse_inner_as_much_as_possible( input : ParseStream< '_ >, attrs : &mut Vec< syn::Attribute > ) -> Result< () >
//   {
//     while input.peek( Token![ # ] ) && input.peek2( Token![ ! ] )
//     {
//       attrs.push( input.call( attrs_parse_inner_single )? );
//       // attrs.push( input.call( parsing::single_parse_inner )? );
//     }
//     Ok( () )
//   }

  //

  impl syn::parse::Parse for Record
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {

      let attrs = input.call( syn::Attribute::parse_outer )?;
      let vis : Visibility = input.parse()?;

      // let mod_token : Token![ mod ] = input.parse()?;

      let mod_token : Option< Token![ mod ] > = input.parse()?;

  //     if lookahead.peek( syn::token::Brace )
  //     {
  //       let input2;
  //       let brace_token = syn::braced!( input2 in input );
  //       // attrs_parse_inner_as_much_as_possible( &input2, &mut attrs )?;
  //       // xxx : test with attributes
  //
  //       let mut elements = Vec::new();
  //       while !input2.is_empty()
  //       {
  //         elements.push( input2.parse()? );
  //       }
  //
  //       Ok( Record
  //       {
  //         attrs,
  //         vis,
  //         mod_token,
  //         elements,
  //         // ident,
  //         // content : Some( ( brace_token, items ) ),
  //         semi : None,
  //       })
  //     }

      let ident : syn::Ident = input.parse()?;
      let lookahead = input.lookahead1();
      if lookahead.peek( Token![ ; ] )
      {
        Ok( Record
        {
          attrs,
          vis,
          mod_token,
          elements : vec!( ident.into() ).into(),
          // content : None,
          semi : Some( input.parse()? ),
        })
      }
  //     else if lookahead.peek( syn::token::Brace )
  //     {
  //       let input2;
  //       let brace_token = syn::braced!( input2 in input );
  //       attrs_parse_inner_as_much_as_possible( &input2, &mut attrs )?;
  //
  //       let mut items = Vec::new();
  //       while !input2.is_empty()
  //       {
  //         items.push( input2.parse()? );
  //       }
  //
  //       Ok( Record
  //       {
  //         attrs,
  //         vis,
  //         mod_token,
  //         ident,
  //         content : Some( ( brace_token, items ) ),
  //         semi : None,
  //       })
  //     }
      else
      {
        Err( lookahead.error() )
      }

    }
  }

  //

  impl quote::ToTokens for Record
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use proc_macro_tools::quote::TokenStreamExt;
      tokens.append_all( &self.attrs );
      self.vis.to_tokens( tokens );
      self.mod_token.to_tokens( tokens );
      // self.ident.to_tokens( tokens );
      // self.content.to_tokens( tokens );
      self.elements.to_tokens( tokens ); // xxx : problem
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

/// Own namespace of the module.
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
  pub use super::internal::
  {
    Record,
    Records,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {
  };
}
