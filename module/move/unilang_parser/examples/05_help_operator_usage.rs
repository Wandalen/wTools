//! Help Operator Usage Example
//!
//! This example demonstrates:
//! - Basic help requests with ?
//! - Contextual help with arguments
//! - Help operator positioning rules

use unilang_parser::{ Parser, UnilangParserOptions };

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Basic command help
  println!( "=== Basic Command Help ===" );
  let cmd = parser.parse_single_instruction( "file.copy ?" )?;
  println!( "Command: {:?}", cmd.command_path_slices );
  println!( "Help requested: {:?}", cmd.help_requested );
  println!( "Arguments: {:?}", cmd.positional_arguments );

  assert!( cmd.help_requested );
  assert_eq!( cmd.command_path_slices, [ "file", "copy" ] );

  // Contextual help with arguments
  println!( "\n=== Contextual Help with Arguments ===" );
  let cmd2 = parser.parse_single_instruction( "database.migrate version::1.2.0 ?" )?;
  println!( "Command: {:?}", cmd2.command_path_slices );
  println!( "Help requested: {:?}", cmd2.help_requested );
  println!( "Context arguments: {:?}", cmd2.named_arguments );

  assert!( cmd2.help_requested );
  assert_eq!
  (
    cmd2.named_arguments
    .get( "version" )
    .map( | arg | &arg.value )
    .unwrap(),
    "1.2.0"
  );

  // Namespace help
  println!( "\n=== Namespace Help ===" );
  let cmd3 = parser.parse_single_instruction( "system ?" )?;
  println!( "Namespace: {:?}", cmd3.command_path_slices );
  println!( "Help requested: {:?}", cmd3.help_requested );

  // Help with multiple arguments for context
  println!( "\n=== Help with Multiple Context Arguments ===" );
  let cmd4 = parser.parse_single_instruction
  (
    "server.deploy target::production config::\"/etc/app.yaml\" replicas::5 ?"
  )?;
  println!( "Command: {:?}", cmd4.command_path_slices );
  println!( "Help with context: {:?}", cmd4.named_arguments );
  println!( "Help requested: {:?}", cmd4.help_requested );

  assert!( cmd4.help_requested );
  assert_eq!( cmd4.named_arguments.len(), 3 );

  println!( "\n✓ Help operator usage parsing successful!" );
  Ok( () )
}