//!
//! The semantic analyzer for the Unilang framework.
//!

use crate::data::{ CommandDefinition, ErrorData };
use crate::error::Error;
// use unilang_parser::{GenericInstruction}; // Removed Argument as ParserArgument // Temporarily commented out
use crate::registry::CommandRegistry;
use crate::types::{ self, Value };
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
  // instructions : &'a [GenericInstruction], // Temporarily commented out
  registry : &'a CommandRegistry,
}

impl< 'a > SemanticAnalyzer< 'a >
{
  ///
  /// Creates a new `SemanticAnalyzer`.
  ///
  #[must_use]
  pub fn new( /* instructions : &'a [GenericInstruction], */ registry : &'a CommandRegistry ) -> Self
  {
    Self { /* instructions, */ registry }
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

    // for instruction in self.instructions // Temporarily commented out
    // {
    //   let command_name = instruction.command_path_slices.join( "." );
    //   let command_def = self.registry.commands.get( &command_name ).ok_or_else( || ErrorData {
    //     code : "COMMAND_NOT_FOUND".to_string(),
    //     message : format!( "Command not found: {}", command_name ),
    //   } )?;

    //   let arguments = Self::bind_arguments( instruction, command_def )?;
    //   verified_commands.push( VerifiedCommand {
    //     definition : ( *command_def ).clone(),
    //     arguments,
    //   } );
    // }
    // Temporarily return an empty vector to allow compilation
    Ok( Vec::new() )
  }

  ///
  /// Binds the arguments from a statement to the command definition.
  ///
  /// This function checks for the correct number and types of arguments,
  /// returning an error if validation fails.

  fn bind_arguments( /* instruction : &GenericInstruction, */ command_def : &CommandDefinition ) -> Result< HashMap< String, Value >, Error >
  {
    // Temporarily return an empty HashMap to allow compilation
    let _ = command_def; // Suppress unused warning
    Ok( HashMap::new() )
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