//! Benchmark comparing compile-time optimizations vs runtime optimizations
//!
//! This benchmark measures the performance impact of compile-time pattern analysis
//! and optimization compared to runtime decision-making.

#![ allow( missing_docs ) ]

use criterion::{ black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput };
use std::time::Instant;

use strs_tools::string::split;
use strs_tools::string::zero_copy::ZeroCopyStringExt;

#[ cfg( feature = "compile_time_optimizations" ) ]
use strs_tools::{ optimize_split, optimize_match };

/// Generate test data for benchmarking
fn generate_benchmark_data( size: usize, pattern: &str ) -> String {
  match pattern {
    "csv" => "field1,field2,field3,field4,field5,field6,field7,field8".repeat( size / 50 + 1 ),
    "structured" => "key1:value1;key2:value2,key3:value3|key4:value4".repeat( size / 60 + 1 ),
    "urls" => "https://example.com,http://test.org,ftp://files.net".repeat( size / 50 + 1 ),
    _ => "a,b,c".repeat( size / 5 + 1 ),
  }
}

/// Benchmark single delimiter splitting
fn bench_single_delimiter_split( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "single_delimiter_split" );
  
  let test_cases = [
    ( "small_1kb", 1024 ),
    ( "medium_10kb", 10240 ),
    ( "large_100kb", 102400 ),
  ];
  
  for ( name, size ) in test_cases {
    let csv_data = generate_benchmark_data( size, "csv" );
    group.throughput( Throughput::Bytes( csv_data.len() as u64 ) );
    
    // Runtime optimization (standard library split)
    group.bench_with_input(
      BenchmarkId::new( "stdlib_split", name ),
      &csv_data,
      |b, data| {
        b.iter( || {
          let result: Vec< &str > = data.split( ',' ).collect();
          black_box( result )
        } );
      },
    );
    
    // Runtime optimization (zero-copy)
    group.bench_with_input(
      BenchmarkId::new( "zero_copy_runtime", name ),
      &csv_data,
      |b, data| {
        b.iter( || {
          let result: Vec< _ > = data.zero_copy_split( &[","] ).collect();
          black_box( result )
        } );
      },
    );
    
    // Compile-time optimization
    #[ cfg( feature = "compile_time_optimizations" ) ]
    group.bench_with_input(
      BenchmarkId::new( "compile_time_optimized", name ),
      &csv_data,
      |b, data| {
        b.iter( || {
          let result: Vec< _ > = optimize_split!( black_box( data ), "," ).collect();
          black_box( result )
        } );
      },
    );
  }
  
  group.finish();
}

/// Benchmark multiple delimiter splitting
fn bench_multiple_delimiter_split( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "multiple_delimiter_split" );
  
  let test_cases = [
    ( "small_1kb", 1024 ),
    ( "medium_10kb", 10240 ),
    ( "large_100kb", 102400 ),
  ];
  
  for ( name, size ) in test_cases {
    let structured_data = generate_benchmark_data( size, "structured" );
    group.throughput( Throughput::Bytes( structured_data.len() as u64 ) );
    
    // Runtime optimization (traditional)
    group.bench_with_input(
      BenchmarkId::new( "traditional_runtime", name ),
      &structured_data,
      |b, data| {
        b.iter( || {
          let result: Vec< String > = split()
            .src( black_box( data ) )
            .delimeter( vec![ ":", ";", ",", "|" ] )
            .perform()
            .map( |split| split.string.into_owned() )
            .collect();
          black_box( result )
        } );
      },
    );
    
    // Runtime optimization (zero-copy)
    group.bench_with_input(
      BenchmarkId::new( "zero_copy_runtime", name ),
      &structured_data,
      |b, data| {
        b.iter( || {
          let result: Vec< _ > = data.zero_copy_split( &[":", ";", ",", "|"] ).collect();
          black_box( result )
        } );
      },
    );
    
    // Compile-time optimization
    #[ cfg( feature = "compile_time_optimizations" ) ]
    group.bench_with_input(
      BenchmarkId::new( "compile_time_optimized", name ),
      &structured_data,
      |b, data| {
        b.iter( || {
          let result: Vec< _ > = optimize_split!( 
            black_box( data ), 
            [":", ";", ",", "|"]
          ).collect();
          black_box( result )
        } );
      },
    );
  }
  
  group.finish();
}

/// Benchmark pattern matching
fn bench_pattern_matching( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "pattern_matching" );
  
  let url_data = generate_benchmark_data( 50000, "urls" );
  group.throughput( Throughput::Bytes( url_data.len() as u64 ) );
  
  // Runtime pattern matching
  group.bench_function( "runtime_pattern_matching", |b| {
    b.iter( || {
      let mut matches = Vec::new();
      let data = black_box( &url_data );
      
      if let Some( pos ) = data.find( "https://" ) {
        matches.push( pos );
      }
      if let Some( pos ) = data.find( "http://" ) {
        matches.push( pos );
      }
      if let Some( pos ) = data.find( "ftp://" ) {
        matches.push( pos );
      }
      
      black_box( matches )
    } );
  } );
  
  // Compile-time optimized pattern matching
  #[ cfg( feature = "compile_time_optimizations" ) ]
  group.bench_function( "compile_time_pattern_matching", |b| {
    b.iter( || {
      let result = optimize_match!(
        black_box( &url_data ),
        ["https://", "http://", "ftp://"],
        strategy = "first_match"
      );
      black_box( result )
    } );
  } );
  
  group.finish();
}

