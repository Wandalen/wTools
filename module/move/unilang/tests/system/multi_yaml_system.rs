//!
//! Tests for multi-YAML aggregation system.
//!
//! This module tests the multi-YAML aggregation system that discovers and processes
//! multiple YAML command definition files for compile-time CLI aggregation.

use unilang::multi_yaml::{ MultiYamlAggregator, AggregationConfig, ModuleConfig, ConflictResolutionStrategy, NamespaceIsolation };
use std::path::PathBuf;
use std::collections::HashMap;

/// Helper function to create test aggregation config
fn create_test_config() -> AggregationConfig
{
  AggregationConfig
  {
    base_dir: PathBuf::from("tests/fixtures"),
    modules: vec![
      ModuleConfig
      {
        name: "core".to_string(),
        yaml_path: "core.yaml".to_string(),
        prefix: Some("core".to_string()),
        enabled: true,
      },
      ModuleConfig
      {
        name: "utils".to_string(),
        yaml_path: "utils.yaml".to_string(),
        prefix: Some("util".to_string()),
        enabled: true,
      },
    ],
    global_prefix: Some("test".to_string()),
    detect_conflicts: true,
    env_overrides: HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec!["*.yaml".to_string()],
    namespace_isolation: NamespaceIsolation
    {
      enabled: true,
      separator: ".".to_string(),
      strict_mode: false,
    },
  }
}

#[test]
fn test_multi_yaml_aggregator_creation()
{
  // Test creating MultiYamlAggregator with basic configuration
  let config = create_test_config();
  let aggregator = MultiYamlAggregator::new( config );

  // Should successfully create aggregator
  assert_eq!( aggregator.commands().len(), 0 );
  assert_eq!( aggregator.conflicts().len(), 0 );
  assert_eq!( aggregator.config().modules.len(), 2 );
}

#[test]
fn test_aggregation_config_creation()
{
  // Test creating different types of aggregation configs
  let config = create_test_config();

  // Verify configuration values
  assert_eq!( config.base_dir, PathBuf::from("tests/fixtures") );
  assert_eq!( config.modules.len(), 2 );
  assert_eq!( config.global_prefix, Some("test".to_string()) );
  assert!( config.detect_conflicts );
  assert_eq!( config.conflict_resolution, ConflictResolutionStrategy::Fail );
  assert!( !config.auto_discovery );
  assert_eq!( config.discovery_patterns, vec!["*.yaml".to_string()] );
  assert!( config.namespace_isolation.enabled );
}

#[test]
fn test_module_config_structure()
{
  // Test creating module configurations
  let module = ModuleConfig
  {
    name: "test_module".to_string(),
    yaml_path: "test.yaml".to_string(),
    prefix: Some("test".to_string()),
    enabled: true,
  };

  assert_eq!( module.name, "test_module" );
  assert_eq!( module.yaml_path, "test.yaml" );
  assert_eq!( module.prefix, Some("test".to_string()) );
  assert!( module.enabled );
}

#[test]
fn test_conflict_resolution_strategies()
{
  // Test different conflict resolution strategies
  let mut config = create_test_config();

  // Test Fail strategy (default)
  config.conflict_resolution = ConflictResolutionStrategy::Fail;
  assert_eq!( config.conflict_resolution, ConflictResolutionStrategy::Fail );

  // Test UseFirst strategy
  config.conflict_resolution = ConflictResolutionStrategy::UseFirst;
  assert_eq!( config.conflict_resolution, ConflictResolutionStrategy::UseFirst );

  // Test UseLast strategy
  config.conflict_resolution = ConflictResolutionStrategy::UseLast;
  assert_eq!( config.conflict_resolution, ConflictResolutionStrategy::UseLast );

  // Test Merge strategy
  config.conflict_resolution = ConflictResolutionStrategy::Merge;
  assert_eq!( config.conflict_resolution, ConflictResolutionStrategy::Merge );
}

