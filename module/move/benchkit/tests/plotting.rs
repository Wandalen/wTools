//! Test plotting functionality

#![allow(clippy::float_cmp)]

#[cfg(feature = "integration")]
use benchkit::prelude::*;
#[cfg(feature = "visualization")]
#[allow(unused_imports)]
use benchkit::plotting::*;
use core::time::Duration;

#[allow(dead_code)]
fn create_test_result(name: &str, ops_per_sec: f64) -> BenchmarkResult
{
  let duration = Duration::from_secs_f64(1.0 / ops_per_sec);
  BenchmarkResult::new(name, vec![duration; 5])
}

#[test]
#[cfg(feature = "visualization")]
fn test_scaling_chart_creation()
{
  let config = ChartConfig::default();
  let mut chart = ScalingChart::new(config);
  
  // Add some test data
  let scaling_results = vec![
    (10, create_test_result("test_10", 1000.0)),
    (100, create_test_result("test_100", 800.0)),
    (1000, create_test_result("test_1000", 600.0)),
  ];
  
  chart.add_scaling_results("Test Series", &scaling_results);
  
  // Verify data was added
  assert_eq!(chart.data_series_count(), 1);
  assert_eq!(chart.data_points_count(0).unwrap(), 3);
}

#[test]
#[cfg(feature = "visualization")]  
fn test_comparison_chart_creation()
{
  let config = ChartConfig::default();
  let mut chart = ComparisonChart::new(config);
  
  let framework_results = vec![
    ("Fast Framework".to_string(), create_test_result("fast", 1000.0)),
    ("Slow Framework".to_string(), create_test_result("slow", 500.0)),
  ];
  
  chart.add_benchmark_results(&framework_results);
  
  // Verify data was added
  assert_eq!(chart.data_count(), 2);
  assert_eq!(chart.ops_per_second(0).unwrap(), 1000.0);
  assert_eq!(chart.ops_per_second(1).unwrap(), 500.0);
}

#[test]
#[cfg(feature = "visualization")]
fn test_chart_format_extensions()
{
  assert_eq!(ChartFormat::SVG.extension(), "svg");
  assert_eq!(ChartFormat::PNG.extension(), "png");
  assert_eq!(ChartFormat::HTML.extension(), "html");
}