//! Test module for verifying list argument parsing and validation functionality.
//!
//! This module tests the unilang framework's ability to handle list-type command arguments
//! with validation rules, specifically ensuring that list arguments with minimum item
//! constraints are properly parsed and validated during command execution.

use unilang::prelude::*;
use unilang::ValidationRule;
use unilang_parser::{ Parser, UnilangParserOptions };

#[ test ]
fn arg_list_test() -> Result< (), unilang::Error >
{
  let mut registry = CommandRegistry::new();

  let arg = ArgumentDefinition::former()
  .description( "Defines hex to place a castle to." )
  .name( "coord" )
  .hint( "" )
  .aliases( vec![] )
  .tags( vec![] )
  .kind( Kind::List( Box::new( Kind::Integer ), None ) )
  .validation_rules( vec![ ValidationRule::MinItems( 2 ) ] )
  .end();

  let buy_castle_def = CommandDefinition::former()
  .name( ".buy_castle" )
  .namespace( ".region" )
  .hint( "Puts a castle to hex" )
  .status( "stable" )
  .version( "1.0.0" )
  .arguments( vec![ arg ] )
  .end();

  let routine = Box::new
  (
    | _cmd, _ctx |
    {
      Ok
      (
        OutputData
        {
          content : String::new(),
          format : String::new(),
        }
      )
    }
  );
  registry.command_add_runtime( &buy_castle_def, routine )?;

  let parser = Parser::new( UnilangParserOptions::default() );

  let input = ".region.buy_castle coord::1,1";
  let instructions = [ parser.parse_single_instruction( input ).map_err( unilang::Error::from )? ];
  let semantic_analyzer = unilang::semantic::SemanticAnalyzer::new( &instructions[ .. ], &registry );
  let commands = semantic_analyzer.analyze()?;
  let interpreter = unilang::interpreter::Interpreter::new( &commands, &registry );
  let mut context = unilang::interpreter::ExecutionContext::default();
  interpreter.run( &mut context )?;

  Ok( () )
}
