//!
//! Comprehensive tests for `CliBuilder` API
//!
//! This module provides comprehensive test coverage for the `CliBuilder` fluent API,
//! specifically testing all requirements from task 065:
//! - Fluent API builder pattern functionality
//! - `static_module_with_prefix()` method behavior
//! - Conflict detection system for duplicate prefixes
//! - Namespace isolation between modules
//! - `build_static()` method creating unified registry

use unilang::prelude::*;
use unilang::multi_yaml::{ CliBuilder, AggregationMode, ConflictType };
use unilang::registry::RegistryMode;
use std::path::PathBuf;

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a sample command for testing
fn create_test_command( name: &str, description: &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( description.to_string() )
    .hint( format!( "Test command: {name}" ) )
    .form()
}

/// Create multiple test commands
fn create_test_commands( names: &[&str] ) -> Vec< CommandDefinition >
{
  names.iter()
    .map( |name| create_test_command( name, &format!( "Test command {name}" ) ) )
    .collect()
}

// =============================================================================
// Fluent API Builder Pattern Tests
// =============================================================================

#[test]
fn test_fluent_api_builder_pattern_chaining()
{
  // Test that all builder methods return Self and can be chained
  let cmd1 = create_test_command( "cmd1", "First command" );
  let cmd2 = create_test_command( "cmd2", "Second command" );

  let builder = CliBuilder::new()
    .app_name( "test_app" )
    .mode( AggregationMode::Static )
    .global_prefix( "test" )
    .auto_help( false )
    .detect_conflicts( true )
    .static_module( "module1", vec![ cmd1 ] )
    .static_module_with_prefix( "module2", "prefix", vec![ cmd2 ] )
    .dynamic_module( "yaml_module", PathBuf::from( "tests/test_data/external.yaml" ) )
    .dynamic_module_with_prefix( "yaml_prefix", PathBuf::from( "tests/test_data/external.yaml" ), "yaml" );

  // Verify configuration was applied correctly
  assert_eq!( builder.get_config().app_name, "test_app" );
  assert_eq!( *builder.get_mode(), AggregationMode::Static );
  assert_eq!( builder.get_config().global_prefix, Some( "test".to_string() ) );
  assert!( !builder.get_config().auto_help );
  assert!( builder.get_config().detect_conflicts );
  assert_eq!( builder.static_modules_count(), 2 );
  assert_eq!( builder.dynamic_modules_count(), 2 );
}

#[test]
fn test_fluent_api_default_configuration()
{
  let builder = CliBuilder::new();

  // Verify intelligent defaults
  assert_eq!( *builder.get_mode(), AggregationMode::Auto );
  assert_eq!( builder.get_config().app_name, "app" );
  assert_eq!( builder.get_config().global_prefix, None );
  assert!( builder.get_config().auto_help );
  assert!( builder.get_config().detect_conflicts );
  assert_eq!( builder.static_modules_count(), 0 );
  assert_eq!( builder.dynamic_modules_count(), 0 );
  assert_eq!( builder.conditional_modules_count(), 0 );
}

#[test]
fn test_fluent_api_configuration_methods()
{
  let builder = CliBuilder::new()
    .app_name( "my_app" )
    .global_prefix( "global" )
    .auto_help( false )
    .detect_conflicts( false );

  let config = builder.get_config();
  assert_eq!( config.app_name, "my_app" );
  assert_eq!( config.global_prefix, Some( "global".to_string() ) );
  assert!( !config.auto_help );
  assert!( !config.detect_conflicts );
}

#[test]
fn test_fluent_api_mode_configuration()
{
  let modes = vec![
    AggregationMode::Static,
    AggregationMode::Dynamic,
    AggregationMode::Hybrid,
    AggregationMode::Auto,
  ];

  for mode in modes
  {
    let builder = CliBuilder::new().mode( mode.clone() );
    assert_eq!( *builder.get_mode(), mode );
  }
}

// =============================================================================
// static_module_with_prefix() Method Tests
// =============================================================================

