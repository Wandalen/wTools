//! SIMD vs Scalar performance comparison benchmarks.
//!
//! This benchmark suite compares SIMD-optimized string operations against
//! their scalar equivalents to validate performance improvements.

use criterion::{ black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput };
use strs_tools::string::split;

#[ cfg( feature = "simd" ) ]
use strs_tools::simd::SIMDStringExt;

/// Generate test data with controlled delimiter density.
fn generate_test_data( size: usize, delimiters: &[ &str ], density: f32 ) -> String 
{
  let mut result = String::with_capacity( size );
  let mut rng_state = 12345u64; // Simple LCG for reproducible results
  
  while result.len() < size 
  {
    // Simple pseudo-random choice
    rng_state = rng_state.wrapping_mul( 1103515245 ).wrapping_add( 12345 );
    let choice = ( rng_state >> 16 ) % 100;
    
    if choice < ( density * 100.0 ) as u64 && !delimiters.is_empty() 
    {
      let delimiter_idx = ( rng_state >> 8 ) % delimiters.len() as u64;
      result.push_str( delimiters[ delimiter_idx as usize ] );
    } 
    else 
    {
      let word_len = ( ( rng_state >> 4 ) % 8 ) + 3; // 3-10 char words
      for i in 0..word_len 
      {
        if result.len() >= size { break; }
        let ch = ( ( 'a' as u8 ) + ( ( rng_state + i ) % 26 ) as u8 ) as char;
        result.push( ch );
        rng_state = rng_state.wrapping_mul( 1103515245 ).wrapping_add( 12345 );
      }
    }
  }
  
  result.truncate( size );
  result
}

/// Benchmark scalar vs SIMD string splitting performance.
fn bench_split_comparison( c: &mut Criterion ) 
{
  let mut group = c.benchmark_group( "split_comparison" );
  
  let test_cases = [
    ( "single_delimiter", vec![ ":" ], 1000 ),
    ( "multi_delimiter_small", vec![ ":", "," ], 1000 ),
    ( "multi_delimiter_medium", vec![ ":", ",", ";", ".", "!" ], 1000 ),
    ( "multi_delimiter_large", vec![ ":", ",", ";", ".", "!", "?", "#", "@", "&", "%" ], 1000 ),
  ];
  
  let sizes = [ 1000, 10000, 100000 ];
  
  for ( name, delimiters, _base_size ) in test_cases 
  {
    for &size in &sizes 
    {
      let test_data = generate_test_data( size, &delimiters, 0.15 ); // 15% delimiter density
      let bench_name = format!( "{}_{}", name, size );
      
      group.throughput( Throughput::Bytes( test_data.len() as u64 ) );
      
      // Scalar implementation benchmark
      group.bench_with_input(
        BenchmarkId::new( "scalar", &bench_name ),
        &test_data,
        |b, data| 
        {
          b.iter( || 
          {
            let result: Vec< _ > = split()
              .src( black_box( data ) )
              .delimeter( delimiters.clone() )
              .perform()
              .collect();
            black_box( result )
          } );
        },
      );
      
      // SIMD implementation benchmark
      #[ cfg( feature = "simd" ) ]
      group.bench_with_input(
        BenchmarkId::new( "simd", &bench_name ),
        &test_data,
        |b, data| 
        {
          b.iter( || 
          {
            match data.simd_split( &delimiters ) 
            {
              Ok( iter ) => 
              {
                let result: Vec< _ > = iter.collect();
                black_box( result )
              },
              Err( _ ) => 
              {
                // Fallback to scalar if SIMD fails
                let result: Vec< _ > = split()
                  .src( black_box( data ) )
                  .delimeter( delimiters.clone() )
                  .perform()
                  .collect();
                black_box( result )
              }
            }
          } );
        },
      );
    }
  }
  
  group.finish();
}

