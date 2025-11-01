//! Tests for ergonomic aggregation APIs
//!
//! This module tests the new ergonomic aggregation APIs that provide simple interfaces
//! for common use cases while preserving complex APIs for advanced scenarios. This includes:
//!
//! - `aggregate_cli`! macro for zero-boilerplate static aggregation
//! - `CliBuilder` for complex scenarios with static, dynamic, and conditional modules
//! - Mode selection APIs with intelligent defaults
//! - Conditional module loading with feature flags
//! - Error handling and validation
//! - Integration with hybrid registry and multi-YAML build system
//! - Backward compatibility with existing `CliAggregator`

#![ allow( deprecated ) ]

use unilang::prelude::*;
use unilang::multi_yaml::{ CliBuilder, AggregationMode, aggregate_cli_simple };
#[ cfg( feature = "advanced_cli_tests" ) ]
use unilang::multi_yaml::aggregate_cli_complex;
use std::path::PathBuf;

#[test]
fn test_cli_builder_creation()
{
  let builder = CliBuilder::new();
  assert_eq!( *builder.get_mode(), AggregationMode::Auto );
  assert_eq!( builder.get_config().app_name, "app" );
  assert!( builder.get_config().auto_help );
  assert!( builder.get_config().detect_conflicts );
}

#[test]
fn test_cli_builder_mode_selection()
{
  let builder = CliBuilder::new()
    .mode( AggregationMode::Static );
  assert_eq!( *builder.get_mode(), AggregationMode::Static );

  let builder = CliBuilder::new()
    .mode( AggregationMode::Dynamic );
  assert_eq!( *builder.get_mode(), AggregationMode::Dynamic );

  let builder = CliBuilder::new()
    .mode( AggregationMode::Hybrid );
  assert_eq!( *builder.get_mode(), AggregationMode::Hybrid );
}

#[test]
fn test_cli_builder_static_modules()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command".to_string() )
    .end();

  let builder = CliBuilder::new()
    .static_module( "test_module", vec![ cmd ] );

  assert_eq!( builder.static_modules_count(), 1 );
}

#[test]
fn test_cli_builder_static_modules_with_prefix()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command".to_string() )
    .end();

  let builder = CliBuilder::new()
    .static_module_with_prefix( "test_module", "test", vec![ cmd ] );

  assert_eq!( builder.static_modules_count(), 1 );
}

#[test]
fn test_cli_builder_dynamic_modules()
{
  let builder = CliBuilder::new()
    .dynamic_module( "yaml_module", PathBuf::from( "test.yaml" ) );

  assert_eq!( builder.dynamic_modules_count(), 1 );
}

#[test]
fn test_cli_builder_dynamic_modules_with_prefix()
{
  let builder = CliBuilder::new()
    .dynamic_module_with_prefix( "yaml_module", PathBuf::from( "test.yaml" ), "yaml" );

  assert_eq!( builder.dynamic_modules_count(), 1 );
}

#[test]
fn test_cli_builder_conditional_modules()
{
  let cmd = CommandDefinition::former()
    .name( ".conditional" )
    .description( "Conditional command".to_string() )
    .end();

  let builder = CliBuilder::new()
    .conditional_module( "cond_module", "test_feature", vec![ cmd ] );

  assert_eq!( builder.conditional_modules_count(), 1 );
}

#[test]
fn test_cli_builder_configuration()
{
  let builder = CliBuilder::new()
    .app_name( "myapp" )
    .global_prefix( "myapp" )
    .auto_help( false )
    .detect_conflicts( false );

  assert_eq!( builder.get_config().app_name, "myapp" );
  assert_eq!( builder.get_config().global_prefix, Some( "myapp".to_string() ) );
  assert!( !builder.get_config().auto_help );
  assert!( !builder.get_config().detect_conflicts );
}

#[test]
fn test_cli_builder_build_static_only()
{
  let cmd = CommandDefinition::former()
    .name( ".version" )
    .description( "Show version".to_string() )
    .end();

  let registry = CliBuilder::new()
    .mode( AggregationMode::Static )
    .static_module( "core", vec![ cmd ] )
    .build()
    .expect( "Failed to build CLI" );

  assert_eq!( registry.registry_mode(), RegistryMode::Hybrid );

  // Debug: print what commands are available
  let commands = registry.commands();
  println!("Available commands in registry: {:?}", commands.keys().collect::<Vec<_>>());

  assert!( registry.command( ".version" ).is_some() );
}

