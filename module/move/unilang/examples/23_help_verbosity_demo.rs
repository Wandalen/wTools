//!
//! Help Verbosity Demonstration
//!
//! This example demonstrates the 5 verbosity levels for help output.
//!
//! # Usage
//!
//! Run with different verbosity levels:
//!
//! ```bash
//! # Level 0 (Minimal): Just name and description
//! UNILANG_HELP_VERBOSITY=0 cargo run --example 23_help_verbosity_demo
//!
//! # Level 1 (Basic): Add parameters with types
//! UNILANG_HELP_VERBOSITY=1 cargo run --example 23_help_verbosity_demo
//!
//! # Level 2 (Standard - DEFAULT): Concise USAGE, PARAMETERS, EXAMPLES
//! cargo run --example 23_help_verbosity_demo
//! # or explicitly:
//! UNILANG_HELP_VERBOSITY=2 cargo run --example 23_help_verbosity_demo
//!
//! # Level 3 (Detailed): Full metadata with version, aliases, tags
//! UNILANG_HELP_VERBOSITY=3 cargo run --example 23_help_verbosity_demo
//!
//! # Level 4 (Comprehensive): Extensive documentation
//! UNILANG_HELP_VERBOSITY=4 cargo run --example 23_help_verbosity_demo
//! ```

use unilang::prelude::*;
use unilang::data::{ ArgumentDefinition, ValidationRule, OutputData };
use unilang::help::HelpVerbosity;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

fn main()
{
  // Create a registry and add a command with rich metadata
  let mut registry = CommandRegistry::new();

  let command = CommandDefinition::former()
    .name( ".config".to_string() )
    .description( "Display current configuration and sources".to_string() )
    .hint( "Show configuration with source tracking".to_string() )
    .status( "stable".to_string() )
    .version( "2.1.0".to_string() )
    .aliases( vec![ "cfg".to_string(), "conf".to_string() ] )
    .tags( vec![ "config".to_string(), "system".to_string(), "settings".to_string() ] )
    .examples( vec![
      ".config".to_string(),
      ".config key::max_tokens".to_string(),
      ".config format::json".to_string(),
      ".config key::model format::yaml".to_string(),
    ] )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "key".to_string(),
        kind : Kind::String,
        description : "Show specific configuration key. If not provided, shows all configuration.".to_string(),
        hint : "Config key name".to_string(),
        attributes : ArgumentAttributes { optional: true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![ "k".to_string() ],
        tags : vec![ "filter".to_string() ],
      },
      ArgumentDefinition
      {
        name : "format".to_string(),
        kind : Kind::String,
        description : "Output format for configuration display. Supports table (default), json, and yaml formats.".to_string(),
        hint : "Output format: table|json|yaml".to_string(),
        attributes : ArgumentAttributes { optional: true, ..Default::default() },
        validation_rules : vec![ ValidationRule::Pattern( "table|json|yaml".to_string() ) ],
        aliases : vec![ "f".to_string(), "fmt".to_string() ],
        tags : vec![ "output".to_string(), "format".to_string() ],
      },
    ] )
    .end();

  // Mock routine (not executed in this demo)
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result< OutputData, unilang::data::ErrorData >
  {
    Ok( OutputData { content: "Config displayed".to_string(), format: "text".to_string(), execution_time_ms: None } )
  });

  registry.command_add_runtime( &command, mock_routine ).unwrap();

  // Create HelpGenerator that respects UNILANG_HELP_VERBOSITY environment variable
  let help_generator = HelpGenerator::from_env( &registry );

  // Get current verbosity level for display
  let verbosity_level = help_generator.verbosity();
  let verbosity_name = match verbosity_level
  {
    HelpVerbosity::Minimal => "Level 0 (Minimal)",
    HelpVerbosity::Basic => "Level 1 (Basic)",
    HelpVerbosity::Standard => "Level 2 (Standard - DEFAULT)",
    HelpVerbosity::Detailed => "Level 3 (Detailed)",
    HelpVerbosity::Comprehensive => "Level 4 (Comprehensive)",
  };

  let separator = "=".repeat( 80 );
  println!( "{separator}" );
  println!( "Help Verbosity Demo - {verbosity_name}" );
  println!( "{separator}" );
  println!();

  // Generate help
  let help = help_generator.command( ".config" ).unwrap();
  println!( "{help}" );

  println!( "{separator}" );
  println!( "Tip: Set UNILANG_HELP_VERBOSITY=0-4 to see different verbosity levels" );
  println!( "{separator}" );
}
