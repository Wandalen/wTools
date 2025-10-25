//! cargo_unilang - Scaffolding and health check tool for unilang CLI projects
//!
//! Prevents common mistakes when using unilang framework:
//! - Custom build.rs (unilang provides this automatically)
//! - Duplicate dependencies (serde_yaml, walkdir, phf)
//! - Deprecated API (CommandRegistry::new())
//!
//! This tool itself is built using unilang, demonstrating correct usage and
//! serving as a reference implementation for CLI rulebook compliance.

#![allow(clippy::all)]

mod commands;
mod templates;
mod checks;

use std::{ env, process };

// Note: cargo_unilang has a commands.yaml file that will be processed by unilang's build.rs
// when this crate is built. This demonstrates that unilang automatically handles YAML processing.
// For simplicity, we use manual command dispatching rather than the Pipeline API.

fn main()
{
  // Exit with appropriate code
  let exit_code = match run()
  {
    Ok( code ) => code,
    Err( e ) =>
    {
      eprintln!( "Error: {}", e );
      1 // General error
    }
  };

  process::exit( exit_code );
}

fn run() -> Result< i32, String >
{
  // Get command line arguments
  let mut args : Vec< String > = env::args().skip( 1 ).collect();

  // If no arguments, show help
  if args.is_empty()
  {
    args.push( ".".to_string() );
  }

  // Parse command name (first argument)
  let command_name = args.get( 0 ).cloned().unwrap_or_else( || ".".to_string() );

  // Parse remaining arguments as key::value pairs
  let params = parse_params( &args[ 1.. ] )?;

  // Dispatch to appropriate handler
  match command_name.as_str()
  {
    "." | ".help" =>
    {
      // General help
      println!( "{}", commands::general_help() );
      Ok( 0 )
    }
    ".new" =>
    {
      // Create new project
      let new_params = commands::NewParams::parse( &params )
        .map_err( |e| format!( "Invalid parameter: {}", e ) )?;
      commands::new::execute( new_params )
    }
    ".new.help" =>
    {
      // Help for .new
      println!( "{}", commands::new_help() );
      Ok( 0 )
    }
    ".check" =>
    {
      // Check existing project
      let check_params = commands::CheckParams::parse( &params )
        .map_err( |e| format!( "Invalid parameter: {}", e ) )?;
      commands::check::execute( check_params )
    }
    ".check.help" =>
    {
      // Help for .check
      println!( "{}", commands::check_help() );
      Ok( 0 )
    }
    _ =>
    {
      eprintln!( "Unknown command: {}", command_name );
      eprintln!( "Run 'cargo_unilang .help' for usage information" );
      Ok( 2 ) // Invalid parameters
    }
  }
}

/// Parse key::value parameters
fn parse_params( args : &[ String ] ) -> Result< Vec< ( String, String ) >, String >
{
  let mut params = Vec::new();

  for arg in args
  {
    // Split on `::`
    if let Some( idx ) = arg.find( "::" )
    {
      let key = arg[ ..idx ].to_string();
      let value = arg[ idx + 2.. ].to_string();
      params.push( ( key, value ) );
    }
    else
    {
      return Err( format!( "Invalid parameter format '{}'. Use param::value format", arg ) );
    }
  }

  Ok( params )
}
