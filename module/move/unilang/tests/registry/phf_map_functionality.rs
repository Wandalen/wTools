//!
//! Tests for PHF (Perfect Hash Function) map functionality.
//!
//! This module tests the core PHF map functionality including `StaticCommandDefinition`
//! conversions, PHF map generation, lookup performance, and integration with the
//! command registry system.

use unilang::prelude::*;
use unilang::static_data::{ StaticCommandDefinition, StaticCommandMap, StaticArgumentDefinition, StaticKind, StaticArgumentAttributes, StaticValidationRule };
use unilang::multi_yaml::{ MultiYamlAggregator, AggregationConfig, ModuleConfig };
use unilang::registry::StaticCommandRegistry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

/// Create a test `StaticCommandDefinition` for testing
fn create_test_static_command() -> &'static StaticCommandDefinition
{
  &StaticCommandDefinition
  {
    name: "test_command",
    namespace: "test",
    description: "A test command for PHF functionality",
    hint: "Test command hint",
    arguments: &[
      StaticArgumentDefinition
      {
        name: "input",
        description: "Input parameter",
        hint: "String input",
        kind: StaticKind::String,
        attributes: StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[],
        aliases: &[],
        tags: &[],
      },
    ],
    routine_link: Some("test_routine"),
    status: "stable",
    version: "1.0.0",
    tags: &["test", "functionality"],
    aliases: &["tc", "test"],
    permissions: &["read"],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &["test_command input_value", "test_command --help"],
  }
}

/// Create a test PHF map for testing
const TEST_PHF_MAP_INTERNAL: phf::Map<&'static str, &'static StaticCommandDefinition> = phf::phf_map!
{
  "test_command" => &StaticCommandDefinition
  {
    name: "test_command",
    namespace: "test",
    description: "A test command for PHF functionality",
    hint: "Test command hint",
    arguments: &[],
    routine_link: Some("test_routine"),
    status: "stable",
    version: "1.0.0",
    tags: &["test"],
    aliases: &["tc"],
    permissions: &["read"],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &["test_command"],
  },
  "another_command" => &StaticCommandDefinition
  {
    name: "another_command",
    namespace: "test",
    description: "Another test command",
    hint: "Another hint",
    arguments: &[],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: false,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[],
  },
};

static TEST_PHF_MAP: StaticCommandMap = StaticCommandMap::from_phf_internal(&TEST_PHF_MAP_INTERNAL);

#[test]
fn test_static_command_definition_structure()
{
  // Test StaticCommandDefinition basic structure
  let static_cmd = create_test_static_command();

  assert_eq!( static_cmd.name, "test_command" );
  assert_eq!( static_cmd.namespace, "test" );
  assert_eq!( static_cmd.description, "A test command for PHF functionality" );
  assert_eq!( static_cmd.hint, "Test command hint" );
  assert_eq!( static_cmd.status, "stable" );
  assert_eq!( static_cmd.version, "1.0.0" );
  assert!( static_cmd.idempotent );
  assert_eq!( static_cmd.routine_link, Some("test_routine") );

  // Test arguments
  assert_eq!( static_cmd.arguments.len(), 1 );
  assert_eq!( static_cmd.arguments[0].name, "input" );
  // Test argument kind (can't use equality since StaticKind doesn't implement PartialEq)
  match static_cmd.arguments[0].kind
  {
    StaticKind::String => { /* Expected */ },
    _ => panic!( "Expected StaticKind::String" ),
  }
  assert!( !static_cmd.arguments[0].attributes.optional );

  // Test arrays
  assert_eq!( static_cmd.tags.len(), 2 );
  assert_eq!( static_cmd.aliases.len(), 2 );
  assert_eq!( static_cmd.permissions.len(), 1 );
  assert_eq!( static_cmd.examples.len(), 2 );
}