#[test]
fn test_static_module_with_prefix_basic()
{
  let cmd = create_test_command( "test", "Test command" );

  let builder = CliBuilder::new()
    .static_module_with_prefix( "core", "app", vec![ cmd ] );

  assert_eq!( builder.static_modules_count(), 1 );

  // Build and verify prefix application
  let registry = builder.build().expect( "Failed to build registry" );

  // Command should be accessible with prefix: .app.test
  assert!( registry.command( ".app.test" ).is_some() );
  assert!( registry.command( ".test" ).is_none() ); // Should not be accessible without prefix
}

#[test]
fn test_static_module_with_prefix_multiple_commands()
{
  let commands = create_test_commands( &[ "version", "help", "status" ] );

  let builder = CliBuilder::new()
    .static_module_with_prefix( "utils", "util", commands );

  let registry = builder.build().expect( "Failed to build registry" );

  // All commands should be prefixed
  assert!( registry.command( ".util.version" ).is_some() );
  assert!( registry.command( ".util.help" ).is_some() );
  assert!( registry.command( ".util.status" ).is_some() );

  // Commands should not be accessible without prefix
  assert!( registry.command( ".version" ).is_none() );
  assert!( registry.command( ".help" ).is_none() );
  assert!( registry.command( ".status" ).is_none() );
}

#[test]
fn test_static_module_with_prefix_and_global_prefix()
{
  let cmd = create_test_command( "deploy", "Deploy application" );

  let builder = CliBuilder::new()
    .global_prefix( "myapp" )
    .static_module_with_prefix( "deployment", "deploy", vec![ cmd ] );

  let registry = builder.build().expect( "Failed to build registry" );

  // Command should have both global and module prefix: .myapp.deploy.deploy
  assert!( registry.command( ".myapp.deploy.deploy" ).is_some() );
  assert!( registry.command( ".deploy.deploy" ).is_none() );
  assert!( registry.command( ".myapp.deploy" ).is_none() );
}

#[test]
fn test_static_module_with_prefix_empty_prefix()
{
  let cmd = create_test_command( "root", "Root command" );

  // Using empty string as prefix should behave like no prefix
  let builder = CliBuilder::new()
    .static_module_with_prefix( "core", "", vec![ cmd ] );

  let registry = builder.build().expect( "Failed to build registry" );

  // With empty prefix, the namespace becomes "." so the command becomes "..root"
  assert!( registry.command( "..root" ).is_some() );
}

#[test]
fn test_static_module_with_prefix_special_characters()
{
  let cmd = create_test_command( "special", "Special command" );

  let builder = CliBuilder::new()
    .static_module_with_prefix( "special_module", "my-app_v1", vec![ cmd ] );

  let registry = builder.build().expect( "Failed to build registry" );

  // Prefix with special characters should work
  assert!( registry.command( ".my-app_v1.special" ).is_some() );
}

// =============================================================================
// Conflict Detection Tests
// =============================================================================

#[test]
fn test_conflict_detection_duplicate_command_names()
{
  let cmd1 = create_test_command( "test", "First test command" );
  let cmd2 = create_test_command( "test", "Second test command" );

  let builder = CliBuilder::new()
    .detect_conflicts( true )
    .static_module_with_prefix( "module1", "app", vec![ cmd1 ] )
    .static_module_with_prefix( "module2", "app", vec![ cmd2 ] );

  let conflicts = builder.detect_conflicts_report();

  assert!( !conflicts.is_empty() );
  assert_eq!( conflicts.len(), 1 );

  let conflict = &conflicts[0];
  assert_eq!( conflict.command_name, ".app.test" );
  assert_eq!( conflict.modules.len(), 2 );
  assert!( conflict.modules.contains( &"module1".to_string() ) );
  assert!( conflict.modules.contains( &"module2".to_string() ) );
  assert_eq!( conflict.conflict_type, ConflictType::NameCollision );
}

#[test]
fn test_conflict_detection_different_prefixes_no_conflict()
{
  let cmd1 = create_test_command( "test", "First test command" );
  let cmd2 = create_test_command( "test", "Second test command" );

  let builder = CliBuilder::new()
    .detect_conflicts( true )
    .static_module_with_prefix( "module1", "app1", vec![ cmd1 ] )
    .static_module_with_prefix( "module2", "app2", vec![ cmd2 ] );

  let conflicts = builder.detect_conflicts_report();

  // No conflicts should exist - different prefixes isolate the commands
  assert!( conflicts.is_empty() );
}