/// Benchmark delimiter preservation
fn bench_delimiter_preservation( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "delimiter_preservation" );
  
  let test_data = "key1:value1;key2:value2,key3:value3".repeat( 500 );
  group.throughput( Throughput::Bytes( test_data.len() as u64 ) );
  
  // Runtime delimiter preservation
  group.bench_function( "runtime_preserve_delimiters", |b| {
    b.iter( || {
      let result: Vec< _ > = test_data.zero_copy_split_preserve( &[":", ";", ","] ).collect();
      black_box( result )
    } );
  } );
  
  // Compile-time optimized delimiter preservation
  #[ cfg( feature = "compile_time_optimizations" ) ]
  group.bench_function( "compile_time_preserve_delimiters", |b| {
    b.iter( || {
      let result: Vec< _ > = optimize_split!( 
        &test_data, 
        [":", ";", ","],
        preserve_delimiters = true
      ).collect();
      black_box( result )
    } );
  } );
  
  group.finish();
}

/// Benchmark counting operations (no allocation)
fn bench_counting_operations( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "counting_operations" );
  
  let large_data = "item1,item2,item3,item4,item5".repeat( 10000 );
  group.throughput( Throughput::Bytes( large_data.len() as u64 ) );
  
  // Runtime counting
  group.bench_function( "runtime_count", |b| {
    b.iter( || {
      let count = large_data.count_segments( &[","] );
      black_box( count )
    } );
  } );
  
  // Compile-time optimized counting  
  #[ cfg( feature = "compile_time_optimizations" ) ]
  group.bench_function( "compile_time_count", |b| {
    b.iter( || {
      let count = optimize_split!( &large_data, "," ).count();
      black_box( count )
    } );
  } );
  
  group.finish();
}

/// Memory usage comparison benchmark
fn bench_memory_usage_patterns( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "memory_usage_patterns" );
  group.sample_size( 20 );
  
  let test_data = generate_benchmark_data( 100000, "csv" );
  group.throughput( Throughput::Bytes( test_data.len() as u64 ) );
  
  // Runtime memory pattern
  group.bench_function( "runtime_memory_pattern", |b| {
    b.iter_custom( |iters| {
      let start_time = Instant::now();
      
      for _ in 0..iters {
        let result: Vec< _ > = test_data.zero_copy_split( &[","] ).collect();
        black_box( result );
      }
      
      start_time.elapsed()
    } );
  } );
  
  // Compile-time optimized memory pattern
  #[ cfg( feature = "compile_time_optimizations" ) ]
  group.bench_function( "compile_time_memory_pattern", |b| {
    b.iter_custom( |iters| {
      let start_time = Instant::now();
      
      for _ in 0..iters {
        let result: Vec< _ > = optimize_split!( &test_data, "," ).collect();
        black_box( result );
      }
      
      start_time.elapsed()
    } );
  } );
  
  group.finish();
}

/// Complex pattern optimization benchmark
#[ cfg( feature = "compile_time_optimizations" ) ]
fn bench_complex_pattern_optimization( c: &mut Criterion ) {
  let mut group = c.benchmark_group( "complex_pattern_optimization" );
  
  let complex_data = "prefix1::item1->value1|prefix2::item2->value2|prefix3::item3->value3".repeat( 1000 );
  group.throughput( Throughput::Bytes( complex_data.len() as u64 ) );
  
  // Runtime complex pattern handling
  group.bench_function( "runtime_complex_patterns", |b| {
    b.iter( || {
      let result: Vec< _ > = complex_data.zero_copy_split( &["::", "->", "|"] ).collect();
      black_box( result )
    } );
  } );
  
  // Compile-time optimized complex patterns
  group.bench_function( "compile_time_complex_patterns", |b| {
    b.iter( || {
      let result: Vec< _ > = optimize_split!( 
        &complex_data, 
        ["::", "->", "|"],
        use_simd = true
      ).collect();
      black_box( result )
    } );
  } );
  
  group.finish();
}

criterion_group!(
  compile_time_benches,
  bench_single_delimiter_split,
  bench_multiple_delimiter_split, 
  bench_pattern_matching,
  bench_delimiter_preservation,
  bench_counting_operations,
  bench_memory_usage_patterns,
);

#[ cfg( feature = "compile_time_optimizations" ) ]
criterion_group!(
  compile_time_advanced_benches,
  bench_complex_pattern_optimization,
);

#[ cfg( feature = "compile_time_optimizations" ) ]
criterion_main!( compile_time_benches, compile_time_advanced_benches );

#[ cfg( not( feature = "compile_time_optimizations" ) ) ]
criterion_main!( compile_time_benches );