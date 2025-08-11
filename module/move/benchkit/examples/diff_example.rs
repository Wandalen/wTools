//! Example demonstrating git-style diff functionality for benchmark results

#[cfg(feature = "diff_analysis")]
use benchkit::prelude::*;
#[cfg(feature = "diff_analysis")]
use core::time::Duration;

fn main()
{
  #[cfg(feature = "diff_analysis")]
  {
  println!("🔄 Benchkit Diff Analysis Example");
  
  // Simulate baseline benchmark results (old implementation)
  let baseline_results = vec![
    (
      "string_concatenation".to_string(),
      BenchmarkResult::new("string_concat_old", vec![Duration::from_millis(100); 5])
    ),
    (
      "hash_computation".to_string(),
      BenchmarkResult::new("hash_comp_old", vec![Duration::from_millis(50); 5])
    ),
    (
      "sorting_algorithm".to_string(),
      BenchmarkResult::new("sort_old", vec![Duration::from_millis(200); 5])
    ),
  ];
  
  // Simulate current benchmark results (new implementation)
  let current_results = vec![
    (
      "string_concatenation".to_string(),
      BenchmarkResult::new("string_concat_new", vec![Duration::from_millis(50); 5])  // 2x faster
    ),
    (
      "hash_computation".to_string(),
      BenchmarkResult::new("hash_comp_new", vec![Duration::from_millis(75); 5])  // 1.5x slower
    ),
    (
      "sorting_algorithm".to_string(),
      BenchmarkResult::new("sort_new", vec![Duration::from_millis(195); 5])  // Slightly faster
    ),
  ];
  
  println!("\n📊 Comparing benchmark results...\n");
  
  // Create diff set
  let diff_set = diff_benchmark_sets(&baseline_results, &current_results);
  
  // Show individual diffs
  for diff in &diff_set.diffs
  {
    println!("{}", diff.to_summary());
  }
  
  // Show detailed diff for significant changes
  println!("\n📋 Detailed Analysis:\n");
  
  for diff in diff_set.significant_changes()
  {
    println!("=== {} ===", diff.benchmark_name);
    println!("{}", diff.to_diff_format());
    println!();
  }
  
  // Show summary report
  println!("📈 Summary Report:");
  println!("==================");
  println!("Total benchmarks: {}", diff_set.summary_stats.total_benchmarks);
  println!("Improvements: {} 📈", diff_set.summary_stats.improvements);
  println!("Regressions: {} 📉", diff_set.summary_stats.regressions);
  println!("No change: {} 🔄", diff_set.summary_stats.no_change);
  println!("Average change: {:.1}%", diff_set.summary_stats.average_change);
  
  // Show regressions if any
  let regressions = diff_set.regressions();
  if !regressions.is_empty()
  {
    println!("\n⚠️  Regressions detected:");
    for regression in regressions
    {
      println!("  - {}: {:.1}% slower", regression.benchmark_name, regression.analysis.ops_per_sec_change.abs());
    }
  }
  
  // Show improvements
  let improvements = diff_set.improvements();
  if !improvements.is_empty()
  {
    println!("\n🎉 Improvements detected:");
    for improvement in improvements
    {
      println!("  - {}: {:.1}% faster", improvement.benchmark_name, improvement.analysis.ops_per_sec_change);
    }
  }
  } // End of cfg(feature = "diff_analysis")
  
  #[cfg(not(feature = "diff_analysis"))]
  {
    println!("🔄 Benchkit Diff Analysis Example (disabled)");
    println!("Enable with --features diff_analysis");
  }
}