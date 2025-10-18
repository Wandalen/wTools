//!
//! Comprehensive tests for `StaticCommandRegistry` functionality.
//!
//! This module provides thorough test coverage for `StaticCommandRegistry` including:
//! - PHF map integration and zero-overhead lookups
//! - Registry mode switching (`StaticOnly`, `DynamicOnly`, `Hybrid`, `Auto`)
//! - Performance metrics tracking
//! - Static command loading and conversion
//! - Hybrid functionality with dynamic fallback
//! - Memory and performance optimizations
//! - Error handling and edge cases
//!
//! All tests use real implementations without mocking.

use unilang::prelude::*;
use unilang::registry::{ StaticCommandRegistry, RegistryMode, CommandRegistryTrait };
use unilang::static_data::{ StaticCommandDefinition, StaticCommandMap, StaticArgumentDefinition, StaticKind, StaticArgumentAttributes };
use std::time::Instant;

/// Create a comprehensive test command map with various command types
const TEST_STATIC_COMMANDS_PHF: phf::Map<&'static str, &'static StaticCommandDefinition> = phf::phf_map!
{
  ".test.version" => &StaticCommandDefinition
  {
    name: ".test.version",
    namespace: ".test",
    description: "Show version information",
    hint: "Display program version",
    arguments: &[],
    routine_link: Some( "version_routine" ),
    status: "stable",
    version: "1.0.0",
    tags: &[ "info", "version" ],
    aliases: &[ "v", "ver" ],
    permissions: &[ "read" ],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[ ".test.version", ".test.version --help" ],
  },

  ".test.add" => &StaticCommandDefinition
  {
    name: ".test.add",
    namespace: ".test",
    description: "Add two numbers",
    hint: "Mathematical addition",
    arguments: &[
      StaticArgumentDefinition
      {
        name: "a",
        kind: StaticKind::Integer,
        attributes: StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        hint: "First number",
        description: "The first number to add",
        validation_rules: &[],
        aliases: &[ "first", "num1" ],
        tags: &[ "required" ],
      },
      StaticArgumentDefinition
      {
        name: "b",
        kind: StaticKind::Integer,
        attributes: StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        hint: "Second number",
        description: "The second number to add",
        validation_rules: &[],
        aliases: &[ "second", "num2" ],
        tags: &[ "required" ],
      },
    ],
    routine_link: Some( "add_routine" ),
    status: "stable",
    version: "1.0.0",
    tags: &[ "math", "calculation" ],
    aliases: &[ "plus", "sum" ],
    permissions: &[ "read" ],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[ ".test.add 5 3", ".test.add --a 10 --b 20" ],
  },

  ".test.file.copy" => &StaticCommandDefinition
  {
    name: ".test.file.copy",
    namespace: ".test.file",
    description: "Copy a file from source to destination",
    hint: "File copying utility",
    arguments: &[
      StaticArgumentDefinition
      {
        name: "source",
        kind: StaticKind::File,
        attributes: StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        hint: "Source file",
        description: "Path to the source file to copy",
        validation_rules: &[],
        aliases: &[ "src", "from" ],
        tags: &[ "file", "required" ],
      },
      StaticArgumentDefinition
      {
        name: "destination",
        kind: StaticKind::Path,
        attributes: StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        hint: "Destination path",
        description: "Path where the file will be copied",
        validation_rules: &[],
        aliases: &[ "dest", "to" ],
        tags: &[ "file", "required" ],
      },
      StaticArgumentDefinition
      {
        name: "overwrite",
        kind: StaticKind::Boolean,
        attributes: StaticArgumentAttributes
        {
          optional: true,
          multiple: false,
          default: Some( "false" ),
          sensitive: false,
          interactive: false,
        },
        hint: "Overwrite existing",
        description: "Whether to overwrite existing files",
        validation_rules: &[],
        aliases: &[ "force", "f" ],
        tags: &[ "optional" ],
      },
    ],
    routine_link: Some( "file_copy_routine" ),
    status: "stable",
    version: "2.0.0",
    tags: &[ "file", "io", "copy" ],
    aliases: &[ "cp", "copy" ],
    permissions: &[ "read", "write" ],
    idempotent: false,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[ ".test.file.copy input.txt output.txt", ".test.file.copy --source data.json --destination backup.json --overwrite true" ],
  },

  ".test.deprecated" => &StaticCommandDefinition
  {
    name: ".test.deprecated",
    namespace: ".test",
    description: "A deprecated test command",
    hint: "Deprecated command",
    arguments: &[],
    routine_link: None,
    status: "deprecated",
    version: "0.9.0",
    tags: &[ "deprecated" ],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "Use .test.version instead",
    http_method_hint: "GET",
    examples: &[],
  },
};

