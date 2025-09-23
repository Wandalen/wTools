//! Test diff functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;
#[ cfg(feature = "diff_analysis") ]
#[ allow(unused_imports) ]
use benchkit ::diff :: *;
use core ::time ::Duration;

#[ allow(dead_code) ]
fn create_test_result(name: &str, mean_duration: Duration) -> BenchmarkResult
{
  BenchmarkResult ::new(name, vec![mean_duration; 10])
}

#[ test ]
#[ cfg(feature = "diff_analysis") ]
fn test_benchmark_diff_improvement()
{
  let baseline = create_test_result("test", Duration ::from_millis(100));
  let current = create_test_result("test", Duration ::from_millis(50));
  
  let diff = BenchmarkDiff ::new("test_benchmark", baseline, current);
  
  assert!(diff.is_improvement());
  assert!(diff.analysis.ops_per_sec_change > 0.0);
}

#[ test ]
#[ cfg(feature = "diff_analysis") ]
fn test_benchmark_diff_regression()
{
  let baseline = create_test_result("test", Duration ::from_millis(50));
  let current = create_test_result("test", Duration ::from_millis(100));
  
  let diff = BenchmarkDiff ::new("test_benchmark", baseline, current);
  
  assert!(diff.is_regression());
  assert!(diff.analysis.ops_per_sec_change < 0.0);
}

#[ test ]
#[ cfg(feature = "diff_analysis") ]
fn test_diff_set_comparison()
{
  let baseline_results = vec![
  ("fast_func".to_string(), create_test_result("fast_func", Duration ::from_millis(10))),
  ("slow_func".to_string(), create_test_result("slow_func", Duration ::from_millis(100))),
 ];
  
  let current_results = vec![
  ("fast_func".to_string(), create_test_result("fast_func", Duration ::from_millis(5))),
  ("slow_func".to_string(), create_test_result("slow_func", Duration ::from_millis(150))),
 ];
  
  let diff_set = BenchmarkDiffSet ::compare_results(&baseline_results, &current_results);
  
  assert_eq!(diff_set.diffs.len(), 2);
  assert_eq!(diff_set.summary_stats.improvements, 1);
  assert_eq!(diff_set.summary_stats.regressions, 1);
}

#[ test ]
#[ cfg(feature = "diff_analysis") ]
fn test_diff_format()
{
  let baseline = create_test_result("test", Duration ::from_millis(100));
  let current = create_test_result("test", Duration ::from_millis(50));
  
  let diff = BenchmarkDiff ::new("test_benchmark", baseline, current);
  let diff_output = diff.to_diff_format();
  
  assert!(diff_output.contains("diff --benchmark"));
  assert!(diff_output.contains("Operations/sec: "));
  assert!(diff_output.contains("Mean time: "));
}