//! Multiple Instructions Example
//!
//! This example demonstrates :
//! - Parsing command sequences separated by ;;
//! - Processing multiple commands in a single input
//! - Real-world workflow scenarios

use unilang_parser :: { Parser, UnilangParserOptions };

fn main() -> Result< (), Box< dyn core ::error ::Error > >
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Parse command sequence
  println!( "=== Multiple Instructions Sequence ===" );
  let instructions = parser.parse_multiple_instructions
  (
  "backup.create name ::daily ;; cloud.upload file ::daily.tar.gz ;; notify.send \"Backup complete\""
 )?;

  println!( "Parsed {} instructions: ", instructions.len() );

  for ( i, instruction ) in instructions.iter().enumerate()
  {
  println!( "\nInstruction {} : {:?}", i + 1, instruction.command_path_slices );
  if !instruction.positional_arguments.is_empty()
  {
   println!( "  Positional args: {:?}", instruction.positional_arguments );
 }
  if !instruction.named_arguments.is_empty()
  {
   println!( "  Named args: {:?}", instruction.named_arguments );
 }
 }

  // Verify specific instructions
  assert_eq!( instructions.len(), 3 );
  assert_eq!( instructions[ 0 ].command_path_slices, [ "backup", "create" ] );
  assert_eq!( instructions[ 1 ].command_path_slices, [ "cloud", "upload" ] );
  assert_eq!( instructions[ 2 ].command_path_slices, [ "notify", "send" ] );

  // Another example: Development workflow
  println!( "\n=== Development Workflow Example ===" );
  let dev_workflow = parser.parse_multiple_instructions
  (
  "git.add . ;; git.commit message :: \"Update parser\" ;; git.push origin ::main ;; deploy.staging"
 )?;

  for ( i, cmd ) in dev_workflow.iter().enumerate()
  {
  println!
  (
   "Step {} : {} with args {:?}",
   i + 1,
   cmd.command_path_slices.join( "." ),
   cmd.named_arguments
 );
 }

  println!( "\n✓ Multiple instructions parsing successful!" );
  Ok( () )
}