/// Wrapper for test static commands
static TEST_STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&TEST_STATIC_COMMANDS_PHF);

#[test]
fn test_static_command_registry_creation()
{
  // Test basic creation with default mode
  let registry = StaticCommandRegistry::new();
  assert_eq!( registry.mode(), RegistryMode::Hybrid );

  // Test creation with specific mode
  let static_registry = StaticCommandRegistry::with_mode( RegistryMode::StaticOnly );
  assert_eq!( static_registry.mode(), RegistryMode::StaticOnly );

  let dynamic_registry = StaticCommandRegistry::with_mode( RegistryMode::DynamicOnly );
  assert_eq!( dynamic_registry.mode(), RegistryMode::DynamicOnly );

  let hybrid_registry = StaticCommandRegistry::with_mode( RegistryMode::Hybrid );
  assert_eq!( hybrid_registry.mode(), RegistryMode::Hybrid );
}

#[test]
fn test_static_command_registry_from_commands()
{
  // Test creation from static command map
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );
  assert_eq!( registry.mode(), RegistryMode::Hybrid );

  // Should contain all static commands
  let commands = registry.commands();
  assert!( !commands.is_empty() );

  // Verify specific commands exist
  assert!( commands.contains_key( ".test.version" ) );
  assert!( commands.contains_key( ".test.add" ) );
  assert!( commands.contains_key( ".test.file.copy" ) );
  assert!( commands.contains_key( ".test.deprecated" ) );
}

#[test]
fn test_static_command_lookup_functionality()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test successful lookups
  let version_cmd = registry.command( ".test.version" );
  assert!( version_cmd.is_some() );
  let version_cmd = version_cmd.unwrap();
  assert_eq!( version_cmd.name, ".test.version" );
  assert_eq!( version_cmd.description, "Show version information" );
  assert_eq!( version_cmd.namespace, ".test" );

  // Test command with arguments
  let add_cmd = registry.command( ".test.add" );
  assert!( add_cmd.is_some() );
  let add_cmd = add_cmd.unwrap();
  assert_eq!( add_cmd.arguments.len(), 2 );
  assert_eq!( add_cmd.arguments[0].name, "a" );
  assert_eq!( add_cmd.arguments[1].name, "b" );

  // Test complex command
  let copy_cmd = registry.command( ".test.file.copy" );
  assert!( copy_cmd.is_some() );
  let copy_cmd = copy_cmd.unwrap();
  assert_eq!( copy_cmd.arguments.len(), 3 );
  assert_eq!( copy_cmd.permissions, vec![ "read", "write" ] );

  // Test non-existent command
  let missing_cmd = registry.command( ".test.nonexistent" );
  assert!( missing_cmd.is_none() );
}

#[test]
fn test_static_command_registry_mode_switching()
{
  let mut registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test mode switching
  registry.set_mode( RegistryMode::StaticOnly );
  assert_eq!( registry.mode(), RegistryMode::StaticOnly );

  registry.set_mode( RegistryMode::DynamicOnly );
  assert_eq!( registry.mode(), RegistryMode::DynamicOnly );

  registry.set_mode( RegistryMode::Hybrid );
  assert_eq!( registry.mode(), RegistryMode::Hybrid );

  // Commands should still be accessible after mode changes
  assert!( registry.command( ".test.version" ).is_some() );
}

