
// #[ allow( unused_imports ) ]
// use quote::{ quote };
// #[ allow( unused_imports ) ]
// use syn::{ parse_quote };

#[ allow( unused_imports ) ]
use proc_macro_tools::prelude::*;
#[ allow( unused_imports ) ]
use proc_macro_tools::{ Result };

// ///
// /// Record.
// ///
//
// pub struct ItemMod
// {
//   pub attrs : Vec< syn::Attribute >,
//   pub vis : syn::Visibility,
//   pub mod_token : syn::token::Mod,
//   pub ident : syn::Ident,
//   pub content : Option< ( syn::token::Brace, Vec< syn::Item > ) >,
//   pub semi : Option< syn::token::Semi >,
// }

///
/// Protocol of modularity unifying interface of a module.
///

pub fn impls( _input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  // let items = syn::parse::< Items2 >( syn::Item )?;

  let result = quote!
  {
  };

  Ok( result )
}
