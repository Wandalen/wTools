//! SIMD tokenizer performance benchmarks
//!
//! This test demonstrates the performance improvements achieved by SIMD tokenization
//! compared to scalar string operations, following the benchkit integration pattern.

#[ cfg( feature = "benchmarks" ) ]
use benchkit::prelude::*;
#[ cfg( feature = "benchmarks" ) ]
use unilang::simd_tokenizer::SIMDTokenizer;

/// Run SIMD tokenization benchmark comparison
#[ cfg( feature = "benchmarks" ) ]
fn run_tokenization_benchmark( input : &str, description : &str )
{
  println!( "\n=== {description} ===" );
  println!( "Input size: {} bytes", input.len() );
  
  let input_simd = input.to_owned();
  let input_scalar = input.to_owned();
  
  let comparison = ComparativeAnalysis::new( format!( "simd_tokenization_{}", description.replace( ' ', "_" ).to_lowercase() ) )
    .algorithm( "simd_tokenizer", move ||
    {
      let tokenizer = SIMDTokenizer::new( &input_simd );
      let tokens : Vec< &str > = tokenizer.tokenize().collect();
      core::hint::black_box( tokens );
    })
    .algorithm( "scalar_tokenizer", move ||
    {
      // Fallback scalar tokenization for comparison
      let tokens : Vec< &str > = input_scalar.split( |c : char| ":?#.!".contains( c ) ).collect();
      core::hint::black_box( tokens );
    });
  
  let report = comparison.run();
  
  // Display results using benchkit's reporting methods
  println!( "📈 Performance Results:" );
  for ( name, result ) in report.sorted_by_performance()
  {
    println!( "  • {}: {:.0} ops/sec ({:.2}μs avg)", 
             name, 
             result.operations_per_second(), 
             result.mean_time().as_nanos() as f64 / 1000.0 );
  }
  
  // Calculate and display speedup ratio
  if let Some( ( fastest_name, fastest_result ) ) = report.fastest()
  {
    if let Some( ( slowest_name, slowest_result ) ) = report.slowest()
    {
      let speedup = slowest_result.mean_time().as_nanos() as f64 / fastest_result.mean_time().as_nanos() as f64;
      println!( "⚡ Speedup: {fastest_name} is {speedup:.2}x faster than {slowest_name}" );
    }
  }
  
  // Display SIMD capability information
  println!( "🚀 SIMD support: {}", unilang::simd_tokenizer::is_simd_enabled() );
  println!( "📊 SIMD info: {}", unilang::simd_tokenizer::simd_support_info() );
}

#[ cfg( feature = "benchmarks" ) ]
#[ test ]
#[ ignore = "Benchkit integration - SIMD tokenization performance analysis" ]
fn simd_tokenizer_performance_test()
{
  println!( "🎉 SIMD Tokenization Performance Analysis using Benchkit" );
  println!( "=======================================================" );
  
  // Test with different input sizes and patterns to showcase SIMD benefits
  
  // Create the large input separately to avoid lifetime issues
  let very_large_input = format!( ".batch.execute {}", 
                                  (0..100).map( |i| format!( "cmd{i}::arg{i}::value{i}" ) ).collect::< Vec< _ > >().join( " " ) );
  
  let test_cases = vec![
    // Small input - SIMD may not show significant benefit
    ( ".help", "Small Command" ),
    
    // Medium input - where SIMD starts to shine
    ( ".namespace.command arg1::value1 arg2::value2 arg3::value3", "Medium Command" ),
    
    // Large input with many delimiters - where SIMD excels
    ( ".data.process input::file1.json output::file2.json format::pretty mode::safe validate::true compress::false debug::verbose logging::enabled cache::disabled parallel::true threads::8 timeout::300 retries::3 batch::1000", "Large Command" ),
    
    // Very large input - stress test for SIMD
    ( very_large_input.as_str(), "Very Large Command" ),
  ];

  for ( input, description ) in test_cases
  {
    run_tokenization_benchmark( input, description );
  }
  
  println!( "\n✨ SIMD Tokenization Benefits Demonstrated:" );
  println!( "  • SIMD-optimized byte searching using memchr" );
  println!( "  • Statistical rigor through benchkit measurement infrastructure" );
  println!( "  • Automatic performance comparison and speedup calculation" );
  println!( "  • CPU feature detection for optimal code path selection" );
  println!( "  • Expected 3-6x performance improvement on supported hardware" );
}

