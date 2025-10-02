//! Task 079: Fix for Shell Argument Handling
//!
//! The issue: Shell strips quotes from arguments before they reach wrun.
//! The solution: Re-quote arguments that contain spaces before joining.

#![allow(clippy::doc_markdown)] // Allow command-line syntax in documentation

use unilang::
{
  data:: { ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes },
  registry::CommandRegistry,
  semantic::SemanticAnalyzer,
  Value,
};
use unilang_parser:: { Parser, UnilangParserOptions };

/// FIXED: Properly handle shell arguments by re-quoting values with spaces
///
/// When shell processes: command::"echo a"
/// We receive: "command::echo a" (quotes stripped)
/// We need to convert back to: command::"echo a" (re-quoted for unilang parser)
fn convert_shell_args_to_unilang( shell_args: &[&str] ) -> String
{
  let processed_args: Vec< String > = shell_args
    .iter()
    .map( | arg |
    {
      // Check if this is a named parameter (contains "::")
      if let Some( pos ) = arg.find( "::" )
      {
        let name = &arg[ ..pos ];
        let value = &arg[ pos + 2.. ];

        // If value contains spaces or is empty, it needs to be re-quoted
        if value.contains( ' ' ) || value.is_empty()
        {
          format!( r#"{name}::"{value}""# )
        }
        else
        {
          (*arg).to_string()
        }
      }
      else
      {
        (*arg).to_string()
      }
    })
    .collect();

  processed_args.join( " " )
}

/// Create command definition for testing
fn create_run_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".run" )
    .description( "Execute multiple commands" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "command" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          multiple: true,
          ..Default::default()
        })
        .description( "Commands to execute" )
        .end(),

      ArgumentDefinition::former()
        .name( "parallel" )
        .kind( Kind::Integer )
        .attributes( ArgumentAttributes
        {
          optional: true,
          default: Some( "2".to_string() ),
          ..Default::default()
        })
        .description( "Parallel count" )
        .end(),
    ])
    .end()
}

/// T079-FIX.1: Test shell argument conversion with 2 commands
#[test]
fn test_fixed_shell_args_two_commands()
{
  // Shell strips quotes: command::"echo a" → "command::echo a"
  let shell_args = vec![ ".run", "command::echo a", "command::echo b", "parallel::2" ];

  // Convert shell args by re-quoting values with spaces
  let instruction_text = convert_shell_args_to_unilang( &shell_args );

  println!( "Fixed instruction text: {instruction_text}" );
  assert_eq!( instruction_text, r#".run command::"echo a" command::"echo b" parallel::2"# );

  let mut registry = CommandRegistry::new();
  registry.register( create_run_command() );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( &instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let verified_commands = analyzer.analyze()
    .expect( "Semantic analysis should succeed" );

  assert!( !verified_commands.is_empty(), "Should have verified commands" );

  let cmd = &verified_commands[ 0 ];
  let command_arg = cmd.arguments.get( "command" ).expect( "command parameter should exist" );

  match command_arg
  {
    Value::List( list ) =>
    {
      println!( "✅ SUCCESS: 2 commands parsed correctly into List" );
      assert_eq!( list.len(), 2, "Should have 2 commands" );
    },
    other => panic!( "Expected Value::List, got {other:?}" ),
  }
}

/// T079-FIX.2: Test shell argument conversion with 4 commands
#[test]
fn test_fixed_shell_args_four_commands()
{
  let shell_args = vec![
    ".run",
    "command::echo a",
    "command::echo b",
    "command::echo c",
    "command::echo d",
    "parallel::2"
  ];

  let instruction_text = convert_shell_args_to_unilang( &shell_args );

  println!( "Fixed instruction text: {instruction_text}" );
  assert_eq!( instruction_text, r#".run command::"echo a" command::"echo b" command::"echo c" command::"echo d" parallel::2"# );

  let mut registry = CommandRegistry::new();
  registry.register( create_run_command() );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( &instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let verified_commands = analyzer.analyze()
    .expect( "Semantic analysis should succeed" );

  let cmd = &verified_commands[ 0 ];
  let command_arg = cmd.arguments.get( "command" ).expect( "command parameter should exist" );

  match command_arg
  {
    Value::List( list ) =>
    {
      println!( "✅ SUCCESS: 4 commands parsed correctly into List" );
      assert_eq!( list.len(), 4, "Should have 4 commands" );
    },
    other => panic!( "Expected Value::List, got {other:?}" ),
  }
}

/// T079-FIX.3: Test with real cargo commands
#[test]
fn test_fixed_real_cargo_commands()
{
  let shell_args = vec![
    ".run",
    "command::cargo build",
    "command::cargo test",
    "command::cargo clippy",
    "parallel::3"
  ];

  let instruction_text = convert_shell_args_to_unilang( &shell_args );

  println!( "Fixed instruction text: {instruction_text}" );

  let mut registry = CommandRegistry::new();
  registry.register( create_run_command() );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( &instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let verified_commands = analyzer.analyze()
    .expect( "Semantic analysis should succeed" );

  let cmd = &verified_commands[ 0 ];
  let command_arg = cmd.arguments.get( "command" ).expect( "command parameter should exist" );

  match command_arg
  {
    Value::List( list ) =>
    {
      println!( "✅ SUCCESS: Real cargo commands parsed correctly into List with {} items", list.len() );
      assert_eq!( list.len(), 3, "Should have 3 commands" );
    },
    other => panic!( "Expected Value::List, got {other:?}" ),
  }
}

/// T079-FIX.4: Test single-word commands (no re-quoting needed)
#[test]
fn test_fixed_single_word_commands()
{
  let shell_args = vec![
    ".run",
    "command::pwd",
    "command::whoami",
    "command::date",
    "command::hostname",
    "parallel::2"
  ];

  let instruction_text = convert_shell_args_to_unilang( &shell_args );

  println!( "Fixed instruction text: {instruction_text}" );
  // Single words dont need quoting
  assert_eq!( instruction_text, ".run command::pwd command::whoami command::date command::hostname parallel::2" );

  let mut registry = CommandRegistry::new();
  registry.register( create_run_command() );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( &instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let verified_commands = analyzer.analyze()
    .expect( "Semantic analysis should succeed" );

  let cmd = &verified_commands[ 0 ];
  let command_arg = cmd.arguments.get( "command" ).expect( "command parameter should exist" );

  match command_arg
  {
    Value::List( list ) =>
    {
      println!( "✅ SUCCESS: Single-word commands work without quoting ({} commands)", list.len() );
      assert_eq!( list.len(), 4, "Should have 4 commands" );
    },
    other => panic!( "Expected Value::List, got {other:?}" ),
  }
}
