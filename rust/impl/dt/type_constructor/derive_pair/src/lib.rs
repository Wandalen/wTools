use proc_macro2::{ TokenStream, Ident };
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, Fields, ItemStruct };


#[ proc_macro_derive( Pair ) ]
pub fn derive_make( input: proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let input = parse_macro_input!( input as syn::ItemStruct );


  proc_macro::TokenStream::from( quote!( ) )
}
  