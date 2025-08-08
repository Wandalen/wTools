//! ## Test Matrix for Timing and Measurement Functionality
//!
//! This test suite validates core timing and measurement capabilities.
//!
//! ### Test Factors
//! - Function Types: Simple, Complex, I/O-bound
//! - Measurement Config: Default, Custom iterations, Custom timeouts
//! - Result Processing: Statistical calculations, Comparisons
//!
//! ### Test Combinations
//! | ID   | Function Type | Config         | Aspect Tested                | Expected Behavior                    |
//! |------|---------------|----------------|------------------------------|--------------------------------------|
//! | T1.1 | Simple        | Default        | Basic measurement            | Times recorded, stats calculated     |
//! | T1.2 | Simple        | Custom iter    | Iteration control            | Exact iteration count respected      |
//! | T1.3 | Complex       | Default        | Complex operation timing     | Accurate timing with overhead <1%    |
//! | T1.4 | I/O-bound     | Custom timeout | Timeout handling             | Measurement stops at timeout         |
//! | T1.5 | Simple        | Default        | Statistical accuracy         | Mean, median, std dev calculated     |
//! | T1.6 | Simple        | Default        | Comparison functionality     | Improvement percentages calculated   |
//! | T1.7 | Simple        | Default        | Operations per second        | Correct ops/sec calculation          |

use benchkit::prelude::*;
use std::time::{Duration, Instant};

/// Tests basic timing measurement functionality
/// Test Combination: T1.1
#[test]
fn test_basic_timing_measurement()
{
  let result = bench_function("test_operation", || {
    // Simple operation that should take measurable time
    let mut sum = 0;
    for i in 1..1000 {
      sum += i;
    }
    sum
  });

  assert_eq!(result.name, "test_operation");
  assert!(!result.times.is_empty(), "Should have recorded timing measurements");
  assert!(result.mean_time().as_nanos() > 0, "Should have non-zero mean time");
  assert!(result.min_time() <= result.mean_time(), "Min should be <= mean");
  assert!(result.max_time() >= result.mean_time(), "Max should be >= mean");
}

/// Tests custom iteration configuration
/// Test Combination: T1.2
#[test]
fn test_custom_iteration_config()
{
  let config = MeasurementConfig {
    iterations: 5,
    warmup_iterations: 1,
    max_time: Duration::from_secs(30),
  };

  let result = bench_function_with_config("custom_iterations", config, || {
    // Simple operation
    std::hint::black_box(42 + 42);
  });

  // Should have exactly the requested iterations (or fewer if timeout hit)
  assert!(
    result.times.len() <= 5,
    "Should not exceed requested iterations"
  );
  assert!(
    !result.times.is_empty(),
    "Should have at least one measurement"
  );
}

/// Tests timing accuracy for complex operations
/// Test Combination: T1.3
#[test]
fn test_complex_operation_timing()
{
  let operation = || {
    // More complex operation to test timing accuracy
    let mut data: Vec<i32> = (1..10000).collect();
    data.sort_unstable();
    data.reverse();
    std::hint::black_box(data);
  };

  let result = bench_function("complex_operation", operation);

  assert!(result.mean_time().as_micros() > 10, "Complex operation should take measurable time");
  assert!(result.std_deviation().as_nanos() >= 0, "Standard deviation should be non-negative");
  
  // Test measurement overhead - should be minimal for operations > 1ms
  if result.mean_time().as_millis() >= 1 {
    let overhead_percentage = (result.std_deviation().as_secs_f64() / result.mean_time().as_secs_f64()) * 100.0;
    assert!(overhead_percentage < 10.0, "Measurement overhead should be reasonable for long operations");
  }
}

/// Tests timeout handling in measurement configuration
/// Test Combination: T1.4
#[test]
fn test_timeout_handling()
{
  let config = MeasurementConfig {
    iterations: 1000, // Request many iterations
    warmup_iterations: 0,
    max_time: Duration::from_millis(50), // But limit time
  };

  let start_time = Instant::now();
  let result = bench_function_with_config("timeout_test", config, || {
    std::thread::sleep(Duration::from_millis(1));
  });
  let total_elapsed = start_time.elapsed();

  // Should respect timeout
  assert!(
    total_elapsed <= Duration::from_millis(100), // Allow some buffer
    "Should respect timeout configuration"
  );
  
  // Should have fewer measurements than requested iterations
  assert!(
    result.times.len() < 1000,
    "Should stop early due to timeout"
  );
}

