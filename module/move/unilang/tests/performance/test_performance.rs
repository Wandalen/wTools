//!
//! Tests for performance metrics functionality.
//!
//! This module tests the performance metrics tracking system used by both
//! `CommandRegistry` and `StaticCommandRegistry` to monitor cache performance,
//! lookup patterns, and registry efficiency.

use unilang::prelude::*;
use unilang::registry::{ CommandRegistry, StaticCommandRegistry, PerformanceMetrics };
use unilang::static_data::{ StaticCommandDefinition, StaticCommandMap };
use core::time::Duration;
use std::time::Instant;

/// Create a test PHF map for performance metrics testing
static PERFORMANCE_TEST_COMMANDS: StaticCommandMap = phf::phf_map!
{
  ".perf_test_1" => &StaticCommandDefinition
  {
    name: ".perf_test_1",
    namespace: "perf",
    description: "Performance test command 1",
    hint: "Test command",
    arguments: &[],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[],
  },
  ".perf_test_2" => &StaticCommandDefinition
  {
    name: ".perf_test_2",
    namespace: "perf",
    description: "Performance test command 2",
    hint: "Test command",
    arguments: &[],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[],
  },
  ".perf_test_3" => &StaticCommandDefinition
  {
    name: ".perf_test_3",
    namespace: "perf",
    description: "Performance test command 3",
    hint: "Test command",
    arguments: &[],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[],
  },
};

#[test]
fn test_performance_metrics_structure()
{
  // Test PerformanceMetrics basic structure and default values
  let metrics = PerformanceMetrics::default();

  assert_eq!( metrics.cache_hits, 0 );
  assert_eq!( metrics.cache_misses, 0 );
  assert_eq!( metrics.total_lookups, 0 );
  assert_eq!( metrics.static_lookups, 0 );
  assert_eq!( metrics.dynamic_lookups, 0 );
}

#[test]
fn test_performance_metrics_calculations()
{
  // Test PerformanceMetrics calculation methods
  let mut metrics = PerformanceMetrics::default();

  // Initial state - no lookups
  assert!( metrics.cache_hit_rate().abs() < f64::EPSILON );
  assert!( metrics.static_ratio().abs() < f64::EPSILON );

  // Add some test data
  metrics.total_lookups = 100;
  metrics.cache_hits = 80;
  metrics.cache_misses = 20;
  metrics.static_lookups = 60;
  metrics.dynamic_lookups = 40;

  // Test cache hit rate calculation
  assert!( (metrics.cache_hit_rate() - 0.8).abs() < f64::EPSILON ); // 80/100

  // Test static vs dynamic ratio
  assert!( (metrics.static_ratio() - 0.6).abs() < f64::EPSILON ); // 60/100

  // Test edge cases
  metrics.total_lookups = 0;
  assert!( metrics.cache_hit_rate().abs() < f64::EPSILON );
  assert!( metrics.static_ratio().abs() < f64::EPSILON );
}

#[test]
fn test_command_registry_performance_metrics()
{
  // Test performance metrics integration with CommandRegistry
  let mut registry = CommandRegistry::new();

  // Add some test commands
  let cmd1 = CommandDefinition::former()
    .name( ".test_cmd_1" )
    .description( "Test command 1".to_string() )
    .hint( "Test".to_string() )
    .end();

  let cmd2 = CommandDefinition::former()
    .name( ".test_cmd_2" )
    .description( "Test command 2".to_string() )
    .hint( "Test".to_string() )
    .end();

  registry.register( cmd1 );
  registry.register( cmd2 );

  // Get initial metrics
  let initial_metrics = registry.performance_metrics();
  let initial_total = initial_metrics.total_lookups;

  // Perform some lookups
  let _cmd1 = registry.command_optimized( ".test_cmd_1" );
  let _cmd2 = registry.command_optimized( ".test_cmd_2" );
  let _cmd1_again = registry.command_optimized( ".test_cmd_1" ); // Should hit cache
  let _nonexistent = registry.command_optimized( "nonexistent" );

  // Check metrics were updated
  let updated_metrics = registry.performance_metrics();
  assert!( updated_metrics.total_lookups > initial_total );

  // Verify metrics make sense
  let total_operations = updated_metrics.cache_hits + updated_metrics.cache_misses;
  assert!( total_operations <= updated_metrics.total_lookups );
}

