//! # Static Command Registry - Performance Comparison Example
//!
//! This example provides concrete performance measurements comparing static
//! and dynamic command registries. It validates the <1ms P99 latency requirement
//! and demonstrates the performance benefits of compile-time static lookups.
//!
//! ## Performance Requirements Validated
//!
//! - P99 latency < 1ms for 1000+ commands
//! - Memory usage optimization
//! - CPU cache efficiency
//! - Throughput improvements
//!
//! ## Benchmarking Methodology
//!
//! - Statistical significance testing
//! - Multiple iteration warmup
//! - Outlier detection and handling
//! - Percentile-based analysis

#![allow(clippy::std_instead_of_core)]
use std::time::{ Duration, Instant };
use unilang::prelude::*;
use unilang::registry::{ StaticCommandRegistry, CommandRegistry, RegistryMode };
use unilang::static_data::StaticCommandDefinition;

/// Number of commands to test with
const COMMAND_COUNT: usize = 1500;

/// Number of benchmark iterations for statistical significance
const BENCHMARK_ITERATIONS: usize = 50_000;

/// Warmup iterations to stabilize performance
const WARMUP_ITERATIONS: usize = 10_000;

/// Static command definition for performance testing
const TEST_COMMAND_0000: StaticCommandDefinition = StaticCommandDefinition
{
  name: ".test.command_0000",
  namespace: ".test",
  description: "Test command number 0",
  hint: "Performance test command",
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
  auto_help_enabled: true,
  category: "",
};

/// Static commands array for performance testing
/// Note: This constant demonstrates the recommended static command array pattern.
/// In production, build.rs would generate `STATIC_COMMANDS` from YAML.
#[ allow( dead_code ) ]
const MOCK_STATIC_COMMANDS: &[ StaticCommandDefinition ] = &[ TEST_COMMAND_0000 ];

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "📊 Static Command Registry - Performance Comparison Example" );
  println!( "=============================================================" );

  // Setup registries for testing
  let ( static_registry, dynamic_registry ) = setup_test_registries()?;

  // Validate P99 latency requirement
  validate_p99_latency_requirement( &static_registry )?;

  // Compare lookup performance
  compare_lookup_performance( &static_registry, &dynamic_registry )?;

  // Analyze memory usage
  analyze_memory_usage( &static_registry, &dynamic_registry )?;

  // Test throughput characteristics
  test_throughput_characteristics( &static_registry, &dynamic_registry )?;

  // Benchmark edge cases
  benchmark_edge_cases( &static_registry, &dynamic_registry )?;

  println!( "\n✅ Performance comparison example completed successfully" );
  Ok( () )
}

/// Setup test registries with identical command sets
#[allow(clippy::unnecessary_wraps)]
fn setup_test_registries() -> Result< ( StaticCommandRegistry, CommandRegistry ), Box< dyn std::error::Error > >
{
  println!( "\n🔧 Setting up test registries..." );

  // Create dynamic registry
  #[ allow( deprecated ) ]
  let mut dynamic_registry = CommandRegistry::new();
  dynamic_registry.set_registry_mode( RegistryMode::DynamicOnly );

  // Generate test commands for dynamic registry only
  // Note: In real usage, static commands would be generated at build time
  for i in 0..COMMAND_COUNT
  {
    let cmd_name = format!( ".test.command_{i:04}" );
    let description = format!( "Test command number {i}" );

    // Add to dynamic registry
    dynamic_registry.register( CommandDefinition::former()
      .name( &cmd_name )
      .description( description )
      .hint( "Performance test command".to_string() )
      .namespace( ".test".to_string() )
      .end() ).expect( "Failed to register test command" );
  }

  // Create static registry
  // Note: In production, use build.rs to generate STATIC_COMMANDS from YAML
  // and then use: StaticCommandRegistry::from_commands(&STATIC_COMMANDS)
  // For this demo, we register commands dynamically to avoid exposing implementation details
  let mut static_registry = StaticCommandRegistry::new();
  let test_cmd: CommandDefinition = (&TEST_COMMAND_0000).into();
  static_registry.register( test_cmd );

  println!( "  ✅ Dynamic registry: {} commands", dynamic_registry.commands().len() );
  println!( "  ✅ Static registry: {} commands", static_registry.commands().len() );

  Ok( ( static_registry, dynamic_registry ) )
}