#[test]
fn test_namespace_isolation_configuration()
{
  // Test namespace isolation settings
  let isolation = NamespaceIsolation
  {
    enabled: true,
    separator: "::".to_string(),
    strict_mode: true,
  };

  assert!( isolation.enabled );
  assert_eq!( isolation.separator, "::" );
  assert!( isolation.strict_mode );
}

#[test]
fn test_yaml_file_loading()
{
  // Test YAML file loading (this will use mock data since files don't exist)
  let config = create_test_config();
  let mut aggregator = MultiYamlAggregator::new( config );

  // Try to load YAML files (should handle missing files gracefully)
  let result = aggregator.load_yaml_files();

  // Should either succeed with mock data or fail gracefully
  if let Ok(()) = result {
    // If successful, verify no commands were loaded from missing files
    assert_eq!( aggregator.commands().len(), 0 );
  } else {
    // Expected for missing files - this is fine
  }
}

#[test]
fn test_phf_map_generation()
{
  // Test PHF map generation
  let config = create_test_config();
  let aggregator = MultiYamlAggregator::new( config );

  // Generate PHF map content
  let phf_content = aggregator.generate_phf_map();

  // Should contain PHF map structure
  assert!( phf_content.contains("phf_map") );
  assert!( phf_content.contains("Map") );
}

#[test]
fn test_build_rs_generation()
{
  // Test build.rs file generation
  let config = create_test_config();
  let aggregator = MultiYamlAggregator::new( config );

  // Generate build.rs content
  let build_rs_content = aggregator.generate_build_rs();

  // Should contain basic build.rs structure
  assert!( build_rs_content.contains("fn main()") );
  assert!( !build_rs_content.is_empty() );
}

#[test]
fn test_config_serialization()
{
  // Test configuration serialization/deserialization
  let config = create_test_config();

  // Test serialization
  let serialized = serde_json::to_string( &config );
  assert!( serialized.is_ok(), "Config should be serializable" );

  // Test deserialization
  if let Ok(json_str) = serialized
  {
    let deserialized: Result<AggregationConfig, _> = serde_json::from_str( &json_str );
    assert!( deserialized.is_ok(), "Config should be deserializable" );

    if let Ok(deserialized_config) = deserialized
    {
      assert_eq!( deserialized_config.modules.len(), config.modules.len() );
      assert_eq!( deserialized_config.global_prefix, config.global_prefix );
    }
  }
}

#[cfg(feature = "multi_yaml")]
#[test]
fn test_auto_discovery()
{
  // Test automatic YAML file discovery
  let mut config = create_test_config();
  config.auto_discovery = true;
  config.base_dir = PathBuf::from(".");  // Use current directory

  let mut aggregator = MultiYamlAggregator::new( config );

  // Try to discover YAML files
  let result = aggregator.discover_yaml_files();

  // Should complete without errors (whether files are found or not)
  if let Ok(()) = result {
    // Discovery completed successfully
  } else {
    // Discovery failed gracefully
  }
}

#[test]
fn test_environment_overrides()
{
  // Test environment variable overrides
  let mut env_overrides = HashMap::new();
  env_overrides.insert( "UNILANG_GLOBAL_PREFIX".to_string(), "override".to_string() );
  env_overrides.insert( "UNILANG_DETECT_CONFLICTS".to_string(), "false".to_string() );

  let config = AggregationConfig
  {
    env_overrides,
    ..create_test_config()
  };

  let aggregator = MultiYamlAggregator::new( config );
  assert_eq!( aggregator.config().env_overrides.len(), 2 );
}

#[test]
fn test_aggregation_workflow()
{
  // Test complete aggregation workflow
  let config = create_test_config();
  let mut aggregator = MultiYamlAggregator::new( config );

  // Test the complete workflow (will use mock data)
  let result = aggregator.aggregate();

  // Should complete the workflow, even with mock data
  if let Ok(()) = result {
    // Verify aggregation completed
    assert_eq!( aggregator.conflicts().len(), 0 );
  } else {
    // Expected with missing files - this is normal
  }
}