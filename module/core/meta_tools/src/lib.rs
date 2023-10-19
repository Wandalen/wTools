#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/meta_tools/latest/meta_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Collection of general purpose meta tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  // #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface;
  #[ cfg( feature = "for_each" ) ]
  pub use ::for_each;
  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index;

  #[ cfg( feature = "collection_make" ) ]
  pub use ::literally;
  #[ cfg( feature = "idents_concat" ) ]
  pub use ::paste;

  // #[ cfg( feature = "former" ) ]
  // pub use ::former;
  // #[ cfg( feature = "options" ) ]
  // pub use ::woptions;

}

//

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{

  layer meta;

}
