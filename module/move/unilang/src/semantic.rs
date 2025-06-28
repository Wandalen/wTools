//!
//! The semantic analyzer for the Unilang framework.
//!

use crate::data::{ CommandDefinition, ErrorData };
use crate::error::Error;
use unilang_instruction_parser::{GenericInstruction, Argument as ParserArgument};
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
    let mut verified_commands = Vec::new();

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
    let mut bound_args = HashMap::new();
    let mut positional_arg_idx = 0;

    for arg_def in &command_def.arguments
    {
      let mut value_found = false;
      let mut raw_value = None;

      // 1. Check named arguments
      if let Some( arg ) = instruction.named_arguments.get( &arg_def.name )
      {
        raw_value = Some( arg );
        value_found = true;
      }
      // Aliases are not supported by ArgumentDefinition, so skipping alias check.
      // 2. Check positional arguments if not already found
      if !value_found
      {
        if let Some( arg ) = instruction.positional_arguments.get( positional_arg_idx )
        {
          raw_value = Some( arg );
          positional_arg_idx += 1;
          value_found = true;
        }
      }

      if let Some( raw_value ) = raw_value
      {
        if arg_def.multiple
        {
          // For multiple arguments, the raw_value is expected to be a list of strings
          // This part needs careful consideration based on how unilang_instruction_parser handles multiple values for a single named argument.
          // Assuming for now that `raw_value` is a single string that needs to be parsed into a list if `multiple` is true.
          // A more robust solution would involve `unilang_instruction_parser` providing a list of raw strings for `multiple` arguments.
          // For now, we'll treat it as a single value and parse it into a list of one element.
          let parsed_value = types::parse_value( &raw_value.value, &arg_def.kind )
          .map_err( |e| ErrorData {
            code : "INVALID_ARGUMENT_TYPE".to_string(),
            message : format!( "Invalid value for argument '{}': {}. Expected {:?}.", arg_def.name, e.reason, e.expected_kind ),
          } )?;
          let collected_values = vec![ parsed_value ];

          for value in &collected_values
          {
            for rule in &arg_def.validation_rules
            {
              if !Self::apply_validation_rule( value, rule )
              {
                return Err( ErrorData {
                  code : "VALIDATION_RULE_FAILED".to_string(),
                  message : format!( "Validation rule '{}' failed for argument '{}'.", rule, arg_def.name ),
                }.into() );
              }
            }
          }
          bound_args.insert( arg_def.name.clone(), Value::List( collected_values ) );
        }
        else
        {
          let parsed_value = types::parse_value( &raw_value.value, &arg_def.kind )
          .map_err( |e| ErrorData {
            code : "INVALID_ARGUMENT_TYPE".to_string(),
            message : format!( "Invalid value for argument '{}': {}. Expected {:?}.", arg_def.name, e.reason, e.expected_kind ),
          } )?;

          for rule in &arg_def.validation_rules
          {
            if !Self::apply_validation_rule( &parsed_value, rule )
            {
              return Err( ErrorData {
                code : "VALIDATION_RULE_FAILED".to_string(),
                message : format!( "Validation rule '{}' failed for argument '{}'.", rule, arg_def.name ),
              }.into() );
            }
          }
          bound_args.insert( arg_def.name.clone(), parsed_value );
        }
      }
      else if !arg_def.optional
      {
        // If no value is found and argument is not optional, it's a missing argument error.
        return Err( ErrorData {
          code : "MISSING_ARGUMENT".to_string(),
          message : format!( "Missing required argument: {}", arg_def.name ),
        }.into() );
      }
      // Default values are not supported by ArgumentDefinition, so skipping default value logic.
    }

    // Check for unconsumed positional arguments
    if positional_arg_idx < instruction.positional_arguments.len()
    {
      return Err( ErrorData {
        code : "TOO_MANY_ARGUMENTS".to_string(),
        message : "Too many positional arguments provided".to_string(),
      }.into() );
    }

    Ok( bound_args )
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