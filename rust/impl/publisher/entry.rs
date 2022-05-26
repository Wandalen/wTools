#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

mod wpublisher;
mod commands;

use std::env;
use wca::instruction;
use wpublisher::*;

//

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

