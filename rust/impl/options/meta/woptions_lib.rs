// #![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/woptions_meta/latest/woptions_meta/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Mechanism to define map of options for a fuction and its defaults laconically.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// extern crate proc_macro_tools;
// mod former;
// #[ cfg( feature = "use_std" ) ]
mod options;

// ///
// /// Attribute macro to generate options adapter and its implementation for structure option.
// ///
//
// #[ allow( non_snake_case ) ]
// #[ proc_macro_attribute ]
// pub fn Options( attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> proc_macro::TokenStream
// {
//   let result = options::options( attr, item );
//   match result
//   {
//     Ok( stream ) => stream.into(),
//     Err( err ) => err.to_compile_error().into(),
//   }
// }

///
/// Function-like macro to generate options adapter and its implementation for structure option.
///

// #[ cfg( feature = "use_std" ) ]
#[ allow( non_snake_case ) ]
#[ proc_macro ]
pub fn Options( item : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let attr = proc_macro::TokenStream::new();
  let result = options::options( attr, item );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