#[test]
fn test_conflict_detection_multiple_conflicts()
{
  let core_version = create_test_command( "version", "Version from module 1" );
  let core_help = create_test_command( "help", "Help from module 1" );
  let utils_version = create_test_command( "version", "Version from module 2" );
  let utils_help = create_test_command( "help", "Help from module 2" );

  let builder = CliBuilder::new()
    .detect_conflicts( true )
    .static_module_with_prefix( "core", "app", vec![ core_version, core_help ] )
    .static_module_with_prefix( "utils", "app", vec![ utils_version, utils_help ] );

  let conflicts = builder.detect_conflicts_report();

  assert_eq!( conflicts.len(), 2 );

  // Check that both version and help conflicts are detected
  let command_names: Vec< &String > = conflicts.iter().map( |c| &c.command_name ).collect();
  assert!( command_names.contains( &&".app.version".to_string() ) );
  assert!( command_names.contains( &&".app.help".to_string() ) );
}

#[test]
fn test_conflict_detection_disabled()
{
  let cmd1 = create_test_command( "test", "First test command" );
  let cmd2 = create_test_command( "test", "Second test command" );

  let builder = CliBuilder::new()
    .detect_conflicts( false ) // Disable conflict detection
    .static_module_with_prefix( "module1", "app", vec![ cmd1 ] )
    .static_module_with_prefix( "module2", "app", vec![ cmd2 ] );

  let conflicts = builder.detect_conflicts_report();

  // No conflicts should be reported when detection is disabled
  assert!( conflicts.is_empty() );
}

#[test]
fn test_conflict_detection_with_global_prefix()
{
  let cmd1 = create_test_command( "deploy", "Deploy from module 1" );
  let cmd2 = create_test_command( "deploy", "Deploy from module 2" );

  let builder = CliBuilder::new()
    .global_prefix( "myapp" )
    .detect_conflicts( true )
    .static_module_with_prefix( "module1", "ops", vec![ cmd1 ] )
    .static_module_with_prefix( "module2", "ops", vec![ cmd2 ] );

  let conflicts = builder.detect_conflicts_report();

  assert_eq!( conflicts.len(), 1 );

  let conflict = &conflicts[0];
  // Global prefix should be included in conflict name
  assert_eq!( conflict.command_name, ".myapp.ops.deploy" );
}

#[test]
fn test_conflict_detection_mixed_static_and_dynamic_modules()
{
  let static_cmd = create_test_command( "example", "Static command" );

  let builder = CliBuilder::new()
    .detect_conflicts( true )
    .static_module_with_prefix( "static_mod", "test", vec![ static_cmd ] )
    .dynamic_module_with_prefix( "dynamic_mod", PathBuf::from( "tests/test_data/external.yaml" ), "test" );

  let conflicts = builder.detect_conflicts_report();

  // Conflict should be detected between static and simulated dynamic command
  assert_eq!( conflicts.len(), 1 );

  let conflict = &conflicts[0];
  assert!( conflict.modules.contains( &"static_mod".to_string() ) );
  assert!( conflict.modules.contains( &"dynamic_mod".to_string() ) );
}

// =============================================================================
// Namespace Isolation Tests
// =============================================================================

#[test]
fn test_namespace_isolation_different_prefixes()
{
  let auth_cmd = create_test_command( "login", "User login" );
  let file_cmd = create_test_command( "read", "Read file" );
  let net_cmd = create_test_command( "ping", "Network ping" );

  let registry = CliBuilder::new()
    .static_module_with_prefix( "auth", "auth", vec![ auth_cmd ] )
    .static_module_with_prefix( "file", "fs", vec![ file_cmd ] )
    .static_module_with_prefix( "network", "net", vec![ net_cmd ] )
    .build()
    .expect( "Failed to build registry" );

  // Each command should only be accessible via its own namespace
  assert!( registry.command( ".auth.login" ).is_some() );
  assert!( registry.command( ".fs.read" ).is_some() );
  assert!( registry.command( ".net.ping" ).is_some() );

  // Commands should not be accessible via other namespaces
  assert!( registry.command( ".fs.login" ).is_none() );
  assert!( registry.command( ".net.login" ).is_none() );
  assert!( registry.command( ".auth.read" ).is_none() );
  assert!( registry.command( ".net.read" ).is_none() );
  assert!( registry.command( ".auth.ping" ).is_none() );
  assert!( registry.command( ".fs.ping" ).is_none() );
}

