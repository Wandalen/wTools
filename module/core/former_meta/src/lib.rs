#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former_derive_meta/latest/former_derive_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
mod derive;

///
/// Derive macro to generate former for a structure. Former is variation of Builder Pattern.
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_former" ) ]
#[ proc_macro_derive( Former, attributes( perform, default, setter, subformer, alias, doc ) ) ]
pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::former::former( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Macro to implement From for each component of a structre.
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_component_from" ) ]
#[ proc_macro_derive( ComponentFrom, attributes( debug ) ) ]
pub fn component_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::component_from::component_from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
