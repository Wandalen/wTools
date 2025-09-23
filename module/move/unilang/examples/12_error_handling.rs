#![allow(clippy::all)]
//! # Error Handling and Type Validation
//!
//! This example demonstrates comprehensive error handling scenarios,
//! type validation, and error recovery patterns in Unilang applications.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::error::Error;
use unilang::help::HelpGenerator;
use unilang_parser::Parser;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Error Handling and Type Validation Demo ===\n" );

  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Step 1: Command with strict validation rules
  let validate_cmd = CommandDefinition::former()
  .name( "validate" )
  .namespace( ".test" )
  .description( "Tests various validation scenarios and error handling".to_string() )
  .hint( "Validation testing command" )
  .status( "experimental" )
  .version( "1.0.0" )
  .aliases( vec![] )
  .tags( vec![ "validation".to_string(), "testing".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    "test.validate age::25 email::user@example.com".to_string(),
    "validate age::30 score::95.5 level::advanced".to_string()
  ])
  .arguments( vec!
  [
    // Integer with range validation
    ArgumentDefinition {
      name: "age".to_string(),
      description: "Age in years (18-120)".to_string(),
      kind: Kind::Integer,
      hint: "Must be between 18 and 120".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::Min(18.0), ValidationRule::Max(120.0) ],
      aliases: vec![ "a".to_string() ],
      tags: vec![ "required".to_string() ],
    },

    // String with pattern validation
    ArgumentDefinition {
      name: "email".to_string(),
      description: "Valid email address".to_string(),
      kind: Kind::String,
      hint: "Standard email format".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::Pattern(r"^[^\s@]+@[^\s@]+\.[^\s@]+$".to_string()) ],
      aliases: vec![ "e".to_string() ],
      tags: vec![ "required".to_string() ],
    },

    // Float with precision validation
    ArgumentDefinition {
      name: "score".to_string(),
      description: "Score percentage (0.0-100.0)".to_string(),
      kind: Kind::Float,
      hint: "Decimal percentage value".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some("0.0".to_string()), ..Default::default() },
      validation_rules: vec![ ValidationRule::Min(0.0), ValidationRule::Max(100.0) ],
      aliases: vec![ "s".to_string() ],
      tags: vec![ "optional".to_string() ],
    },

    // Enum with restricted choices
    ArgumentDefinition {
      name: "level".to_string(),
      description: "Skill level selection".to_string(),
      kind: Kind::Enum( vec![ "beginner".to_string(), "intermediate".to_string(), "advanced".to_string(), "expert".to_string() ] ),
      hint: "Choose from predefined levels".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some("beginner".to_string()), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "l".to_string() ],
      tags: vec![ "choice".to_string() ],
    },

    // Interactive argument that triggers special error
    ArgumentDefinition {
      name: "password".to_string(),
      description: "User password (interactive input required)".to_string(),
      kind: Kind::String,
      hint: "Secure password".to_string(),
      attributes: ArgumentAttributes { 
        optional: true, 
        interactive: true, 
        sensitive: true,
        ..Default::default() 
      },
      validation_rules: vec![ ValidationRule::MinLength(8) ],
      aliases: vec![ "p".to_string() ],
      tags: vec![ "security".to_string() ],
    },
  ])
  .end();

  let validate_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "✓ Validation passed for all arguments!" );
    println!( "Processed {} arguments successfully", cmd.arguments.len() );
    
    for ( name, value ) in &cmd.arguments
    {
      println!( "  ✓ {name}: {value}" );
    }

    Ok( OutputData
    {
      content : "All validations passed successfully".to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &validate_cmd, validate_routine )?;
  println!( "✓ Registered validation test command" );

  println!( "\n=== Error Scenarios Demonstration ===\n" );

  let options = unilang_parser::UnilangParserOptions::default();
  let parser = Parser::new( options );
  let help_generator = HelpGenerator::new( &registry );

  // Test cases with different error types
  let test_cases = vec![
    // 1. Type conversion errors
    ("test.validate age::not_a_number email::test@example.com", "Invalid integer conversion"),
    ("test.validate age::25 email::test@example.com score::invalid_float", "Invalid float conversion"),
    ("test.validate age::25 email::test@example.com level::invalid_choice", "Enum choice validation"),
    
    // 2. Range validation errors
    ("test.validate age::15 email::test@example.com", "Age below minimum (18)"),
    ("test.validate age::150 email::test@example.com", "Age above maximum (120)"),
    ("test.validate age::25 email::test@example.com score::-5.0", "Score below minimum (0.0)"),
    ("test.validate age::25 email::test@example.com score::150.0", "Score above maximum (100.0)"),
    
    // 3. Pattern validation errors
    ("test.validate age::25 email::invalid_email", "Email pattern validation"),
    ("test.validate age::25 email::@invalid.com", "Email format validation"),
    
    // 4. Missing required arguments
    ("test.validate email::test@example.com", "Missing required age argument"),
    ("test.validate age::25", "Missing required email argument"),
    
    // 5. Interactive argument signaling
    ("test.validate age::25 email::test@example.com password::secret123", "Interactive argument error"),
    
    // 6. Command not found
    ("nonexistent.command arg::value", "Command not found"),
    
    // 7. Parsing errors
    ("test.validate age::25 email::test@example.com invalid::syntax::", "Parsing error"),
  ];

  for ( input, expected_error ) in test_cases
  {
    println!( "🧪 Testing: {input}" );
    println!( "   Expected: {expected_error}" );
    
    match parser.parse_single_instruction( input )
    {
      Ok( instruction ) =>
      {
        let instructions = vec![ instruction ];
        let analyzer = SemanticAnalyzer::new( &instructions, &registry );
        match analyzer.analyze()
        {
          Ok( _verified_commands ) =>
          {
            println!( "   ❌ Unexpectedly succeeded (should have failed)" );
          },
          Err( error ) =>
          {
            println!( "   ✓ Caught error: {}", format_error( &error ) );
          }
        }
      },
      Err( parse_error ) =>
      {
        println!( "   ✓ Parse error: {parse_error}" );
      }
    }
    println!();
  }

  println!( "=== Error Type Classification ===" );
  println!( "• Parse Errors - Syntax issues in command string" );
  println!( "• Type Errors - Invalid type conversions (UNILANG_TYPE_MISMATCH)" );
  println!( "• Validation Errors - Failed validation rules" );
  println!( "• Missing Argument Errors - Required arguments not provided" );
  println!( "• Interactive Argument Errors - UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" );
  println!( "• Command Not Found - Unknown command or namespace" );
  println!( "• Registration Errors - Runtime command registration issues" );

  println!( "\n=== Error Recovery Patterns ===\n" );

  // Demonstrate error recovery with fallback commands
  println!( "🔄 Error Recovery Example:" );
  let problematic_input = "test.validate age::invalid email::test@example.com";
  
  match parser.parse_single_instruction( problematic_input )
  {
    Ok( instruction ) =>
    {
      let instructions = vec![ instruction ];
      let analyzer = SemanticAnalyzer::new( &instructions, &registry );
      match analyzer.analyze()
      {
        Ok( _verified ) => println!( "   ✓ Command executed successfully" ),
        Err( error ) =>
        {
          println!( "   ❌ Command failed: {}", format_error( &error ) );
          println!( "   💡 Recovery suggestion:" );
          
          if let Some( help ) = help_generator.command( "test.validate" )
          {
            println!( "   📖 Command help:\n{help}" );
          }
          
          println!( "   🔧 Corrected command:" );
          println!( "      test.validate age::25 email::test@example.com" );
        }
      }
    },
    Err( parse_error ) =>
    {
      println!( "   ❌ Parse failed: {parse_error}" );
    }
  }

  println!( "\n=== Best Practices for Error Handling ===\n" );
  println!( "✨ Error Handling Guidelines:" );
  println!( "  • Always check command syntax before execution" );
  println!( "  • Provide clear, actionable error messages" );
  println!( "  • Use validation rules to prevent invalid input" );
  println!( "  • Handle interactive arguments appropriately" );
  println!( "  • Implement graceful degradation for non-critical failures" );
  println!( "  • Log errors with sufficient context for debugging" );
  println!( "  • Provide help information when commands fail" );

  println!( "\n=== Usage Examples ===" );
  println!( "# Valid command:" );
  println!( "cargo run --bin unilang_cli test.validate age::25 email::user@example.com score::95.5 level::advanced" );
  
  println!( "\n# Invalid commands (for testing):" );
  println!( "cargo run --bin unilang_cli test.validate age::15 email::user@example.com    # Age too low" );
  println!( "cargo run --bin unilang_cli test.validate age::25 email::invalid_email      # Invalid email" );
  println!( "cargo run --bin unilang_cli test.validate email::user@example.com           # Missing age" );

  Ok( () )
}

/// Format error with appropriate styling and context
fn format_error( error : &Error ) -> String
{
  match error
  {
    Error::Execution( error_data ) =>
    {
      match error_data.code.as_str()
      {
        "UNILANG_TYPE_MISMATCH" => format!( "🔢 Type Error: {}", error_data.message ),
        "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" => format!( "🔒 Interactive Input: {}", error_data.message ),
        _ => format!( "⚠️ Execution Error: {}", error_data.message ),
      }
    },
    Error::Registration( msg ) => format!( "📝 Registration: {msg}" ),
    Error::Yaml( err ) => format!( "📄 YAML: {err}" ),
    Error::Json( err ) => format!( "📄 JSON: {err}" ),
    Error::Parse( err ) => format!( "🔍 Parse: {err}" ),
  }
}