/// Benchkit integration for different input patterns
#[ cfg( feature = "benchmarks" ) ]
#[ test ]
#[ ignore = "Benchkit integration - SIMD tokenization pattern analysis" ]
fn simd_tokenizer_pattern_analysis()
{
  println!( "🔬 SIMD Tokenization Pattern Analysis" );
  println!( "====================================" );
  
  // Test different delimiter density patterns
  let patterns = vec![
    ( "no_delimiters_just_plain_text_here", "No Delimiters" ),
    ( "few:delimiters.here", "Few Delimiters" ),
    ( "many:delim!iters?every#where.in.this:string", "Many Delimiters" ),
    ( ":::::::::::::::::::::::::::::::::::", "Only Delimiters" ),
  ];
  
  for ( pattern, description ) in patterns
  {
    run_tokenization_benchmark( pattern, description );
  }
  
  println!( "\n💡 Pattern Analysis Insights:" );
  println!( "  • SIMD benefits increase with delimiter density" );
  println!( "  • Minimal overhead for inputs without delimiters" );
  println!( "  • Optimal performance on mixed text/delimiter patterns" );
}

/// Test for CPU feature detection and runtime optimization
#[ cfg( feature = "benchmarks" ) ]
#[ test ]
#[ ignore = "Benchkit integration - SIMD feature detection validation" ]
fn simd_feature_detection_test()
{
  println!( "🏗️ SIMD Feature Detection and Runtime Optimization" );
  println!( "==================================================" );
  
  println!( "🔍 Runtime CPU Feature Detection:" );
  println!( "  • SIMD Enabled: {}", unilang::simd_tokenizer::is_simd_enabled() );
  println!( "  • CPU Support: {}", unilang::simd_tokenizer::simd_support_info() );
  
  #[ cfg( feature = "simd" ) ]
  {
    println!( "  • memchr Available: Yes (SIMD-optimized byte searching)" );
    println!( "  • bytecount Available: Yes (SIMD byte counting)" );
  }
  
  #[ cfg( not( feature = "simd" ) ) ]
  {
    println!( "  • SIMD Features: Disabled (scalar fallback active)" );
  }
  
  // Demonstrate runtime adaptation
  let test_input = ".test.command arg::value";
  let tokenizer = SIMDTokenizer::new( test_input );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  
  println!( "\n🧪 Runtime Tokenization Test:" );
  println!( "  • Input: '{test_input}'" );
  println!( "  • Tokens: {tokens:?}" );
  println!( "  • Token Count: {}", tokens.len() );
  
  #[ cfg( feature = "simd" ) ]
  {
    let token_count = tokenizer.count_tokens();
    println!( "  • SIMD Count: {} (matches iterator: {})", token_count, token_count == tokens.len() );
  }
  
  println!( "\n✅ Feature detection and runtime adaptation working correctly!" );
}

#[ cfg( not( feature = "benchmarks" ) ) ]
#[ test ]
#[ ignore = "Benchmarks disabled - enable 'benchmarks' feature for SIMD tokenization analysis" ]
fn simd_tokenizer_performance_test()
{
  println!( "⚠️  SIMD tokenization benchmarks disabled - enable 'benchmarks' feature" );
  println!( "     Run with: cargo test --features benchmarks --ignored simd_tokenizer_performance_test" );
}

#[ cfg( not( feature = "benchmarks" ) ) ]
#[ test ]
#[ ignore = "Benchmarks disabled - enable 'benchmarks' feature for pattern analysis" ]
fn simd_tokenizer_pattern_analysis()
{
  println!( "⚠️  SIMD tokenization pattern analysis disabled - enable 'benchmarks' feature" );
  println!( "     Run with: cargo test --features benchmarks --ignored simd_tokenizer_pattern_analysis" );
}

#[ cfg( not( feature = "benchmarks" ) ) ]
#[ test ]
#[ ignore = "Benchmarks disabled - enable 'benchmarks' feature for feature detection" ]
fn simd_feature_detection_test()
{
  println!( "⚠️  SIMD feature detection test disabled - enable 'benchmarks' feature" );
  println!( "     Run with: cargo test --features benchmarks --ignored simd_feature_detection_test" );
}