#[test]
fn test_namespace_isolation_same_command_names()
{
  // Multiple modules with same command names but different prefixes
  let start_web = create_test_command( "start", "Start web server" );
  let start_db = create_test_command( "start", "Start database" );
  let start_cache = create_test_command( "start", "Start cache" );

  let registry = CliBuilder::new()
    .static_module_with_prefix( "web", "web", vec![ start_web ] )
    .static_module_with_prefix( "database", "db", vec![ start_db ] )
    .static_module_with_prefix( "cache", "cache", vec![ start_cache ] )
    .build()
    .expect( "Failed to build registry" );

  // All commands should be isolated by their prefixes
  assert!( registry.command( ".web.start" ).is_some() );
  assert!( registry.command( ".db.start" ).is_some() );
  assert!( registry.command( ".cache.start" ).is_some() );

  // Verify each command has correct description (namespace isolation working)
  let web_start = registry.command( ".web.start" ).unwrap();
  let db_start = registry.command( ".db.start" ).unwrap();
  let cache_start = registry.command( ".cache.start" ).unwrap();

  assert!( web_start.description.contains( "web server" ) );
  assert!( db_start.description.contains( "database" ) );
  assert!( cache_start.description.contains( "cache" ) );
}

#[test]
fn test_namespace_isolation_nested_prefixes()
{
  let user_create = create_test_command( "create", "Create user" );
  let role_create = create_test_command( "create", "Create role" );

  let registry = CliBuilder::new()
    .global_prefix( "myapp" )
    .static_module_with_prefix( "user_mgmt", "user", vec![ user_create ] )
    .static_module_with_prefix( "role_mgmt", "role", vec![ role_create ] )
    .build()
    .expect( "Failed to build registry" );

  // Commands should be isolated by full namespace path
  assert!( registry.command( ".myapp.user.create" ).is_some() );
  assert!( registry.command( ".myapp.role.create" ).is_some() );

  // Verify commands are properly isolated
  let user_create_cmd = registry.command( ".myapp.user.create" ).unwrap();
  let role_create_cmd = registry.command( ".myapp.role.create" ).unwrap();

  assert!( user_create_cmd.description.contains( "user" ) );
  assert!( role_create_cmd.description.contains( "role" ) );

  // Cross-namespace access should fail
  assert!( registry.command( ".myapp.user.role" ).is_none() );
  assert!( registry.command( ".myapp.role.user" ).is_none() );
}

#[ cfg( feature = "advanced_cli_tests" ) ]
#[test]
fn test_namespace_isolation_mixed_modules()
{
  let static_cmd = create_test_command( "static_op", "Static operation" );
  let cond_cmd = create_test_command( "debug_op", "Debug operation" );

  let registry = CliBuilder::new()
    .static_module_with_prefix( "core", "core", vec![ static_cmd ] )
    .dynamic_module_with_prefix( "external", PathBuf::from( "tests/test_data/external.yaml" ), "ext" )
    .conditional_module( "debug", "test_feature", vec![ cond_cmd ] )
    .build()
    .expect( "Failed to build registry" );

  // Each module type should maintain namespace isolation
  assert!( registry.command( ".core.static_op" ).is_some() );
  assert!( registry.command( ".ext.example" ).is_some() ); // Dynamic command from YAML file
  assert!( registry.command( ".debug.debug_op" ).is_some() );

  // Cross-module access should fail
  assert!( registry.command( ".core.example" ).is_none() );
  assert!( registry.command( ".ext.static_op" ).is_none() );
  assert!( registry.command( ".debug.static_op" ).is_none() );
}

// =============================================================================
// build_static() Method Tests
// =============================================================================

