// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/impls_index_meta/latest/impls_index_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
mod impls;

///
/// Macros to put each function under a named macro to index every function in a class.
///

// xxx : make it default impls implementation
#[ cfg( feature = "enabled" ) ]
#[ proc_macro ]
pub fn impls3( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = impls::impls( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