/// Benchmark scalar vs SIMD substring search performance.
fn bench_search_comparison( c: &mut Criterion ) 
{
  let mut group = c.benchmark_group( "search_comparison" );
  
  let sizes = [ 1000, 10000, 100000 ];
  let needles = [ "pattern", "xyz", "target", "needle" ];
  
  for &size in &sizes 
  {
    let test_data = generate_test_data( size, &[], 0.0 ); // No delimiters, just text
    
    group.throughput( Throughput::Bytes( test_data.len() as u64 ) );
    
    for needle in &needles 
    {
      let bench_name = format!( "{}_{}", needle, size );
      
      // Scalar search
      group.bench_with_input(
        BenchmarkId::new( "scalar", &bench_name ),
        &( &test_data, needle ),
        |b, ( data, needle )| 
        {
          b.iter( || 
          {
            let result = data.find( *black_box( needle ) );
            black_box( result )
          } );
        },
      );
      
      // SIMD search
      group.bench_with_input(
        BenchmarkId::new( "simd", &bench_name ),
        &( &test_data, needle ),
        |b, ( data, needle )| 
        {
          b.iter( || 
          {
            let result = data.simd_find( black_box( needle ) );
            black_box( result )
          } );
        },
      );
    }
  }
  
  group.finish();
}

/// Benchmark scalar vs SIMD character counting performance.
fn bench_count_comparison( c: &mut Criterion ) 
{
  let mut group = c.benchmark_group( "count_comparison" );
  
  let sizes = [ 1000, 10000, 100000 ];
  let chars = [ 'a', 'x', ':', ',' ];
  
  for &size in &sizes 
  {
    let test_data = generate_test_data( size, &[ ":", "," ], 0.1 ); // 10% delimiter density
    
    group.throughput( Throughput::Bytes( test_data.len() as u64 ) );
    
    for ch in &chars 
    {
      let bench_name = format!( "{}_{}", ch, size );
      
      // Scalar counting
      group.bench_with_input(
        BenchmarkId::new( "scalar", &bench_name ),
        &( &test_data, ch ),
        |b, ( data, ch )| 
        {
          b.iter( || 
          {
            let result = data.chars().filter( |&c| c == **black_box( ch ) ).count();
            black_box( result )
          } );
        },
      );
      
      // SIMD counting
      group.bench_with_input(
        BenchmarkId::new( "simd", &bench_name ),
        &( &test_data, ch ),
        |b, ( data, ch )| 
        {
          b.iter( || 
          {
            let result = data.simd_count( **black_box( ch ) );
            black_box( result )
          } );
        },
      );
    }
  }
  
  group.finish();
}

/// Benchmark multi-pattern search performance.
fn bench_multi_pattern_search( c: &mut Criterion ) 
{
  let mut group = c.benchmark_group( "multi_pattern_search" );
  
  let sizes = [ 1000, 10000, 100000 ];
  let pattern_sets = [
    vec![ "error", "warning", "info" ],
    vec![ "http", "https", "ftp", "ssh", "tcp" ],
    vec![ "class", "struct", "enum", "trait", "impl", "fn", "let", "const" ],
  ];
  
  for &size in &sizes 
  {
    let test_data = generate_test_data( size, &[], 0.0 );
    
    group.throughput( Throughput::Bytes( test_data.len() as u64 ) );
    
    for ( idx, patterns ) in pattern_sets.iter().enumerate() 
    {
      let bench_name = format!( "patterns_{}_{}", idx, size );
      
      // Scalar multi-pattern search (find first occurrence of any pattern)
      group.bench_with_input(
        BenchmarkId::new( "scalar", &bench_name ),
        &( &test_data, patterns ),
        |b, ( data, patterns )| 
        {
          b.iter( || 
          {
            let mut earliest_pos = data.len();
            let mut found = false;
            
            for pattern in black_box( patterns ).iter() 
            {
              if let Some( pos ) = data.find( pattern ) 
              {
                if pos < earliest_pos 
                {
                  earliest_pos = pos;
                  found = true;
                }
              }
            }
            
            black_box( if found { Some( earliest_pos ) } else { None } )
          } );
        },
      );
      
      // SIMD multi-pattern search
      group.bench_with_input(
        BenchmarkId::new( "simd", &bench_name ),
        &( &test_data, patterns ),
        |b, ( data, patterns )| 
        {
          b.iter( || 
          {
            let patterns_slice: Vec< &str > = patterns.iter().map( |s| s.as_ref() ).collect();
            let result = data.simd_find_any( black_box( &patterns_slice ) );
            black_box( result )
          } );
        },
      );
    }
  }
  
  group.finish();
}

criterion_group!(
  simd_benches,
  bench_split_comparison,
  bench_search_comparison,
  bench_count_comparison,
  bench_multi_pattern_search
);
criterion_main!( simd_benches );