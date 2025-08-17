//! Comprehensive Basic Usage Example for `unilang_parser`
//!
//! This example demonstrates the core functionality of the `unilang_parser` crate:
//! - Creating a Parser with default configuration
//! - Parsing single instructions with various argument types
//! - Parsing multiple instructions separated by ;;
//! - Accessing parsed command components (paths, arguments, named arguments)
//!
//! Run this example with: `cargo run --example unilang_parser_basic`

use unilang_parser::{ Parser, UnilangParserOptions };
// Removed: use unilang_parser::Argument; // This import is no longer strictly needed for the `unwrap_or` fix, but keep it for clarity if `Argument` is used elsewhere.

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Unilang Parser Basic Usage Examples ===\n" );

  // Create a parser with default options (permissive parsing)
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );

  // Example 1: Single instruction with mixed argument types
  println!( "1. Single Instruction with Mixed Arguments:" );
  let input_single = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose";
  println!( "   Input: {input_single}" );

  let instruction = parser.parse_single_instruction( input_single )?;

  println!( "   Command path: {:?}", instruction.command_path_slices );
  println!( "   Positional args: {:?}", instruction.positional_arguments );
  println!( "   Named arguments: {:?}", instruction.named_arguments );
  println!( "   Help requested: {:?}", instruction.help_requested );

  // Example 2: Accessing specific argument values
  println!( "\n2. Accessing Specific Arguments:" );
  if let Some( severity ) = instruction.named_arguments.get( "severity" )
  {
    println!( "   Severity level: {severity:?}" );
  }
  if let Some( message ) = instruction.named_arguments.get( "message" )
  {
    println!( "   Log message: {message:?}" );
  }

  // Example 3: Multiple instructions (command sequence)
  println!( "\n3. Multiple Instructions (Command Sequence):" );
  let input_multiple = "system.info ? ;; file.read path::\"/etc/hosts\" --binary ;; user.add 'John Doe' email::john.doe@example.com";
  println!( "   Input: {input_multiple}" );

  let instructions = parser.parse_multiple_instructions( input_multiple )?;

  println!( "   Parsed {} instructions:", instructions.len() );
  for ( i, instruction ) in instructions.iter().enumerate()
  {
    println!( "   Instruction {}: {:?}", i + 1, instruction.command_path_slices );

    // Show specific details for each instruction
    match i
    {
      0 => println!( "     -> Help request for system.info: {:?}", instruction.help_requested ),
      1 =>
      {
        println!
        (
          "     -> File path: {}",
          instruction.named_arguments.get( "path" ).map_or( & "unknown".to_string(), | arg | &arg.value )
        );
        println!
        (
          "     -> Binary mode: {}",
          instruction.positional_arguments.iter().any( | arg | arg.value == "--binary" )
        );
      },
      2 =>
      {
        println!
        (
          "     -> User name: {}",
          instruction.positional_arguments.first().map_or( & "unknown".to_string(), | arg | &arg.value )
        );
        println!
        (
          "     -> Email: {}",
          instruction.named_arguments.get( "email" ).map_or( & "unknown".to_string(), | arg | &arg.value )
        );
      },
      _ => {}
    }
  }

  // Example 4: Command path analysis
  println!( "\n4. Command Path Analysis:" );
  let complex_path = parser.parse_single_instruction( "system.network.diagnostics.ping host::\"example.com\" count::5" )?;

  println!( "   Full command path: {:?}", complex_path.command_path_slices );
  println!( "   Namespace: {:?}", &complex_path.command_path_slices[ ..complex_path.command_path_slices.len() - 1 ] );
  println!( "   Command name: {}", complex_path.command_path_slices.last().unwrap_or( & String::new() ) );
  println!( "   Joined path: {}", complex_path.command_path_slices.join( "." ) );

  // Example 5: Help operator demonstration
  println!( "\n5. Help Operator Usage:" );
  let help_examples = vec!
  [
    "file.copy ?", // Basic help
    "database.query sql::\"SELECT * FROM users\" ?", // Contextual help
  ];

  for help_cmd in help_examples
  {
    println!( "   Help command: {help_cmd}" );
    let help_instruction = parser.parse_single_instruction( help_cmd )?;

    println!( "     Command: {:?}", help_instruction.command_path_slices );
    println!( "     Help requested: {:?}", help_instruction.help_requested );
    if !help_instruction.named_arguments.is_empty()
    {
      println!( "     Context args: {:?}", help_instruction.named_arguments );
    }
  }

  println!( "\n✓ All basic usage examples completed successfully!" );
  println!( "\nFor more advanced examples, see the other files in the examples/ directory:" );
  println!( "  - 01_basic_command_parsing.rs" );
  println!( "  - 02_named_arguments_quoting.rs" );
  println!( "  - 03_complex_argument_patterns.rs" );
  println!( "  - 04_multiple_instructions.rs" );
  println!( "  - 05_help_operator_usage.rs" );
  println!( "  - 06_advanced_escaping_quoting.rs" );
  println!( "  - 07_error_handling_diagnostics.rs" );
  println!( "  - 08_custom_parser_configuration.rs" );
  println!( "  - 09_integration_command_frameworks.rs" );
  println!( "  - 10_performance_optimization_patterns.rs" );

  Ok( () )
}