#[test]
fn test_static_to_dynamic_command_conversion()
{
  // Test conversion from StaticCommandDefinition to CommandDefinition
  let static_cmd = create_test_static_command();
  let dynamic_cmd: CommandDefinition = static_cmd.into();

  // Verify basic fields
  assert_eq!( dynamic_cmd.name, "test_command" );
  assert_eq!( dynamic_cmd.namespace, "test" );
  assert_eq!( dynamic_cmd.description, "A test command for PHF functionality" );
  assert_eq!( dynamic_cmd.hint, "Test command hint" );
  assert_eq!( dynamic_cmd.status, "stable" );
  assert_eq!( dynamic_cmd.version, "1.0.0" );
  assert!( dynamic_cmd.idempotent );

  // Verify arguments were converted
  assert_eq!( dynamic_cmd.arguments.len(), 1 );
  assert_eq!( dynamic_cmd.arguments[0].name, "input" );
  assert!( !dynamic_cmd.arguments[0].attributes.optional );

  // Verify collections were converted
  assert_eq!( dynamic_cmd.tags.len(), 2 );
  assert!( dynamic_cmd.tags.contains( &"test".to_string() ) );
  assert!( dynamic_cmd.tags.contains( &"functionality".to_string() ) );

  assert_eq!( dynamic_cmd.aliases.len(), 2 );
  assert!( dynamic_cmd.aliases.contains( &"tc".to_string() ) );

  assert_eq!( dynamic_cmd.permissions.len(), 1 );
  assert!( dynamic_cmd.permissions.contains( &"read".to_string() ) );

  assert_eq!( dynamic_cmd.examples.len(), 2 );
}

#[test]
fn test_phf_map_basic_functionality()
{
  // Test basic PHF map operations
  assert_eq!( TEST_PHF_MAP.len(), 2 );

  // Test contains_key
  assert!( TEST_PHF_MAP.contains_key("test_command") );
  assert!( TEST_PHF_MAP.contains_key("another_command") );
  assert!( !TEST_PHF_MAP.contains_key("nonexistent") );

  // Test get
  let cmd1 = TEST_PHF_MAP.get("test_command");
  assert!( cmd1.is_some() );
  assert_eq!( cmd1.unwrap().name, "test_command" );

  let cmd2 = TEST_PHF_MAP.get("another_command");
  assert!( cmd2.is_some() );
  assert_eq!( cmd2.unwrap().name, "another_command" );

  let cmd3 = TEST_PHF_MAP.get("nonexistent");
  assert!( cmd3.is_none() );
}

#[test]
fn test_phf_map_iteration()
{
  // Test PHF map iteration
  let mut count = 0;
  let mut names = Vec::new();

  for (key, value) in TEST_PHF_MAP.entries()
  {
    count += 1;
    names.push( (*key).to_string() );
    assert_eq!( *key, value.name );
  }

  assert_eq!( count, 2 );
  assert!( names.contains( &"test_command".to_string() ) );
  assert!( names.contains( &"another_command".to_string() ) );

  // Test keys()
  let keys: Vec<&str> = TEST_PHF_MAP.keys().copied().collect();
  assert_eq!( keys.len(), 2 );
  assert!( keys.contains( &"test_command" ) );
  assert!( keys.contains( &"another_command" ) );
}

#[test]
fn test_phf_map_lookup_performance()
{
  // Test PHF map lookup performance characteristics
  let iterations = 100_000;
  let commands_to_test = vec!["test_command", "another_command", "nonexistent"];

  // Warmup
  for _ in 0..10_000
  {
    for cmd_name in &commands_to_test
    {
      let _result = TEST_PHF_MAP.get(cmd_name);
    }
  }

  // Benchmark
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &commands_to_test
    {
      let _result = TEST_PHF_MAP.get(cmd_name);
    }
  }
  let duration = start.elapsed();

  let total_lookups = iterations * commands_to_test.len();
  let avg_lookup_time = duration / u32::try_from(total_lookups).unwrap_or(1);

  // PHF lookups should be very fast (< 100ns in optimized builds)
  println!( "PHF lookup performance: {total_lookups} lookups in {duration:?}, avg: {avg_lookup_time:?} per lookup" );

  // For debug builds and workspace context, be lenient with timing
  // Allow up to 10μs for debug/workspace builds since optimization and build context affect performance
  assert!( avg_lookup_time.as_nanos() < 10_000, "PHF lookup should be < 10μs, got: {avg_lookup_time:?}" );
}

