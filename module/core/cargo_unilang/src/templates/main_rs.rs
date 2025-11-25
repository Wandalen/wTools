//! main.rs template generator

/// Generate main.rs content for unilang project
pub fn main_rs_minimal() -> &'static str
{
r#"#![allow(clippy::all)]
//! Minimal unilang CLI example
//!
//! This demonstrates the CORRECT way to use unilang:
//! - NO custom build.rs (unilang provides it)
//! - NO serde_yaml, walkdir, phf dependencies (unilang includes them)
//! - Using StaticCommandRegistry::from_commands() (NOT ::new())

use unilang::prelude::*;

// Include compile-time generated commands
// (created automatically by unilang's build.rs - you didn't need to write it!)
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  // Zero-cost static registry (~80ns lookup vs ~4,000ns runtime HashMap)
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Get command from args
  let args : Vec< String > = std::env::args().skip( 1 ).collect();
  let command_str = if args.is_empty()
  {
    ".help".to_string()
  }
  else
  {
    args.join( " " )
  };

  // Execute command with O(1) lookup
  let result = pipeline.process_command_simple( &command_str )?;

  // Print output
  for output in result.outputs
  {
    println!( "{}", output.content );
  }

  Ok( () )
}
"#
}

/// Generate main.rs content for full-featured project
pub fn main_rs_full() -> &'static str
{
r#"#![allow(clippy::all)]
//! Full-featured unilang CLI example
//!
//! This demonstrates advanced unilang usage with error handling,
//! verbosity control, and proper exit codes.

use unilang::prelude::*;
use std::{ env, process };

// Include compile-time generated commands
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main()
{
  let exit_code = match run()
  {
    Ok( () ) => 0,
    Err( e ) =>
    {
      eprintln!( "Error: {}", e );
      1
    }
  };

  process::exit( exit_code );
}

fn run() -> Result< (), unilang::Error >
{
  // Create static registry
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Get command from args
  let args : Vec< String > = env::args().skip( 1 ).collect();
  let command_str = if args.is_empty()
  {
    ".help".to_string()
  }
  else
  {
    args.join( " " )
  };

  // Execute command
  let result = pipeline.process_command_simple( &command_str )?;

  // Print output based on verbosity
  for output in result.outputs
  {
    println!( "{}", output.content );
  }

  // Check for errors
  if !result.success
  {
    return Err( unilang::Error::new( "Command execution failed" ) );
  }

  Ok( () )
}
"#
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_main_rs_minimal_contains_key_elements()
  {
    let content = main_rs_minimal();
    assert!( content.contains( "StaticCommandRegistry::from_commands" ) );
    assert!( content.contains( "include!" ) );
    assert!( content.contains( "STATIC_COMMANDS" ) );
    assert!( content.contains( "NO custom build.rs" ) );
  }

  #[test]
  fn test_main_rs_full_has_error_handling()
  {
    let content = main_rs_full();
    assert!( content.contains( "process::exit" ) );
    assert!( content.contains( "match run()" ) );
  }
}
