//!
//! The semantic analyzer for the Unilang framework.
//!

use crate::data::{ CommandDefinition, ErrorData };
use crate::parsing::{ Program, Statement, Token };
use crate::registry::CommandRegistry;
use std::collections::HashMap;

///
/// Represents a command that has been verified against the command registry.
///
#[ derive( Debug, Clone ) ]
pub struct VerifiedCommand
{
  /// The definition of the command.
  pub definition : CommandDefinition,
  /// The arguments provided for the command.
  pub arguments : HashMap< String, Token >,
}

///
/// The semantic analyzer.
///
#[ derive( Debug ) ]
pub struct SemanticAnalyzer< 'a >
{
  program : &'a Program,
  registry : &'a CommandRegistry,
}

impl< 'a > SemanticAnalyzer< 'a >
{
  ///
  /// Creates a new `SemanticAnalyzer`.
  ///
  pub fn new( program : &'a Program, registry : &'a CommandRegistry ) -> Self
  {
    Self { program, registry }
  }

  ///
  /// Analyzes the program and returns a list of verified commands or an error.
  ///
  pub fn analyze( &self ) -> Result< Vec< VerifiedCommand >, ErrorData >
  {
    let mut verified_commands = Vec::new();

    for statement in &self.program.statements
    {
      let command_def = self.registry.commands.get( &statement.command ).ok_or_else( || ErrorData {
        code : "COMMAND_NOT_FOUND".to_string(),
        message : format!( "Command not found: {}", statement.command ),
      } )?;

      let arguments = self.bind_arguments( statement, command_def )?;
      verified_commands.push( VerifiedCommand {
        definition : ( *command_def ).clone(),
        arguments,
      } );
    }

    Ok( verified_commands )
  }

  ///
  /// Binds the arguments from a statement to the command definition.
  ///
  fn bind_arguments( &self, statement : &Statement, command_def : &CommandDefinition ) -> Result< HashMap< String, Token >, ErrorData >
  {
    let mut bound_args = HashMap::new();
    let mut arg_iter = statement.args.iter().peekable();

    for arg_def in &command_def.arguments
    {
      if let Some( token ) = arg_iter.next()
      {
        // Basic type checking
        let type_matches = match ( &token, arg_def.kind.as_str() )
        {
          ( Token::String( _ ), "String" ) => true,
          ( Token::Integer( _ ), "Integer" ) => true,
          ( Token::Float( _ ), "Float" ) => true,
          ( Token::Boolean( _ ), "Boolean" ) => true,
          _ => false,
        };

        if !type_matches
        {
          return Err( ErrorData {
            code : "INVALID_ARGUMENT_TYPE".to_string(),
            message : format!( "Invalid type for argument '{}'. Expected {}, got {:?}", arg_def.name, arg_def.kind, token ),
          } );
        }
        bound_args.insert( arg_def.name.clone(), token.clone() );
      }
      else if !arg_def.optional
      {
        return Err( ErrorData {
          code : "MISSING_ARGUMENT".to_string(),
          message : format!( "Missing required argument: {}", arg_def.name ),
        } );
      }
    }

    if arg_iter.next().is_some()
    {
      return Err( ErrorData {
        code : "TOO_MANY_ARGUMENTS".to_string(),
        message : "Too many arguments provided".to_string(),
      } );
    }

    Ok( bound_args )
  }
}