#[test]
fn test_static_command_registry_with_phf()
{
  // Test StaticCommandRegistry integration with PHF map
  let registry = StaticCommandRegistry::from_commands( &TEST_PHF_MAP );

  // Test command lookup
  let cmd1 = registry.command( "test_command" );
  assert!( cmd1.is_some() );
  assert_eq!( cmd1.unwrap().name, "test_command" );

  let cmd2 = registry.command( "another_command" );
  assert!( cmd2.is_some() );
  assert_eq!( cmd2.unwrap().name, "another_command" );

  let cmd3 = registry.command( "nonexistent" );
  assert!( cmd3.is_none() );

  // Test commands() method
  let all_commands = registry.commands();
  assert_eq!( all_commands.len(), 2 );
  assert!( all_commands.contains_key( "test_command" ) );
  assert!( all_commands.contains_key( "another_command" ) );

  // Test static_command_count
  assert_eq!( registry.static_command_count(), 2 );
}

#[test]
fn test_multi_yaml_aggregator_phf_generation()
{
  // Test PHF map generation from MultiYamlAggregator
  let config = AggregationConfig
  {
    base_dir: PathBuf::from("test"),
    modules: vec![
      ModuleConfig
      {
        name: "test_module".to_string(),
        yaml_path: "test.yaml".to_string(),
        prefix: Some("test".to_string()),
        enabled: true,
      },
    ],
    global_prefix: None,
    detect_conflicts: false,
    env_overrides: HashMap::new(),
    ..Default::default()
  };

  let aggregator = MultiYamlAggregator::new( config );

  // Generate PHF map content
  let phf_content = aggregator.generate_phf_map();

  // Verify PHF map structure
  assert!( phf_content.contains("use phf::{phf_map, Map}") );
  assert!( phf_content.contains("StaticCommandDefinition") );
  assert!( phf_content.contains("phf_map!") );

  // Print content for debugging
  println!( "Generated PHF map content:\n{phf_content}" );

  // Verify the content has basic PHF structure (the exact type name may vary)
  assert!( phf_content.contains("phf_map") || phf_content.contains("Map") );
}

#[test]
fn test_static_command_attributes()
{
  // Test StaticArgumentAttributes structure
  let attrs = StaticArgumentAttributes
  {
    optional: true,
    multiple: false,
    default: Some("default_value"),
    sensitive: true,
    interactive: false,
  };

  assert!( attrs.optional );
  assert!( !attrs.multiple );
  assert_eq!( attrs.default, Some("default_value") );
  assert!( attrs.sensitive );
  assert!( !attrs.interactive );
}

#[test]
fn test_static_argument_definition()
{
  // Test StaticArgumentDefinition structure
  let arg_def = StaticArgumentDefinition
  {
    name: "test_arg",
    description: "Test argument",
    hint: "String input",
    kind: StaticKind::String,
    attributes: StaticArgumentAttributes
    {
      optional: false,
      multiple: true,
      default: None,
      sensitive: false,
      interactive: true,
    },
    validation_rules: &[StaticValidationRule::MinLength(1), StaticValidationRule::MaxLength(100)],
    aliases: &["ta", "test"],
    tags: &["required", "input"],
  };

  assert_eq!( arg_def.name, "test_arg" );
  assert_eq!( arg_def.description, "Test argument" );
  // Test argument kind (can't use equality since StaticKind doesn't implement PartialEq)
  match arg_def.kind
  {
    StaticKind::String => { /* Expected */ },
    _ => panic!( "Expected StaticKind::String" ),
  }
  assert!( !arg_def.attributes.optional );
  assert!( arg_def.attributes.multiple );
  assert!( arg_def.attributes.interactive );

  assert_eq!( arg_def.validation_rules.len(), 2 );
  assert_eq!( arg_def.aliases.len(), 2 );
  assert_eq!( arg_def.tags.len(), 2 );
}

