#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/reflect_tools_meta/latest/reflect_tools_meta/")]
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
/// Derive macro to generate reflection capabilities for types.
///
/// This procedural macro generates code that enables runtime type introspection
/// for structs. The current implementation is a stub returning empty `TokenStream`,
/// awaiting full Entity trait implementation.
///
/// ### Basic Usage
///
/// ```rust
/// use reflect_tools_meta::Reflect;
///
/// #[ derive( Reflect ) ]
/// struct Person
/// {
///   name: String,
///   age: u32,
/// }
/// ```
///
/// ### Debug Mode
///
/// Use the `#[debug]` attribute to print macro expansion details during compilation:
///
/// ```rust
/// use reflect_tools_meta::Reflect;
///
/// #[ derive( Reflect ) ]
/// #[ debug ]
/// struct Config
/// {
///   host: String,
/// }
/// ```
///
/// ### Implementation Note
///
/// The current implementation is a stub that parses the input struct definition
/// and returns empty `TokenStream`. Full Entity trait generation is pending.
/// See parent `reflect_tools` crate for complete reflection functionality.
///
#[ cfg( all( feature = "enabled", feature = "reflect_derive" ) ) ]
#[ proc_macro_derive( Reflect, attributes( debug ) ) ]
pub fn derive_reflect( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = implementation::reflect::reflect( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
