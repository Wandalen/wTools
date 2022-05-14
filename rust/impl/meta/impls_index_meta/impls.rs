use quote::{ quote };
use syn::{ DeriveInput };
// use iter_tools::{ Itertools, process_results };
// use proc_macro_tools::*;

pub type Result< T > = std::result::Result< T, syn::Error >;

//

pub fn impls( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let _ast = match syn::parse::< DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  // let name_ident = &ast.ident;
  // let generics = &ast.generics;
  // let former_name = format!( "{}Former", name_ident );
  // let former_name_ident = syn::Ident::new( &former_name, name_ident.span() );

  let result = quote!
  {
  };

  Ok( result )
}