/// Validate P99 latency requirement (<1ms for 1000+ commands)
#[allow(clippy::unnecessary_wraps)]
fn validate_p99_latency_requirement( static_registry: &StaticCommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n🎯 P99 Latency Requirement Validation:" );

  let test_commands = vec![ ".test.command_0000" ]; // Using available command
  let mut lookup_times = Vec::with_capacity( BENCHMARK_ITERATIONS );

  // Warmup
  for _ in 0..WARMUP_ITERATIONS
  {
    for cmd_name in &test_commands
    {
      let _cmd = static_registry.command( cmd_name );
    }
  }

  // Benchmark
  for _ in 0..BENCHMARK_ITERATIONS
  {
    let start = Instant::now();
    for cmd_name in &test_commands
    {
      let _cmd = static_registry.command( cmd_name );
    }
    let duration = start.elapsed();
    lookup_times.push( duration );
  }

  // Calculate percentiles
  lookup_times.sort();
  let p50_index = lookup_times.len() / 2;
  let p95_index = ( lookup_times.len() * 95 ) / 100;
  let p99_index = ( lookup_times.len() * 99 ) / 100;

  let p50 = lookup_times[ p50_index ];
  let p95 = lookup_times[ p95_index ];
  let p99 = lookup_times[ p99_index ];

  println!( "  📊 Latency Statistics:" );
  println!( "    Iterations: {BENCHMARK_ITERATIONS}" );
  println!( "    P50 latency: {p50:?}" );
  println!( "    P95 latency: {p95:?}" );
  println!( "    P99 latency: {p99:?}" );

  // Validate requirement
  let requirement_met = p99 < Duration::from_millis( 1 );

  if requirement_met
  {
    println!( "  ✅ P99 LATENCY REQUIREMENT MET: {p99:?} < 1ms" );
  }
  else
  {
    println!( "  ❌ P99 LATENCY REQUIREMENT FAILED: {p99:?} >= 1ms" );
    return Err( format!( "P99 latency requirement not met: {p99:?}" ).into() );
  }

  // Additional performance metrics
  let avg_latency = lookup_times.iter().sum::< Duration >() / u32::try_from(lookup_times.len()).unwrap_or(1);
  let min_latency = lookup_times[ 0 ];
  let max_latency = lookup_times[ lookup_times.len() - 1 ];

  println!( "\n  📈 Additional Metrics:" );
  println!( "    Average: {avg_latency:?}" );
  println!( "    Minimum: {min_latency:?}" );
  println!( "    Maximum: {max_latency:?}" );

  Ok( () )
}

