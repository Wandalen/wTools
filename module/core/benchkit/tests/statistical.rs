//! Test statistical analysis functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;
#[ cfg(feature = "statistical_analysis") ]
#[ allow(unused_imports) ]
use benchkit ::statistical :: *;
use core ::time ::Duration;

#[ allow(dead_code) ]
fn create_test_result(name: &str, durations: Vec< u64 >) -> BenchmarkResult 
{
  let times: Vec< Duration > = durations.into_iter().map(Duration ::from_millis).collect();
  BenchmarkResult ::new(name, times)
}

#[ test ]
#[ cfg(feature = "statistical_analysis") ]
fn test_confidence_interval() 
{
  let ci = ConfidenceInterval ::new(
  Duration ::from_millis(100),
  Duration ::from_millis(10),
  0.95
 );
  
  assert_eq!(ci.point_estimate, Duration ::from_millis(100));
  assert_eq!(ci.lower_bound, Duration ::from_millis(90));
  assert_eq!(ci.upper_bound, Duration ::from_millis(110));
  assert!(ci.contains(Duration ::from_millis(95)));
  assert!(!ci.contains(Duration ::from_millis(120)));
}

#[ test ]
#[ cfg(feature = "statistical_analysis") ]
fn test_statistical_analysis() 
{
  let result = create_test_result("test", vec![95, 100, 105, 98, 102, 97, 103, 99, 101, 96]);
  let analysis = StatisticalAnalysis ::analyze(&result, SignificanceLevel ::Standard).unwrap();
  
  assert_eq!(analysis.benchmark_result.name, "test");
  assert!(analysis.coefficient_of_variation > 0.0);
  assert!(analysis.coefficient_of_variation < 0.1); // Should be low for this data
  assert!(analysis.statistical_power > 0.0);
}

#[ test ]
#[ cfg(feature = "statistical_analysis") ]
fn test_statistical_comparison() 
{
  let result_a = create_test_result("fast", vec![90, 95, 92, 88, 94]);
  let result_b = create_test_result("slow", vec![110, 115, 112, 108, 114]);
  
  let test = StatisticalAnalysis ::compare(&result_a, &result_b, SignificanceLevel ::Standard).unwrap();
  
  assert!(test.effect_size.abs() > 0.0);
  assert!(test.p_value >= 0.0 && test.p_value <= 1.0);
  assert_eq!(test.effect_size_interpretation(), "large"); // Should be large difference
}

#[ test ]
#[ cfg(feature = "statistical_analysis") ]
fn test_outlier_detection() 
{
  let times = vec![
  Duration ::from_millis(100),
  Duration ::from_millis(102),
  Duration ::from_millis(98),
  Duration ::from_millis(101),
  Duration ::from_millis(500), // Outlier
 ];
  
  let outliers = StatisticalAnalysis ::detect_outliers(&times);
  assert_eq!(outliers, 1);
}