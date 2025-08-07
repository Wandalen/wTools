//! Named Arguments and Quoting Example
//!
//! This example demonstrates:
//! - Named arguments with :: separator
//! - Single and double quoted values
//! - Complex strings containing SQL and special characters

use unilang_parser::{ Parser, UnilangParserOptions };

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Named arguments with quoting
  println!( "=== Named Arguments with Quoting ===" );
  let cmd = parser.parse_single_instruction
  (
    r#"database.query sql::"SELECT * FROM users WHERE name = 'John'" timeout::30"#
  )?;

  println!( "Command: {:?}", cmd.command_path_slices );
  println!( "Named arguments:" );
  for ( key, value ) in &cmd.named_arguments
  {
    println!( "  {}: {:?}", key, value );
  }

  // Access specific named arguments
  if let Some( sql ) = cmd.named_arguments.get( "sql" )
  {
    println!( "\nSQL Query: {:?}", sql );
  }
  if let Some( timeout ) = cmd.named_arguments.get( "timeout" )
  {
    println!( "Timeout: {:?}", timeout );
  }

  // Example with single quotes
  println!( "\n=== Single Quote Example ===" );
  let cmd2 = parser.parse_single_instruction( "config.set key::'my_value' priority::high" )?;
  println!( "Config command: {:?}", cmd2.named_arguments );

  println!( "\n✓ Named arguments and quoting parsing successful!" );
  Ok( () )
}