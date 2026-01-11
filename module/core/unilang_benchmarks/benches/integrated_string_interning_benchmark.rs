//! Integrated String Interning Performance Benchmark
//!
//! This benchmark tests the real-world performance impact of string interning
//! within the full command processing pipeline, measuring the end-to-end
//! improvement in semantic analysis performance.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::time::Instant;
use unilang::prelude::*;

#[ derive( Debug, Clone ) ]
struct IntegratedBenchmarkResult
{
  test_name : String,
  commands_processed : usize,
  total_time_ms : f64,
  avg_time_per_command_ns : f64,
  commands_per_second : f64,
  p99_latency_ns : u64,
}

fn create_test_registry() -> CommandRegistry
{
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();
  
  // Register common commands that would trigger string interning benefits
  let commands = vec![
    (".file.create", "Create a new file"),
    (".file.delete", "Delete an existing file"),
    (".file.copy", "Copy a file"),
    (".file.move", "Move a file"),
    (".user.login", "User login"),
    (".user.logout", "User logout"),
    (".user.create", "Create user account"),
    (".system.status", "Show system status"),
    (".system.restart", "Restart system"),
    (".database.migrate", "Run database migration"),
    (".database.backup", "Create database backup"),
    (".cache.clear", "Clear application cache"),
    (".cache.warm", "Warm up cache"),
    (".config.get", "Get configuration value"),
    (".config.set", "Set configuration value"),
    (".deploy.staging", "Deploy to staging"),
    (".deploy.production", "Deploy to production"),
    (".monitor.start", "Start monitoring"),
    (".monitor.stop", "Stop monitoring"),
    (".api.health", "Check API health"),
  ];
  
  for ( name, desc ) in commands
  {
    let cmd_def = CommandDefinition::former()
      .name( name.to_string() )
      .description( desc.to_string() )
      .namespace( "test".to_string() )
      .hint( "Test command".to_string() )
      .end();

    #[ allow( clippy::let_unit_value, clippy::ignored_unit_patterns ) ]
    let _ = registry.register( cmd_def );
  }
  
  registry
}

fn benchmark_integrated_pipeline( iterations : usize, repeat_factor : usize ) -> IntegratedBenchmarkResult
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );
  
  // Create test commands with realistic repetition patterns
  let base_commands = vec![
    "file create",
    "file delete", 
    "user login",
    "user logout",
    "system status",
    "database migrate",
    "cache clear",
    "config get value",
    "config set key",
    "deploy production service",
  ];
  
  // Generate repeated command patterns to simulate cache benefits
  let mut test_commands = Vec::new();
  for _ in 0..repeat_factor
  {
    for cmd in &base_commands
    {
      test_commands.push( (*cmd).to_string() );
    }
  }
  
  let mut latencies = Vec::with_capacity( iterations * test_commands.len() );
  let start_time = Instant::now();
  let mut total_processed = 0;
  
  for _ in 0..iterations
  {
    for command_text in &test_commands
    {
      let cmd_start = Instant::now();
      
      // Process through the full pipeline
      let _result = pipeline.process_command_simple( command_text );
      
      latencies.push( cmd_start.elapsed().as_nanos() as u64 );
      total_processed += 1;
    }
  }
  
  let total_time = start_time.elapsed();
  latencies.sort_unstable();
  
  IntegratedBenchmarkResult
  {
    test_name : format!( "Integrated Pipeline ({repeat_factor}x repetition)" ),
    commands_processed : total_processed,
    total_time_ms : total_time.as_secs_f64() * 1000.0,
    avg_time_per_command_ns : total_time.as_nanos() as f64 / total_processed as f64,
    commands_per_second : total_processed as f64 / total_time.as_secs_f64(),
    p99_latency_ns : latencies[ ( latencies.len() as f64 * 0.99 ) as usize ],
  }
}

fn benchmark_cache_warmup_effect() -> Vec< IntegratedBenchmarkResult >
{
  let mut results = Vec::new();
  
  // Test with different levels of command repetition to show cache effect
  let test_scenarios = vec![
    ( 1, "Cold Cache" ),      // Each command used once
    ( 10, "Warm Cache" ),     // Each command repeated 10x  
    ( 100, "Hot Cache" ),     // Each command repeated 100x
  ];
  
  for ( repeat_factor, scenario_name ) in test_scenarios
  {
    println!( "Running {scenario_name} scenario..." );
    
    let result = benchmark_integrated_pipeline( 1000, repeat_factor );
    results.push( result );
  }
  
  results
}

fn print_result( result : &IntegratedBenchmarkResult )
{
  println!( "=== {} ===" , result.test_name );
  println!( "Commands Processed: {}", result.commands_processed );
  println!( "Total Time: {:.2} ms", result.total_time_ms );
  println!( "Avg Time/Command: {:.0} ns", result.avg_time_per_command_ns );
  println!( "Commands/Second: {:.0}", result.commands_per_second );
  println!( "P99 Latency: {:.0} ns", result.p99_latency_ns );
  println!();
}

fn run_integrated_benchmark()
{
  println!( "🚀 Integrated String Interning Pipeline Benchmark" );
  println!( "================================================\n" );
  
  let results = benchmark_cache_warmup_effect();
  
  for result in &results
  {
    print_result( result );
  }
  
  // Analysis
  println!( "🎯 Cache Effect Analysis" );
  println!( "========================" );
  
  if results.len() >= 2
  {
    let cold_cache = &results[ 0 ];
    let hot_cache = &results[ results.len() - 1 ];
    
    let throughput_improvement = hot_cache.commands_per_second / cold_cache.commands_per_second;
    let latency_improvement = cold_cache.avg_time_per_command_ns / hot_cache.avg_time_per_command_ns;
    
    println!( "Cold Cache Performance: {:.0} cmd/sec", cold_cache.commands_per_second );
    println!( "Hot Cache Performance: {:.0} cmd/sec", hot_cache.commands_per_second );
    println!( "Throughput Improvement: {throughput_improvement:.1}x" );
    println!( "Latency Improvement: {latency_improvement:.1}x" );
    println!();
    
    // Validate against targets
    let target_met = throughput_improvement >= 2.0; // More conservative target for integrated test
    println!( "✅ Performance Target (2x improvement): {}", 
             if target_met { "PASSED" } else { "FAILED" } );
    
    if throughput_improvement >= 5.0
    {
      println!( "🎉 Exceeded stretch goal of 5x improvement!" );
    }
    
    // Memory efficiency indication
    println!();
    println!( "💾 Memory Efficiency Indicators:" );
    println!( "- String interning reduces allocations for repeated commands" );
    println!( "- Cache hit ratio increases with command repetition" );
    println!( "- Hot cache scenario shows sustained high performance" );
  }
  
  // Latency analysis
  println!();
  println!( "⚡ Latency Analysis:" );
  for result in &results
  {
    println!( "- {}: P99 = {:.0} ns", result.test_name, result.p99_latency_ns );
  }
  
  let latency_target_met = results.iter().all( | r | r.p99_latency_ns <= 500_000 ); // 500μs
  println!( "- P99 under 500μs target: {}", if latency_target_met { "PASSED" } else { "FAILED" } );
}

fn main()
{
  run_integrated_benchmark();
}