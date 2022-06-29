
// #[ allow( unused_imports ) ]
// use quote::{ quote };
// #[ allow( unused_imports ) ]
// use syn::{ parse_quote };

#[ allow( unused_imports ) ]
use proc_macro_tools::prelude::*;
#[ allow( unused_imports ) ]
use proc_macro_tools::{ Result };

///
/// Template.
///

pub fn impls( _input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  // let items = syn::parse::< Items2 >( syn::Item )?;

  let result = qt!
  {
  };

  Ok( result )
}
