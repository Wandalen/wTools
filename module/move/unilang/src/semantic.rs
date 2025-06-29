//!
//! The semantic analyzer for the Unilang framework.
//!

use crate::data::{ CommandDefinition, ErrorData };
use crate::error::Error;
use unilang_instruction_parser::{GenericInstruction}; // Removed Argument as ParserArgument
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

    eprintln!( "--- bind_arguments debug ---" );
    eprintln!( "Instruction: {:?}", instruction );
    eprintln!( "Command Definition: {:?}", command_def );

    for arg_def in &command_def.arguments
    {
      eprintln!( "Processing argument definition: {:?}", arg_def );
      let mut raw_values_for_current_arg: Vec<String> = Vec::new();

      // 1. Try to find a named argument
      if let Some( arg ) = instruction.named_arguments.get( &arg_def.name )
      {
        raw_values_for_current_arg.push( arg.value.clone() );
        eprintln!( "Found named argument '{}': {:?}", arg_def.name, arg.value );
      }

      // 2. If not found by name, try to find positional arguments
      // If 'multiple' is true, consume all remaining positional arguments
      // Otherwise, consume only one positional argument
      if raw_values_for_current_arg.is_empty() // Only look for positional if not found by name
      {
        if arg_def.multiple
        {
          while positional_arg_idx < instruction.positional_arguments.len()
          {
            raw_values_for_current_arg.push( instruction.positional_arguments[ positional_arg_idx ].value.clone() );
            eprintln!( "Found positional (multiple) argument: {:?}", instruction.positional_arguments[ positional_arg_idx ].value );
            positional_arg_idx += 1;
          }
        }
        else
        {
          if positional_arg_idx < instruction.positional_arguments.len()
          {
            raw_values_for_current_arg.push( instruction.positional_arguments[ positional_arg_idx ].value.clone() );
            eprintln!( "Found positional (single) argument: {:?}", instruction.positional_arguments[ positional_arg_idx ].value );
            positional_arg_idx += 1;
          }
        }
      }

      eprintln!( "Raw values for current arg '{}': {:?}", arg_def.name, raw_values_for_current_arg );

      // Now, process the collected raw string values
      if !raw_values_for_current_arg.is_empty()
      {
        if arg_def.multiple
        {
          let mut collected_values = Vec::new();
          for raw_value_str in raw_values_for_current_arg
          {
            eprintln!( "Parsing multiple argument item: '{}' as {:?}", raw_value_str, arg_def.kind );
            let parsed_value = types::parse_value( &raw_value_str, &arg_def.kind )
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
            collected_values.push( parsed_value );
          }
          bound_args.insert( arg_def.name.clone(), Value::List( collected_values ) );
        }
        else
        {
          // For non-multiple arguments, there should be only one value
          let raw_value_str = raw_values_for_current_arg.remove( 0 ); // Take the first (and only) value
          eprintln!( "Parsing single argument: '{}' as {:?}", raw_value_str, arg_def.kind );
          let parsed_value = types::parse_value( &raw_value_str, &arg_def.kind )
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
        eprintln!( "Error: Missing required argument: {}", arg_def.name );
        return Err( ErrorData {
          code : "MISSING_ARGUMENT".to_string(),
          message : format!( "Missing required argument: {}", arg_def.name ),
        }.into() );
      }
    }

    // Check for unconsumed positional arguments
    if positional_arg_idx < instruction.positional_arguments.len()
    {
      eprintln!( "Error: Too many positional arguments provided. Unconsumed: {:?}", &instruction.positional_arguments[ positional_arg_idx.. ] );
      return Err( ErrorData {
        code : "TOO_MANY_ARGUMENTS".to_string(),
        message : "Too many positional arguments provided".to_string(),
      }.into() );
    }

    eprintln!( "--- bind_arguments end ---" );
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