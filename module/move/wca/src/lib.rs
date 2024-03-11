#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wca/latest/wca/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "doc/", "wca.md" ) ) ]

#![ allow( where_clauses_object_safety ) ] // https://github.com/chris-morgan/anymap/issues/31

use mod_interface::mod_interface;
/// Tools
pub mod wtools;

// qqq : maybe remove this?
// /// Errors.
// #[ cfg( not( feature = "no_std" ) ) ]
// use wtools::error::BasicError;
// xxx : check

crate::mod_interface!
{
  /// Commands aggregator library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer ca;
}
