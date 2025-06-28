//!
//! The semantic analyzer for the Unilang framework.
//!

use crate::data::{ CommandDefinition, ErrorData };
use crate::error::Error;
use crate::parsing::Program;
use crate::registry::CommandRegistry;
use crate::types::{ self, Value };
use std::collections::HashMap;

///
/// Represents a command that has been verified against the command registry.
///
/// This struct holds the command's definition and the arguments provided
/// by the user, ensuring that the command is valid and ready for execution.
#[ derive( Debug, Clone ) ]
pub struct VerifiedCommand
{
  /// The definition of the command.
  pub definition : CommandDefinition,
  /// The arguments provided for the command, parsed and typed.
  pub arguments : HashMap< String, Value >,
}

///
/// The semantic analyzer, responsible for validating the parsed program.
///
/// The analyzer checks the program against the command registry to ensure
/// that commands exist, arguments are correct, and types match.
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
  #[must_use]
  pub fn new( program : &'a Program, registry : &'a CommandRegistry ) -> Self
  {
    Self { program, registry }
  }

  ///
  /// Analyzes the program and returns a list of verified commands or an error.
  ///
  /// This is the main entry point for semantic analysis, processing each
  /// statement in the program.
  ///
  /// # Errors
  ///
  /// Returns an error if any command is not found, if arguments are invalid,
  /// or if any other semantic rule is violated.
  pub fn analyze( &self ) -> Result< Vec< VerifiedCommand >, Error >
  {
    let mut verified_commands = Vec::new();

    for statement in &self.program.statements
    {
      let command_def = self.registry.commands.get( &statement.command ).ok_or_else( || ErrorData {
        code : "COMMAND_NOT_FOUND".to_string(),
        message : format!( "Command not found: {}", statement.command ),
      } )?;

      // For now, we'll treat the parsed tokens as raw strings for the purpose of this integration.
      // A more advanced implementation would handle Generic Instructions properly.
      let raw_args: Vec<String> = statement.args.iter().map( ToString::to_string ).collect();

      let arguments = Self::bind_arguments( &raw_args, command_def )?; // Changed to Self::
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
  /// This function checks for the correct number and types of arguments,
  /// returning an error if validation fails.
  fn bind_arguments( raw_args : &[ String ], command_def : &CommandDefinition ) -> Result< HashMap< String, Value >, Error >
  {
    let mut bound_args = HashMap::new();
    let mut arg_iter = raw_args.iter();

    for arg_def in &command_def.arguments
    {
      if let Some( raw_value ) = arg_iter.next()
      {
        let parsed_value = types::parse_value( raw_value, &arg_def.kind )
        .map_err( |e| ErrorData {
          code : "INVALID_ARGUMENT_TYPE".to_string(),
          message : format!( "Invalid value for argument '{}': {}. Expected {:?}.", arg_def.name, e.reason, e.expected_kind ),
        } )?;

        bound_args.insert( arg_def.name.clone(), parsed_value );
      }
      else if !arg_def.optional
      {
        return Err( ErrorData {
          code : "MISSING_ARGUMENT".to_string(),
          message : format!( "Missing required argument: {}", arg_def.name ),
        }.into() );
      }
    }

    if arg_iter.next().is_some()
    {
      return Err( ErrorData {
        code : "TOO_MANY_ARGUMENTS".to_string(),
        message : "Too many arguments provided".to_string(),
      }.into() );
    }

    Ok( bound_args )
  }
}