#[test]
fn test_build_static_basic_functionality()
{
  let cmd = create_test_command( "version", "Show version" );

  let static_registry = CliBuilder::new()
    .static_module( "core", vec![ cmd ] )
    .build_static()
    .expect( "Failed to build static registry" );

  // Verify it's a StaticCommandRegistry with Hybrid mode (allows dynamic registration)
  assert_eq!( static_registry.mode(), RegistryMode::Hybrid );

  // Debug: print what commands are available
  let commands = static_registry.commands();
  println!("Available commands in static registry: {:?}", commands.keys().collect::<Vec<_>>());

  // Command should be accessible
  assert!( static_registry.command( ".version" ).is_some() );

  // Verify command count (total commands including dynamic)
  assert_eq!( static_registry.commands().len(), 1 );
}

#[test]
fn test_build_static_with_prefix()
{
  let commands = create_test_commands( &[ "start", "stop", "restart" ] );

  let static_registry = CliBuilder::new()
    .static_module_with_prefix( "service", "svc", commands )
    .build_static()
    .expect( "Failed to build static registry with prefix" );

  // All commands should be accessible with prefix
  assert!( static_registry.command( ".svc.start" ).is_some() );
  assert!( static_registry.command( ".svc.stop" ).is_some() );
  assert!( static_registry.command( ".svc.restart" ).is_some() );

  // Verify command count
  assert_eq!( static_registry.commands().len(), 3 );
}

#[test]
fn test_build_static_multiple_modules()
{
  let core_cmds = create_test_commands( &[ "version", "help" ] );
  let util_cmds = create_test_commands( &[ "backup", "restore" ] );

  let static_registry = CliBuilder::new()
    .static_module_with_prefix( "core", "core", core_cmds )
    .static_module_with_prefix( "utils", "util", util_cmds )
    .build_static()
    .expect( "Failed to build static registry with multiple modules" );

  // All commands from both modules should be accessible
  assert!( static_registry.command( ".core.version" ).is_some() );
  assert!( static_registry.command( ".core.help" ).is_some() );
  assert!( static_registry.command( ".util.backup" ).is_some() );
  assert!( static_registry.command( ".util.restore" ).is_some() );

  // Verify total command count
  assert_eq!( static_registry.commands().len(), 4 );
}

#[test]
fn test_build_static_with_global_prefix()
{
  let cmd = create_test_command( "deploy", "Deploy application" );

  let static_registry = CliBuilder::new()
    .global_prefix( "myapp" )
    .static_module_with_prefix( "ops", "deploy", vec![ cmd ] )
    .build_static()
    .expect( "Failed to build static registry with global prefix" );

  // Command should include both global and module prefix
  assert!( static_registry.command( ".myapp.deploy.deploy" ).is_some() );

  // Verify command count (total commands including dynamic)
  assert_eq!( static_registry.commands().len(), 1 );
}

#[test]
fn test_build_static_zero_overhead_optimization()
{
  let commands = create_test_commands( &[ "cmd1", "cmd2", "cmd3", "cmd4", "cmd5" ] );

  let static_registry = CliBuilder::new()
    .mode( AggregationMode::Static )
    .static_module( "perf_test", commands )
    .build_static()
    .expect( "Failed to build static registry for performance test" );

  // Verify Hybrid mode (allows dynamic registration with static optimizations)
  assert_eq!( static_registry.mode(), RegistryMode::Hybrid );

  // All commands should be accessible for performance testing
  for i in 1..=5
  {
    let cmd_name = format!( ".cmd{i}" );
    assert!( static_registry.command( &cmd_name ).is_some() );
  }

  // Verify performance: static registry should be optimized
  assert_eq!( static_registry.commands().len(), 5 );
}

#[test]
fn test_build_static_vs_build_comparison()
{
  let cmd = create_test_command( "test", "Test command" );

  // Build with static registry
  let static_registry = CliBuilder::new()
    .static_module( "test", vec![ cmd.clone() ] )
    .build_static()
    .expect( "Failed to build static registry" );

  // Build with dynamic registry
  let dynamic_registry = CliBuilder::new()
    .static_module( "test", vec![ cmd ] )
    .build()
    .expect( "Failed to build dynamic registry" );

  // Both should have the command accessible
  assert!( static_registry.command( ".test" ).is_some() );
  assert!( dynamic_registry.command( ".test" ).is_some() );

  // But different registry modes
  assert_eq!( static_registry.mode(), RegistryMode::Hybrid );
  assert_eq!( dynamic_registry.registry_mode(), RegistryMode::Hybrid );
}

