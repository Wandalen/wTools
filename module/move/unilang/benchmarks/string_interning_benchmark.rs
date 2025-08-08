//! String Interning Performance Microbenchmarks
//!
//! This benchmark validates the performance improvement from implementing
//! string interning in command name construction. Tests both cache hit
//! and cache miss scenarios to validate the 5-10x performance target.
//!
//! Expected improvements:
//! - Command name construction: 5-10x faster (38K → 190K-380K cmd/sec)
//! - Memory allocation reduction: ~90% fewer allocations for repeated commands
//! - P99 latency: Under 500μs for command resolution

#[ cfg( feature = "benchmarks" ) ]
use std::time::Instant;
#[ cfg( feature = "benchmarks" ) ]
use unilang::interner::{ StringInterner, intern_command_name };

#[ derive( Debug, Clone ) ]
#[ cfg( feature = "benchmarks" ) ]
struct StringInterningResult
{
  test_name : String,
  iterations : usize,
  total_time_ns : u128,
  avg_time_ns : f64,
  p50_time_ns : u64,
  p95_time_ns : u64,
  p99_time_ns : u64,
  operations_per_second : f64,
  memory_allocations : usize, // Estimated based on new string constructions
}

/// Benchmark traditional string construction (current hot path)
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_string_construction( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  let mut total_allocations = 0;
  
  let start_time = Instant::now();
  
  for _ in 0..iterations
  {
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      // Replicate the current hot path logic
      let _command_name = if slices[ 0 ].is_empty() && slices.len() > 1
      {
        total_allocations += 2; // format!() + join()
        format!( ".{}", slices[ 1.. ].join( "." ) )
      }
      else
      {
        total_allocations += 2; // format!() + join()
        format!( ".{}", slices.join( "." ) )
      };
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "String Construction".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : total_allocations,
  }
}

/// Benchmark string interning (cache miss scenario)
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_string_interning_miss( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  let mut total_allocations = 0;
  
  let start_time = Instant::now();
  
  for _i in 0..iterations
  {
    // Create unique interner for each iteration to simulate cache miss
    let interner = StringInterner::new();
    
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      let _command_name = interner.intern_command_name( slices );
      // Cache miss = 1 allocation (Box::leak), then cached
      total_allocations += 1;
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "String Interning (Cache Miss)".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : total_allocations,
  }
}

/// Benchmark string interning (cache hit scenario)
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_string_interning_hit( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  let interner = StringInterner::new();
  
  // Pre-populate cache
  for slices in command_slices
  {
    interner.intern_command_name( slices );
  }
  
  let start_time = Instant::now();
  
  for _ in 0..iterations
  {
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      let _command_name = interner.intern_command_name( slices );
      // Cache hit = 0 allocations, just hash lookup
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "String Interning (Cache Hit)".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : 0, // All cache hits
  }
}

/// Benchmark global string interning convenience functions
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_global_interner( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  
  // Pre-populate global cache
  for slices in command_slices
  {
    intern_command_name( slices );
  }
  
  let start_time = Instant::now();
  
  for _ in 0..iterations
  {
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      let _command_name = intern_command_name( slices );
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "Global String Interner".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : 0, // Pre-cached
  }
}

#[ cfg( feature = "benchmarks" ) ]
fn print_result( result : &StringInterningResult )
{
  println!( "=== {} ===" , result.test_name );
  println!( "Iterations: {}", result.iterations );
  println!( "Total Time: {:.2} ms", result.total_time_ns as f64 / 1_000_000.0 );
  println!( "Average Time: {:.0} ns", result.avg_time_ns );
  println!( "P50 Latency: {} ns", result.p50_time_ns );
  println!( "P95 Latency: {} ns", result.p95_time_ns );
  println!( "P99 Latency: {} ns", result.p99_time_ns );
  println!( "Operations/sec: {:.0}", result.operations_per_second );
  println!( "Memory Allocations: {}", result.memory_allocations );
  println!();
}