#[test]
#[allow(clippy::no_effect_underscore_binding)]
fn test_static_kind_variants()
{
  // Test StaticKind enum variants (can't use equality since StaticKind doesn't implement PartialEq)
  let string_kind = StaticKind::String;
  let integer_kind = StaticKind::Integer;
  let _float_kind = StaticKind::Float;
  let _boolean_kind = StaticKind::Boolean;
  let _path_kind = StaticKind::Path;

  // Test that variants can be created and cloned
  let _cloned_string = string_kind;
  let _cloned_integer = integer_kind;

  // Test pattern matching instead of equality
  match string_kind
  {
    StaticKind::String => { /* Expected */ },
    _ => panic!( "Expected StaticKind::String" ),
  }

  match integer_kind
  {
    StaticKind::Integer => { /* Expected */ },
    _ => panic!( "Expected StaticKind::Integer" ),
  }

  // Test that all variants exist
  let _all_variants = [
    StaticKind::String,
    StaticKind::Integer,
    StaticKind::Float,
    StaticKind::Boolean,
    StaticKind::Path,
  ];
}

#[test]
fn test_phf_map_memory_characteristics()
{
  // Test memory characteristics of PHF maps
  let map_size = core::mem::size_of_val( &TEST_PHF_MAP );
  let cmd_size = core::mem::size_of::< &StaticCommandDefinition >();
  let key_size = core::mem::size_of::< &str >();

  println!( "PHF map size: {map_size} bytes" );
  println!( "Command reference size: {cmd_size} bytes" );
  println!( "Key reference size: {key_size} bytes" );

  // PHF maps should have minimal memory overhead
  // The exact size will depend on the PHF implementation, but should be reasonable
  assert!( map_size < 1000, "PHF map should have reasonable memory footprint" );
}

/// Static complex command for testing
static COMPLEX_COMMAND: StaticCommandDefinition = StaticCommandDefinition
  {
    name: "complex_command",
    namespace: "advanced",
    description: "A complex command with multiple arguments",
    hint: "Complex operation",
    arguments: &[
      StaticArgumentDefinition
      {
        name: "input_file",
        description: "Input file path",
        hint: "File path",
        kind: StaticKind::Path,
        attributes: StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[StaticValidationRule::Pattern("exists"), StaticValidationRule::Pattern("readable")],
        aliases: &["i", "input"],
        tags: &["required", "file"],
      },
      StaticArgumentDefinition
      {
        name: "output_file",
        description: "Output file path",
        hint: "File path",
        kind: StaticKind::Path,
        attributes: StaticArgumentAttributes
        {
          optional: true,
          multiple: false,
          default: Some("output.txt"),
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[StaticValidationRule::Pattern("writable")],
        aliases: &["o", "output"],
        tags: &["optional", "file"],
      },
      StaticArgumentDefinition
      {
        name: "verbose",
        description: "Enable verbose output",
        hint: "Boolean flag",
        kind: StaticKind::Boolean,
        attributes: StaticArgumentAttributes
        {
          optional: true,
          multiple: false,
          default: Some("false"),
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[],
        aliases: &["v"],
        tags: &["flag"],
      },
    ],
    routine_link: Some("complex_routine"),
    status: "stable",
    version: "2.1.0",
    tags: &["complex", "advanced", "file-processing"],
    aliases: &["cc", "complex"],
    permissions: &["read", "write"],
    idempotent: false,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[
      "complex_command input.txt",
      "complex_command input.txt --output output.txt",
      "complex_command input.txt -o output.txt --verbose",
    ],
  };

#[test]
fn test_complex_command_with_multiple_arguments()
{
  // Test conversion to dynamic command
  let dynamic_cmd: CommandDefinition = (&COMPLEX_COMMAND).into();

  assert_eq!( dynamic_cmd.name, "complex_command" );
  assert_eq!( dynamic_cmd.arguments.len(), 3 );

  // Test first argument (required file)
  assert_eq!( dynamic_cmd.arguments[0].name, "input_file" );
  assert!( !dynamic_cmd.arguments[0].attributes.optional );

  // Test second argument (optional file with default)
  assert_eq!( dynamic_cmd.arguments[1].name, "output_file" );
  assert!( dynamic_cmd.arguments[1].attributes.optional );

  // Test third argument (boolean flag)
  assert_eq!( dynamic_cmd.arguments[2].name, "verbose" );
  assert!( dynamic_cmd.arguments[2].attributes.optional );

  // Test collections
  assert_eq!( dynamic_cmd.tags.len(), 3 );
  assert_eq!( dynamic_cmd.aliases.len(), 2 );
  assert_eq!( dynamic_cmd.permissions.len(), 2 );
  assert_eq!( dynamic_cmd.examples.len(), 3 );
}