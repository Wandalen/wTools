#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn_meta/latest/clone_dyn_meta/" ) ]
// #![ allow( non_snake_case ) ]
// #![ allow( non_upper_case_globals ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg
(
  any
  (
    feature = "reflect_reflect",
  )
)]
#[ cfg( feature = "enabled" ) ]
mod implementation;
#[ cfg
(
  any
  (
    feature = "reflect_reflect",
  )
)]
#[ cfg( feature = "enabled" ) ]
use implementation::*;

///
/// Reflect structure of any kind.
///
/// ### Sample :: trivial.
///
/// qqq : write, please
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_reflect" ) ]
#[ proc_macro_derive( Reflect, attributes( debug ) ) ]
pub fn derive_reflect( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = reflect::reflect( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
