//!
//! The semantic analyzer for the Unilang framework.
//!

use crate::data::{ CommandDefinition, ErrorData };
use crate::error::Error;
use unilang_parser::{GenericInstruction, Argument as ParserArgument};
use crate::registry::CommandRegistry;
use crate::types::{ self, Value, parse_value }; // Import parse_value
use std::collections::HashMap;
use regex::Regex; // Added for validation rules

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
#[ derive( /* Debug */ ) ] // Removed Debug
#[ allow( missing_debug_implementations ) ]
pub struct SemanticAnalyzer< 'a >
{
  instructions : &'a [GenericInstruction],
  registry : &'a CommandRegistry,
}

impl< 'a > SemanticAnalyzer< 'a >
{
  ///
  /// Creates a new `SemanticAnalyzer`.
  ///
  #[must_use]
  pub fn new( instructions : &'a [GenericInstruction], registry : &'a CommandRegistry ) -> Self
  {
    Self { instructions, registry }
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
    let mut verified_commands: Vec<VerifiedCommand> = Vec::new();

    for instruction in self.instructions
    {
      let command_name = instruction.command_path_slices.join( "." );
      let command_def = self.registry.commands.get( &command_name ).ok_or_else( || ErrorData {
        code : "COMMAND_NOT_FOUND".to_string(),
        message : format!( "Command not found: {}", command_name ),
      } )?;

      let arguments = Self::bind_arguments( instruction, command_def )?;
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

  fn bind_arguments( instruction : &GenericInstruction, command_def : &CommandDefinition ) -> Result< HashMap< String, Value >, Error >
  {
    let mut bound_arguments = HashMap::new();
    let mut positional_idx = 0;

    for arg_def in &command_def.arguments
    {
      let mut value_found = false;

      // Try to find by named argument
      if let Some( parser_arg ) = instruction.named_arguments.get( &arg_def.name )
      {
        bound_arguments.insert( arg_def.name.clone(), parse_value( &parser_arg.value, &arg_def.kind )? );
        value_found = true;
      }
      else
      {
        // Try to find by alias
        for alias in &arg_def.aliases
        {
          if let Some( parser_arg ) = instruction.named_arguments.get( alias )
          {
            bound_arguments.insert( arg_def.name.clone(), parse_value( &parser_arg.value, &arg_def.kind )? );
            value_found = true;
            break;
          }
        }
      }

      // If not found by name or alias, try positional
      if !value_found && positional_idx < instruction.positional_arguments.len()
      {
        let parser_arg = &instruction.positional_arguments[ positional_idx ];
        bound_arguments.insert( arg_def.name.clone(), parse_value( &parser_arg.value, &arg_def.kind )? );
        value_found = true;
        positional_idx += 1;
      }

      // Handle missing required arguments or default values
      if !value_found
      {
        if !arg_def.optional
        {
          return Err( Error::Execution( ErrorData {
            code : "MISSING_ARGUMENT".to_string(),
            message : format!( "Missing required argument: {}", arg_def.name ),
          } ) );
        }
        else if arg_def.is_default_arg
        {
          if let Some( default_value ) = &arg_def.default_value
          {
            bound_arguments.insert( arg_def.name.clone(), parse_value( default_value, &arg_def.kind )? );
            value_found = true;
          }
        }
      }

      // Apply validation rules if value was found
      if value_found
      {
        if let Some( value ) = bound_arguments.get( &arg_def.name )
        {
          for rule in &arg_def.validation_rules
          {
            if !Self::apply_validation_rule( value, rule )
            {
              return Err( Error::Execution( ErrorData {
                code : "VALIDATION_RULE_FAILED".to_string(),
                message : format!( "Validation rule '{rule}' failed for argument '{}' with value '{value:?}'", arg_def.name ),
              } ) );
            }
          }
        }
      }
    }

    // Check for too many positional arguments
    if positional_idx < instruction.positional_arguments.len()
    {
      return Err( Error::Execution( ErrorData {
        code : "TOO_MANY_ARGUMENTS".to_string(),
        message : "Too many positional arguments provided.".to_string(),
      } ) );
    }

    Ok( bound_arguments )
  }

  /// Applies a single validation rule to a parsed value.
  #[allow( clippy::cast_precision_loss )] // Allow casting i64 to f64 for min/max comparison
  fn apply_validation_rule( value: &Value, rule: &str ) -> bool
  {
    if let Some( min_val_str ) = rule.strip_prefix( "min:" )
    {
      let min_val: f64 = min_val_str.parse().unwrap_or( f64::MIN );
      match value
      {
        Value::Integer( i ) => *i as f64 >= min_val,
        Value::Float( f ) => *f >= min_val,
        _ => false, // Rule not applicable or type mismatch
      }
    }
    else if let Some( max_val_str ) = rule.strip_prefix( "max:" )
    {
      let max_val: f64 = max_val_str.parse().unwrap_or( f64::MAX );
      match value
      {
        Value::Integer( i ) => *i as f64 <= max_val,
        Value::Float( f ) => *f <= max_val,
        _ => false, // Rule not applicable or type mismatch
      }
    }
    else if let Some( pattern_str ) = rule.strip_prefix( "regex:" )
    {
      let regex = Regex::new( pattern_str ).unwrap(); // Panics if regex is invalid, should be caught earlier
      match value
      {
        Value::String( s ) => regex.is_match( s ),
        _ => false, // Rule not applicable or type mismatch
      }
    }
    else if let Some( min_len_str ) = rule.strip_prefix( "min_length:" )
    {
      let min_len: usize = min_len_str.parse().unwrap_or( 0 );
      match value
      {
        Value::String( s ) => s.len() >= min_len,
        Value::List( l ) => l.len() >= min_len,
        _ => false,
      }
    }
    else
    {
      // Unknown rule, treat as failure or log warning
      false
    }
  }
}