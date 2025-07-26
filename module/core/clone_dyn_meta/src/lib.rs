#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/clone_dyn_meta/latest/clone_dyn_meta/")]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
mod internal {}

/// Derive macro for `CloneDyn` trait.
///
/// It is a procedural macro that generates an implementation of the `CloneDyn` trait for a given type.
///
/// ### Sample.
///
/// ```rust
/// #[ cfg( feature = "derive_clone_dyn" ) ]
/// #[ clone_dyn ]
/// pub trait Trait1
/// {
///   fn f1( &self );
/// }
///
/// #[ cfg( feature = "derive_clone_dyn" ) ]
/// #[ clone_dyn ]
/// pub trait Trait2 : Trait1
/// {
///   fn f2( &self );
/// }
/// ```
///
/// To learn more about the feature, study the module [`clone_dyn`](https://docs.rs/clone_dyn/latest/clone_dyn/).
#[proc_macro_attribute]
pub fn clone_dyn(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let result = clone_dyn::clone_dyn(attr, item);
  match result {
    Ok(stream) => stream.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

/// Implementation of `clone_dyn` macro.
mod clone_dyn;
