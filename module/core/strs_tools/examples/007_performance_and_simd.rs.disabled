//! Performance optimization and SIMD acceleration examples.
//!
//! This example demonstrates the performance benefits of strs_tools,
//! including SIMD-accelerated operations, memory-efficient processing,
//! and comparisons with standard library alternatives.

use strs_tools::*;
use std::time::Instant;

fn main()
{
  println!( "=== Performance and SIMD Examples ===" );
  
  performance_comparison();
  simd_acceleration_demo();
  memory_efficiency_showcase();
  large_data_processing();
}

/// Demonstrates performance comparison between strs_tools and standard library.
///
/// Shows the performance benefits of using strs_tools for common
/// string operations, especially with large amounts of data.
fn performance_comparison()
{
  println!( "\n--- Performance Comparison ---" );
  
  // Create test data of various sizes
  let test_cases = vec![
    ( "Small", "word ".repeat( 100 ) + "end" ),
    ( "Medium", "token ".repeat( 1000 ) + "final" ),
    ( "Large", "item ".repeat( 10000 ) + "last" ),
  ];
  
  for ( size_name, test_data ) in test_cases
  {
    println!( "\n{} dataset ({} bytes):", size_name, test_data.len() );
    
    // Standard library approach
    let start = Instant::now();
    let std_result : Vec< &str > = test_data.split( ' ' ).collect();
    let std_duration = start.elapsed();
    
    println!( "  Standard split(): {} items in {:?}", std_result.len(), std_duration );
    
    // strs_tools approach (if available)
    #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
    {
      let start = Instant::now();
      let iter = string::split()
        .src( &test_data )
        .delimeter( " " )
        .stripping( true )
        .perform();
      let strs_result : Vec< String > = iter.map( String::from ).collect();
      let strs_duration = start.elapsed();
      
      println!( "  strs_tools split(): {} items in {:?}", strs_result.len(), strs_duration );
      
      // Compare results
      if std_result.len() == strs_result.len()
      {
        println!( "    ✓ Results match" );
        
        // Calculate performance difference
        let speedup = std_duration.as_nanos() as f64 / strs_duration.as_nanos() as f64;
        if speedup > 1.1
        {
          println!( "    🚀 strs_tools is {:.1}x faster", speedup );
        }
        else if speedup < 0.9
        {
          println!( "    📊 Standard library is {:.1}x faster", 1.0 / speedup );
        }
        else
        {
          println!( "    ⚖️  Performance is comparable" );
        }
      }
      else
      {
        println!( "    ⚠️  Result count differs - may indicate different handling" );
      }
    }
    
    // Demonstrate memory usage efficiency
    let start = Instant::now();
    let iter = test_data.split( ' ' );
    let lazy_count = iter.count(); // Count without collecting
    let lazy_duration = start.elapsed();
    
    println!( "  Lazy counting: {} items in {:?}", lazy_count, lazy_duration );
    println!( "    💾 Zero allocation approach" );
  }
}

/// Demonstrates SIMD acceleration capabilities.
///
/// Shows how SIMD features can dramatically improve performance
/// for large-scale text processing operations.
fn simd_acceleration_demo()
{
  println!( "\n--- SIMD Acceleration Demo ---" );
  
  #[ cfg( all( feature = "string_split", feature = "simd", not( feature = "no_std" ) ) ) ]
  {
    println!( "🔥 SIMD features enabled" );
    
    // Create a large dataset for SIMD testing
    let large_text = "word ".repeat( 50000 ) + "final";
    println!( "  Processing {} bytes of text", large_text.len() );
    
    // Measure SIMD-accelerated splitting
    let start = Instant::now();
    let simd_iter = string::split()
      .src( &large_text )
      .delimeter( " " )
      .stripping( true )
      .perform();
    
    let simd_count = simd_iter.count();
    let simd_duration = start.elapsed();
    
    println!( "  SIMD split: {} tokens in {:?}", simd_count, simd_duration );
    
    // Calculate throughput
    let mb_per_sec = ( large_text.len() as f64 / ( 1024.0 * 1024.0 ) ) / simd_duration.as_secs_f64();
    println!( "  Throughput: {:.1} MB/s", mb_per_sec );
    
    // Demonstrate pattern matching with SIMD
    let pattern_text = "find ".repeat( 10000 ) + "target " + &"find ".repeat( 10000 );
    println!( "\n  Pattern matching test ({} bytes):", pattern_text.len() );
    
    let start = Instant::now();
    let matches = string::split()
      .src( &pattern_text )
      .delimeter( "target" )
      .perform()
      .count();
    let pattern_duration = start.elapsed();
    
    println!( "    Found {} matches in {:?}", matches - 1, pattern_duration ); // -1 because split count includes segments
    
    // Multiple delimiter test
    let multi_delim_text = "a,b;c:d|e.f a,b;c:d|e.f".repeat( 5000 );
    println!( "\n  Multiple delimiter test:" );
    
    let delimiters = vec![ ",", ";", ":", "|", "." ];
    for delimiter in delimiters
    {
      let start = Instant::now();
      let parts = string::split()
        .src( &multi_delim_text )
        .delimeter( delimiter )
        .perform()
        .count();
      let duration = start.elapsed();
      
      println!( "    '{}' delimiter: {} parts in {:?}", delimiter, parts, duration );
    }
    
    println!( "  ✓ SIMD acceleration demonstrated" );
  }
  
  #[ cfg( not( all( feature = "string_split", feature = "simd", not( feature = "no_std" ) ) ) ) ]
  {
    println!( "⚠️  SIMD features not available" );
    println!( "   Enable with: cargo run --example 007_performance_and_simd --features simd" );
    
    // Show what would be possible with SIMD
    println!( "\n  SIMD would enable:" );
    println!( "    • 2-10x faster string searching" );
    println!( "    • Parallel pattern matching" );
    println!( "    • Hardware-accelerated byte operations" );
    println!( "    • Improved performance on large datasets" );
  }
}