#[test]
fn test_static_command_registry_performance_metrics()
{
  let mut registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Initial metrics should be zero
  let initial_metrics = registry.performance_metrics();
  let initial_total_lookups = initial_metrics.total_lookups;
  assert_eq!( initial_total_lookups, 0 );
  assert_eq!( initial_metrics.cache_hits, 0 );
  assert_eq!( initial_metrics.cache_misses, 0 );

  // Perform some lookups (using mutable methods to update metrics)
  let _cmd1 = registry.command_with_metrics( ".test.version" );
  let _cmd2 = registry.command_with_metrics( ".test.add" );
  let _cmd3 = registry.command_with_metrics( ".test.nonexistent" );

  // Metrics should be updated
  let updated_metrics = registry.performance_metrics();
  assert!( updated_metrics.total_lookups > initial_total_lookups );

  // Test metrics reset by clearing the registry
  registry.clear();
  let reset_metrics = registry.performance_metrics();
  assert_eq!( reset_metrics.total_lookups, 0 );
  assert_eq!( reset_metrics.cache_hits, 0 );
  assert_eq!( reset_metrics.cache_misses, 0 );
}

#[test]
fn test_static_command_registry_performance_characteristics()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test lookup performance - should be sub-microsecond
  let iterations = 10_000;
  let commands_to_test = vec![ ".test.version", ".test.add", ".test.file.copy" ];

  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &commands_to_test
    {
      let _cmd = registry.command( cmd_name );
    }
  }
  let duration = start.elapsed();

  let total_lookups = iterations * commands_to_test.len();
  let avg_lookup_time = duration / u32::try_from(total_lookups).unwrap_or(1);

  // Should be fast lookup (allow up to 30 microseconds in debug builds due to lack of optimization and system variance)
  assert!( avg_lookup_time.as_nanos() < 30_000, "Average lookup time should be < 30μs, got: {avg_lookup_time:?}" );

  println!( "Performance test: {total_lookups} lookups in {duration:?}, avg: {avg_lookup_time:?} per lookup" );
}

#[test]
fn test_static_command_registry_with_dynamic_commands()
{
  let mut registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Add dynamic command
  let dynamic_cmd = CommandDefinition::former()
    .name( ".test.dynamic" )
    .description( "Dynamic test command".to_string() )
    .namespace( ".test".to_string() )
    .end();

  registry.register( dynamic_cmd );

  // Should have both static and dynamic commands
  let commands = registry.commands();
  assert!( commands.contains_key( ".test.version" ) ); // Static
  assert!( commands.contains_key( ".test.dynamic" ) ); // Dynamic

  // Both should be accessible
  assert!( registry.command( ".test.version" ).is_some() );
  assert!( registry.command( ".test.dynamic" ).is_some() );
}

#[test]
fn test_static_command_registry_command_registry_trait()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test that StaticCommandRegistry implements CommandRegistryTrait
  let trait_registry: &dyn CommandRegistryTrait = &registry;

  // Test trait methods
  let commands = trait_registry.commands();
  assert!( !commands.is_empty() );

  let cmd = trait_registry.command( ".test.version" );
  assert!( cmd.is_some() );

  let help = trait_registry.get_help_for_command( ".test.version" );
  assert!( help.is_some() );

  let routine = trait_registry.get_routine( ".test.version" );
  assert!( routine.is_none() ); // No routine registered in test
}

#[test]
fn test_static_command_registry_static_commands_count()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Should report correct static command count
  let static_count = registry.static_command_count();
  assert_eq!( static_count, TEST_STATIC_COMMANDS.len() );

  // Static commands list should match PHF map
  let static_commands = registry.static_commands();
  assert_eq!( static_commands.len(), TEST_STATIC_COMMANDS.len() );

  for key in TEST_STATIC_COMMANDS.keys()
  {
    assert!( static_commands.contains( &(*key).to_string() ) );
  }
}

