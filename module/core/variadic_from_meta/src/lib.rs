#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from_meta/latest/variadic_from_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

/// Derive macro for `VariadicFrom`.
#[ proc_macro_derive( VariadicFrom ) ]
pub fn variadic_from_derive( input : TokenStream ) -> TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let name = &ast.ident;

  // For now, just return an empty impl. We will fill this in Increment 3.
  let result = quote!
  {
    // impl From< i32 > for #name
    // {
    //   fn from( value : i32 ) -> Self
    //   {
    //     Self( value )
    //   }
    // }
  };
  result.into()
}