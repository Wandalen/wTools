#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/meta_tools/latest/meta_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of general purpose meta tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Dependencies.
pub mod dependency
{

  #[ cfg( feature = "for_each" ) ]
  pub use ::for_each;
  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index;
  #[ cfg( feature = "collection_make" ) ]
  pub use ::literally;
  #[ cfg( feature = "idents_concat" ) ]
  pub use ::paste;

  #[ cfg( feature = "former" ) ]
  pub use ::former;
  #[ cfg( feature = "options" ) ]
  pub use ::woptions;

}

//

mod_interface::mod_interface!
{

  layer meta;

}