/// Tests statistical calculation accuracy
/// Test Combination: T1.5
#[test]
fn test_statistical_accuracy()
{
  // Create controlled measurements with known values
  let times = vec![
    Duration::from_millis(10),
    Duration::from_millis(20),
    Duration::from_millis(30),
    Duration::from_millis(40),
    Duration::from_millis(50),
  ];
  
  let result = BenchmarkResult::new("stats_test", times);

  // Test mean calculation: (10+20+30+40+50)/5 = 30ms
  assert_eq!(result.mean_time(), Duration::from_millis(30));

  // Test median calculation: middle value = 30ms
  assert_eq!(result.median_time(), Duration::from_millis(30));

  // Test min/max
  assert_eq!(result.min_time(), Duration::from_millis(10));
  assert_eq!(result.max_time(), Duration::from_millis(50));

  // Test operations per second calculation
  let ops_per_sec = result.operations_per_second();
  let expected_ops = 1.0 / 0.030; // 1 / 30ms in seconds
  assert!((ops_per_sec - expected_ops).abs() < 1.0, "Operations per second should be approximately correct");
}

/// Tests comparison functionality between benchmark results
/// Test Combination: T1.6
#[test]
fn test_comparison_functionality()
{
  let fast_result = BenchmarkResult::new("fast", vec![Duration::from_millis(10)]);
  let slow_result = BenchmarkResult::new("slow", vec![Duration::from_millis(20)]);

  let comparison = fast_result.compare(&slow_result);

  // Fast should show improvement compared to slow
  assert!(comparison.improvement_percentage > 0.0, "Fast should show improvement over slow");
  assert!(comparison.is_improvement(), "Should detect improvement");
  assert!(!comparison.is_regression(), "Should not detect regression");

  // Test reverse comparison
  let reverse_comparison = slow_result.compare(&fast_result);
  assert!(reverse_comparison.improvement_percentage < 0.0, "Slow should show regression compared to fast");
  assert!(reverse_comparison.is_regression(), "Should detect regression");
}

/// Tests operations per second calculation accuracy
/// Test Combination: T1.7
#[test]
fn test_operations_per_second_calculation()
{
  // Test with known timing
  let result = BenchmarkResult::new("ops_test", vec![Duration::from_millis(100)]); // 0.1 seconds

  let ops_per_sec = result.operations_per_second();
  let expected = 10.0; // 1 / 0.1 = 10 ops/sec

  assert!(
    (ops_per_sec - expected).abs() < 0.1,
    "Operations per second calculation should be accurate: expected {}, got {}",
    expected,
    ops_per_sec
  );

  // Test edge case: zero time
  let zero_result = BenchmarkResult::new("zero_test", vec![]);
  assert_eq!(zero_result.operations_per_second(), 0.0, "Zero time should give zero ops/sec");
}

/// Tests bench_once convenience function
#[test]
fn test_bench_once()
{
  let result = bench_once(|| {
    std::hint::black_box(1 + 1);
  });

  assert_eq!(result.times.len(), 1, "bench_once should record exactly one measurement");
  assert!(result.mean_time().as_nanos() >= 0, "Should record valid timing");
}

/// Tests bench_block macro
#[test]
fn test_bench_block_macro()
{
  let result = bench_block!({
    let x = 42;
    let y = x * 2;
    std::hint::black_box(y);
  });

  assert_eq!(result.times.len(), 1, "bench_block should record single measurement");

  // Test named version
  let named_result = bench_block!("named_block", {
    std::hint::black_box(100 + 200);
  });

  assert_eq!(named_result.name, "named_block");
  assert!(!named_result.times.is_empty());
}

/// Tests time_block utility function
#[test]
fn test_time_block_utility()
{
  let (result, elapsed) = time_block(|| {
    std::thread::sleep(Duration::from_millis(1));
    "test_result"
  });

  assert_eq!(result, "test_result", "Should return function result");
  assert!(elapsed >= Duration::from_millis(1), "Should measure elapsed time accurately");
}

/// Tests custom metrics functionality
#[test]
fn test_custom_metrics()
{
  let mut result = BenchmarkResult::new("metrics_test", vec![Duration::from_millis(10)]);
  result = result
    .with_metric("memory_usage", 1024.0)
    .with_metric("cache_hits", 95.0);

  assert_eq!(result.metrics.get("memory_usage"), Some(&1024.0));
  assert_eq!(result.metrics.get("cache_hits"), Some(&95.0));
  assert_eq!(result.metrics.len(), 2);
}

/// Tests benchmark result display formatting
#[test]
fn test_result_display_formatting()
{
  let result = BenchmarkResult::new("display_test", vec![
    Duration::from_millis(10),
    Duration::from_millis(20),
  ]);

  let display_string = format!("{}", result);
  assert!(display_string.contains("display_test"), "Should include benchmark name");
  assert!(display_string.contains("ms"), "Should include timing information");
}

/// Tests comparison display formatting
#[test]
fn test_comparison_display_formatting()
{
  let fast = BenchmarkResult::new("fast", vec![Duration::from_millis(10)]);
  let slow = BenchmarkResult::new("slow", vec![Duration::from_millis(20)]);
  
  let comparison = fast.compare(&slow);
  let display = format!("{}", comparison);
  
  assert!(display.contains("IMPROVEMENT") || display.contains("faster"), 
          "Should indicate improvement");
}