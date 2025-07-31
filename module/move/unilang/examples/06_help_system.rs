//! # Help System Demo
//!
//! This example demonstrates the built-in help generation system,
//! showing how to create comprehensive documentation for commands.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;
use unilang::types::Value;

fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Help System Demo ===\n" );

  let mut registry = CommandRegistry::new();

  // Step 1: Create a well-documented command
  let process_command = CommandDefinition::former()
  .name( "process" )
  .namespace( ".data".to_string() )
  .description( "Processes data files with various transformations and filters".to_string() )
  .hint( "Transform and filter data files" )
  .status( "stable" )
  .version( "2.1.3" )
  .aliases( vec![ "proc".to_string(), "transform".to_string(), "filter".to_string() ] )
  .tags( vec!
  [
    "data-processing".to_string(),
    "transformation".to_string(),
    "filtering".to_string(),
    "batch".to_string()
  ])
  .permissions( vec![ "read_file".to_string(), "write_file".to_string() ] )
  .idempotent( false ) // Processing may have side effects
  .deprecation_message( "".to_string() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    "data.process --input data.csv --output results.csv --format json".to_string(),
    "proc -i *.txt -o combined.txt --filter 'size>1000'".to_string(),
    "transform --input logs/ --format xml --validate".to_string(),
    "data.process --help  # Show this help".to_string()
  ])
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "input".to_string(),
      description: "Input file or directory path. Can be a single file, directory, or glob pattern. Multiple inputs will be processed in order.".to_string(),
      kind: Kind::Path,
      hint: "Source data location".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "i".to_string(), "source".to_string(), "src".to_string() ],
      tags: vec![ "required".to_string(), "input".to_string() ],
    },

    ArgumentDefinition {
      name: "output".to_string(),
      description: "Output file path where processed results will be written. If not specified, results are written to stdout.".to_string(),
      kind: Kind::Path,
      hint: "Destination file path".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("-".to_string()), // stdout
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![ "o".to_string(), "dest".to_string(), "destination".to_string() ],
      tags: vec![ "output".to_string() ],
    },

    ArgumentDefinition {
      name: "format".to_string(),
      description: "Output format for the processed data. Controls how the data is serialized and structured in the output.".to_string(),
      kind: Kind::Enum( vec![
        "json".to_string(),
        "csv".to_string(),
        "xml".to_string(),
        "yaml".to_string(),
        "text".to_string()
      ]),
      hint: "Data serialization format".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("json".to_string()),
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![ "f".to_string(), "fmt".to_string() ],
      tags: vec![ "formatting".to_string(), "serialization".to_string() ],
    },

    ArgumentDefinition {
      name: "filter".to_string(),
      description: "Filter expression to apply to the data. Supports field comparisons, size limits, and pattern matching. Use quotes for complex expressions.".to_string(),
      kind: Kind::Pattern,
      hint: "Filter criteria (e.g., 'size>1000', 'name=*.log')".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength(3) ],
      aliases: vec![ "where".to_string(), "condition".to_string() ],
      tags: vec![ "filtering".to_string(), "query".to_string() ],
    },

    ArgumentDefinition {
      name: "validate".to_string(),
      description: "Enable data validation during processing. When enabled, validates input data structure and content before processing.".to_string(),
      kind: Kind::Boolean,
      hint: "Enable validation checks".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("false".to_string()),
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![ "v".to_string(), "check".to_string() ],
      tags: vec![ "validation".to_string(), "quality".to_string() ],
    },

    ArgumentDefinition {
      name: "batch_size".to_string(),
      description: "Number of records to process in each batch. Larger batches use more memory but may be faster. Set to 0 for unlimited batch size.".to_string(),
      kind: Kind::Integer,
      hint: "Records per batch (0=unlimited)".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("1000".to_string()),
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::Min(0.0), ValidationRule::Max(100000.0) ],
      aliases: vec![ "batch".to_string(), "chunk".to_string() ],
      tags: vec![ "performance".to_string(), "memory".to_string() ],
    },

    ArgumentDefinition {
      name: "config".to_string(),
      description: "Configuration key-value pairs for advanced processing options. Format: key=value,key2=value2".to_string(),
      kind: Kind::Map(
        Box::new( Kind::String ),
        Box::new( Kind::String ),
        Some( ',' ),
        Some( '=' )
      ),
      hint: "Advanced configuration options".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "cfg".to_string(), "options".to_string() ],
      tags: vec![ "configuration".to_string(), "advanced".to_string() ],
    },

    ArgumentDefinition {
      name: "threads".to_string(),
      description: "Number of processing threads to use. Higher values may improve performance on multi-core systems but use more resources.".to_string(),
      kind: Kind::Integer,
      hint: "Thread count for parallel processing".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("1".to_string()),
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::Min(1.0), ValidationRule::Max(16.0) ],
      aliases: vec![ "t".to_string(), "parallel".to_string(), "workers".to_string() ],
      tags: vec![ "performance".to_string(), "concurrency".to_string() ],
    },

    ArgumentDefinition {
      name: "api_key".to_string(),
      description: "API key for external service integration. Keep this secure and do not log or display.".to_string(),
      kind: Kind::String,
      hint: "Secret API authentication key".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        sensitive: true, // Mark as sensitive
        interactive: true, // May prompt user
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::MinLength(16) ],
      aliases: vec![ "key".to_string(), "auth".to_string() ],
      tags: vec![ "authentication".to_string(), "security".to_string() ],
    },
  ])
  .end();

  let process_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "🔄 Processing data with configuration:" );

    for ( name, value ) in &cmd.arguments
    {
      let display_value = match name.as_str()
      {
        "api_key" =>
        {
          if let Value::String( s ) = value
          {
            format!( "{}...{} (hidden)", &s[ ..2.min( s.len() ) ], &s[ s.len().saturating_sub( 2 ).. ] )
          }
          else
          {
            "***".to_string()
          }
        },
        _ => format!( "{:?}", value ),
      };
      println!( "  {}: {}", name, display_value );
    }

    println!( "✅ Data processing completed successfully" );

    Ok( OutputData
    {
      content : "Data processed successfully".to_string(),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &process_command, process_routine )?;

  // Step 2: Create a simple command for comparison
  let simple_command = CommandDefinition::former()
  .name( "ping" )
  .namespace( "".to_string() ) // Global namespace
  .description( "Test connectivity".to_string() )
  .hint( "Simple connectivity test" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "test".to_string() ] )
  .tags( vec![ "network".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( "".to_string() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "ping".to_string() ] )
  .arguments( vec![] )
  .end();

  let ping_routine = Box::new( | _cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "🏓 Pong!" );
    Ok( OutputData
    {
      content : "pong".to_string(),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &simple_command, ping_routine )?;

  println!( "✓ Registered commands with comprehensive documentation" );

  // Step 3: Demonstrate help generation
  let help_generator = HelpGenerator::new( &registry );

  println!( "\n=== Command List Help ===" );
  println!( "{}", help_generator.list_commands() );

  println!( "\n=== Detailed Command Help ===" );
  if let Some( detailed_help ) = help_generator.command( "data.process" )
  {
    println!( "{}", detailed_help );
  }

  println!( "\n=== Simple Command Help ===" );
  if let Some( simple_help ) = help_generator.command( "ping" )
  {
    println!( "{}", simple_help );
  }

  println!( "\n=== Help System Features ===" );
  println!( "✨ The help system automatically generates:" );
  println!( "  • Command usage syntax" );
  println!( "  • Version information" );
  println!( "  • Command aliases" );
  println!( "  • Status indicators" );
  println!( "  • Comprehensive descriptions" );
  println!( "  • Argument details with types" );
  println!( "  • Validation rules" );
  println!( "  • Default values" );
  println!( "  • Aliases for arguments" );
  println!( "  • Tags and categorization" );
  println!( "  • Usage examples" );

  println!( "\n=== Help Access Methods ===" );
  println!( "1. List all commands:" );
  println!( "   cargo run --bin unilang_cli --help" );
  println!( "   cargo run --bin unilang_cli help" );

  println!( "\n2. Get help for specific command:" );
  println!( "   cargo run --bin unilang_cli help data.process" );
  println!( "   cargo run --bin unilang_cli help ping" );

  println!( "\n3. Using aliases:" );
  println!( "   cargo run --bin unilang_cli help proc" );
  println!( "   cargo run --bin unilang_cli help transform" );

  println!( "\n=== Best Practices for Documentation ===" );
  println!( "📋 When creating commands, include:" );
  println!( "  • Clear, concise descriptions" );
  println!( "  • Helpful hints for each argument" );
  println!( "  • Realistic usage examples" );
  println!( "  • Meaningful aliases" );
  println!( "  • Appropriate tags for categorization" );
  println!( "  • Version information" );
  println!( "  • Status (stable, beta, experimental, deprecated)" );
  println!( "  • Validation rules for data integrity" );
  println!( "  • Default values where appropriate" );
  println!( "  • Permission requirements" );

  Ok(())
}