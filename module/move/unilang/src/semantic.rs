//!
//! The semantic analyzer for the Unilang framework.
//!

/// Internal namespace.
mod private
{
  use crate::data::{ CommandDefinition, ErrorData };
  use crate::error::Error;
  use crate::registry::CommandRegistry;
  use crate::types::{ parse_value, Value }; // Import parse_value
  use regex::Regex; // Added for validation rules
  use std::collections::HashMap;
  use unilang_parser::GenericInstruction;

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
#[ derive() ] // Removed Debug
#[ allow( missing_debug_implementations ) ]
pub struct SemanticAnalyzer< 'a >
{
  instructions : & 'a [ GenericInstruction ],
  registry : & 'a CommandRegistry,
}

impl< 'a > SemanticAnalyzer< 'a >
{
  ///
  /// Creates a new `SemanticAnalyzer`.
  ///
  #[ must_use ]
  pub fn new( instructions : & 'a [ GenericInstruction ], registry : & 'a CommandRegistry ) -> Self
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
    // Catch panics and convert them to user-friendly errors
    let result = std::panic::catch_unwind( core::panic::AssertUnwindSafe( || {
      self.analyze_internal()
    }));

    match result
    {
      Ok( analysis_result ) => analysis_result,
      Err( _panic_info ) => Err( Error::Execution( ErrorData::new(
        "UNILANG_INTERNAL_ERROR".to_string(),
        "Internal Error: An unexpected system error occurred during command analysis. This may indicate a bug in the framework.".to_string(),
      )))
    }
  }

  ///
  /// Internal analysis implementation that can panic.
  ///
  fn analyze_internal( &self ) -> Result< Vec< VerifiedCommand >, Error >
  {
    let mut verified_commands : Vec< VerifiedCommand > = Vec::new();

    for instruction in self.instructions
    {
      // Handle special case: single dot "." should show help
      if instruction.command_path_slices.is_empty()
      {
        return self.generate_help_listing();
      }
      
      let command_name = if instruction.command_path_slices[ 0 ].is_empty()
      {
        format!( ".{}", instruction.command_path_slices[ 1.. ].join( "." ) )
      }
      else
      {
        format!( ".{}", instruction.command_path_slices.join( "." ) )
      };

      let command_def = self.registry.command( &command_name ).ok_or_else( || ErrorData::new(
        "UNILANG_COMMAND_NOT_FOUND".to_string(),
        format!( "Command Error: The command '{command_name}' was not found. Use '.' to see all available commands or check for typos." ),
      ))?;

      // Check if help was requested for this command
      if instruction.help_requested
      {
        // Generate help for this specific command
        let help_generator = crate::help::HelpGenerator::new( self.registry );
        let help_content = help_generator.command( &command_name )
          .unwrap_or( format!( "No help available for command '{command_name}'" ) );
        
        return Err( Error::Execution( ErrorData::new(
          "HELP_REQUESTED".to_string(),
          help_content,
        )));
      }

      let arguments = Self::bind_arguments( instruction, &command_def )?;
      verified_commands.push( VerifiedCommand
      {
        definition : command_def,
        arguments,
      });
    }
    Ok( verified_commands )
  }

