//! Build script for `claude_runner` CLI using unilang framework.
//!
//! Implements compile-time command generation using unilang's Multi-YAML approach.
//! Commands are processed at build time and generate a static PHF map for
//! ~80ns command lookups (50x faster than `HashMap`).
//!
//! ## Commands
//!
//! **`claude_runner` provides 2 commands:**
//! - `.claude` - AI-assisted development with Claude Code
//! - `.claude.help` - Show help for `.claude` command
//!
//! Planning orchestration (`.plan.claude`) has moved to `wplan` as the
//! runner plugin system, where it is invoked via `.plan runner::claude`.

use std::env;
use std::path::PathBuf;

fn main()
{
  // Auto-bump version on release builds
  if let Err( e ) = version_bump::build_support::auto_bump_on_release()
  {
    eprintln!( "Warning: Version auto-bump failed: {e}" );
  }

  // Trigger rebuild if files change
  println!( "cargo:rerun-if-changed=claude.commands.yaml" );
  println!( "cargo:rerun-if-changed=build.rs" );

  // Get output directory for generated code
  let out_dir = env::var( "OUT_DIR" )
    .expect( "OUT_DIR environment variable not set" );

  // Load and transform commands, generate static registry
  let commands = load_and_transform_commands();
  generate_static_commands( &out_dir, &commands );
}

/// Load and transform commands from simplified YAML to unilang's nested schema.
///
/// Loads the 2 `claude_runner` commands (`.claude`, `.claude.help`).
/// Injects crate version into all commands.
fn load_and_transform_commands() -> Vec< serde_yaml::Value >
{
  let crate_version = env::var( "CARGO_PKG_VERSION" )
    .unwrap_or_else( | _ | "0.0.0".to_string() );

  let mut commands = load_yaml_and_transform( "claude.commands.yaml" );

  // Inject crate version into all commands
  for command in &mut commands
  {
    if let Some( map ) = command.as_mapping_mut()
    {
      map.insert(
        serde_yaml::Value::String( "version".to_string() ),
        serde_yaml::Value::String( crate_version.clone() ),
      );
    }
  }

  commands
}

/// Load a single YAML file and transform its commands.
fn load_yaml_and_transform( path : &str ) -> Vec< serde_yaml::Value >
{
  let yaml_content = std::fs::read_to_string( path )
    .unwrap_or_else( | e | panic!( "Failed to read {path}: {e}" ) );

  let mut commands : Vec< serde_yaml::Value > = serde_yaml::from_str( &yaml_content )
    .unwrap_or_else( | e | panic!( "Failed to parse {path}: {e}" ) );

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

  commands
}

/// Transform a single argument from flat to nested attributes structure.
fn transform_argument_attributes( arg_map : &mut serde_yaml::Mapping )
{
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

  let mut attributes = serde_yaml::Mapping::new();
  attributes.insert(
    serde_yaml::Value::String( "optional".to_string() ),
    serde_yaml::Value::Bool( optional ),
  );
  attributes.insert(
    serde_yaml::Value::String( "sensitive".to_string() ),
    serde_yaml::Value::Bool( false ),
  );
  attributes.insert(
    serde_yaml::Value::String( "interactive".to_string() ),
    serde_yaml::Value::Bool( false ),
  );
  attributes.insert(
    serde_yaml::Value::String( "multiple".to_string() ),
    serde_yaml::Value::Bool( false ),
  );

  if let Some( default_str ) = default_val
  {
    attributes.insert(
      serde_yaml::Value::String( "default".to_string() ),
      serde_yaml::Value::String( default_str ),
    );
  }

  arg_map.insert(
    serde_yaml::Value::String( "attributes".to_string() ),
    serde_yaml::Value::Mapping( attributes ),
  );
}

/// Generate static command registry from transformed commands.
fn generate_static_commands( out_dir : &str, commands : &[ serde_yaml::Value ] )
{
  let output_path = PathBuf::from( out_dir ).join( "static_commands.rs" );

  let temp_yaml_path = PathBuf::from( out_dir ).join( "temp_commands.yaml" );
  let commands_yaml = serde_yaml::to_string( &commands )
    .expect( "Failed to serialize commands" );
  std::fs::write( &temp_yaml_path, commands_yaml )
    .expect( "Failed to write temporary commands YAML" );

  let config = unilang::multi_yaml::AggregationConfig
  {
    base_dir : PathBuf::from( out_dir ),
    modules : vec!
    [
      unilang::multi_yaml::ModuleConfig
      {
        name : "claude_runner".to_string(),
        yaml_path : "temp_commands.yaml".to_string(),
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

  let mut aggregator = unilang::multi_yaml::MultiYamlAggregator::new( config );

  match aggregator.aggregate()
  {
    Ok( () ) =>
    {
      if !aggregator.conflicts().is_empty()
      {
        eprintln!( "ERROR: Command conflicts detected:" );
        for conflict in aggregator.conflicts()
        {
          eprintln!( "  - Command '{}' in modules: {:?}", conflict.command_name, conflict.modules );
        }
        panic!( "Build failed due to command conflicts" );
      }

      let mut registry_source = aggregator.generate_static_registry_source();

      // Fix(issue-unilang-show-version): Inject missing show_version_in_help field.
      // Root cause: unilang v0.45+ requires this field but MultiYamlAggregator doesnt generate it.
      // Pitfall: Build scripts must inject new required fields until generator is updated.

      // Inject doc comment for AGGREGATED_COMMANDS to satisfy missing_docs lint.
      // Root cause: unilang's code generator does not emit doc comments for the static map.
      registry_source = registry_source.replace(
        "pub static AGGREGATED_COMMANDS:",
        "/// Compile-time generated command map for `claude_runner`.\npub static AGGREGATED_COMMANDS:",
      );

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

      if let Err( e ) = std::fs::write( &output_path, &registry_source )
      {
        panic!( "Failed to write static commands: {e}" );
      }
    }
    Err( e ) =>
    {
      panic!( "Failed to aggregate YAML commands: {e}" );
    }
  }
}