#[test]
fn test_static_command_registry_registry_mode_behavior()
{
  let mut registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Add a dynamic command
  let dynamic_cmd = CommandDefinition::former()
    .name( ".test.dynamic_only" )
    .description( "Dynamic only command".to_string() )
    .end();
  registry.register( dynamic_cmd );

  // Test StaticOnly mode
  registry.set_mode( RegistryMode::StaticOnly );
  assert!( registry.command( ".test.version" ).is_some() ); // Static should work
  assert!( registry.command( ".test.dynamic_only" ).is_none() ); // Dynamic should not work

  // Test DynamicOnly mode
  registry.set_mode( RegistryMode::DynamicOnly );
  assert!( registry.command( ".test.version" ).is_none() ); // Static should not work
  assert!( registry.command( ".test.dynamic_only" ).is_some() ); // Dynamic should work

  // Test Hybrid mode
  registry.set_mode( RegistryMode::Hybrid );
  assert!( registry.command( ".test.version" ).is_some() ); // Static should work
  assert!( registry.command( ".test.dynamic_only" ).is_some() ); // Dynamic should work
}

#[test]
fn test_static_command_registry_edge_cases()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test empty string lookup
  assert!( registry.command( "" ).is_none() );

  // Test whitespace
  assert!( registry.command( " " ).is_none() );
  assert!( registry.command( "\t" ).is_none() );
  assert!( registry.command( "\n" ).is_none() );

  // Test case sensitivity
  assert!( registry.command( ".Test.Version" ).is_none() );
  assert!( registry.command( ".TEST.VERSION" ).is_none() );

  // Test partial matches
  assert!( registry.command( ".test" ).is_none() );
  assert!( registry.command( ".test.ver" ).is_none() );

  // Test invalid command names
  assert!( registry.command( "test.version" ).is_none() ); // Missing dot prefix
  assert!( registry.command( ".test.version.extra" ).is_none() );
}

#[test]
fn test_static_command_registry_memory_efficiency()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test that multiple lookups of the same command return the same data
  let cmd1 = registry.command( ".test.version" ).unwrap();
  let cmd2 = registry.command( ".test.version" ).unwrap();

  // Commands should be equal
  assert_eq!( cmd1.name, cmd2.name );
  assert_eq!( cmd1.description, cmd2.description );
  assert_eq!( cmd1.namespace, cmd2.namespace );

  // Test that all commands are accessible
  let all_commands = registry.commands();
  for (name, _) in TEST_STATIC_COMMANDS.entries()
  {
    assert!( all_commands.contains_key( *name ), "Command {name} should be accessible" );
  }
}

#[test]
fn test_static_command_registry_command_conversion_accuracy()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test that static commands are converted accurately to dynamic commands
  let add_cmd = registry.command( ".test.add" ).unwrap();

  // Verify all fields are converted correctly
  assert_eq!( add_cmd.name, ".test.add" );
  assert_eq!( add_cmd.namespace, ".test" );
  assert_eq!( add_cmd.description, "Add two numbers" );
  assert_eq!( add_cmd.hint, "Mathematical addition" );
  assert_eq!( add_cmd.status, "stable" );
  assert_eq!( add_cmd.version, "1.0.0" );
  assert_eq!( add_cmd.tags, vec![ "math", "calculation" ] );
  assert_eq!( add_cmd.aliases, vec![ "plus", "sum" ] );
  assert_eq!( add_cmd.permissions, vec![ "read" ] );
  assert!( add_cmd.idempotent );
  assert_eq!( add_cmd.deprecation_message, "" );
  assert_eq!( add_cmd.http_method_hint, "POST" );
  assert_eq!( add_cmd.examples, vec![ ".test.add 5 3", ".test.add --a 10 --b 20" ] );

  // Test arguments conversion
  assert_eq!( add_cmd.arguments.len(), 2 );

  let arg_a = &add_cmd.arguments[0];
  assert_eq!( arg_a.name, "a" );
  assert_eq!( arg_a.hint, "First number" );
  assert_eq!( arg_a.description, "The first number to add" );
  assert!( matches!( arg_a.kind, Kind::Integer ) );
  assert!( !arg_a.attributes.optional );
  assert_eq!( arg_a.aliases, vec![ "first", "num1" ] );

  let arg_b = &add_cmd.arguments[1];
  assert_eq!( arg_b.name, "b" );
  assert_eq!( arg_b.hint, "Second number" );
  assert!( matches!( arg_b.kind, Kind::Integer ) );
}