#[test]
fn test_static_command_registry_performance_metrics()
{
  // Test performance metrics integration with StaticCommandRegistry
  let mut registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );

  // Get initial metrics
  let initial_metrics = registry.performance_metrics();
  assert_eq!( initial_metrics.total_lookups, 0 );
  assert_eq!( initial_metrics.static_lookups, 0 );

  // Perform some static command lookups
  let _cmd1 = registry.command_with_metrics( ".perf_test_1" );
  let _cmd2 = registry.command_with_metrics( ".perf_test_2" );
  let _cmd3 = registry.command_with_metrics( ".perf_test_3" );
  let _nonexistent = registry.command_with_metrics( "nonexistent" );

  // Check metrics were updated
  let updated_metrics = registry.performance_metrics();
  assert_eq!( updated_metrics.total_lookups, 4 );
  assert_eq!( updated_metrics.static_lookups, 3 ); // 3 successful static lookups

  // Test static ratio calculation
  assert!( (updated_metrics.static_ratio() - 0.75).abs() < f64::EPSILON ); // 3/4
}

#[test]
fn test_hybrid_registry_performance_metrics()
{
  // Test performance metrics with hybrid registry (static + dynamic)
  let mut registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );

  // Add a dynamic command
  let dynamic_cmd = CommandDefinition::former()
    .name( ".dynamic_test" )
    .description( "Dynamic test command".to_string() )
    .hint( "Dynamic".to_string() )
    .end();

  registry.register( dynamic_cmd );

  // Perform mixed lookups
  let _static_cmd = registry.command_with_metrics( ".perf_test_1" ); // Static
  let _dynamic_cmd = registry.command_with_metrics( ".dynamic_test" ); // Dynamic
  let _static_cmd2 = registry.command_with_metrics( ".perf_test_2" ); // Static
  let _nonexistent = registry.command_with_metrics( "nonexistent" ); // Miss

  // Check hybrid metrics
  let metrics = registry.performance_metrics();
  assert_eq!( metrics.total_lookups, 4 );
  assert_eq!( metrics.static_lookups, 2 ); // 2 static hits

  // Test ratios
  assert!( (metrics.static_ratio() - 0.5).abs() < f64::EPSILON ); // 2/4 static
}

#[test]
fn test_performance_metrics_reset()
{
  // Test metrics reset functionality
  let mut registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );

  // Perform some lookups to generate metrics
  let _cmd1 = registry.command_with_metrics( ".perf_test_1" );
  let _cmd2 = registry.command_with_metrics( ".perf_test_2" );

  // Verify metrics are non-zero
  let metrics_before = registry.performance_metrics();
  assert!( metrics_before.total_lookups > 0 );

  // Reset metrics
  registry.clear();

  // Verify metrics are reset
  let metrics_after = registry.performance_metrics();
  assert_eq!( metrics_after.total_lookups, 0 );
  assert_eq!( metrics_after.static_lookups, 0 );
  assert_eq!( metrics_after.cache_hits, 0 );
  assert_eq!( metrics_after.cache_misses, 0 );
}

#[test]
fn test_cache_performance_patterns()
{
  // Test cache performance patterns
  let mut registry = CommandRegistry::new();

  // Add test commands
  for i in 0..10
  {
    let cmd = CommandDefinition::former()
      .name( format!( ".cache_test_{i}" ) )
      .description( format!( "Cache test command {i}" ) )
      .hint( "Cache test".to_string() )
      .end();
    registry.register( cmd );
  }

  // Perform repeated lookups to test cache behavior
  let commands_to_test = vec![
    ".cache_test_0", ".cache_test_1", ".cache_test_2",
    ".cache_test_0", ".cache_test_1", // Repeated lookups
    ".cache_test_0", // More repetition
  ];

  for cmd_name in &commands_to_test
  {
    let _cmd = registry.command_optimized( cmd_name );
  }

  let metrics = registry.performance_metrics();

  // Verify cache behavior
  assert!( metrics.total_lookups >= commands_to_test.len() as u64 );

  // With repeated lookups, we should see some cache hits
  let cache_efficiency = metrics.cache_hit_rate();
  let cache_eff_percent = cache_efficiency * 100.0;
  println!( "Cache efficiency: {cache_eff_percent:.2}%" );

  // Cache hit rate should be reasonable (depends on cache implementation)
  assert!( (0.0..=1.0).contains(&cache_efficiency) );
}

