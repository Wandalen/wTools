//! Basic Command Parsing Example
//!
//! This example demonstrates the fundamental command parsing capabilities:
//! - Simple command paths (namespace.command)
//! - Positional arguments
//! - Command path extraction

use unilang_parser::{ Parser, UnilangParserOptions };

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simple command with namespace
  println!( "=== Simple Command ===" );
  let cmd = parser.parse_single_instruction( "system.info" )?;
  println!( "Command path: {:?}", cmd.command_path_slices );
  println!( "Arguments: {:?}", cmd.positional_arguments );

  // Command with positional arguments
  println!( "\n=== Command with Positional Arguments ===" );
  let cmd = parser.parse_single_instruction( "log.write \"Error occurred\" 5" )?;
  println!( "Command path: {:?}", cmd.command_path_slices );
  println!( "Positional arguments: {:?}", cmd.positional_arguments );

  // Verify the parsing results
  assert_eq!( cmd.command_path_slices, [ "log", "write" ] );
  assert_eq!( cmd.positional_arguments.len(), 2 );

  println!( "\n✓ Basic command parsing successful!" );
  Ok( () )
}