#[test]
fn test_static_command_registry_deprecated_commands()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test deprecated command is accessible
  let deprecated_cmd = registry.command( ".test.deprecated" );
  assert!( deprecated_cmd.is_some() );

  let cmd = deprecated_cmd.unwrap();
  assert_eq!( cmd.status, "deprecated" );
  assert_eq!( cmd.deprecation_message, "Use .test.version instead" );
}

#[test]
fn test_static_command_registry_concurrent_access()
{
  use std::sync::Arc;
  use std::thread;

  let registry = Arc::new( StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS ) );
  let mut handles = vec![];

  // Spawn multiple threads doing concurrent lookups
  for i in 0..10
  {
    let registry_clone = Arc::clone( &registry );
    let handle = thread::spawn( move ||
    {
      let commands = vec![ ".test.version", ".test.add", ".test.file.copy" ];

      for _ in 0..100
      {
        for cmd_name in &commands
        {
          let cmd = registry_clone.command( cmd_name );
          assert!( cmd.is_some(), "Thread {i} failed to lookup {cmd_name}" );
        }
      }
    } );
    handles.push( handle );
  }

  // Wait for all threads to complete
  for handle in handles
  {
    handle.join().expect( "Thread panicked" );
  }
}

#[test]
fn test_static_command_registry_large_scale_lookup()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test large number of lookups for stability
  let iterations = 100_000;
  let mut successful_lookups = 0;
  let mut failed_lookups = 0;

  for i in 0..iterations
  {
    let cmd_name = match i % 4
    {
      0 => ".test.version",
      1 => ".test.add",
      2 => ".test.file.copy",
      _ => ".test.nonexistent",
    };

    match registry.command( cmd_name )
    {
      Some( _ ) => successful_lookups += 1,
      None => failed_lookups += 1,
    }
  }

  // Should have 75% success rate (3 valid commands out of 4 tested)
  let expected_successful = iterations * 3 / 4;
  let expected_failed = iterations / 4;

  assert_eq!( successful_lookups, expected_successful );
  assert_eq!( failed_lookups, expected_failed );
}

#[test]
fn test_static_command_registry_help_generation()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test help generation for different commands
  let version_help = registry.get_help_for_command( ".test.version" );
  assert!( version_help.is_some() );
  let help_text = version_help.unwrap();
  assert!( help_text.contains( "Show version information" ) );
  assert!( help_text.contains( ".test.version" ) );

  // Test help for command with arguments
  let add_help = registry.get_help_for_command( ".test.add" );
  assert!( add_help.is_some() );
  let add_help_text = add_help.unwrap();
  assert!( add_help_text.contains( "Add two numbers" ) );
  assert!( add_help_text.contains( "first number" ) );
  assert!( add_help_text.contains( "second number" ) );

  // Test help for non-existent command
  let missing_help = registry.get_help_for_command( ".test.nonexistent" );
  assert!( missing_help.is_none() );
}

#[test]
fn test_static_command_registry_routine_management()
{
  let registry = StaticCommandRegistry::from_commands( &TEST_STATIC_COMMANDS );

  // Test routine retrieval for static commands (should be None since no routines registered)
  let routine = registry.get_routine( ".test.version" );
  assert!( routine.is_none() );

  // Test routine for non-existent command
  let missing_routine = registry.get_routine( ".test.nonexistent" );
  assert!( missing_routine.is_none() );

  // Test that we can check for routine existence
  assert!( registry.routine( ".test.version" ).is_none() );
}