// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn_meta/latest/clone_dyn_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
mod derive;

///
/// Derive macro to generate former for a structure. Former is variation of Builder Pattern.
///

#[ cfg( feature = "enabled" ) ]
#[ proc_macro_attribute ]
pub fn clone_dyn( _attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::clone_dyn( _attr, item );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
