// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former_derive_meta/latest/former_derive_meta/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Former - a variation of builder pattern.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// use macro_tools::prelude::*;

// #[ cfg( not( feature = "no_std" ) ) ]
mod former_impl;

///
/// Derive macro to generate former for a structure. Former is variation of Builder Pattern.
///

// #[ cfg( not( feature = "no_std" ) ) ]
#[ proc_macro_derive( Former, attributes( perform, default, setter, alias, doc ) ) ]
pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = former_impl::former( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