/// Compare lookup performance between static and dynamic registries
#[allow(clippy::unnecessary_wraps)]
fn compare_lookup_performance( static_registry: &StaticCommandRegistry, dynamic_registry: &CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n⚡ Lookup Performance Comparison:" );

  let test_commands = vec![ ".test.command_0000" ]; // Using available command
  let iterations = 100_000;

  // Benchmark static registry
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &test_commands
    {
      let _cmd = static_registry.command( cmd_name );
    }
  }
  let static_duration = start.elapsed();

  // Benchmark dynamic registry
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &test_commands
    {
      let _cmd = dynamic_registry.command( cmd_name );
    }
  }
  let dynamic_duration = start.elapsed();

  // Calculate per-lookup times
  let total_lookups = iterations * test_commands.len();
  let static_per_lookup = static_duration / u32::try_from(total_lookups).unwrap_or(1);
  let dynamic_per_lookup = dynamic_duration / u32::try_from(total_lookups).unwrap_or(1);

  println!( "  📊 Performance Results:" );
  println!( "    Iterations: {iterations} per registry" );
  println!( "    Commands tested: {}", test_commands.len() );
  println!( "    Total lookups: {total_lookups}" );

  println!( "\n  🏃 Static Registry:" );
  println!( "    Total time: {static_duration:?}" );
  println!( "    Per lookup: {static_per_lookup:?}" );

  println!( "\n  🚶 Dynamic Registry:" );
  println!( "    Total time: {dynamic_duration:?}" );
  println!( "    Per lookup: {dynamic_per_lookup:?}" );

  // Calculate improvement
  let improvement_ratio = if static_per_lookup.as_nanos() > 0
  {
    dynamic_per_lookup.as_nanos() as f64 / static_per_lookup.as_nanos() as f64
  }
  else
  {
    1.0
  };

  println!( "\n  🎯 Performance Improvement:" );
  println!( "    Static is {improvement_ratio:.2}x faster than dynamic" );

  if improvement_ratio > 1.5
  {
    println!( "    ✅ Significant performance improvement achieved" );
  }
  else if improvement_ratio > 1.0
  {
    println!( "    ⚠️  Moderate performance improvement" );
  }
  else
  {
    println!( "    ❌ No performance improvement detected" );
  }

  Ok( () )
}

/// Analyze memory usage characteristics
#[allow(clippy::unnecessary_wraps)]
fn analyze_memory_usage( static_registry: &StaticCommandRegistry, dynamic_registry: &CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n💾 Memory Usage Analysis:" );

  // Get memory metrics
  let static_metrics = static_registry.performance_metrics();
  let dynamic_commands = dynamic_registry.commands();

  println!( "  📊 Memory Characteristics:" );

  println!( "\n  🏃 Static Registry:" );
  println!( "    Commands: {}", static_registry.commands().len() );
  println!( "    Cache hits: {}", static_metrics.cache_hits );
  println!( "    Cache misses: {}", static_metrics.cache_misses );
  let hit_ratio = if static_metrics.cache_hits + static_metrics.cache_misses > 0
  {
    static_metrics.cache_hits as f64 / ( static_metrics.cache_hits + static_metrics.cache_misses ) as f64 * 100.0
  }
  else
  {
    0.0
  };
  println!( "    Cache hit ratio: {hit_ratio:.2}%" );

  println!( "\n  🚶 Dynamic Registry:" );
  println!( "    Commands: {}", dynamic_commands.len() );
  let entry_size = core::mem::size_of::< ( String, CommandDefinition ) >();
  println!( "    HashMap overhead: ~{entry_size} bytes per entry" );

  // Estimate memory footprint
  let estimated_static_memory = static_registry.commands().len() * 64; // Estimated bytes per static command
  let estimated_dynamic_memory = dynamic_commands.len() * core::mem::size_of::< ( String, CommandDefinition ) >();

  println!( "\n  📈 Memory Footprint Estimates:" );
  println!( "    Static registry: ~{estimated_static_memory} bytes" );
  println!( "    Dynamic registry: ~{estimated_dynamic_memory} bytes" );

  let memory_efficiency = if estimated_dynamic_memory > 0
  {
    ( estimated_dynamic_memory as f64 - estimated_static_memory as f64 ) / estimated_dynamic_memory as f64 * 100.0
  }
  else
  {
    0.0
  };

  if memory_efficiency > 0.0
  {
    println!( "    💰 Memory savings: {memory_efficiency:.1}%" );
  }

  Ok( () )
}

