use criterion::{ black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput };
use strs_tools::string::split;

// Test data generation constants
const TEST_SIZES : &[ usize ] = &[ 100, 1_000, 10_000, 100_000, 1_000_000 ];
const DELIMITER_COUNTS : &[ usize ] = &[ 1, 5, 10, 25, 50 ];

// Common delimiters used in real-world scenarios
const SINGLE_DELIMITERS : &[ &str ] = &[ " ", ",", ".", ":", ";", "\n" ];
const MULTI_DELIMITERS : &[ &[ &str ] ] = &[
  &[ ":", ".", "!" ],
  &[ " ", ",", ".", ":", ";" ],
  &[ "=", ":", ",", ".", ";", " ", "\t", "\n", "\"", "'" ],
  &[ "::", "->", "=>", "..", "//", "/*", "*/", "&&", "||", "==", "!=", "<=", ">=", "<<", ">>", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=" ],
  &[ "namespace", "command", "arg", "value", "option", "flag", "param", "config", "setting", "property", "attribute", "field", "member", "method", "function", "class", "struct", "enum", "trait", "impl", "use", "pub", "mod", "fn", "let", "const", "static", "mut", "ref", "move", "async", "await", "loop", "while", "for", "if", "else", "match", "return", "break", "continue", "unsafe", "extern", "crate", "type", "where", "super", "self", "Self" ]
];

/// Generate test data with specified size and delimiter density
fn generate_test_data( size : usize, delimiter_density : f32 ) -> String
{
  let mut result = String::with_capacity( size );
  let words = [ "namespace", "command", "arg", "value", "option", "flag", "param", "config" ];
  let delimiters = [ ":", ".", "!", " ", ",", ";", "\t" ];
  
  let mut rng_state = 12345u64; // Simple LCG for reproducible results
  
  while result.len() < size
  {
    // Add a word
    let word_idx = ( rng_state % words.len() as u64 ) as usize;
    result.push_str( words[ word_idx ] );
    rng_state = rng_state.wrapping_mul( 1103515245 ).wrapping_add( 12345 );
    
    // Maybe add delimiter based on density
    if ( rng_state as f32 / u64::MAX as f32 ) < delimiter_density
    {
      let delim_idx = ( rng_state % delimiters.len() as u64 ) as usize;
      result.push_str( delimiters[ delim_idx ] );
      rng_state = rng_state.wrapping_mul( 1103515245 ).wrapping_add( 12345 );
    }
  }
  
  result.truncate( size );
  result
}

/// Generate pattern sets for multi-pattern benchmarks
fn generate_patterns( count : usize ) -> Vec< &'static str >
{
  let all_patterns = [
    ":", ".", "!", " ", ",", ";", "\t", "\n", "=", "\"", "'", "(", ")", "[", "]", "{", "}", "<", ">",
    "::", "->", "=>", "..", "//", "/*", "*/", "&&", "||", "==", "!=", "<=", ">=", "<<", ">>",
    "namespace", "command", "arg", "value", "option", "flag", "param", "config", "setting"
  ];
  
  all_patterns.iter().take( count ).copied().collect()
}

/// Benchmark single delimiter split operations
fn bench_single_delimiter_split( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "single_delimiter_split" );
  
  for &size in TEST_SIZES
  {
    for &delimiter in SINGLE_DELIMITERS
    {
      let input = generate_test_data( size, 0.3 );
      group.throughput( Throughput::Bytes( input.len() as u64 ) );
      
      group.bench_with_input(
        BenchmarkId::new( format!( "size_{}_delim_{}", size, delimiter.escape_debug() ), size ),
        &input,
        |b, input|
        {
          b.iter( ||
          {
            let result : Vec< _ > = split()
              .src( black_box( input ) )
              .delimeter( vec![ delimiter ] )
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

/// Benchmark multi-delimiter split operations
fn bench_multi_delimiter_split( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "multi_delimiter_split" );
  
  for &size in TEST_SIZES
  {
    for ( _i, &delimiters ) in MULTI_DELIMITERS.iter().enumerate()
    {
      let input = generate_test_data( size, 0.4 );
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
              .delimeter( delimiters.to_vec() )
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

/// Benchmark substring search operations
fn bench_substring_search( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "substring_search" );
  
  let needles = [ "namespace", "command", "value", "::", "->", "/*", "config" ];
  
  for &size in TEST_SIZES
  {
    let input = generate_test_data( size, 0.3 );
    group.throughput( Throughput::Bytes( input.len() as u64 ) );
    
    for &needle in &needles
    {
      group.bench_with_input(
        BenchmarkId::new( format!( "size_{}_needle_{}", size, needle ), size ),
        &input,
        |b, input|
        {
          b.iter( ||
          {
            let result = black_box( input ).find( needle );
            black_box( result )
          } );
        }
      );
    }
  }
  
  group.finish();
}

/// Benchmark character counting operations
fn bench_character_counting( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "character_counting" );
  
  let chars = [ ' ', ':', '.', '!', ',', ';', '\t', '\n' ];
  
  for &size in TEST_SIZES
  {
    let input = generate_test_data( size, 0.4 );
    group.throughput( Throughput::Bytes( input.len() as u64 ) );
    
    for &ch in &chars
    {
      group.bench_with_input(
        BenchmarkId::new( format!( "size_{}_char_{}", size, ch.escape_debug() ), size ),
        &input,
        |b, input|
        {
          b.iter( ||
          {
            let result = black_box( input ).chars().filter( |&c| c == ch ).count();
            black_box( result )
          } );
        }
      );
    }
  }
  
  group.finish();
}

/// Benchmark pattern compilation overhead
fn bench_pattern_compilation( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "pattern_compilation" );
  
  for &count in DELIMITER_COUNTS
  {
    let patterns = generate_patterns( count );
    
    group.bench_with_input(
      BenchmarkId::new( "pattern_count", count ),
      &patterns,
      |b, patterns|
      {
        b.iter( ||
        {
          // Measure the cost of creating and using split builder with patterns
          let result : Vec< _ > = split()
            .src( "test input for pattern compilation overhead measurement" )
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

/// Benchmark realistic Unilang-style parsing workloads
fn bench_unilang_parsing( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "unilang_parsing" );
  
  // Typical Unilang command patterns
  let unilang_samples = [
    ".namespace.command arg1::value1 arg2::value2",
    ".system.process.spawn cmd::/bin/bash args::[\"-c\", \"echo test\"] env::{PATH:/usr/bin}",
    ".config.set database.url::postgresql://user:pass@localhost/db timeout::30s",
    ".build.rust.compile target::release features::[\"simd\", \"optimized\"] profile::production",
    ".network.http.request url::https://api.example.com/data method::GET headers::{\"Authorization\": \"Bearer token123\", \"Content-Type\": \"application/json\"}"
  ];
  
  let delimiters = vec![ ":", ".", "!", " ", "::" ];
  
  for ( i, &sample ) in unilang_samples.iter().enumerate()
  {
    // Create larger test data by repeating the sample
    let large_sample = sample.repeat( 1000 );
    group.throughput( Throughput::Bytes( large_sample.len() as u64 ) );
    
    group.bench_with_input(
      BenchmarkId::new( "unilang_pattern", i ),
      &large_sample,
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
  bench_single_delimiter_split,
  bench_multi_delimiter_split,
  bench_substring_search,
  bench_character_counting,
  bench_pattern_compilation,
  bench_unilang_parsing
);

criterion_main!( benches );