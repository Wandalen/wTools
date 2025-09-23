#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/clone_dyn_meta/latest/clone_dyn_meta/")]
// #![ allow( non_snake_case ) ]
// #![ allow( non_upper_case_globals ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Reflection tools macro support" ) ]

// #[ cfg( feature = "enabled" ) ]
// use macro_tools ::prelude :: *;

#[ cfg( feature = "enabled" ) ]
mod implementation
{
  #[ cfg( feature = "reflect_derive" ) ]
  pub mod reflect;
}


///
/// Reflect structure of any kind.
///
/// ### Sample `::trivial`.
///
/// qqq: write, please
///
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "reflect_derive" ) ]
#[ proc_macro_derive(Reflect, attributes(debug)) ]
pub fn derive_reflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
  let result = implementation::reflect::reflect(input);
  match result 
  {
  Ok(stream) => stream.into(),
  Err(err) => err.to_compile_error().into(),
 }
}