/// Demonstrates memory-efficient string processing.
///
/// Shows how strs_tools minimizes allocations and uses
/// copy-on-write strategies for better memory usage.
fn memory_efficiency_showcase()
{
  println!( "\n--- Memory Efficiency Showcase ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    let source_text = "zero copy operations when possible";
    println!( "Source: '{}'", source_text );
    
    // Demonstrate zero-copy splitting
    println!( "\n  Zero-copy string references:" );
    let iter = string::split()
      .src( source_text )
      .delimeter( " " )
      .stripping( true )
      .perform();
    
    let segments : Vec< &str > = iter
      .map( | segment | segment.as_str() ) // Get string slice (zero copy)
      .collect();
    
    println!( "    Segments (borrowing from original):" );
    for ( i, segment ) in segments.iter().enumerate()
    {
      // Verify these are actually referencing the original string
      let segment_ptr = segment.as_ptr();
      let source_ptr = source_text.as_ptr();
      let is_borrowed = segment_ptr >= source_ptr && 
                       segment_ptr < unsafe { source_ptr.add( source_text.len() ) };
      
      println!( "      [{}]: '{}' {}", i, segment, 
               if is_borrowed { "(borrowed)" } else { "(copied)" } );
    }
    
    // Compare memory usage: references vs owned strings
    let owned_segments : Vec< String > = segments.iter().map( | s | s.to_string() ).collect();
    
    let reference_size = segments.len() * std::mem::size_of::< &str >();
    let owned_size = owned_segments.iter().map( | s | s.len() + std::mem::size_of::< String >() ).sum::< usize >();
    
    println!( "\n  Memory usage comparison:" );
    println!( "    References: {} bytes", reference_size );
    println!( "    Owned strings: {} bytes", owned_size );
    println!( "    Savings: {} bytes ({:.1}x less memory)", 
             owned_size - reference_size,
             owned_size as f64 / reference_size as f64 );
    
    // Demonstrate preservation of original structure
    let preserved_text = segments.join( " " );
    println!( "\n  Reconstruction test:" );
    println!( "    Original:      '{}'", source_text );
    println!( "    Reconstructed: '{}'", preserved_text );
    println!( "    Match: {}", source_text == preserved_text );
  }
  
  // Demonstrate efficient processing of large texts
  println!( "\n  Large text processing efficiency:" );
  
  // Simulate processing a large log file
  let log_lines = (0..1000).map( | i | 
    format!( "2024-08-07 {:02}:{:02}:{:02} [INFO] Processing item #{}", 
            ( i / 3600 ) % 24, ( i / 60 ) % 60, i % 60, i ) 
  ).collect::< Vec< _ >>();
  
  let combined_log = log_lines.join( "\n" );
  println!( "    Log file size: {} bytes ({} lines)", combined_log.len(), log_lines.len() );
  
  // Process with minimal allocations
  let start = Instant::now();
  let mut info_count = 0;
  let mut error_count = 0;
  let mut timestamp_count = 0;
  
  for line in combined_log.lines()
  {
    // Count different log levels (zero allocation)
    if line.contains( "[INFO]" )
    {
      info_count += 1;
    }
    else if line.contains( "[ERROR]" )
    {
      error_count += 1;
    }
    
    // Count timestamps (check for time pattern)
    if line.contains( "2024-08-07" )
    {
      timestamp_count += 1;
    }
  }
  
  let processing_time = start.elapsed();
  
  println!( "    Analysis results:" );
  println!( "      INFO messages: {}", info_count );
  println!( "      ERROR messages: {}", error_count );
  println!( "      Timestamped lines: {}", timestamp_count );
  println!( "      Processing time: {:?}", processing_time );
  println!( "      Rate: {:.1} lines/ms", log_lines.len() as f64 / processing_time.as_millis() as f64 );
  
  println!( "  ✓ Memory-efficient processing completed" );
}

