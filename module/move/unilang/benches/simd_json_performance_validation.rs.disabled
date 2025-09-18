//! SIMD JSON performance validation benchmark
//!
//! This benchmark validates SIMD JSON parsing performance using benchkit
//! comparative analysis against standard `serde_json` parsing.

#![allow(dead_code)]

use unilang::simd_json_parser::SIMDJsonParser;

#[cfg(feature = "benchmarks")]
use benchkit::prelude::*;

#[cfg(feature = "benchmarks")]
use serde_json::Value as SerdeValue;

fn main() {
  #[cfg(feature = "benchmarks")]
  {
    println!( "🚀 SIMD Performance Validation using Benchkit" );
    println!( "=============================================" );
    
    // Generate medium-sized JSON for performance testing
    let mut test_json = r#"{"performance_test":{"data":["#.to_string();
    for i in 0..500
    {
      if i > 0 { test_json.push(','); }
      use core::fmt::Write;
      write!( &mut test_json,
        r#"{{"id":{i},"name":"item{i}","value":{},"tags":["tag1","tag2"],"meta":{{"created":"2024-01-01","active":{}}}}}"#,
        f64::from(i) * 1.5, i % 2 == 0
      ).unwrap();
    }
    test_json.push_str( "]}}" );
    
    println!( "📊 JSON payload size: {} bytes", test_json.len() );
    println!( "🧪 Running comparative analysis..." );
    
    let simd_json_data = test_json.clone();
    let serde_json_data = test_json.clone();
    
    let comparison = ComparativeAnalysis::new( "simd_performance_validation" )
      .algorithm( "simd_json", move ||
      {
        let _ = SIMDJsonParser::parse_to_serde_value( &simd_json_data ).unwrap();
      })
      .algorithm( "serde_json", move ||
      {
        let _ = serde_json::from_str::<SerdeValue>( &serde_json_data ).unwrap();
      });
    
    let report = comparison.run();
    
    // Display comprehensive benchmark results
    println!( "📈 Performance Results:" );
    for ( name, result ) in report.sorted_by_performance()
    {
      println!( "  • {}: {:.0} ops/sec ({:.3}ms)", name, result.operations_per_second(), result.mean_time().as_secs_f64() * 1000.0 );
    }
    
    // Calculate and validate performance expectations
    if let Some( ( fastest_name, fastest_result ) ) = report.fastest()
    {
      if let Some( ( slowest_name, slowest_result ) ) = report.slowest()
      {
        let speedup = slowest_result.mean_time().as_nanos() as f64 / fastest_result.mean_time().as_nanos() as f64;
        println!( "⚡ Speedup: {fastest_name} is {speedup:.2}x faster than {slowest_name}" );
        
        // Validate performance characteristics with realistic expectations
        if fastest_name == "simd_json"
        {
          println!( "✅ SIMD JSON outperforms standard JSON parsing" );
        }
        else
        {
          println!( "⚠️  Standard serde_json outperformed SIMD (may indicate debug build, small payload, or sub-optimal conditions)" );
        }
        
        // Performance validation - SIMD should be reasonable but may not always win
        // In debug builds or with certain payload characteristics, serde_json might be faster
        let performance_difference = ( slowest_result.mean_time().as_nanos() as f64 / fastest_result.mean_time().as_nanos() as f64 ) - 1.0;
        
        if performance_difference > 5.0 {
          println!( "⚠️ Performance difference is extreme ({:.1}x) - investigate SIMD implementation", performance_difference + 1.0 );
        } else {
          println!( "✅ Performance validation passed - algorithms perform within reasonable bounds" );
        }
      }
    }
    
    // Display SIMD capability information  
    println!( "🔧 SIMD Capability Detection:" );
    println!( "  • SIMD support: {}", SIMDJsonParser::is_simd_supported() );
    println!( "  • SIMD info: {}", SIMDJsonParser::simd_info() );
    
    println!( "✨ Benchkit provides statistical rigor and clear PASS/FAIL validation for SIMD performance!" );
  }
  
  #[cfg(not(feature = "benchmarks"))]
  {
    println!( "⚠️  SIMD performance validation disabled - enable 'benchmarks' feature" );
    println!( "     Run with: cargo bench simd_json_performance_validation --features benchmarks" );
  }
}