#[test]
fn test_cli_builder_build_with_prefix()
{
  let cmd = CommandDefinition::former()
    .name( ".version" )
    .description( "Show version".to_string() )
    .end();

  let registry = CliBuilder::new()
    .mode( AggregationMode::Static )
    .global_prefix( "myapp" )
    .static_module_with_prefix( "core", "core", vec![ cmd ] )
    .build()
    .expect( "Failed to build CLI with prefix" );

  // Debug what commands are available
  let commands = registry.commands();
  println!("Available commands with prefix: {:?}", commands.keys().collect::<Vec<_>>());

  // Command should be registered with full prefix: .myapp.core.version
  let cmd_with_prefix = registry.command( ".myapp.core.version" );
  println!("Command lookup result for '.myapp.core.version': {:?}", cmd_with_prefix.is_some());

  // Try looking up commands that we know exist
  let version_cmd = registry.command( ".version" );
  println!("Command lookup result for '.version': {:?}", version_cmd.is_some());

  assert!( cmd_with_prefix.is_some(), "Command with prefix should exist" );
}

#[test]
fn test_cli_builder_auto_mode_detection()
{
  // Only static modules should result in StaticOnly mode
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .end();
  let builder = CliBuilder::new()
    .mode( AggregationMode::Auto )
    .static_module( "test", vec![ cmd ] );

  let detected_mode = builder.detect_optimal_mode();
  assert_eq!( detected_mode, RegistryMode::Hybrid ); // Static modules use dynamic registration

  // Only dynamic modules should result in DynamicOnly mode
  let builder = CliBuilder::new()
    .mode( AggregationMode::Auto )
    .dynamic_module( "test", PathBuf::from( "test.yaml" ) );

  let detected_mode = builder.detect_optimal_mode();
  assert_eq!( detected_mode, RegistryMode::DynamicOnly );

  // Mixed modules should result in Hybrid mode
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .end();
  let builder = CliBuilder::new()
    .mode( AggregationMode::Auto )
    .static_module( "static", vec![ cmd ] )
    .dynamic_module( "dynamic", PathBuf::from( "test.yaml" ) );

  let detected_mode = builder.detect_optimal_mode();
  assert_eq!( detected_mode, RegistryMode::Hybrid );
}

#[ cfg( feature = "advanced_cli_tests" ) ]
#[test]
fn test_cli_builder_conditional_modules_enabled()
{
  let cmd = CommandDefinition::former()
    .name( ".debug" )
    .description( "Debug command".to_string() )
    .end();

  let registry = CliBuilder::new()
    .conditional_module( "debug_module", "test_feature", vec![ cmd ] )
    .build()
    .expect( "Failed to build CLI with conditional modules" );

  // Debug what commands are available
  let commands = registry.commands();
  println!("Conditional module commands: {:?}", commands.keys().collect::<Vec<_>>());

  // test_feature is enabled in our simulation, so debug command should exist
  let debug_cmd = registry.command( ".debug_module.debug" );
  println!("Debug command lookup for '.debug_module.debug': {:?}", debug_cmd.is_some());
  assert!( debug_cmd.is_some() );
}

#[test]
fn test_cli_builder_conditional_modules_disabled()
{
  let cmd = CommandDefinition::former()
    .name( ".disabled" )
    .description( "Disabled command".to_string() )
    .end();

  let registry = CliBuilder::new()
    .conditional_module( "disabled_module", "disabled_feature", vec![ cmd ] )
    .build()
    .expect( "Failed to build CLI" );

  // disabled_feature is not enabled, so command should not exist
  assert!( registry.command( ".disabled_module.disabled" ).is_none() );
}

#[test]
fn test_aggregate_cli_simple_macro()
{
  let registry = aggregate_cli_simple()
    .expect( "Failed to create simple aggregated CLI" );

  assert!( registry.command( ".version" ).is_some() );
  assert_eq!( registry.registry_mode(), RegistryMode::Hybrid );
}

#[ cfg( feature = "advanced_cli_tests" ) ]
#[test]
fn test_aggregate_cli_complex_macro()
{
  let registry = aggregate_cli_complex()
    .expect( "Failed to create complex aggregated CLI" );

  assert_eq!( registry.registry_mode(), RegistryMode::Hybrid );

  // Debug what commands are available
  let commands = registry.commands();
  println!("Complex registry commands: {:?}", commands.keys().collect::<Vec<_>>());

  // Should have prefixed commands
  assert!( registry.command( ".myapp.core.version" ).is_some() );

  // Should have conditional command (test_feature is enabled)
  assert!( registry.command( ".myapp.advanced.debug" ).is_some() );

  // Should have dynamic module command (from multi-YAML aggregation)
  assert!( registry.command( ".myapp.util.example" ).is_some() );
}

