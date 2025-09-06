//! Test throughput functionality

#[cfg(feature = "integration")]
use benchkit::prelude::*;
use core::time::Duration;
use std::collections::HashMap;

fn create_test_result(time_ms: u64) -> BenchmarkResult
{
  let times = vec![Duration::from_millis(time_ms); 5];
  BenchmarkResult::new("test", times)
}

#[test]
fn test_throughput_calculation()
{
  let analyzer = ThroughputAnalyzer::new("string_processing", 1024);
  let result = create_test_result(100); // 100ms
  
  let metrics = analyzer.analyze(&result);
  
  assert_eq!(metrics.data_size_bytes, 1024);
  assert!(metrics.bytes_per_second > 0.0);
  assert!(metrics.megabytes_per_second > 0.0);
}

#[test]
fn test_throughput_with_items()
{
  let analyzer = ThroughputAnalyzer::new("item_processing", 2048).with_items(100);
  let result = create_test_result(200); // 200ms
  
  let metrics = analyzer.analyze(&result);
  
  assert_eq!(metrics.item_count, Some(100));
  assert!(metrics.items_per_second.is_some());
  assert!(metrics.items_per_second.unwrap() > 0.0);
}

#[test]
fn test_throughput_comparison()
{
  let analyzer = ThroughputAnalyzer::new("comparison_test", 1024);
  
  let mut results = HashMap::new();
  results.insert("fast".to_string(), create_test_result(50));   // 50ms
  results.insert("slow".to_string(), create_test_result(200));  // 200ms
  
  let comparison = analyzer.compare_throughput(&results);
  
  assert!(comparison.metrics.len() == 2);
  
  let (fastest_name, _) = comparison.fastest_throughput().unwrap();
  assert_eq!(fastest_name, "fast");
  
  let speedups = comparison.calculate_speedups("slow").unwrap();
  assert!(speedups["fast"] > speedups["slow"]);
}

#[test]
fn test_bandwidth_analysis()
{
  let metrics = BandwidthAnalyzer::analyze_memory_bandwidth(
    1024 * 1024, // 1MB
    Duration::from_millis(100), // 100ms
    2, // 2 read passes
    1, // 1 write pass  
  );
  
  assert_eq!(metrics.data_size, 1024 * 1024);
  assert_eq!(metrics.total_bytes_accessed, 3 * 1024 * 1024); // 3MB total
  assert!(metrics.bandwidth_bytes_per_second > 0.0);
}

#[test]
fn test_throughput_descriptions()
{
  let metrics = ThroughputMetrics
  {
    operation: "test".to_string(),
    data_size_bytes: 1024,
    item_count: Some(100),
    processing_time: Duration::from_millis(100),
    bytes_per_second: 10_240.0, // 10KB/s
    items_per_second: Some(1000.0),
    megabytes_per_second: 0.01,
    gigabytes_per_second: 0.00001,
  };
  
  assert!(metrics.throughput_description().contains("KB/s"));
  assert!(metrics.items_description().unwrap().contains("items/s"));
}