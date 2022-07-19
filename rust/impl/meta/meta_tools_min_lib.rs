#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/meta_tools_min/latest/meta_tools_min/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// xxx : instead of this module introduce one which encapsulates Options and Former

//!
//! Collection of general purpose meta tools. Minimal Set.
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

}

//

/* mod_interface is the optional dependency, we cannot use it for all cases */
// mod_interface::mod_interface!
// {
//
//   layer meta_min;
//
// }

/// Collection of general purpose meta tools. Minimal Set.
#[ path = "meta_min.rs" ]
pub mod meta;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  pub use super::meta::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::meta::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::meta::prelude::*;
}