#[ cfg( feature = "benchmarks" ) ]
fn run_string_interning_benchmarks()
{
  println!( "🚀 String Interning Performance Benchmarks" );
  println!( "============================================\n" );
  
  // Realistic command patterns from typical usage
  let test_commands = vec![
    vec![ "file", "create" ],
    vec![ "file", "delete" ],
    vec![ "user", "login" ],
    vec![ "user", "logout" ],
    vec![ "system", "status" ],
    vec![ "database", "migrate" ],
    vec![ "cache", "clear" ],
    vec![ "config", "get", "value" ],
    vec![ "config", "set", "key" ],
    vec![ "deploy", "production", "service" ],
  ];
  
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( | v | v.as_slice() ).collect();
  let iterations = 10_000; // Enough iterations for statistical significance
  
  println!( "Test Configuration:" );
  println!( "- Command patterns: {}", command_slices.len() );
  println!( "- Iterations per pattern: {}", iterations );
  println!( "- Total operations: {}", command_slices.len() * iterations );
  println!();
  
  // Benchmark 1: Traditional string construction (baseline)
  println!( "Running baseline string construction benchmark..." );
  let baseline = benchmark_string_construction( &command_slices, iterations );
  print_result( &baseline );
  
  // Benchmark 2: String interning cache miss
  println!( "Running string interning (cache miss) benchmark..." );
  let interner_miss = benchmark_string_interning_miss( &command_slices, iterations );
  print_result( &interner_miss );
  
  // Benchmark 3: String interning cache hit
  println!( "Running string interning (cache hit) benchmark..." );
  let interner_hit = benchmark_string_interning_hit( &command_slices, iterations );
  print_result( &interner_hit );
  
  // Benchmark 4: Global interner
  println!( "Running global interner benchmark..." );
  let global_interner = benchmark_global_interner( &command_slices, iterations );
  print_result( &global_interner );
  
  // Performance Analysis
  println!( "🎯 Performance Analysis" );
  println!( "======================" );
  
  let baseline_ops = baseline.operations_per_second;
  let miss_improvement = interner_miss.operations_per_second / baseline_ops;
  let hit_improvement = interner_hit.operations_per_second / baseline_ops;
  let global_improvement = global_interner.operations_per_second / baseline_ops;
  
  println!( "Improvement vs String Construction:" );
  println!( "- Cache Miss: {:.1}x faster ({:.0} vs {:.0} ops/sec)", 
           miss_improvement, interner_miss.operations_per_second, baseline_ops );
  println!( "- Cache Hit: {:.1}x faster ({:.0} vs {:.0} ops/sec)",
           hit_improvement, interner_hit.operations_per_second, baseline_ops );
  println!( "- Global Interner: {:.1}x faster ({:.0} vs {:.0} ops/sec)",
           global_improvement, global_interner.operations_per_second, baseline_ops );
  println!();
  
  let alloc_reduction = ( ( baseline.memory_allocations - interner_hit.memory_allocations ) as f64 
                         / baseline.memory_allocations as f64 ) * 100.0;
  println!( "Memory Allocation Reduction (Cache Hit): {:.0}%", alloc_reduction );
  
  // Success criteria validation
  let target_met = hit_improvement >= 5.0;
  println!();
  println!( "✅ Target Validation (5x minimum improvement): {}", 
           if target_met { "PASSED" } else { "FAILED" } );
  
  if hit_improvement >= 10.0
  {
    println!( "🎉 Exceeded stretch goal of 10x improvement!" );
  }
  
  // Latency analysis
  println!();
  println!( "Latency Analysis:" );
  println!( "- Baseline P99: {} ns", baseline.p99_time_ns );
  println!( "- Interner P99: {} ns", interner_hit.p99_time_ns );
  let target_latency_met = interner_hit.p99_time_ns <= 500_000; // 500μs
  println!( "- P99 under 500μs target: {}", if target_latency_met { "PASSED" } else { "FAILED" } );
}

#[ cfg( feature = "benchmarks" ) ]
fn main()
{
  run_string_interning_benchmarks();
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  println!( "String interning benchmarks require the 'benchmarks' feature flag." );
  println!( "Run with: cargo run --bin string_interning_benchmark --features benchmarks" );
}