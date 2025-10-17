//!
//! The semantic analyzer for the Unilang framework.
//!
//! # Interactive Argument Handling Implementation
//!
//! This module implements the critical `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error 
//! signaling system for REPL applications:
//!
//! ## Key Implementation Details (lines 196-203)
//! - Interactive arguments are detected during semantic analysis, NOT during execution
//! - The specific error code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` is returned
//! - This allows REPL loops to catch the error and prompt for secure input
//! - Optional interactive arguments with defaults do NOT trigger the error
//!
//! ## Security Considerations
//! - Interactive validation occurs before any command execution
//! - Sensitive arguments should be marked with both `interactive: true` and `sensitive: true`
//! - The semantic analyzer never logs or stores interactive argument values
//! - Error messages for interactive arguments are deliberately generic to avoid information leakage
//!
//! ## REPL Integration Pattern
//! ```rust,ignore
//! # use unilang::semantic::SemanticAnalyzer;
//! # use unilang::error::Error;
//! # let semantic_analyzer = SemanticAnalyzer::new(&[], &registry);
//! match semantic_analyzer.analyze() {
//!     Err(Error::Execution(error_data)) 
//!         if error_data.code == "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" => {
//!         // Handle secure input prompting at REPL level
//!         prompt_for_secure_input(&error_data.message);
//!     },
//!     // ... other error handling
//! }
//! # fn prompt_for_secure_input(_msg: &str) {}
//! ```
//!

/// Internal namespace.
mod private
{
  use crate::data::{ ArgumentDefinition, CommandDefinition, ErrorData };
  use crate::error::Error;
  use crate::registry::CommandRegistry;
  use crate::types::{ parse_value, Value }; // Import parse_value
  use regex::Regex; // Added for validation rules
  use unilang_parser::{ Argument, GenericInstruction };
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
      
      let command_path_refs : Vec< &str > = instruction.command_path_slices.iter().map( std::string::String::as_str ).collect();
      let command_name = crate::interner::intern_command_name( &command_path_refs );

      let command_def = self.registry.command( command_name ).ok_or_else( || ErrorData::new(
        "UNILANG_COMMAND_NOT_FOUND".to_string(),
        format!( "Command Error: The command '{command_name}' was not found. Use '.' to see all available commands or check for typos." ),
      ))?;

      // Check for double question mark parameter (alternative help access)
      let has_double_question_mark = instruction.positional_arguments.iter()
        .any( | arg | arg.value == "??" ) ||
        instruction.named_arguments.values()
        .flatten()
        .any( | arg | arg.value == "??" );

      // Check if help was requested for this command (via ? operator or ?? parameter)
      if instruction.help_requested || has_double_question_mark
      {
        // Generate help for this specific command
        let help_generator = crate::help::HelpGenerator::new( self.registry );
        let help_content = help_generator.command( command_name )
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
      let value_found = Self::try_bind_named_argument( instruction, arg_def, &mut bound_arguments )?
        || Self::try_bind_positional_argument( instruction, arg_def, &mut bound_arguments, &mut positional_idx )?;

      if value_found
      {
        Self::validate_bound_argument( &bound_arguments, arg_def )?;
      }
      else
      {
        Self::handle_missing_argument( arg_def, &mut bound_arguments )?;
      }
    }