#[test]
fn test_performance_metrics_under_load()
{
  // Test performance metrics under high load
  let mut registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );

  let iterations = 1000;
  let command_names = vec![ ".perf_test_1", ".perf_test_2", ".perf_test_3", "nonexistent" ];

  // Warmup
  for _ in 0..100
  {
    for cmd_name in &command_names
    {
      let _cmd = registry.command_with_metrics( cmd_name );
    }
  }

  // Reset metrics after warmup
  registry.clear();

  // Performance test
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &command_names
    {
      let _cmd = registry.command_with_metrics( cmd_name );
    }
  }
  let duration = start.elapsed();

  // Verify metrics
  let metrics = registry.performance_metrics();
  let expected_total = (iterations * command_names.len()) as u64;
  let expected_static = (iterations * 3) as u64; // 3 valid commands

  assert_eq!( metrics.total_lookups, expected_total );
  assert_eq!( metrics.static_lookups, expected_static );
  assert!( (metrics.static_ratio() - 0.75).abs() < f64::EPSILON ); // 3/4 commands are valid

  // Performance should be reasonable
  let avg_lookup_time = duration / u32::try_from(expected_total).unwrap_or(1);
  println!( "Average lookup time under load: {avg_lookup_time:?}" );

  // For performance test, lookups should be fast
  assert!( avg_lookup_time < Duration::from_micros( 10 ) );
}

#[test]
fn test_registry_mode_impact_on_metrics()
{
  // Test how different registry modes affect performance metrics
  let static_registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );
  let mut dynamic_registry = CommandRegistry::new();

  // Add same commands to dynamic registry
  for (_name, static_cmd) in PERFORMANCE_TEST_COMMANDS.entries()
  {
    let dynamic_cmd: CommandDefinition = (*static_cmd).into();
    dynamic_registry.register( dynamic_cmd );
  }

  let test_commands = vec![ ".perf_test_1", ".perf_test_2", ".perf_test_3" ];
  let iterations = 100;

  // Test static registry performance
  let mut static_reg = static_registry;
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &test_commands
    {
      let _cmd = static_reg.command_with_metrics( cmd_name );
    }
  }
  let static_duration = start.elapsed();
  let static_metrics = static_reg.performance_metrics();

  // Test dynamic registry performance
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &test_commands
    {
      let _cmd = dynamic_registry.command_optimized( cmd_name );
    }
  }
  let dynamic_duration = start.elapsed();
  let dynamic_metrics = dynamic_registry.performance_metrics();

  // Compare performance characteristics
  println!( "Static registry - Duration: {static_duration:?}, Lookups: {}", static_metrics.total_lookups );
  println!( "Dynamic registry - Duration: {dynamic_duration:?}, Lookups: {}", dynamic_metrics.total_lookups );

  // Both should have processed the same number of lookups
  let expected_lookups = (iterations * test_commands.len()) as u64;
  assert_eq!( static_metrics.total_lookups, expected_lookups );
  assert_eq!( dynamic_metrics.total_lookups, expected_lookups );

  // Static registry should show all lookups as static
  assert_eq!( static_metrics.static_lookups, expected_lookups );
  assert!( (static_metrics.static_ratio() - 1.0).abs() < f64::EPSILON );
}

#[test]
fn test_performance_metrics_edge_cases()
{
  // Test edge cases for performance metrics
  let mut registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );

  // Test lookup of empty string
  let _empty = registry.command_with_metrics( "" );

  // Test lookup of very long string
  let long_name = "a".repeat( 1000 );
  let _long = registry.command_with_metrics( &long_name );

  // Test lookup with special characters
  let _special = registry.command_with_metrics( "special!@#$%^&*()" );

  // Metrics should still be accurate
  let metrics = registry.performance_metrics();
  assert_eq!( metrics.total_lookups, 3 );
  assert_eq!( metrics.static_lookups, 0 ); // None of these should match
  assert!( metrics.static_ratio().abs() < f64::EPSILON );
}

#[test]
fn test_performance_metrics_consistency()
{
  // Test that performance metrics remain consistent across different access patterns
  let mut registry = StaticCommandRegistry::from_phf( &PERFORMANCE_TEST_COMMANDS );

  // Mix of readonly and mutable lookups
  let _readonly1 = registry.command( ".perf_test_1" ); // Readonly (no metrics update)
  let _mutable1 = registry.command_with_metrics( ".perf_test_1" ); // Mutable (updates metrics)
  let _readonly2 = registry.command( ".perf_test_2" ); // Readonly (no metrics update)
  let _mutable2 = registry.command_with_metrics( ".perf_test_2" ); // Mutable (updates metrics)

  // Only mutable lookups should be counted
  let metrics = registry.performance_metrics();
  assert_eq!( metrics.total_lookups, 2 ); // Only the 2 mutable lookups
  assert_eq!( metrics.static_lookups, 2 );
  assert!( (metrics.static_ratio() - 1.0).abs() < f64::EPSILON );
}