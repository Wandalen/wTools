use criterion::{ black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput };
use strs_tools::string::split;

/// Generate test data for memory benchmarks
fn generate_memory_test_data( size : usize, pattern_density : f32 ) -> String
{
  let mut result = String::with_capacity( size );
  let words = [ "memory", "allocation", "benchmark", "testing", "performance", "optimization" ];
  let separators = [ ":", ".", " ", ",", "::" ];
  
  let mut rng_state = 54321u64;
  
  while result.len() < size
  {
    let word_idx = ( rng_state % words.len() as u64 ) as usize;
    result.push_str( words[ word_idx ] );
    rng_state = rng_state.wrapping_mul( 1103515245 ).wrapping_add( 12345 );
    
    if ( rng_state as f32 / u64::MAX as f32 ) < pattern_density
    {
      let sep_idx = ( rng_state % separators.len() as u64 ) as usize;
      result.push_str( separators[ sep_idx ] );
      rng_state = rng_state.wrapping_mul( 1103515245 ).wrapping_add( 12345 );
    }
  }
  
  result.truncate( size );
  result
}

/// Benchmark memory allocation patterns during string splitting
fn bench_split_memory_usage( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "split_memory_usage" );
  
  let test_sizes = [ 1_000, 10_000, 100_000 ];
  let delimiter_sets = [
    vec![ ":" ],
    vec![ ":", ".", " " ],
    vec![ ":", ".", " ", ",", "::", "->", "=>" ],
  ];
  
  for &size in &test_sizes
  {
    for ( _i, delimiters ) in delimiter_sets.iter().enumerate()
    {
      let input = generate_memory_test_data( size, 0.3 );
      group.throughput( Throughput::Bytes( input.len() as u64 ) );
      
      group.bench_with_input(
        BenchmarkId::new( format!( "size_{}_delims_{}", size, delimiters.len() ), size ),
        &input,
        |b, input|
        {
          b.iter( ||
          {
            let result : Vec< _ > = split()
              .src( black_box( input ) )
              .delimeter( delimiters.clone() )
              .perform()
              .collect();
            black_box( result )
          } );
        }
      );
    }
  }
  
  group.finish();
}

/// Benchmark iterator memory efficiency - collect vs fold patterns
fn bench_iterator_memory_patterns( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "iterator_memory_patterns" );
  
  let sizes = [ 10_000, 100_000 ];
  
  for &size in &sizes
  {
    let input = generate_memory_test_data( size, 0.4 );
    let delimiters = vec![ ":", ".", " ", "," ];
    
    group.throughput( Throughput::Bytes( input.len() as u64 ) );
    
    // Test collect() vs fold() patterns
    group.bench_with_input(
      BenchmarkId::new( "collect_all", size ),
      &input,
      |b, input|
      {
        b.iter( ||
        {
          let result : Vec< _ > = split()
            .src( black_box( input ) )
            .delimeter( delimiters.clone() )
            .perform()
            .collect();
          black_box( result )
        } );
      }
    );
    
    group.bench_with_input(
      BenchmarkId::new( "fold_count", size ),
      &input,
      |b, input|
      {
        b.iter( ||
        {
          let count = split()
            .src( black_box( input ) )
            .delimeter( delimiters.clone() )
            .perform()
            .fold( 0, |acc, _| acc + 1 );
          black_box( count )
        } );
      }
    );
  }
  
  group.finish();
}

/// Benchmark pattern compilation overhead
fn bench_pattern_compilation_memory( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "pattern_compilation_memory" );
  
  let pattern_counts = [ 1, 5, 10, 25, 50 ];
  
  for &count in &pattern_counts
  {
    let patterns : Vec< String > = ( 0..count )
      .map( |i| format!( "pattern_{}", i ) )
      .collect();
    let pattern_refs : Vec< &str > = patterns.iter().map( |s| s.as_str() ).collect();
    
    group.bench_with_input(
      BenchmarkId::new( "pattern_compilation", count ),
      &pattern_refs,
      |b, patterns|
      {
        b.iter( ||
        {
          let result : Vec< _ > = split()
            .src( "test input for memory measurement" )
            .delimeter( black_box( patterns.clone() ) )
            .perform()
            .collect();
          black_box( result )
        } );
      }
    );
  }
  
  group.finish();
}

/// Benchmark performance under different input patterns
fn bench_input_pattern_memory( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "input_pattern_memory" );
  
  let size = 50_000;
  let delimiters = vec![ ":", ".", " " ];
  
  // Different delimiter densities affect performance patterns
  let densities = [ 0.1, 0.3, 0.5, 0.7 ];
  
  for &density in &densities
  {
    let input = generate_memory_test_data( size, density );
    group.throughput( Throughput::Bytes( input.len() as u64 ) );
    
    group.bench_with_input(
      BenchmarkId::new( "delimiter_density", ( density * 100.0 ) as u32 ),
      &input,
      |b, input|
      {
        b.iter( ||
        {
          let result : Vec< _ > = split()
            .src( black_box( input ) )
            .delimeter( delimiters.clone() )
            .perform()
            .collect();
          black_box( result )
        } );
      }
    );
  }
  
  group.finish();
}

criterion_group!(
  benches,
  bench_split_memory_usage,
  bench_iterator_memory_patterns,
  bench_pattern_compilation_memory,
  bench_input_pattern_memory
);

criterion_main!( benches );