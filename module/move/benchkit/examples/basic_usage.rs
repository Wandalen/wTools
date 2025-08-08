//! Basic benchkit usage example
//!
//! This example demonstrates the fundamental benchmarking capabilities:
//! - Simple function timing
//! - Comparative analysis
//! - Basic report generation

use benchkit::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
  println!("=== benchkit Basic Usage Example ===\n");

  // Example 1: Simple function timing
  println!("1. Simple Function Timing");
  println!("--------------------------");
  
  let result = bench_function("string_processing", || {
    // Simulate some string processing work
    let text = "hello world ".repeat(100);
    text.chars().filter(|c| c.is_alphabetic()).count()
  });
  
  println!("String processing: {}", result);
  println!("Throughput: {:.0} operations/sec\n", result.operations_per_second());

  // Example 2: Quick before/after comparison
  println!("2. Before/After Comparison");
  println!("--------------------------");
  
  let before = bench_function("inefficient_sort", || {
    let mut vec: Vec<i32> = (1..=100).rev().collect();
    vec.sort(); // Standard sort
    vec
  });
  
  let after = bench_function("optimized_sort", || {
    let mut vec: Vec<i32> = (1..=100).rev().collect(); 
    vec.sort_unstable(); // Potentially faster sort
    vec
  });
  
  let comparison = after.compare(&before);
  println!("Performance comparison: {}", comparison);
  
  if comparison.is_improvement() {
    println!("✅ Optimization successful!");
  } else if comparison.is_regression() {
    println!("❌ Performance regression detected!");
  } else {
    println!("➡️  No significant change");
  }
  println!();

  // Example 3: Comparative analysis with multiple algorithms
  println!("3. Multi-Algorithm Comparison");
  println!("-----------------------------");
  
  let comparison = ComparativeAnalysis::new("vector_operations")
    .algorithm("push_extend", || {
      let mut vec = Vec::new();
      vec.extend(1..=1000);
      vec
    })
    .algorithm("collect", || {
      (1..=1000).collect::<Vec<i32>>()
    })
    .algorithm("with_capacity", || {
      let mut vec = Vec::with_capacity(1000);
      vec.extend(1..=1000);
      vec
    });
    
  let report = comparison.run();
  report.print_summary();

  // Example 4: Using data generators
  println!("4. Using Data Generators");
  println!("------------------------");
  
  // Test different data sizes
  for size in DataSize::standard_sizes() {
    let data = generate_list_data(size);
    let result = bench_function(&format!("parse_{:?}", size), || {
      // Simulate parsing the generated data
      data.split(',').count()
    });
    
    println!("{:?} dataset: {} items processed in {:.2?}", 
             size, size.size(), result.mean_time());
  }
  println!();

  // Example 5: Custom metrics
  println!("5. Custom Metrics");
  println!("-----------------");
  
  let mut counter = 0;
  let result = bench_function("operation_with_side_effects", || {
    // Simulate work that produces measurable side effects
    for i in 1..=100 {
      if i % 7 == 0 {
        counter += 1;
      }
    }
  }).with_metric("multiples_of_seven", counter as f64);
  
  println!("Operation completed: {}", result);
  if let Some(&count) = result.metrics.get("multiples_of_seven") {
    println!("Side effect metric - multiples of seven found: {}", count);
  }
  println!();

  // Example 6: Statistical analysis
  println!("6. Statistical Analysis");
  println!("----------------------");
  
  // Run a potentially noisy operation multiple times
  let result = bench_function_with_config(
    "noisy_operation",
    MeasurementConfig {
      iterations: 20,
      warmup_iterations: 5,
      ..Default::default()
    },
    || {
      // Simulate work with some variability
      thread::sleep(Duration::from_millis(1 + (fastrand::u64(..) % 3)));
    }
  );
  
  println!("Noisy operation statistics:");
  println!("  Mean: {:.2?}", result.mean_time());
  println!("  Median: {:.2?}", result.median_time());
  println!("  Range: {:.2?} - {:.2?}", result.min_time(), result.max_time());
  println!("  Std Dev: {:.2?}", result.std_deviation());
  println!("  Samples: {}", result.times.len());
  
  println!("\n=== Example Complete ===");
}

// Simulate fastrand for the example
mod fastrand {
  use std::cell::Cell;
  
  thread_local! {
    static SEED: Cell<u64> = Cell::new(1);
  }
  
  pub fn u64(_: std::ops::RangeFull) -> u64 {
    SEED.with(|s| {
      let current = s.get();
      let next = current.wrapping_mul(1103515245).wrapping_add(12345);
      s.set(next);
      next
    })
  }
}