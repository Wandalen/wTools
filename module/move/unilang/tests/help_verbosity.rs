//!
//! Tests for help verbosity levels (FR-HELP-7)
//!

use unilang::prelude::*;
use unilang::data::{ OutputData, ErrorData, ValidationRule };
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Mock routine for test commands
fn mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync >
{
  Box::new( |_cmd, _ctx| Ok( OutputData { content: "test".to_string(), format: "text".to_string() } ) )
}

/// Create a test command with various metadata for testing help formatting
fn create_test_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".config".to_string() )
    .description( "Display current configuration and sources".to_string() )
    .hint( "Show configuration".to_string() )
    .status( "stable".to_string() )
    .version( "1.0.0".to_string() )
    .aliases( vec![ "cfg".to_string() ] )
    .tags( vec![ "config".to_string(), "system".to_string() ] )
    .examples( vec![
      ".config".to_string(),
      ".config key::max_tokens".to_string(),
      ".config format::json".to_string(),
    ] )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "key".to_string(),
        kind : Kind::String,
        description : "Show specific config key".to_string(),
        hint : "Key name to display".to_string(),
        attributes : ArgumentAttributes { optional: true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition
      {
        name : "format".to_string(),
        kind : Kind::String,
        description : "Output format: table|json|yaml".to_string(),
        hint : "Format for output".to_string(),
        attributes : ArgumentAttributes { optional: true, ..Default::default() },
        validation_rules : vec![ ValidationRule::Pattern( "table|json|yaml".to_string() ) ],
        aliases : vec![],
        tags : vec![],
      },
    ] )
    .end()
}

#[test]
fn test_verbosity_level_0_minimal()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  let help_gen = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Minimal );
  let help = help_gen.command( ".config" ).expect( "Command should exist" );

  // Level 0: Just name and description
  assert!( help.contains( ".config" ) );
  assert!( help.contains( "Display current configuration" ) );

  // Should NOT contain detailed metadata
  assert!( !help.contains( "USAGE:" ) );
  assert!( !help.contains( "PARAMETERS:" ) );
  assert!( !help.contains( "v1.0.0" ) );
  assert!( !help.contains( "Aliases:" ) );
}

#[test]
fn test_verbosity_level_1_basic()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  let help_gen = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Basic );
  let help = help_gen.command( ".config" ).expect( "Command should exist" );

  // Level 1: Name, description, and parameters with types
  assert!( help.contains( ".config" ) );
  assert!( help.contains( "Display current configuration" ) );
  assert!( help.contains( "PARAMETERS:" ) );
  assert!( help.contains( "key::string" ) );
  assert!( help.contains( "format::string" ) );

  // Should NOT contain detailed explanations
  assert!( !help.contains( "USAGE:" ) );
  assert!( !help.contains( "EXAMPLES:" ) );
  assert!( !help.contains( "Show specific config key" ) );
}

#[test]
fn test_verbosity_level_2_standard_default()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  // Test both default and explicit Standard
  let help_gen_default = HelpGenerator::new( &registry );
  let help_gen_explicit = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard );

  let help_default = help_gen_default.command( ".config" ).expect( "Command should exist" );
  let help_explicit = help_gen_explicit.command( ".config" ).expect( "Command should exist" );

  // Both should produce the same output
  assert_eq!( help_default, help_explicit );

  // Level 2: Concise like unikit - USAGE, PARAMETERS with descriptions, EXAMPLES
  assert!( help_default.contains( "USAGE:" ) );
  assert!( help_default.contains( "PARAMETERS:" ) );
  assert!( help_default.contains( "EXAMPLES:" ) );
  assert!( help_default.contains( ".config [key::string]" ) );
  assert!( help_default.contains( "Show specific config key" ) || help_default.contains( "Key name to display" ) );
  assert!( help_default.contains( ".config format::json" ) );

  // Should NOT contain verbose metadata
  assert!( !help_default.contains( "v1.0.0" ) );
  assert!( !help_default.contains( "Aliases:" ) );
  assert!( !help_default.contains( "Tags:" ) );
  assert!( !help_default.contains( "Status:" ) );
}

