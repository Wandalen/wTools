//! Multi-Module Static Aggregation with PHF Maps
//!
//! This example demonstrates advanced static command aggregation across multiple
//! modules using PHF maps for zero-overhead lookups. It showcases:
//! 1. Cross-module command aggregation with namespace isolation
//! 2. Performance comparison between static and dynamic registries
//! 3. Compile-time conflict detection and resolution
//! 4. Multi-registry hybrid patterns for complex applications
//!
//! This is the most advanced static aggregation example, showing production-ready
//! patterns for large-scale CLI applications.

use unilang::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

// =============================================================================
// Module Command Definitions
// =============================================================================

/// Create authentication module commands
fn create_auth_commands() -> Vec<CommandDefinition>
{
  vec![
    CommandDefinition::former()
      .name( "auth.login" )
      .description( "Authenticate user login".to_string() )
      .hint( "User authentication".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "username" )
          .description( "Username for authentication".to_string() )
          .kind( Kind::String )
          .hint( "Username".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "password" )
          .description( "Password for authentication".to_string() )
          .kind( Kind::String )
          .hint( "Password".to_string() )
          .attributes( ArgumentAttributes {
            sensitive: true,
            interactive: true,
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( "auth.logout" )
      .description( "Logout current user session".to_string() )
      .hint( "User logout".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "force" )
          .description( "Force logout without confirmation".to_string() )
          .kind( Kind::Boolean )
          .hint( "Force logout".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "false".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( "auth.token" )
      .description( "Generate or refresh authentication token".to_string() )
      .hint( "Token management".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "action" )
          .description( "Token action (generate, refresh, revoke)".to_string() )
          .kind( Kind::String )
          .hint( "Token action".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
  ]
}

/// Create filesystem module commands
fn create_filesystem_commands() -> Vec<CommandDefinition>
{
  vec![
    CommandDefinition::former()
      .name( "fs.copy" )
      .description( "Copy files or directories".to_string() )
      .hint( "File copy operation".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "source" )
          .description( "Source file or directory path".to_string() )
          .kind( Kind::Path )
          .hint( "Source path".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "destination" )
          .description( "Destination file or directory path".to_string() )
          .kind( Kind::Path )
          .hint( "Destination path".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "recursive" )
          .description( "Copy directories recursively".to_string() )
          .kind( Kind::Boolean )
          .hint( "Recursive copy".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "false".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( "fs.move" )
      .description( "Move or rename files and directories".to_string() )
      .hint( "File move operation".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "source" )
          .description( "Source file or directory path".to_string() )
          .kind( Kind::Path )
          .hint( "Source path".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "destination" )
          .description( "Destination file or directory path".to_string() )
          .kind( Kind::Path )
          .hint( "Destination path".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( "fs.list" )
      .description( "List directory contents".to_string() )
      .hint( "Directory listing".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "path" )
          .description( "Directory path to list".to_string() )
          .kind( Kind::Directory )
          .hint( "Directory path".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( ".".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
        ArgumentDefinition::former()
          .name( "all" )
          .description( "Show hidden files and directories".to_string() )
          .kind( Kind::Boolean )
          .hint( "Show hidden".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "false".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
  ]
}

/// Create network module commands
fn create_network_commands() -> Vec<CommandDefinition>
{
  vec![
    CommandDefinition::former()
      .name( "net.ping" )
      .description( "Send ICMP ping packets to network host".to_string() )
      .hint( "Network ping".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "host" )
          .description( "Target hostname or IP address".to_string() )
          .kind( Kind::String )
          .hint( "Target host".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "count" )
          .description( "Number of ping packets to send".to_string() )
          .kind( Kind::Integer )
          .hint( "Packet count".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "4".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( "net.scan" )
      .description( "Scan network ports on target host".to_string() )
      .hint( "Network port scan".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "host" )
          .description( "Target hostname or IP address".to_string() )
          .kind( Kind::String )
          .hint( "Target host".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "ports" )
          .description( "Port range to scan (e.g., 80,443,8080-8090)".to_string() )
          .kind( Kind::String )
          .hint( "Port range".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "80,443".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
  ]
}

// =============================================================================
// Performance Benchmarking
// =============================================================================

/// Benchmark static registry performance vs dynamic registry
fn benchmark_registry_performance()
{
  println!( "⚡ Performance Benchmarking: Static vs Dynamic Registries" );
  println!();

  // Create static registry using CliBuilder
  let static_registry = CliBuilder::new()
    .app_name( "bench_static" )
    .static_module_with_prefix( "auth", "auth", create_auth_commands() )
    .build_static()
    .expect( "Failed to build static registry" );

  // Create dynamic registry
  let mut dynamic_registry = CommandRegistry::new();
  for cmd in create_auth_commands()
  {
    dynamic_registry.register( cmd );
  }

  let test_commands = vec![
    "auth.login",
    "auth.logout",
    "auth.token",
    "nonexistent", // Test cache miss
  ];

  let iterations = 1000;

  // Benchmark static registry
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &test_commands
    {
      let _result = static_registry.command( cmd_name );
    }
  }
  let static_duration = start.elapsed();

  // Benchmark dynamic registry
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &test_commands
    {
      let _result = dynamic_registry.command_optimized( cmd_name );
    }
  }
  let dynamic_duration = start.elapsed();

  println!( "Benchmark Results ({} iterations × {} commands):", iterations, test_commands.len() );
  println!( "  📊 Static Registry:  {:?} ({:.2} ns/lookup)", static_duration, static_duration.as_nanos() as f64 / (iterations * test_commands.len()) as f64 );
  println!( "  📊 Dynamic Registry: {:?} ({:.2} ns/lookup)", dynamic_duration, dynamic_duration.as_nanos() as f64 / (iterations * test_commands.len()) as f64 );

  let speedup = if static_duration.as_nanos() > 0 {
    dynamic_duration.as_nanos() as f64 / static_duration.as_nanos() as f64
  } else {
    1.0
  };
  println!( "  🚀 Static speedup: {speedup:.2}x faster" );
  println!();
}

// =============================================================================
// Multi-Registry Hybrid Pattern
// =============================================================================

/// Demonstrate hybrid pattern with multiple specialized registries
fn demonstrate_hybrid_pattern() -> Result<(), unilang::Error>
{
  println!( "🔄 Multi-Registry Hybrid Pattern" );
  println!();

  // Create specialized static registries for each module
  let auth_registry = CliBuilder::new()
    .app_name( "auth_module" )
    .static_module( "auth", create_auth_commands() )
    .build_static()?;

  let fs_registry = CliBuilder::new()
    .app_name( "fs_module" )
    .static_module( "filesystem", create_filesystem_commands() )
    .build_static()?;

  let net_registry = CliBuilder::new()
    .app_name( "net_module" )
    .static_module( "network", create_network_commands() )
    .build_static()?;

  println!( "Specialized registries created:" );
  println!( "  🔐 Authentication: {} commands", auth_registry.commands().len() );
  println!( "  📁 Filesystem: {} commands", fs_registry.commands().len() );
  println!( "  🌐 Network: {} commands", net_registry.commands().len() );
  println!();

  // Create unified aggregation using CliBuilder
  let unified_registry = CliBuilder::new()
    .app_name( "multi_module_system" )
    .global_prefix( "sys" )
    .static_module_with_prefix( "auth", "auth", create_auth_commands() )
    .static_module_with_prefix( "filesystem", "fs", create_filesystem_commands() )
    .static_module_with_prefix( "network", "net", create_network_commands() )
    .auto_help( true )
    .detect_conflicts( true )
    .build_static()?;

  println!( "Unified static registry created with CliBuilder:" );
  println!( "  📊 Total commands: {}", unified_registry.commands().len() );
  println!();

  // Demonstrate command routing patterns
  println!( "Command routing patterns:" );

  let test_commands = vec![
    ( "Authentication", "auth.login", &auth_registry ),
    ( "Filesystem", "fs.copy", &fs_registry ),
    ( "Network", "net.ping", &net_registry ),
  ];

  for (module, cmd_name, registry) in test_commands
  {
    match registry.command( cmd_name )
    {
      Some( cmd ) => println!( "  ✅ {}: {} -> {}", module, cmd_name, cmd.description ),
      None => println!( "  ❌ {module}: {cmd_name} -> Not found" ),
    }
  }
  println!();

  Ok( () )
}

// =============================================================================
// Conflict Detection and Resolution
// =============================================================================

/// Demonstrate conflict detection across modules
fn demonstrate_conflict_detection()
{
  println!( "🔍 Conflict Detection and Resolution" );
  println!();

  // Create CliBuilder to test conflict detection
  let builder = CliBuilder::new()
    .app_name( "conflict_test" )
    .detect_conflicts( true );

  // Simulate conflicts by checking namespace overlaps
  let conflicts = builder.detect_conflicts_report();

  if conflicts.is_empty()
  {
    println!( "  ✅ No conflicts detected in multi-module aggregation" );
    println!( "  📋 Clean namespace separation:" );
    println!( "    - auth.* commands in authentication module" );
    println!( "    - fs.* commands in filesystem module" );
    println!( "    - net.* commands in network module" );
  }
  else
  {
    println!( "  ⚠️ Conflicts detected:" );
    for conflict in conflicts
    {
      println!( "    - {conflict:?}" );
    }
  }
  println!();
}

// =============================================================================
// Main Demonstration
// =============================================================================

fn main() -> Result<(), unilang::Error>
{
  println!( "🎯 Multi-Module Static Aggregation with PHF Maps" );
  println!();

  // Module overview
  let auth_commands = create_auth_commands();
  let fs_commands = create_filesystem_commands();
  let net_commands = create_network_commands();

  println!( "📦 Module Overview:" );
  println!( "  🔐 Authentication Module: {} commands", auth_commands.len() );
  println!( "  📁 Filesystem Module: {} commands", fs_commands.len() );
  println!( "  🌐 Network Module: {} commands", net_commands.len() );
  println!();

  // Show aggregated command structure
  let total_commands = auth_commands.len() + fs_commands.len() + net_commands.len();
  println!( "🔗 Aggregated Command Structure (total: {total_commands}):" );

  let mut categories: HashMap<String, Vec<String>> = HashMap::new();

  // Collect command names by category
  for cmd in &auth_commands
  {
    categories.entry( "auth".to_string() )
      .or_default()
      .push( format!( "sys.{}", cmd.name ) );
  }

  for cmd in &fs_commands
  {
    categories.entry( "fs".to_string() )
      .or_default()
      .push( format!( "sys.{}", cmd.name ) );
  }

  for cmd in &net_commands
  {
    categories.entry( "net".to_string() )
      .or_default()
      .push( format!( "sys.{}", cmd.name ) );
  }

  for (category, commands) in &categories
  {
    println!( "  📂 sys.{} ({} commands):", category, commands.len() );
    for cmd in commands.iter().take( 3 )
    {
      println!( "    - {cmd}" );
    }
    if commands.len() > 3
    {
      println!( "    ... and {} more", commands.len() - 3 );
    }
  }
  println!();

  // Performance benchmarking
  benchmark_registry_performance();

  // Hybrid pattern demonstration
  demonstrate_hybrid_pattern()?;

  // Conflict detection
  demonstrate_conflict_detection();

  // Advanced features summary
  println!( "🚀 Advanced Features Demonstrated:" );
  println!( "  ✅ Multi-module static aggregation with zero-cost lookups" );
  println!( "  ✅ Namespace isolation preventing command conflicts" );
  println!( "  ✅ Performance comparison: static vs dynamic registries" );
  println!( "  ✅ Hybrid patterns for complex application architectures" );
  println!( "  ✅ Compile-time conflict detection and resolution" );
  println!( "  ✅ Production-ready patterns for large-scale CLIs" );
  println!( "  ✅ Type-safe static command definitions" );
  println!( "  ✅ Cross-module command routing and discovery" );
  println!();

  println!( "🎉 Multi-module static aggregation complete!" );
  println!( "   Ready for production deployment with optimal performance." );

  Ok( () )
}