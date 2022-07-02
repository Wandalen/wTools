
#[ allow( unused_imports ) ]
use quote::{ quote };
#[ allow( unused_imports ) ]
use syn::{ parse_quote };
#[ allow( unused_imports ) ]
use proc_macro_tools::prelude::*;
#[ allow( unused_imports ) ]
// use proc_macro_tools::{ Result, Items };
use proc_macro_tools::{ Result, Many, syn };

///
/// Module-specific item.
///

#[ derive( Debug ) ]
pub struct Items2
(
  pub Vec< syn::Item >,
);

// zzz : use?
// types!
// {
//   ///
//   /// Module-specific item.
//   ///
//
//   #[ derive( Debug, PartialEq, Eq, Clone ) ]
//   pub many Many : syn::Item
// }

impl From< Many< syn::Item > > for Items2
{
  fn from( src : Many< syn::Item > ) -> Self
  {
    Self( src.0 )
  }
}

//

impl syn::parse::Parse for Items2
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

//

impl quote::ToTokens for Items2
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.0.iter().for_each( | item |
    {

      let declare_aliased = quote!
      {
        ( as $Name2 : ident ) =>
        {
          ::impls_index::fn_rename!
          {
            @Name { $Name2 }
            @Fn
            {
              #item
            }
          }
        };
      };

      let name_str = item.name();
      let name_ident = syn::Ident::new( &name_str[ .. ], proc_macro2::Span::call_site() );
      let result = quote!
      {
        #[ deny( unused_macros ) ]
        macro_rules! #name_ident
        {
          #declare_aliased
          () =>
          {
            #item
          };
        }
      };
      // tree_print!( result );
      result.to_tokens( tokens )
    });
  }
}

//

pub fn impls( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let items2 = syn::parse::< Items2 >( input )?;

  let result = quote!
  {
    #items2
  };

  Ok( result )
}
