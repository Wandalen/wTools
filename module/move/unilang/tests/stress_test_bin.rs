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
  let lookup_count = 1_000_000;
  let mut latencies = Vec::with_capacity( lookup_count );
  
  println!( "Starting {lookup_count} command lookups..." );
  
  // Generate command names on-the-fly to save memory
  for i in 0..lookup_count
  {
    let cmd_name = format!( ".perf.cmd_{}", i % 1_000_000 );
    
    let lookup_start = Instant::now();
    let _command = registry.command( &cmd_name );
    let lookup_time = lookup_start.elapsed();
    
    latencies.push( lookup_time );
    
    // Progress reporting every 100k lookups
    if i % 100_000 == 0 && i > 0 {
      println!( "  Completed {i} lookups..." );
    }
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
  
  // Output metrics in standardized format for test parsing
  let p99_micros = p99.as_nanos() as f64 / 1000.0;
  let startup_micros = init_time.as_nanos() as f64 / 1000.0;
  
  println!( "P99_LATENCY_MICROS: {p99_micros:.2}" );
  println!( "STARTUP_TIME_MICROS: {startup_micros:.2}" );
  
  // Check if we meet both requirements
  let p99_ok = p99_micros < 1000.0;
  let startup_ok = startup_micros < 5000.0; // < 5ms startup
  
  if p99_ok && startup_ok
  {
    println!( "✅ All performance requirements MET!" );
  }
  else
  {
    if !p99_ok {
      println!( "❌ P99 latency requirement FAILED: {p99_micros:.2} μs >= 1000 μs" );
    }
    if !startup_ok {
      println!( "❌ Startup time requirement FAILED: {startup_micros:.2} μs >= 5000 μs" );
    }
  }
  
  println!( "Ready" );
}