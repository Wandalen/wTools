//! Simple JSON performance comparison test

// Performance test for SIMD JSON parsing
use unilang::simd_json_parser::SIMDJsonParser;
use serde_json::Value as SerdeValue;
use std::time::Instant;

#[test]
#[ignore] // Run with: cargo test simple_json_perf_test --release --features simd -- --ignored --nocapture
fn simple_json_perf_test()
{
  // Test with different JSON sizes to see where SIMD helps
  let test_cases = vec![
    (r#"{"small":"test"}"#, "Small JSON"),
    (r#"{"medium":{"nested":{"data":[1,2,3,4,5],"info":"test data","values":[true,false,null],"metadata":{"created":"2024-01-01","version":1.0}}}}"#, "Medium JSON"),
  ];

  for (json_str, description) in test_cases {
    println!("\n=== {} ===", description);
    println!("JSON size: {} bytes", json_str.len());
    
    let iterations = 10000;
    
    // Test serde_json
    let start = Instant::now();
    for _ in 0..iterations {
      let _ = serde_json::from_str::<SerdeValue>(json_str).unwrap();
    }
    let serde_duration = start.elapsed();
    let serde_ops_sec = iterations as f64 / serde_duration.as_secs_f64();
    
    // Test SIMD JSON
    let start = Instant::now();
    for _ in 0..iterations {
      let _ = SIMDJsonParser::parse_to_serde_value(json_str).unwrap();
    }
    let simd_duration = start.elapsed();
    let simd_ops_sec = iterations as f64 / simd_duration.as_secs_f64();
    
    println!("serde_json: {:.2}ms ({:.0} ops/sec)", 
             serde_duration.as_secs_f64() * 1000.0, serde_ops_sec);
    println!("SIMD JSON:  {:.2}ms ({:.0} ops/sec)", 
             simd_duration.as_secs_f64() * 1000.0, simd_ops_sec);
    
    let speedup = simd_ops_sec / serde_ops_sec;
    println!("SIMD speedup: {:.2}x", speedup);
    
    // Test SIMD info
    println!("SIMD support: {}", SIMDJsonParser::is_simd_supported());
    println!("SIMD info: {}", SIMDJsonParser::simd_info());
  }
}