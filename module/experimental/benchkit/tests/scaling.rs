//! Test scaling analysis functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;

#[ test ]
fn test_scaling_analysis()
{
  let analysis = power_of_10_scaling(
  "test_operation",
  |scale|
  {
   // Simulate O(n) operation
   for i in 0..scale
   {
  core ::hint ::black_box(i);
 }
 },
  Some(ScalingConfig ::quick())
 );

  assert!(!analysis.results.is_empty(), "Scaling analysis should have results");

  let complexity = analysis.complexity_analysis();

  // Validate complexity analysis results instead of just printing
  assert!(!complexity.operation_name.is_empty(), "Operation name should not be empty");
  assert!(!complexity.estimated_complexity.is_empty(), "Estimated complexity should not be empty");
  assert!(complexity.estimated_complexity.contains("Linear") || complexity.estimated_complexity.contains("O(n)")
    || complexity.estimated_complexity.contains("Constant") || complexity.estimated_complexity.contains("O(1)")
    || complexity.estimated_complexity.contains("O(n²)") || complexity.estimated_complexity.contains("O(n log n)"),
    "Complexity analysis should contain a recognized complexity class, got: {}", complexity.estimated_complexity);
  assert!(complexity.correlation_coefficient >= 0.0 && complexity.correlation_coefficient <= 1.0,
    "Correlation coefficient should be between 0 and 1");
}