//! Simple JSON performance comparison test using benchkit
//!
//! This test demonstrates benchkit's `ComparativeAnalysis` API for statistical
//! benchmarking with professional-grade performance measurement.

#[ cfg( feature = "benchmarks" ) ]
use benchkit::prelude::*;
#[ cfg( feature = "benchmarks" ) ]
use unilang::simd_json_parser::SIMDJsonParser;
#[ cfg( feature = "benchmarks" ) ]
use serde_json::Value as SerdeValue;

/// Run JSON parsing comparison using benchkit
#[ cfg( feature = "benchmarks" ) ]
fn run_json_parsing_comparison( json_str : &str, description : &str )
{
  println!( "\n=== {description} ===" );
  println!( "JSON size: {} bytes", json_str.len() );
  
  let json_data = json_str.to_owned();
  let json_data_simd = json_str.to_owned();
  
  let comparison = ComparativeAnalysis::new( format!( "json_parsing_{}", description.replace( ' ', "_" ).to_lowercase() ) )
    .algorithm( "serde_json", move ||
    {
      let _ = serde_json::from_str::< SerdeValue >( &json_data ).unwrap();
    })
    .algorithm( "simd_json", move ||
    {
      let _ = SIMDJsonParser::parse_to_serde_value( &json_data_simd ).unwrap();
    });
  
  let report = comparison.run();
  
  // Display results using benchkit's reporting methods
  println!( "📈 Performance Results:" );
  for ( name, result ) in report.sorted_by_performance()
  {
    println!( "  • {name}: {:.0} ops/sec ({:.2}ms)", result.operations_per_second(), result.mean_time().as_secs_f64() * 1000.0 );
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
  println!( "🚀 SIMD support: {}", SIMDJsonParser::is_simd_supported() );
  println!( "📊 SIMD info: {}", SIMDJsonParser::simd_info() );
}

#[ cfg( feature = "benchmarks" ) ]
#[ test ]
#[ ignore = "Benchkit integration - run explicitly with --features benchmarks" ]
fn simple_json_perf_test()
{
  println!( "🎉 JSON Performance Comparison using Benchkit" );
  println!( "===============================================" );
  
  // Test with different JSON sizes to see where SIMD helps
  let test_cases = vec![
    (r#"{"small":"test"}"#, "Small JSON"),
    (r#"{"medium":{"nested":{"data":[1,2,3,4,5],"info":"test data","values":[true,false,null],"metadata":{"created":"2024-01-01","version":1.0}}}}"#, "Medium JSON"),
  ];

  for ( json_str, description ) in test_cases
  {
    run_json_parsing_comparison( json_str, description );
  }
  
  println!( "\n✨ Benchkit Benefits Demonstrated:" );
  println!( "  • Statistical rigor through built-in measurement infrastructure" );
  println!( "  • Automatic performance comparison and speedup calculation" );
  println!( "  • Clean, maintainable benchmarking code" );
  println!( "  • Professional-grade performance analysis" );
}

#[ cfg( not( feature = "benchmarks" ) ) ]
#[ test ]
#[ ignore = "Benchmarks disabled - enable 'benchmarks' feature" ]
fn simple_json_perf_test()
{
  println!( "⚠️  JSON performance benchmarks disabled - enable 'benchmarks' feature" );
}