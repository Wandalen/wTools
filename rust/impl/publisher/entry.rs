#![ cfg_attr( not( feature = "use_std"), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wpublisher/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

mod wpublisher;
mod commands;

#[ cfg( feature = "use_std" ) ]
use std::env;
#[ allow( unused_imports ) ]
use wca::instruction;
#[ allow( unused_imports ) ]
use wpublisher::*;

//

#[ cfg( feature = "use_std" ) ]
fn main() -> Result<(), wtools::error::Error>
{

  let ca_map = commands::commands_form();

  let args = env::args().skip( 1 ).collect::<Vec<String>>();
  let instruction = instruction::instruction_parse()
  .instruction( args.join( " " ).as_str() )
  .perform();

  let result = match ca_map.get( &instruction.command_name )
  {
    Some( command ) => command.perform( &instruction ),
    None => commands::print_help( &ca_map ),
  };

  result
}

#[ cfg( not( feature = "use_std" ) ) ]
fn main()
{
}