    Self::check_excess_positional_arguments( instruction, positional_idx )?;
    Self::check_unknown_named_arguments( instruction, command_def )?;
    Ok( bound_arguments )
  }

  fn try_bind_named_argument( instruction : &GenericInstruction, arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value > ) -> Result< bool, Error >
  {
    // TASK 024 ENHANCEMENT: Collect all arguments matching canonical name AND all aliases
    let mut all_matching_args = Vec::new();
    let mut found_any = false;

    // Collect arguments by canonical name
    if let Some( parser_args ) = instruction.named_arguments.get( &arg_def.name )
    {
      all_matching_args.extend_from_slice( parser_args );
      found_any = true;
    }

    // Collect arguments by all aliases
    for alias in &arg_def.aliases
    {
      if let Some( parser_args ) = instruction.named_arguments.get( alias )
      {
        all_matching_args.extend_from_slice( parser_args );
        found_any = true;
      }
    }

    if found_any
    {
      Self::bind_argument_values( &all_matching_args, arg_def, bound_arguments )?;
      Ok( true )
    }
    else
    {
      Ok( false )
    }
  }

  fn try_bind_positional_argument( instruction : &GenericInstruction, arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value >, positional_idx : &mut usize ) -> Result< bool, Error >
  {
    if *positional_idx >= instruction.positional_arguments.len()
    {
      return Ok( false );
    }

    if arg_def.attributes.multiple
    {
      let mut values = Vec::new();
      while *positional_idx < instruction.positional_arguments.len()
      {
        let parser_arg = &instruction.positional_arguments[ *positional_idx ];
        values.push( parse_value( &parser_arg.value, &arg_def.kind )? );
        *positional_idx += 1;
      }
      bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
    }
    else
    {
      let parser_arg = &instruction.positional_arguments[ *positional_idx ];
      bound_arguments.insert( arg_def.name.clone(), parse_value( &parser_arg.value, &arg_def.kind )? );
      *positional_idx += 1;
    }

    Ok( true )
  }

  fn bind_argument_values( parser_args : &Vec< Argument >, arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value > ) -> Result< (), Error >
  {
    // TASK 024 FIX: Automatic Multiple Parameter Collection
    // Always collect multiple values into a list, regardless of the `multiple` attribute
    // This implements requirement R1: "When the same parameter name appears multiple times, collect ALL values into a list"

    if parser_args.len() > 1
    {
      // Multiple values detected - always collect into a list
      let mut values = Vec::new();
      for parser_arg in parser_args
      {
        values.push( parse_value( &parser_arg.value, &arg_def.kind )? );
      }
      bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
    }
    else if arg_def.attributes.multiple
    {
      // Single value but multiple=true - wrap in list for consistency
      let mut values = Vec::new();
      if let Some( parser_arg ) = parser_args.first()
      {
        values.push( parse_value( &parser_arg.value, &arg_def.kind )? );
      }
      bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
    }
    else if let Some( parser_arg ) = parser_args.first()
    {
      // Single value and multiple=false - keep as single value
      bound_arguments.insert( arg_def.name.clone(), parse_value( &parser_arg.value, &arg_def.kind )? );
    }

    Ok( () )
  }

  fn handle_missing_argument( arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value > ) -> Result< (), Error >
  {
    if !arg_def.attributes.optional
    {
      if arg_def.attributes.interactive
      {
        // Critical REPL Implementation: Interactive Argument Signaling
        // This is the core implementation of FR-INTERACTIVE-1 requirement
        // ✅ SPECIFICATION COMPLIANCE: Return exact error code as specified
        // This error is designed to be caught by REPL loops for secure input prompting
        //
        // ⚠️ SECURITY NOTE: The error message intentionally doesn't contain the argument value
        // to prevent sensitive data (passwords, API keys) from being logged or displayed
        //
        // 📝 REPL INTEGRATION: REPL implementations should:
        // 1. Catch this specific error code
        // 2. Present secure input prompt to user
        // 3. Mask input if arg_def.attributes.sensitive is true
        // 4. Re-execute the command with the provided interactive value
        return Err( Error::Execution( ErrorData::new(
          "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED".to_string(),
          format!( "Interactive Argument Required: The argument '{}' is marked as interactive and must be provided interactively. The application should prompt the user for this value.", arg_def.name ),
        )));
      }

      return Err( Error::Execution( ErrorData::new(
        "UNILANG_ARGUMENT_MISSING".to_string(),
        format!( "Argument Error: The required argument '{}' is missing. Please provide a value for this argument.", arg_def.name ),
      )));
    }
    else if let Some( default_value ) = &arg_def.attributes.default
    {
      bound_arguments.insert( arg_def.name.clone(), parse_value( default_value, &arg_def.kind )? );
    }

    Ok( () )
  }

  fn validate_bound_argument( bound_arguments : &HashMap< String, Value >, arg_def : &ArgumentDefinition ) -> Result< (), Error >
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

    Ok( () )
  }

  fn check_excess_positional_arguments( instruction : &GenericInstruction, positional_idx : usize ) -> Result< (), Error >
  {
    if positional_idx < instruction.positional_arguments.len()
    {
      return Err( Error::Execution( ErrorData::new(
        "UNILANG_TOO_MANY_ARGUMENTS".to_string(),
        "Argument Error: Too many arguments provided for this command. Please check the command usage and remove extra arguments.".to_string(),
      )));
    }

    Ok( () )
  }

  ///
  /// Checks for unknown named arguments that don't match any defined parameter.
  ///
  /// This function validates that all named parameters in the instruction correspond
  /// to actual parameter definitions (including aliases). If unknown parameters are found,
  /// it returns an error with helpful suggestions for similar parameter names.
  ///
  /// # Arguments
  /// * `instruction` - The parsed instruction containing named arguments
  /// * `command_def` - The command definition with valid parameter names
  ///
  /// # Returns
  /// * `Ok(())` if all named arguments are valid
  /// * `Err` with UNILANG_UNKNOWN_PARAMETER if invalid parameters are found
  ///
  /// # Error Format
  /// - Single unknown: "Unknown parameter 'drry'. Did you mean 'dry'?"
  /// - Multiple unknown: "Unknown parameters: 'drry', 'foo'. Check command help for valid parameters."
  fn check_unknown_named_arguments( instruction : &GenericInstruction, command_def : &CommandDefinition ) -> Result< (), Error >
  {
    // Collect all valid parameter names (canonical names + aliases)
    let mut valid_names = std::collections::HashSet::new();
    for arg_def in &command_def.arguments
    {
      valid_names.insert( arg_def.name.as_str() );
      for alias in &arg_def.aliases
      {
        valid_names.insert( alias.as_str() );
      }
    }

    // Find unknown parameters in the instruction
    let mut unknown_params: Vec< &str > = Vec::new();
    for param_name in instruction.named_arguments.keys()
    {
      if !valid_names.contains( param_name.as_str() )
      {
        unknown_params.push( param_name );
      }
    }

    // If no unknown parameters, validation passes
    if unknown_params.is_empty()
    {
      return Ok( () );
    }

    // Generate helpful error message with suggestions
    let error_message = if unknown_params.len() == 1
    {
      let unknown = unknown_params[ 0 ];

      // Find best suggestion using Levenshtein distance
      let suggestion = Self::find_closest_parameter_name( unknown, &valid_names );

      if let Some( suggested_name ) = suggestion
      {
        format!(
          "Argument Error: Unknown parameter '{}'. Did you mean '{}'? Use '.{} ??' for help.",
          unknown,
          suggested_name,
          command_def.name
        )
      }
      else
      {
        format!(
          "Argument Error: Unknown parameter '{}'. Use '.{} ??' to see valid parameters.",
          unknown,
          command_def.name
        )
      }
    }
    else
    {
      // Multiple unknown parameters
      let params_list = unknown_params.iter()
        .map( | p | format!( "'{}'", p ) )
        .collect::< Vec< _ > >()
        .join( ", " );

      format!(
        "Argument Error: Unknown parameters: {}. Use '.{} ??' to see valid parameters.",
        params_list,
        command_def.name
      )
    };

    Err( Error::Execution( ErrorData::new(
      "UNILANG_UNKNOWN_PARAMETER".to_string(),
      error_message,
    )))
  }

  ///
  /// Finds the closest matching parameter name using Levenshtein distance.
  ///
  /// This provides helpful "Did you mean..." suggestions when users make typos
  /// in parameter names. Only suggests if the similarity is high enough (distance <= 2).
  ///
  /// # Arguments
  /// * `unknown` - The unknown parameter name
  /// * `valid_names` - Set of all valid parameter names
  ///
  /// # Returns
  /// * `Some(name)` - Best matching parameter name if similarity threshold met
  /// * `None` - No close match found
  ///
  /// # Examples
  /// - "drry" → Some("dry") (distance: 1)
  /// - "verbse" → Some("verbose") (distance: 1)
  /// - "xyz" → None (no close matches)
  fn find_closest_parameter_name( unknown : &str, valid_names : &std::collections::HashSet< &str > ) -> Option< String >
  {
    let mut best_match: Option< ( &str, usize ) > = None;

    for valid_name in valid_names
    {
      let distance = Self::levenshtein_distance( unknown, valid_name );

      // Only suggest if distance is small (good match)
      // and it's better than previous best
      if distance <= 2
      {
        match best_match
        {
          None => best_match = Some( ( valid_name, distance ) ),
          Some( ( _, prev_distance ) ) if distance < prev_distance =>
          {
            best_match = Some( ( valid_name, distance ) );
          },
          _ => {},
        }
      }
    }

    best_match.map( | ( name, _ ) | name.to_string() )
  }

  ///
  /// Calculates Levenshtein distance between two strings.
  ///
  /// Levenshtein distance is the minimum number of single-character edits
  /// (insertions, deletions, or substitutions) required to change one string
  /// into another. Used for fuzzy matching and typo detection.
  ///
  /// # Arguments
  /// * `a` - First string
  /// * `b` - Second string
  ///
  /// # Returns
  /// * `usize` - The edit distance between the strings
  ///
  /// # Algorithm
  /// Classic dynamic programming approach with O(n*m) time and space complexity.
  ///
  /// # Examples
  /// - levenshtein("drry", "dry") = 1 (delete 'r')
  /// - levenshtein("verbse", "verbose") = 1 (insert 'o')
  /// - levenshtein("cat", "dog") = 3 (substitute all)
  fn levenshtein_distance( a : &str, b : &str ) -> usize
  {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0
    {
      return b_len;
    }
    if b_len == 0
    {
      return a_len;
    }

    // Create distance matrix
    let mut matrix = vec![ vec![ 0usize; b_len + 1 ]; a_len + 1 ];

    // Initialize first row and column
    for i in 0..=a_len
    {
      matrix[ i ][ 0 ] = i;
    }
    for j in 0..=b_len
    {
      matrix[ 0 ][ j ] = j;
    }

    // Compute distances
    let a_chars: Vec< char > = a.chars().collect();
    let b_chars: Vec< char > = b.chars().collect();

    for i in 1..=a_len
    {
      for j in 1..=b_len
      {
        let cost = usize::from( a_chars[ i - 1 ] != b_chars[ j - 1 ] );

        matrix[ i ][ j ] = std::cmp::min(
          std::cmp::min(
            matrix[ i - 1 ][ j ] + 1,      // deletion
            matrix[ i ][ j - 1 ] + 1       // insertion
          ),
          matrix[ i - 1 ][ j - 1 ] + cost  // substitution
        );
      }
    }

    matrix[ a_len ][ b_len ]
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
#[allow(clippy::format_push_string)]
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
