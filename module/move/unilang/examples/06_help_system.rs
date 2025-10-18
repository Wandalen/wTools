#![allow(clippy::all)]
//! # Help System Demo
//!
//! This example demonstrates the built-in help generation system in Unilang,
//! showing how to create comprehensive documentation for commands that users
//! can access through various help interfaces.
//!
//! ## Help System Overview
//!
//! The Unilang help system automatically generates user-friendly documentation
//! from `CommandDefinition` metadata. It provides two main modes:
//!
//! 1. **Command List Mode** (`list_commands()`): Shows a summary of all available
//!    commands with their basic information, aliases, and status.
//!
//! 2. **Detailed Command Mode** (`command(name)`): Shows comprehensive help for
//!    a specific command including arguments, examples, validation rules, and more.
//!
//! ## Key Benefits
//!
//! - **Automatic Generation**: Help is generated from command definitions, ensuring
//!   documentation stays in sync with actual command behavior.
//! - **Consistent Format**: All help output follows the same structure and formatting.
//! - **Rich Metadata**: Includes types, validation, defaults, aliases, examples, and more.
//! - **User-Friendly**: Provides clear, actionable information for command usage.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;
use unilang::types::Value;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Help System Demo ===\n" );

  // Initialize the command registry - this will store all our command definitions
  // and their associated runtime implementations
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Step 1: Create a well-documented command that showcases all help system features
  //
  // This demonstrates how to structure a CommandDefinition for optimal help generation.
  // Each field contributes specific information to the help output:
  //
  // - name: The primary command identifier
  // - namespace: Groups related commands (appears as "namespace.command")
  // - description: Main explanatory text shown in detailed help
  // - hint: Short one-liner shown in command lists
  // - status: Lifecycle indicator (stable, beta, experimental, deprecated)
  // - version: Command version for tracking changes
  // - aliases: Alternative names users can use to invoke the command
  // - tags: Categorization labels for filtering and organization
  // - permissions: Required system permissions
  // - examples: Real-world usage scenarios with actual command syntax
  let process_command = CommandDefinition::former()
  // Core identification - appears in help as the primary command name
  .name( ".process" )
  
  // Namespace creates hierarchical command structure (shows as "data.process")
  // Empty namespace means global scope, ".data" creates data.* family
  .namespace( ".data".to_string() )
  
  // Main description - shown in detailed help, should be comprehensive but concise
  // This appears as the primary explanatory text for what the command does
  .description( "Processes data files with various transformations and filters".to_string() )
  
  // Short hint - appears in command lists, should be a brief one-liner
  // This gives users a quick understanding without reading the full description
  .hint( "Transform and filter data files" )
  
  // Status indicates command maturity and stability for users
  // Options: "stable", "beta", "experimental", "deprecated"
  .status( "stable" )
  
  // Version helps users understand command evolution and compatibility
  // Shown in detailed help and useful for troubleshooting
  .version( "2.1.3" )
  
  // Aliases provide alternative invocation methods - improves user experience
  // Users can use any of these names to invoke the same command
  // Good practice: include short forms, common synonyms, and legacy names
  .aliases( vec![ ".proc".to_string(), ".transform".to_string(), ".filter".to_string() ] )
  
  // Tags enable categorization and filtering in help systems
  // Helps users discover related commands and understand command purpose
  .tags( vec!
  [
    "data-processing".to_string(),
    "transformation".to_string(),
    "filtering".to_string(),
    "batch".to_string()
  ])
  
  // Permissions indicate what system access the command requires
  // Helps administrators understand security implications
  .permissions( vec![ "read_file".to_string(), "write_file".to_string() ] )
  
  // Idempotent flag indicates whether repeated execution produces same result
  // Important for understanding command behavior and safety
  .idempotent( false ) // Processing may have side effects
  
  // Deprecation message - use when phasing out commands (empty = not deprecated)
  .deprecation_message( String::new() )
  
  // HTTP method hint for REST API integration scenarios
  .http_method_hint( "POST".to_string() )
  
  // Examples are crucial for help effectiveness - show real usage patterns
  // Best practices for examples:
  // 1. Show common use cases first
  // 2. Demonstrate different argument combinations
  // 3. Include edge cases and advanced usage
  // 4. Use realistic file names and values
  // 5. Show both long and short form arguments
  // 6. Include a help example as the last entry
  .examples( vec!
  [
    "data.process --input data.csv --output results.csv --format json".to_string(),
    "proc -i *.txt -o combined.txt --filter 'size>1000'".to_string(),
    "transform --input logs/ --format xml --validate".to_string(),
    "data.process --help  # Show this help".to_string()
  ])
  // Arguments define the command's interface - each argument contributes to help
  // The help system shows: type, description, hint, default, aliases, validation
  .arguments( vec!
  [
    // === REQUIRED PATH ARGUMENT ===
    // Demonstrates: required argument, multiple aliases, clear description
    ArgumentDefinition {
      // Argument name - appears in help as --input and in usage syntax
      name: "input".to_string(),
      
      // Description explains the argument's purpose and accepted values
      // Good descriptions: explain what it accepts, how it's used, any constraints
      description: "Input file or directory path. Can be a single file, directory, or glob pattern. Multiple inputs will be processed in order.".to_string(),
      
      // Kind defines the argument type - affects parsing and help display
      // Kind::Path indicates filesystem paths, enables path completion
      kind: Kind::Path,
      
      // Hint provides quick guidance in help output
      // Should be concise but informative, appears in parentheses
      hint: "Source data location".to_string(),
      
      // Attributes control argument behavior and help presentation
      // optional: false means this argument is required (shown in help)
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      
      // Validation rules are shown in help and enforced at runtime
      // Empty vec means any valid path is accepted
      validation_rules: vec![],
      
      // Aliases allow multiple ways to specify the same argument
      // Good practice: include short form (-i), descriptive alternatives
      aliases: vec![ "i".to_string(), "source".to_string(), "src".to_string() ],
      
      // Tags help categorize arguments in help output
      tags: vec![ "required".to_string(), "input".to_string() ],
    },

    // === OPTIONAL PATH ARGUMENT WITH DEFAULT ===
    // Demonstrates: optional argument, default value, meaningful default behavior
    ArgumentDefinition {
      name: "output".to_string(),
      
      // Description explains default behavior when argument is omitted
      description: "Output file path where processed results will be written. If not specified, results are written to stdout.".to_string(),
      
      kind: Kind::Path,
      hint: "Destination file path".to_string(),
      
      // Shows how to make arguments optional with sensible defaults
      // The default value is displayed in help output
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("-".to_string()), // stdout convention
        ..Default::default()
      },
      
      validation_rules: vec![],
      aliases: vec![ "o".to_string(), "dest".to_string(), "destination".to_string() ],
      tags: vec![ "output".to_string() ],
    },

    // === ENUM ARGUMENT ===
    // Demonstrates: enumerated values, help shows all possible options
    ArgumentDefinition {
      name: "format".to_string(),
      description: "Output format for the processed data. Controls how the data is serialized and structured in the output.".to_string(),
      
      // Kind::Enum restricts values to a specific set - all options appear in help
      // This provides clear guidance on acceptable values and prevents user errors
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

    // === PATTERN ARGUMENT WITH VALIDATION ===
    // Demonstrates: pattern matching, validation rules, helpful hint with examples
    ArgumentDefinition {
      name: "filter".to_string(),
      
      // Description includes usage guidance and examples of valid patterns
      description: "Filter expression to apply to the data. Supports field comparisons, size limits, and pattern matching. Use quotes for complex expressions.".to_string(),
      
      // Kind::Pattern indicates this accepts pattern/regex-like expressions
      kind: Kind::Pattern,
      
      // Hint shows concrete examples - very helpful for pattern arguments
      hint: "Filter criteria (e.g., 'size>1000', 'name=*.log')".to_string(),
      
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      
      // Validation rules appear in help and guide users on constraints
      // MinLength(3) prevents trivial or accidental filter expressions
      validation_rules: vec![ ValidationRule::MinLength(3) ],
      
      aliases: vec![ "where".to_string(), "condition".to_string() ],
      tags: vec![ "filtering".to_string(), "query".to_string() ],
    },

    // === BOOLEAN FLAG ===
    // Demonstrates: boolean arguments, flags that enable/disable features
    ArgumentDefinition {
      name: "validate".to_string(),
      
      // Boolean descriptions should explain what the flag enables/disables
      description: "Enable data validation during processing. When enabled, validates input data structure and content before processing.".to_string(),
      
      // Kind::Boolean creates a flag - can be used as --validate or --no-validate
      kind: Kind::Boolean,
      
      hint: "Enable validation checks".to_string(),
      
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("false".to_string()), // Explicit default for clarity
        ..Default::default()
      },
      
      validation_rules: vec![],
      aliases: vec![ "v".to_string(), "check".to_string() ],
      tags: vec![ "validation".to_string(), "quality".to_string() ],
    },

    // === INTEGER ARGUMENT WITH RANGE VALIDATION ===
    // Demonstrates: numeric types, min/max validation, performance tuning parameters
    ArgumentDefinition {
      name: "batch_size".to_string(),
      
      // Description explains the parameter's impact and special values
      description: "Number of records to process in each batch. Larger batches use more memory but may be faster. Set to 0 for unlimited batch size.".to_string(),
      
      // Kind::Integer for numeric values - help shows expected format
      kind: Kind::Integer,
      
      // Hint clarifies special values and usage
      hint: "Records per batch (0=unlimited)".to_string(),
      
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("1000".to_string()), // Reasonable default
        ..Default::default()
      },
      
      // Validation rules are shown in help - guide users on acceptable ranges
      // Min/Max prevent nonsensical values and potential system issues
      validation_rules: vec![ ValidationRule::Min(0.0), ValidationRule::Max(100_000.0) ],
      
      aliases: vec![ "batch".to_string(), "chunk".to_string() ],
      tags: vec![ "performance".to_string(), "memory".to_string() ],
    },

    // === MAP ARGUMENT ===
    // Demonstrates: complex types, key-value pairs, format specification
    ArgumentDefinition {
      name: "config".to_string(),
      
      // Description must specify the expected format for complex types
      description: "Configuration key-value pairs for advanced processing options. Format: key=value,key2=value2".to_string(),
      
      // Kind::Map defines structured key-value data with separators
      // Help system shows the format: "key=value,key2=value2"
      kind: Kind::Map(
        Box::new( Kind::String ), // Key type
        Box::new( Kind::String ), // Value type
        Some( ',' ),              // Entry separator
        Some( '=' )               // Key-value separator
      ),
      
      hint: "Advanced configuration options".to_string(),
      
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "cfg".to_string(), "options".to_string() ],
      tags: vec![ "configuration".to_string(), "advanced".to_string() ],
    },

    // === PERFORMANCE TUNING ARGUMENT ===
    // Demonstrates: resource limits, performance implications in description
    ArgumentDefinition {
      name: "threads".to_string(),
      
      // Performance-related descriptions should explain trade-offs
      description: "Number of processing threads to use. Higher values may improve performance on multi-core systems but use more resources.".to_string(),
      
      kind: Kind::Integer,
      hint: "Thread count for parallel processing".to_string(),
      
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("1".to_string()), // Conservative default
        ..Default::default()
      },
      
      // Validation prevents resource abuse and system instability
      validation_rules: vec![ ValidationRule::Min(1.0), ValidationRule::Max(16.0) ],
      
      // Multiple aliases for different user preferences
      aliases: vec![ "t".to_string(), "parallel".to_string(), "workers".to_string() ],
      tags: vec![ "performance".to_string(), "concurrency".to_string() ],
    },

    // === SENSITIVE ARGUMENT ===
    // Demonstrates: security-sensitive data, interactive prompting, validation
    ArgumentDefinition {
      name: "api_key".to_string(),
      
      // Security-related descriptions should warn about sensitive nature
      description: "API key for external service integration. Keep this secure and do not log or display.".to_string(),
      
      kind: Kind::String,
      hint: "Secret API authentication key".to_string(),
      
      attributes: ArgumentAttributes {
        optional: true,
        sensitive: true,    // Prevents value display in logs/help examples
        interactive: true,  // May prompt user for input securely
        ..Default::default()
      },
      
      // Validation for security tokens - minimum length prevents weak keys
      validation_rules: vec![ ValidationRule::MinLength(16) ],
      
      aliases: vec![ "key".to_string(), "auth".to_string() ],
      tags: vec![ "authentication".to_string(), "security".to_string() ],
    },
  ])
  .end();

  // Implementation routine - demonstrates handling of sensitive arguments
  // Note how sensitive arguments are masked in output for security
  let process_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "🔄 Processing data with configuration:" );

    // Display all arguments, but mask sensitive ones for security
    for ( name, value ) in &cmd.arguments
    {
      let display_value = match name.as_str()
      {
        // Special handling for sensitive arguments - never show full value
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
        _ => format!( "{value:?}" ),
      };
      println!( "  {name}: {display_value}" );
    }

    println!( "✅ Data processing completed successfully" );

    Ok( OutputData
    {
      content : "Data processed successfully".to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &process_command, process_routine )?;

  // Step 2: Create a simple command for comparison
  //
  // This minimal command demonstrates the contrast between complex and simple
  // command definitions, showing how the help system adapts to different levels
  // of documentation complexity.
  let simple_command = CommandDefinition::former()
  .name( ".ping" )
  .namespace( String::new() ) // Global namespace - command appears as just "ping"
  .description( "Test connectivity".to_string() )
  .hint( "Simple connectivity test" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".test".to_string() ] )
  .tags( vec![ "network".to_string() ] )
  .permissions( vec![] ) // No special permissions needed
  .idempotent( true )    // Safe to run multiple times
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "ping".to_string() ] ) // Simple usage
  .arguments( vec![] ) // No arguments - demonstrates minimal command
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

  #[allow(deprecated)]
  registry.command_add_runtime( &simple_command, ping_routine )?;

  println!( "✓ Registered commands with comprehensive documentation" );

  // Step 3: Demonstrate help generation
  //
  // The HelpGenerator provides two main methods for accessing help information:
  //
  // 1. list_commands() - Shows summary of all registered commands
  //    - Command names and namespaces
  //    - Brief hints for quick overview
  //    - Status and version information
  //    - Available aliases
  //
  // 2. command(name) - Shows detailed help for specific command
  //    - Full description and usage syntax  
  //    - Complete argument details with types and validation
  //    - Examples showing real usage scenarios
  //    - All metadata (tags, permissions, etc.)
  let help_generator = HelpGenerator::new( &registry );

  // Demonstrate list_commands() - provides overview of all commands
  // This is what users see when they run: command_tool --help
  println!( "\n=== Command List Help ===" );
  println!( "{}", help_generator.list_commands() );

  // Demonstrate command() method for detailed help on complex command
  // This shows comprehensive documentation with all argument details
  println!( "\n=== Detailed Command Help ===" );
  if let Some( detailed_help ) = help_generator.command( "data.process" )
  {
    println!( "{detailed_help}" );
  }

  // Show help for simple command to demonstrate format consistency
  // Even minimal commands get properly formatted help output
  println!( "\n=== Simple Command Help ===" );
  if let Some( simple_help ) = help_generator.command( "ping" )
  {
    println!( "{simple_help}" );
  }

  // Educational summary of what the help system provides
  println!( "\n=== Help System Features ===" );
  println!( "✨ The help system automatically generates:" );
  println!( "  • Command usage syntax with proper argument formatting" );
  println!( "  • Version information for command tracking" );
  println!( "  • Command aliases (alternative names users can invoke)" );
  println!( "  • Status indicators (stable, beta, experimental, deprecated)" );
  println!( "  • Comprehensive descriptions explaining command purpose" );
  println!( "  • Argument details with types, constraints, and formats" );
  println!( "  • Validation rules showing acceptable value ranges" );
  println!( "  • Default values for optional parameters" );
  println!( "  • Aliases for arguments (short forms and alternatives)" );
  println!( "  • Tags and categorization for command organization" );
  println!( "  • Usage examples demonstrating real-world scenarios" );
  println!( "  • Security considerations for sensitive arguments" );

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

  // Best practices guidance for command authors
  println!( "\n=== Best Practices for Documentation ===" );
  println!( "📋 When creating commands, include:" );
  println!( "  • Clear, concise descriptions explaining what the command does" );
  println!( "  • Helpful hints for each argument showing expected format/usage" );
  println!( "  • Realistic usage examples covering common scenarios" );
  println!( "  • Meaningful aliases (short forms, synonyms, legacy names)" );
  println!( "  • Appropriate tags for categorization and discoverability" );
  println!( "  • Version information for tracking command evolution" );
  println!( "  • Status indicators (stable, beta, experimental, deprecated)" );
  println!( "  • Validation rules for data integrity and user guidance" );
  println!( "  • Sensible default values where appropriate" );
  println!( "  • Permission requirements for security transparency" );
  println!( "  • Format specifications for complex argument types" );
  println!( "  • Security considerations for sensitive data" );
  
  println!( "\n=== Key Differences: list_commands() vs command() ===" );
  println!( "🔍 list_commands():" );
  println!( "  • Shows overview of ALL registered commands" );
  println!( "  • Displays basic info: name, namespace, hint, status, version" );
  println!( "  • Lists available aliases for each command" );
  println!( "  • Used when user wants to discover available commands" );
  println!( "  • Compact format suitable for browsing" );
  
  println!( "\n🔍 command(name):" );
  println!( "  • Shows DETAILED help for ONE specific command" );
  println!( "  • Includes full description, usage syntax, and examples" );
  println!( "  • Lists all arguments with types, validation, and defaults" );
  println!( "  • Shows comprehensive metadata (tags, permissions, etc.)" );
  println!( "  • Used when user needs full documentation for command usage" );
  println!( "  • Detailed format optimized for implementation guidance" );

  Ok(())
}