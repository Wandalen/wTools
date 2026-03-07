//! Build script for `claude_storage` CLI using unilang framework
//!
//! Generates static command registry at compile-time for O(1) command lookups
//! Transforms simplified YAML to unilang's nested attributes schema

use std::{ env, path::PathBuf };

fn main()
{
  // Trigger rebuild if files change
  println!( "cargo:rerun-if-changed=unilang.commands.yaml" );
  println!( "cargo:rerun-if-changed=build.rs" );

  // Only generate commands if cli feature is enabled
  if env::var( "CARGO_FEATURE_CLI" ).is_ok()
  {
    generate_static_commands();
  }
}

/// Generate static command registry from YAML
fn generate_static_commands()
{
  let out_dir = env::var( "OUT_DIR" )
    .expect( "OUT_DIR environment variable not set" );

  // Read and transform YAML
  let yaml_content = std::fs::read_to_string( "unilang.commands.yaml" )
    .expect( "Failed to read unilang.commands.yaml" );

  let mut commands : Vec< serde_yaml::Value > = serde_yaml::from_str( &yaml_content )
    .expect( "Failed to parse unilang.commands.yaml" );

  // Transform arguments to nested attributes structure
  for command in &mut commands
  {
    if let Some( args ) = command.get_mut( "arguments" ).and_then( | v | v.as_sequence_mut() )
    {
      for arg in args
      {
        if let Some( map ) = arg.as_mapping_mut()
        {
          transform_argument_attributes( map );
        }
      }
    }
  }

  // Write transformed YAML
  let temp_yaml_path = PathBuf::from( &out_dir ).join( "commands.yaml" );
  let transformed_yaml = serde_yaml::to_string( &commands )
    .expect( "Failed to serialize transformed commands" );
  std::fs::write( &temp_yaml_path, transformed_yaml )
    .expect( "Failed to write transformed commands" );

  // Configure aggregation
  let config = unilang::multi_yaml::AggregationConfig
  {
    base_dir : PathBuf::from( &out_dir ),
    modules : vec!
    [
      unilang::multi_yaml::ModuleConfig
      {
        name : "claude_storage".to_string(),
        yaml_path : "commands.yaml".to_string(),
        prefix : None,
        enabled : true,
      },
    ],
    global_prefix : None,
    detect_conflicts : true,
    env_overrides : std::collections::HashMap::new(),
    conflict_resolution : unilang::multi_yaml::ConflictResolutionStrategy::Fail,
    auto_discovery : false,
    discovery_patterns : vec![],
    namespace_isolation : unilang::multi_yaml::NamespaceIsolation
    {
      enabled : false,
      separator : ".".to_string(),
      strict_mode : false,
    },
  };

  // Generate static registry
  let mut aggregator = unilang::multi_yaml::MultiYamlAggregator::new( config );

  match aggregator.aggregate()
  {
    Ok( () ) =>
    {
      // Check for conflicts
      if !aggregator.conflicts().is_empty()
      {
        eprintln!( "ERROR: Command conflicts detected:" );
        for conflict in aggregator.conflicts()
        {
          eprintln!( "  - Command '{}' in modules: {:?}", conflict.command_name, conflict.modules );
        }
        panic!( "Build failed due to command conflicts" );
      }

      // Generate source code
      let mut registry_source = aggregator.generate_static_registry_source();

      // Fix(issue-unilang-show-version): Inject missing show_version_in_help field.
      // Root cause: unilang v0.45+ requires this field but MultiYamlAggregator doesn't generate it.
      // Pitfall: Build scripts must inject new required fields until generator is updated.
      // Also must check if field already exists to avoid duplicates with newer unilang versions.
      if !registry_source.contains( "show_version_in_help:" )
      {
        registry_source = registry_source.replace(
          "  auto_help_enabled: true,\n",
          "  auto_help_enabled: true,\n  show_version_in_help: true,\n"
        );
        registry_source = registry_source.replace(
          "  auto_help_enabled: false,\n",
          "  auto_help_enabled: false,\n  show_version_in_help: true,\n"
        );
      }

      // Write to output file
      let output_path = PathBuf::from( &out_dir ).join( "static_commands.rs" );
      std::fs::write( &output_path, &registry_source )
        .expect( "Failed to write static_commands.rs" );

      let command_count = commands.len();
      println!( "cargo:warning=Generated static command registry with {command_count} commands" );
    }
    Err( e ) =>
    {
      panic!( "Failed to aggregate commands: {e}" );
    }
  }
}

/// Transform argument from flat to nested attributes structure
///
/// **Why this transformation exists:**
/// unilang v0.35+ requires argument attributes in a nested structure, but writing
/// that structure manually is verbose and error-prone. This function transforms a
/// simplified YAML format (flat `optional` and `default` fields) into the nested
/// `attributes` object that unilang expects.
///
/// **Input format** (simplified, user-friendly):
/// ```yaml
/// - name: "verbosity"
///   kind: "Integer"
///   optional: true
///   default: "1"
/// ```
///
/// **Output format** (unilang schema):
/// ```yaml
/// - name: "verbosity"
///   kind: "Integer"
///   attributes:
///     optional: true
///     default: "1"
///     sensitive: false
///     interactive: false
///     multiple: false
/// ```
///
/// **Design decision**: This transformation happens at build time (not runtime) so
/// that command definitions remain readable while still conforming to unilang's schema.
fn transform_argument_attributes( arg_map : &mut serde_yaml::Mapping )
{
  // Extract flat attributes (will be removed from root)
  let optional = arg_map
    .remove( serde_yaml::Value::String( "optional".to_string() ) )
    .and_then( | v | v.as_bool() )
    .unwrap_or( false );

  let default_val = arg_map
    .remove( serde_yaml::Value::String( "default".to_string() ) )
    .and_then( | v | match v
    {
      serde_yaml::Value::String( s ) => Some( s ),
      serde_yaml::Value::Number( n ) => Some( n.to_string() ),
      serde_yaml::Value::Bool( b ) => Some( b.to_string() ),
      _ => None,
    });

  // Build nested attributes structure
  let mut attributes = serde_yaml::Mapping::new();
  attributes.insert
  (
    serde_yaml::Value::String( "optional".to_string() ),
    serde_yaml::Value::Bool( optional )
  );
  attributes.insert
  (
    serde_yaml::Value::String( "sensitive".to_string() ),
    serde_yaml::Value::Bool( false )
  );
  attributes.insert
  (
    serde_yaml::Value::String( "interactive".to_string() ),
    serde_yaml::Value::Bool( false )
  );
  attributes.insert
  (
    serde_yaml::Value::String( "multiple".to_string() ),
    serde_yaml::Value::Bool( false )
  );

  if let Some( default_str ) = default_val
  {
    attributes.insert
    (
      serde_yaml::Value::String( "default".to_string() ),
      serde_yaml::Value::String( default_str )
    );
  }

  // Insert attributes into argument
  arg_map.insert
  (
    serde_yaml::Value::String( "attributes".to_string() ),
    serde_yaml::Value::Mapping( attributes )
  );
}
