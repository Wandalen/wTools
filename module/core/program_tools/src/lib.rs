// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/program_tools/latest/program_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#![ allow( unused_imports, dead_code, missing_docs ) ] // xxx : rid off

#[ cfg( feature = "enabled" ) ]
use mod_interface::mod_interface;

// xxx : move is_cicd here
// println!( "MODULES_PATH : {}", env!( "MODULES_PATH" ) );
// println!( "WORKSPACE_PATH : {}", env!( "WORKSPACE_PATH" ) );
// // xxx : add to program_tools::{ path::modules(), path::workspace() }

#[ cfg( feature = "enabled" ) ]
mod_interface!
{

  /// Compile and run a Rust program.
  layer program;

}
