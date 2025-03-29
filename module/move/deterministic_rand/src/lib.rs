#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/deterministic_rand/latest/deterministic_rand/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use mod_interface::mod_interface;

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "determinism" ) ]
pub mod hrng_deterministic;
#[ cfg( any( not( feature = "determinism" ), feature = "no_std" ) ) ]
pub mod hrng_non_deterministic;

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "determinism" ) ]
pub use hrng_deterministic as hrng;
#[ cfg( any( not( feature = "determinism" ), feature = "no_std" ) ) ]
pub use hrng_non_deterministic as hrng;

mod private {}

mod_interface!
{

  own use ::rand::*;

  use super::hrng;

  // xxx : make it working
  // #[ cfg( feature = "determinism" ) ]
  // use super::hrng_deterministic as hrng;
  // #[ cfg( not( feature = "determinism" ) ) ]
  // use super::hrng_non_deterministic as hrng;

  // xxx : make it working
  // #[ cfg( feature = "determinism" ) ]
  // layer hrng_deterministic as hrng;
  // #[ cfg( not( feature = "determinism" ) ) ]
  // layer hrng_non_deterministic as hrng;

  layer iter;
  #[ cfg( not( feature = "no_std" ) ) ]
  layer seed;
}