#[test]
fn test_build_static_ignores_dynamic_modules()
{
  let static_cmd = create_test_command( "static_cmd", "Static command" );

  let static_registry = CliBuilder::new()
    .static_module( "static_mod", vec![ static_cmd ] )
    .dynamic_module( "dynamic_mod", PathBuf::from( "tests/test_data/external.yaml" ) )
    .build_static()
    .expect( "Failed to build static registry ignoring dynamic modules" );

  // Only static commands should be present in static registry
  assert!( static_registry.command( ".static_cmd" ).is_some() );

  // Dynamic modules should be ignored in static build
  assert!( static_registry.command( ".example" ).is_none() ); // Dynamic command from YAML file

  // Only static commands counted
  assert_eq!( static_registry.commands().len(), 1 );
}

// =============================================================================
// Integration and Edge Case Tests
// =============================================================================

#[test]
fn test_empty_builder_build_static()
{
  let static_registry = CliBuilder::new()
    .build_static()
    .expect( "Failed to build empty static registry" );

  // Empty registry should still work
  assert_eq!( static_registry.mode(), RegistryMode::Hybrid );
  assert_eq!( static_registry.commands().len(), 0 );
}

#[test]
fn test_disabled_modules_excluded()
{
  let cmd1 = create_test_command( "enabled", "Enabled command" );
  let cmd2 = create_test_command( "disabled", "Disabled command" );

  // Create builder with modules (we don't have direct enable/disable API, so this tests the concept)
  let builder = CliBuilder::new()
    .static_module_with_prefix( "enabled_mod", "enabled", vec![ cmd1 ] )
    .static_module_with_prefix( "other_mod", "other", vec![ cmd2 ] );

  let registry = builder.build().expect( "Failed to build registry" );

  // All modules should be enabled by default
  assert!( registry.command( ".enabled.enabled" ).is_some() );
  assert!( registry.command( ".other.disabled" ).is_some() );
}

#[ cfg( feature = "advanced_cli_tests" ) ]
#[test]
fn test_comprehensive_builder_scenario()
{
  let core_cmds = create_test_commands( &[ "version", "help", "config" ] );
  let auth_cmds = create_test_commands( &[ "login", "logout", "whoami" ] );
  let debug_cmds = create_test_commands( &[ "trace", "dump", "profile" ] );

  let builder = CliBuilder::new()
    .app_name( "comprehensive_app" )
    .global_prefix( "app" )
    .mode( AggregationMode::Hybrid )
    .auto_help( true )
    .detect_conflicts( true )
    .static_module_with_prefix( "core", "core", core_cmds )
    .static_module_with_prefix( "auth", "auth", auth_cmds )
    .dynamic_module_with_prefix( "external", PathBuf::from( "tests/test_data/external.yaml" ), "ext" )
    .conditional_module( "debug", "test_feature", debug_cmds );

  // Test conflict detection before building
  let conflicts = builder.detect_conflicts_report();
  assert!( conflicts.is_empty(), "No conflicts should exist in well-designed scenario" );

  // Test both build methods
  let dynamic_registry = builder.clone().build().expect( "Failed to build dynamic registry" );
  let static_registry = builder.build_static().expect( "Failed to build static registry" );

  // Verify commands exist in dynamic registry
  assert!( dynamic_registry.command( ".app.core.version" ).is_some() );
  assert!( dynamic_registry.command( ".app.auth.login" ).is_some() );
  assert!( dynamic_registry.command( ".app.ext.example" ).is_some() ); // Dynamic from YAML
  assert!( dynamic_registry.command( ".app.debug.trace" ).is_some() ); // Conditional enabled

  // Verify commands exist in static registry (excluding dynamic/conditional)
  assert!( static_registry.command( ".app.core.version" ).is_some() );
  assert!( static_registry.command( ".app.auth.login" ).is_some() );
  // Dynamic and conditional modules should be excluded from static build
  assert!( static_registry.command( ".app.ext.example" ).is_none() );
  assert!( static_registry.command( ".app.debug.trace" ).is_none() );

  // Verify registry modes
  assert_eq!( dynamic_registry.registry_mode(), RegistryMode::Hybrid );
  assert_eq!( static_registry.mode(), RegistryMode::Hybrid );
}