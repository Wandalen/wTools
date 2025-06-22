#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
#![ allow( clippy::mod_module_files ) ]

/// Internal namespace.
mod private
{
}

///
#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{

  /// Namespace with dependencies.
  #[ cfg( feature = "enabled" ) ]
  pub mod dependency
  {
  }

  // use super::private as i;
  // pub use i::exposed::*;
  // pub use i::prelude::*;

}
