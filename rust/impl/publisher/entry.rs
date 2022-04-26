#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

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