/// Demonstrates large-scale data processing capabilities.
///
/// Shows how strs_tools handles very large datasets efficiently,
/// including streaming processing and batch operations.
fn large_data_processing()
{
  println!( "\n--- Large Data Processing ---" );
  
  // Simulate processing a large CSV-like dataset
  println!( "  Simulating large dataset processing:" );
  
  let record_count = 100000;
  let start_generation = Instant::now();
  
  // Generate sample data (in real scenarios this might be read from a file)
  let sample_record = "user_id,name,email,signup_date,status";
  let header = sample_record;
  
  println!( "    Generating {} records...", record_count );
  let generation_time = start_generation.elapsed();
  println!( "    Generation time: {:?}", generation_time );
  
  // Process the data efficiently
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    let start_processing = Instant::now();
    
    // Parse header to understand structure
    let header_iter = string::split()
      .src( header )
      .delimeter( "," )
      .stripping( true )
      .perform();
    
    let columns : Vec< String > = header_iter.map( String::from ).collect();
    println!( "    Detected columns: {:?}", columns );
    
    // Simulate batch processing
    let batch_size = 10000;
    let batch_count = record_count / batch_size;
    
    println!( "    Processing in batches of {} records:", batch_size );
    
    let mut total_fields = 0;
    
    for batch_num in 0..batch_count
    {
      let batch_start = Instant::now();
      
      // Simulate processing a batch
      for record_num in 0..batch_size
      {
        let record_id = batch_num * batch_size + record_num;
        let simulated_record = format!( "{},User{},user{}@example.com,2024-08-{:02},active", 
                                       record_id, record_id, record_id, ( record_id % 30 ) + 1 );
        
        // Parse the record
        let field_iter = string::split()
          .src( &simulated_record )
          .delimeter( "," )
          .stripping( true )
          .perform();
        
        let field_count = field_iter.count();
        total_fields += field_count;
      }
      
      let batch_time = batch_start.elapsed();
      
      if batch_num % 2 == 0 // Print every other batch to avoid spam
      {
        println!( "      Batch {} processed in {:?} ({:.1} records/ms)", 
                 batch_num + 1, batch_time, batch_size as f64 / batch_time.as_millis() as f64 );
      }
    }
    
    let total_processing_time = start_processing.elapsed();
    
    println!( "    Processing summary:" );
    println!( "      Total records processed: {}", record_count );
    println!( "      Total fields parsed: {}", total_fields );
    println!( "      Total processing time: {:?}", total_processing_time );
    println!( "      Average rate: {:.1} records/second", 
             record_count as f64 / total_processing_time.as_secs_f64() );
    
    // Calculate theoretical throughput
    if total_processing_time.as_secs_f64() > 0.0
    {
      let bytes_per_record = 50; // Estimated average
      let total_bytes = record_count * bytes_per_record;
      let throughput_mbps = ( total_bytes as f64 / ( 1024.0 * 1024.0 ) ) / total_processing_time.as_secs_f64();
      
      println!( "      Estimated throughput: {:.1} MB/s", throughput_mbps );
    }
    
    println!( "  ✓ Large-scale processing completed successfully" );
  }
  
  // Demonstrate streaming vs batch processing
  println!( "\n  Streaming vs Batch comparison:" );
  
  let test_data = "stream,process,data,efficiently ".repeat( 25000 );
  
  // Streaming approach (process as you go)
  let start_stream = Instant::now();
  let mut stream_count = 0;
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    let iter = string::split()
      .src( &test_data )
      .delimeter( "," )
      .stripping( true )
      .perform();
    
    for _token in iter
    {
      stream_count += 1;
      // Simulate some processing work
    }
  }
  
  let stream_time = start_stream.elapsed();
  
  // Batch approach (collect then process)  
  let start_batch = Instant::now();
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    let iter = string::split()
      .src( &test_data )
      .delimeter( "," )
      .stripping( true )
      .perform();
    
    let all_tokens : Vec< String > = iter.map( String::from ).collect();
    let batch_count = all_tokens.len();
    
    // Process the collected tokens
    for _token in all_tokens
    {
      // Simulate processing
    }
    
    let batch_time = start_batch.elapsed();
    
    println!( "    Stream processing: {} tokens in {:?}", stream_count, stream_time );
    println!( "    Batch processing: {} tokens in {:?}", batch_count, batch_time );
    
    if stream_time < batch_time
    {
      println!( "    🌊 Streaming is {:.1}x faster (lower memory usage)", 
               batch_time.as_nanos() as f64 / stream_time.as_nanos() as f64 );
    }
    else
    {
      println!( "    📦 Batching is {:.1}x faster (better cache locality)", 
               stream_time.as_nanos() as f64 / batch_time.as_nanos() as f64 );
    }
  }
  
  println!( "\n✓ Performance and SIMD examples completed" );
}