//! Complex Argument Patterns Example
//!
//! This example demonstrates:
//! - Mixed positional and named arguments
//! - Flag-like arguments (starting with --)
//! - Complex real-world command patterns

use unilang_parser::{ Parser, UnilangParserOptions };

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Mixed positional and named arguments
  println!( "=== Mixed Argument Types ===" );
  let cmd = parser.parse_single_instruction
  (
    "server.deploy production config::\"/etc/app.conf\" replicas::3 --verbose --dry-run"
  )?;

  println!( "Command: {:?}", cmd.command_path_slices );
  println!( "All arguments: {:?}", cmd.positional_arguments );
  println!( "Named arguments: {:?}", cmd.named_arguments );

  // Access different argument types
  if !cmd.positional_arguments.is_empty()
  {
    println!( "First positional argument: {:?}", cmd.positional_arguments[ 0 ] );
  }

  if let Some( config ) = cmd.named_arguments.get( "config" )
  {
    println!( "Config file: {config:?}" );
  }

  if let Some( replicas ) = cmd.named_arguments.get( "replicas" )
  {
    println!( "Replica count: {replicas:?}" );
  }

  // Another example with file operations
  println!( "\n=== File Operation Example ===" );
  let cmd2 = parser.parse_single_instruction
  (
    "file.backup \"/home/user/documents\" destination::\"/backup/daily\" compress::true --incremental"
  )?;

  println!( "Backup command: {:?}", cmd2.command_path_slices );
  println!( "Source (positional): {:?}", cmd2.positional_arguments[ 0 ] );
  println!
  (
    "Destination: {}",
    cmd2.named_arguments
    .get( "destination" )
    .map_or( & "not found".to_string(), | arg | &arg.value ),
  );
  println!
  (
    "Compress: {}",
    cmd2.named_arguments
    .get( "compress" )
    .map_or( & "not found".to_string(), | arg | &arg.value ),
  );

  println!( "\n✓ Complex argument patterns parsing successful!" );
  Ok( () )
}