#[test]
fn test_verbosity_level_3_detailed()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  let help_gen = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Detailed );
  let help = help_gen.command( ".config" ).expect( "Command should exist" );

  // Level 3: Full metadata including version, aliases, tags, validation rules
  assert!( help.contains( "Usage: .config (v1.0.0)" ) );
  assert!( help.contains( "Aliases: cfg" ) );
  assert!( help.contains( "Tags: config, system" ) );
  assert!( help.contains( "Status: stable" ) );
  assert!( help.contains( "Arguments:" ) );
  assert!( help.contains( "Type: String" ) );
  assert!( help.contains( "Optional" ) );
  assert!( help.contains( "Rules:" ) );
}

#[test]
fn test_verbosity_level_4_comprehensive()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  let help_gen = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Comprehensive );
  let help = help_gen.command( ".config" ).expect( "Command should exist" );

  // Level 4: Extensive like runbox - USAGE, DESCRIPTION, PARAMETERS (detailed), EXAMPLES, TAGS
  assert!( help.contains( "USAGE:" ) );
  assert!( help.contains( "DESCRIPTION:" ) );
  assert!( help.contains( "PARAMETERS:" ) );
  assert!( help.contains( "key::<value>" ) );
  assert!( help.contains( "Type: String" ) );
  assert!( help.contains( "Optional: yes" ) );
  assert!( help.contains( "Validation:" ) );
  assert!( help.contains( "EXAMPLES:" ) );
  assert!( help.contains( "TAGS:" ) );
  assert!( help.contains( "v1.0.0" ) );
  assert!( help.contains( "Aliases: cfg" ) );
}

#[test]
fn test_verbosity_from_level()
{
  assert_eq!( HelpVerbosity::from_level( 0 ), HelpVerbosity::Minimal );
  assert_eq!( HelpVerbosity::from_level( 1 ), HelpVerbosity::Basic );
  assert_eq!( HelpVerbosity::from_level( 2 ), HelpVerbosity::Standard );
  assert_eq!( HelpVerbosity::from_level( 3 ), HelpVerbosity::Detailed );
  assert_eq!( HelpVerbosity::from_level( 4 ), HelpVerbosity::Comprehensive );
  assert_eq!( HelpVerbosity::from_level( 5 ), HelpVerbosity::Comprehensive ); // Capped at 4
  assert_eq!( HelpVerbosity::from_level( 100 ), HelpVerbosity::Comprehensive );
}

#[test]
fn test_verbosity_default()
{
  assert_eq!( HelpVerbosity::default(), HelpVerbosity::Standard );
}

#[test]
fn test_verbosity_set_and_get()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  let mut help_gen = HelpGenerator::new( &registry );
  assert_eq!( help_gen.verbosity(), HelpVerbosity::Standard );

  help_gen.set_verbosity( HelpVerbosity::Minimal );
  assert_eq!( help_gen.verbosity(), HelpVerbosity::Minimal );

  let help = help_gen.command( ".config" ).expect( "Command should exist" );
  assert!( !help.contains( "USAGE:" ) ); // Minimal format
}

#[test]
fn test_verbosity_progressive_information()
{
  let mut registry = CommandRegistry::new();
  let command = create_test_command();
  registry.command_add_runtime( &command, mock_routine() ).unwrap();

  // Test that each level contains more information than the previous
  let help_0 = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Minimal )
    .command( ".config" ).unwrap();
  let help_1 = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Basic )
    .command( ".config" ).unwrap();
  let help_2 = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .command( ".config" ).unwrap();
  let help_3 = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Detailed )
    .command( ".config" ).unwrap();
  let help_4 = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Comprehensive )
    .command( ".config" ).unwrap();

  // Each level should have more content than the previous
  assert!( help_0.len() < help_1.len() );
  assert!( help_1.len() < help_2.len() );
  assert!( help_2.len() < help_3.len() );
  assert!( help_3.len() < help_4.len() );

  println!( "Level 0 ({} chars):\n{}\n", help_0.len(), help_0 );
  println!( "Level 1 ({} chars):\n{}\n", help_1.len(), help_1 );
  println!( "Level 2 ({} chars):\n{}\n", help_2.len(), help_2 );
  println!( "Level 3 ({} chars):\n{}\n", help_3.len(), help_3 );
  println!( "Level 4 ({} chars):\n{}\n", help_4.len(), help_4 );
}
