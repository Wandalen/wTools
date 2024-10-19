// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/process_tools/latest/process_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
use mod_interface::mod_interface;

mod private {}

#[ cfg( feature = "enabled" ) ]
mod_interface!
{

  /// Basic functionality.
  // #[ cfg( not( feature = "no_std" ) ) ]
  layer process;

  /// Inspection of running environment.
  // #[ cfg( not( feature = "no_std" ) ) ]
  layer environment;

}