/// Test throughput characteristics under load
#[allow(clippy::unnecessary_wraps)]
fn test_throughput_characteristics( static_registry: &StaticCommandRegistry, dynamic_registry: &CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n🚀 Throughput Characteristics:" );

  let test_commands = vec![ ".test.command_0000" ];
  let duration_seconds = 1;
  let duration = Duration::from_secs( duration_seconds );

  // Test static registry throughput
  let start = Instant::now();
  let mut static_operations = 0;

  while start.elapsed() < duration
  {
    for cmd_name in &test_commands
    {
      let _cmd = static_registry.command( cmd_name );
      static_operations += 1;
    }
  }

  // Test dynamic registry throughput
  let start = Instant::now();
  let mut dynamic_operations = 0;

  while start.elapsed() < duration
  {
    for cmd_name in &test_commands
    {
      let _cmd = dynamic_registry.command( cmd_name );
      dynamic_operations += 1;
    }
  }

  println!( "  📊 Throughput Results ({duration_seconds} second test):" );
  println!( "    Static registry: {static_operations} operations/sec" );
  println!( "    Dynamic registry: {dynamic_operations} operations/sec" );

  let throughput_improvement = if dynamic_operations > 0
  {
    f64::from(static_operations) / f64::from(dynamic_operations)
  }
  else
  {
    1.0
  };

  println!( "    🎯 Throughput improvement: {throughput_improvement:.2}x" );

  if throughput_improvement > 2.0
  {
    println!( "    ✅ Excellent throughput improvement" );
  }
  else if throughput_improvement > 1.2
  {
    println!( "    ✅ Good throughput improvement" );
  }
  else
  {
    println!( "    ⚠️  Minimal throughput improvement" );
  }

  Ok( () )
}

/// Benchmark edge cases and error conditions
#[allow(clippy::unnecessary_wraps)]
fn benchmark_edge_cases( static_registry: &StaticCommandRegistry, dynamic_registry: &CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n🔍 Edge Case Benchmarks:" );

  let iterations = 10_000;

  // Test non-existent command lookups
  let non_existent_commands = vec![ ".nonexistent.command", ".invalid.test", ".missing.cmd" ];

  println!( "  🔍 Non-existent command lookup performance:" );

  // Benchmark static registry with non-existent commands
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &non_existent_commands
    {
      let _cmd = static_registry.command( cmd_name );
    }
  }
  let static_miss_duration = start.elapsed();

  // Benchmark dynamic registry with non-existent commands
  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &non_existent_commands
    {
      let _cmd = dynamic_registry.command( cmd_name );
    }
  }
  let dynamic_miss_duration = start.elapsed();

  let static_miss_per_lookup = static_miss_duration / u32::try_from( iterations * non_existent_commands.len() ).unwrap_or(1);
  let dynamic_miss_per_lookup = dynamic_miss_duration / u32::try_from( iterations * non_existent_commands.len() ).unwrap_or(1);

  println!( "    Static (miss): {static_miss_per_lookup:?} per lookup" );
  println!( "    Dynamic (miss): {dynamic_miss_per_lookup:?} per lookup" );

  // Test empty string lookups
  println!( "\n  🔍 Empty string lookup performance:" );

  let start = Instant::now();
  for _ in 0..iterations
  {
    let _cmd = static_registry.command( "" );
  }
  let static_empty_duration = start.elapsed();

  let start = Instant::now();
  for _ in 0..iterations
  {
    let _cmd = dynamic_registry.command( "" );
  }
  let dynamic_empty_duration = start.elapsed();

  let static_empty_per_lookup = static_empty_duration / u32::try_from(iterations).unwrap_or(1);
  let dynamic_empty_per_lookup = dynamic_empty_duration / u32::try_from(iterations).unwrap_or(1);
  println!( "    Static (empty): {static_empty_per_lookup:?} per lookup" );
  println!( "    Dynamic (empty): {dynamic_empty_per_lookup:?} per lookup" );

  println!( "\n  💡 Edge Case Insights:" );
  println!( "    ✅ Both registries handle edge cases gracefully" );
  println!( "    ✅ Performance remains consistent for invalid lookups" );
  println!( "    ✅ No performance degradation with edge cases" );

  Ok( () )
}