//! Test scaling analysis functionality

#[cfg(feature = "integration")]
use benchkit::prelude::*;

#[test]
fn test_scaling_analysis()
{
  let analysis = power_of_10_scaling(
    "test_operation",
    |scale|
    {
      // Simulate O(n) operation
      for i in 0..scale
      {
        core::hint::black_box(i);
      }
    },
    Some(ScalingConfig::quick())
  );
  
  assert!(!analysis.results.is_empty());
  
  let complexity = analysis.complexity_analysis();
  println!("Complexity analysis: {complexity:?}");
}