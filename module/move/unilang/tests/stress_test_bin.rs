//!
//! Binary for performance stress testing of static command registry.
//! 
//! This binary initializes the `CommandRegistry` with static commands and
//! performs intensive lookups to measure p99 latency.
//!

use std::time::Instant;
use unilang::registry::CommandRegistry;

fn main()
{
  let start_time = Instant::now();
  
  // Initialize the registry (this should be very fast with static commands)
  let registry = CommandRegistry::new();
  let init_time = start_time.elapsed();
  
  println!( "Registry initialization time: {init_time:?}" );
  
  // Perform many command lookups to measure p99 latency
  let lookup_count = 10_000;
  let mut latencies = Vec::with_capacity( lookup_count );
  
  // Test commands from our generated set
  let test_commands = (0..1000).map( |i| format!( ".perf.cmd_{i}" ) ).collect::<Vec<_>>();
  
  println!( "Starting {lookup_count} command lookups..." );
  
  for i in 0..lookup_count
  {
    let cmd_name = &test_commands[ i % test_commands.len() ];
    
    let lookup_start = Instant::now();
    let _command = registry.command( cmd_name );
    let lookup_time = lookup_start.elapsed();
    
    latencies.push( lookup_time );
  }
  
  // Calculate statistics
  latencies.sort();
  let p50 = latencies[ lookup_count / 2 ];
  let p95 = latencies[ (lookup_count as f64 * 0.95) as usize ];
  let p99 = latencies[ (lookup_count as f64 * 0.99) as usize ];
  let max = latencies[ lookup_count - 1 ];
  
  let total_time = start_time.elapsed();
  
  println!( "Performance Results:" );
  println!( "  Total execution time: {total_time:?}" );
  println!( "  Registry init time: {init_time:?}" );
  println!( "  Total lookups: {lookup_count}" );
  println!( "  Latency p50: {p50:?}" );
  println!( "  Latency p95: {p95:?}" );
  println!( "  Latency p99: {p99:?}" );
  println!( "  Latency max: {max:?}" );
  
  // Output the p99 latency in microseconds for easy parsing
  let p99_micros = p99.as_nanos() as f64 / 1000.0;
  println!( "P99_LATENCY_MICROS: {p99_micros:.2}" );
  
  // Check if we meet the requirement (< 1ms = 1000 microseconds)
  if p99_micros < 1000.0
  {
    println!( "✅ Performance requirement MET: p99 < 1ms" );
  }
  else
  {
    println!( "❌ Performance requirement FAILED: p99 >= 1ms" );
  }
  
  println!( "Ready" );
}