  ///
  /// Binds the arguments from a statement to the command definition.
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
        if arg_def.attributes.multiple
        {
          let mut values = Vec::new();
          while positional_idx < instruction.positional_arguments.len()
          {
            let parser_arg = &instruction.positional_arguments[ positional_idx ];
            values.push( parse_value( &parser_arg.value, &arg_def.kind )? );
            positional_idx += 1;
          }
          bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
          value_found = true;
        }
        else
        {
          let parser_arg = &instruction.positional_arguments[ positional_idx ];
          bound_arguments.insert( arg_def.name.clone(), parse_value( &parser_arg.value, &arg_def.kind )? );
          value_found = true;
          positional_idx += 1;
        }
      }

      // Handle missing required arguments or default values
      if !value_found
      {
        if !arg_def.attributes.optional
        {
          return Err( Error::Execution( ErrorData::new(
            "UNILANG_ARGUMENT_MISSING".to_string(),
            format!( "Argument Error: The required argument '{}' is missing. Please provide a value for this argument.", arg_def.name ),
          )));
        }
        else if let Some( default_value ) = &arg_def.attributes.default
        {
          bound_arguments.insert( arg_def.name.clone(), parse_value( default_value, &arg_def.kind )? );
          value_found = true;
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
              return Err( Error::Execution( ErrorData::new(
                "UNILANG_VALIDATION_RULE_FAILED".to_string(),
                format!
                (
                  "Validation Error: The value provided for argument '{}' does not meet the required criteria. Please check the value and try again.",
                  arg_def.name
                ),
              )));
            }
          }
        }
      }
    }

    // Check for too many positional arguments
    if positional_idx < instruction.positional_arguments.len()
    {
      return Err( Error::Execution( ErrorData::new(
        "UNILANG_TOO_MANY_ARGUMENTS".to_string(),
        "Argument Error: Too many arguments provided for this command. Please check the command usage and remove extra arguments.".to_string(),
      )));
    }

    Ok( bound_arguments )
  }

  /// Applies a single validation rule to a parsed value.
  #[ allow( clippy::cast_precision_loss ) ] // Allow casting i64 to f64 for min/max comparison
  fn apply_validation_rule( value : &Value, rule : &crate::data::ValidationRule ) -> bool
  {
    use crate::data::ValidationRule;
    match rule
    {
      ValidationRule::Min( min_val ) => match value
      {
        Value::Integer( i ) => *i as f64 >= *min_val,
        Value::Float( f ) => *f >= *min_val,
        _ => false, // Rule not applicable or type mismatch
      },
      ValidationRule::Max( max_val ) => match value
      {
        Value::Integer( i ) => *i as f64 <= *max_val,
        Value::Float( f ) => *f <= *max_val,
        _ => false, // Rule not applicable or type mismatch
      },
      ValidationRule::MinLength( min_len ) => match value
      {
        Value::String( s ) => s.len() >= *min_len,
        Value::List( l ) => l.len() >= *min_len,
        _ => false,
      },
      ValidationRule::MaxLength( max_len ) => match value
      {
        Value::String( s ) => s.len() <= *max_len,
        Value::List( l ) => l.len() <= *max_len,
        _ => false,
      },
      ValidationRule::Pattern( pattern_str ) => match value
      {
        Value::String( s ) => 
        {
          if let Ok( regex ) = Regex::new( pattern_str )
          {
            regex.is_match( s )
          }
          else
          {
            false
          }
        },
        _ => false, // Rule not applicable or type mismatch
      },
      ValidationRule::MinItems( min_items ) => match value
      {
        Value::List( l ) => l.len() >= *min_items,
        _ => false,
      },
    }
  }

  ///
  /// Generates a help listing showing all available commands with descriptions.
  /// This is called when a user enters just "." as a command.
  ///
  fn generate_help_listing( &self ) -> Result< Vec< VerifiedCommand >, Error >
  {
    // Create a synthetic help output
    let all_commands = self.registry.commands();
    let mut help_content = String::new();
    
    if all_commands.is_empty()
    {
      help_content.push_str("No commands are currently available.\n");
    }
    else
    {
      help_content.push_str("Available commands:\n\n");
      
      // Sort commands by name for consistent display
      let mut sorted_commands: Vec<_> = all_commands.iter().collect();
      sorted_commands.sort_by_key(|(name, _)| *name);
      
      for (name, cmd_def) in sorted_commands
      {
        help_content.push_str(&format!("  {:<20} {}\n", name, cmd_def.description));
      }
      help_content.push_str("\nUse '<command> ?' to get detailed help for a specific command.\n");
    }

    // Return a special error that can be handled by the CLI to display help
    Err( Error::Execution( ErrorData::new(
      "HELP_REQUESTED".to_string(),
      help_content,
    )))
  }
}

}

mod_interface::mod_interface!
{
  exposed use private::VerifiedCommand;
  exposed use private::SemanticAnalyzer;
  
  prelude use private::VerifiedCommand;
  prelude use private::SemanticAnalyzer;
}