#[test]
fn test_backward_compatibility_with_existing_apis()
{
  // Test that we can still use the existing CommandRegistry API
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".legacy" )
    .description( "Legacy command".to_string() )
    .end();

  registry.register( cmd ).expect( "Registration should succeed" );
  assert!( registry.command( ".legacy" ).is_some() );

  // Test that new CliBuilder can coexist with existing registries
  let new_cmd = CommandDefinition::former()
    .name( ".new" )
    .description( "New command".to_string() )
    .end();

  let new_registry = CliBuilder::new()
    .static_module( "new", vec![ new_cmd ] )
    .build()
    .expect( "Failed to build new CLI" );

  // Debug: print what commands are available in new_registry
  let new_commands = new_registry.commands();
  println!("New registry commands: {:?}", new_commands.keys().collect::<Vec<_>>());

  assert!( new_registry.command( ".new" ).is_some() );

  // Both should work independently
  assert!( registry.command( ".legacy" ).is_some() );
  assert!( new_registry.command( ".new" ).is_some() );
}

#[test]
fn test_integration_with_hybrid_registry()
{
  let cmd = CommandDefinition::former()
    .name( ".hybrid_test" )
    .description( "Test hybrid registry integration".to_string() )
    .end();

  let mut registry = CliBuilder::new()
    .mode( AggregationMode::Hybrid )
    .static_module( "hybrid", vec![ cmd ] )
    .build()
    .expect( "Failed to build hybrid CLI" );

  // Test hybrid registry features
  assert_eq!( registry.registry_mode(), RegistryMode::Hybrid );

  // Test that we can use optimized lookup
  let cmd = registry.command_optimized( ".hybrid_test" );
  assert!( cmd.is_some() );

  // Test performance metrics are available
  let metrics = registry.performance_metrics();
  assert!( metrics.total_lookups > 0 );
}

#[test]
fn test_error_handling_invalid_configuration()
{
  // Test that building with no modules still works (empty registry)
  let registry = CliBuilder::new()
    .build()
    .expect( "Failed to build empty CLI" );

  // Should have auto-detected mode (no modules = StaticOnly)
  assert_eq!( registry.registry_mode(), RegistryMode::StaticOnly );
}

#[test]
fn test_mode_selection_apis()
{
  // Test all aggregation modes
  let modes = vec![
    AggregationMode::Static,
    AggregationMode::Dynamic,
    AggregationMode::Hybrid,
    AggregationMode::Auto,
  ];

  for mode in modes
  {
    let registry = CliBuilder::new()
      .mode( mode.clone() )
      .build()
      .unwrap_or_else(|_| panic!("Failed to build CLI with mode {mode:?}"));

    // Each should build successfully
    match mode
    {
      AggregationMode::Dynamic => assert_eq!( registry.registry_mode(), RegistryMode::DynamicOnly ),
      AggregationMode::Static | AggregationMode::Hybrid => assert_eq!( registry.registry_mode(), RegistryMode::Hybrid ),
      AggregationMode::Auto => {
        // Auto mode with no modules should default to StaticOnly
        assert_eq!( registry.registry_mode(), RegistryMode::StaticOnly );
      },
    }
  }
}

#[ cfg( feature = "advanced_cli_tests" ) ]
#[test]
fn test_complex_scenario_with_all_features()
{
  let static_cmd = CommandDefinition::former()
    .name( ".static_cmd" )
    .description( "Static command".to_string() )
    .end();

  let cond_cmd = CommandDefinition::former()
    .name( ".cond_cmd" )
    .description( "Conditional command".to_string() )
    .end();

  let registry = CliBuilder::new()
    .app_name( "complex_app" )
    .global_prefix( "app" )
    .mode( AggregationMode::Hybrid )
    .static_module_with_prefix( "static", "st", vec![ static_cmd ] )
    .dynamic_module_with_prefix( "dynamic", PathBuf::from( "tests/test_data/dynamic.yaml" ), "dyn" )
    .conditional_module( "conditional", "test_feature", vec![ cond_cmd ] )
    .auto_help( true )
    .detect_conflicts( true )
    .build()
    .expect( "Failed to build complex CLI" );

  // Debug all commands
  let commands = registry.commands();
  println!("Complex scenario commands: {:?}", commands.keys().collect::<Vec<_>>());

  // Verify all components are working together
  assert_eq!( registry.registry_mode(), RegistryMode::Hybrid );

  // Static command with full prefix: .app.st.static_cmd
  assert!( registry.command( ".app.st.static_cmd" ).is_some() );

  // Conditional command: .app.conditional.cond_cmd
  assert!( registry.command( ".app.conditional.cond_cmd" ).is_some() );

  // Dynamic command: .app.dyn.example
  assert!( registry.command( ".app.dyn.example" ).is_some() );
}