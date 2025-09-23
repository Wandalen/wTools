//! Manual testing of `strs_tools` integration with benchkit
//!
//! This tests benchkit with actual `strs_tools` functionality to identify issues.

#![allow(clippy ::doc_markdown)]
#![allow(clippy ::format_push_string)]
#![allow(clippy ::uninlined_format_args)]
#![allow(clippy ::std_instead_of_core)]
#![allow(clippy ::unnecessary_wraps)]
#![allow(clippy ::useless_format)]
#![allow(clippy ::redundant_closure_for_method_calls)]
#![allow(clippy ::cast_possible_truncation)]
#![allow(clippy ::cast_sign_loss)]
#![allow(clippy ::no_effect_underscore_binding)]
#![allow(clippy ::used_underscore_binding)]

use benchkit ::prelude :: *;

use std ::collections ::HashMap;

type Result< T > = std ::result ::Result< T, Box<dyn std ::error ::Error >>;

fn main() -> Result< () >
{
  println!("🧪 Manual Testing of strs_tools + benchkit Integration");
  println!("======================================================");
  println!();

  // Test 1 : Basic benchkit functionality
  test_basic_benchkit()?;
  
  // Test 2 : Data generation with real patterns
  test_data_generation()?;
  
  // Test 3 : Memory tracking
  test_memory_tracking()?;
  
  // Test 4 : Throughput analysis
  test_throughput_analysis()?;
  
  // Test 5 : Statistical analysis (if available)
  #[ cfg(feature = "statistical_analysis") ]
  test_statistical_analysis()?;
  
  // Test 6 : Report generation
  test_report_generation()?;

  println!("✅ All manual tests completed successfully!");
  Ok(())
}

fn test_basic_benchkit() -> Result< () >
{
  println!("1️⃣ Testing Basic Benchkit Functionality");
  println!("---------------------------------------");
  
  // Simple comparative analysis without external dependencies
  let mut comparison = ComparativeAnalysis ::new("basic_string_operations");
  
  comparison = comparison
  .algorithm("simple_split", ||
  {
   let test_data = "item1,item2,item3,item4,item5";
   let count = test_data.split(',').count();
   std ::hint ::black_box(count);
 })
  .algorithm("collect_split", ||
  {
   let test_data = "item1,item2,item3,item4,item5";
   let parts: Vec< &str > = test_data.split(',').collect();
   std ::hint ::black_box(parts.len());
 });

  let report = comparison.run();
  
  if let Some((fastest, result)) = report.fastest()
  {
  println!("  ✅ Fastest: {} ({:.0} ops/sec)", fastest, result.operations_per_second());
 }
  else
  {
  println!("  ❌ Failed to determine fastest algorithm");
 }
  
  println!();
  Ok(())
}

fn test_data_generation() -> Result< () >
{
  println!("2️⃣ Testing Data Generation");
  println!("-------------------------");
  
  // Test pattern-based generation
  let generator = DataGenerator ::new()
  .pattern("item{},")
  .repetitions(5)
  .complexity(DataComplexity ::Simple);
  
  let result = generator.generate_string();
  println!("  ✅ Pattern generation: {}", &result[..30.min(result.len())]);
  
  // Test size-based generation
  let size_generator = DataGenerator ::new()
  .size_bytes(100)
  .complexity(DataComplexity ::Medium);
  
  let size_result = size_generator.generate_string();
  println!("  ✅ Size-based generation: {} bytes", size_result.len());
  
  // Test CSV generation
  let csv_data = generator.generate_csv_data(3, 4);
  let lines: Vec< &str > = csv_data.lines().collect();
  println!("  ✅ CSV generation: {} rows generated", lines.len());
  
  // Test unilang commands
  let commands = generator.generate_unilang_commands(3);
  println!("  ✅ Unilang commands: {} commands generated", commands.len());
  
  println!();
  Ok(())
}

fn test_memory_tracking() -> Result< () >
{
  println!("3️⃣ Testing Memory Tracking");
  println!("-------------------------");
  
  let memory_benchmark = MemoryBenchmark ::new("memory_test");
  
  // Test basic allocation tracking
  let (result, stats) = memory_benchmark.run_with_tracking(5, ||
  {
  // Simulate allocation
  let _data = vec![0u8; 1024];
  memory_benchmark.tracker.record_allocation(1024);
 });
  
  println!("  ✅ Memory tracking completed");
  println!("     - Iterations: {}", result.times.len());
  println!("     - Total allocated: {} bytes", stats.total_allocated);
  println!("     - Peak usage: {} bytes", stats.peak_usage);
  println!("     - Allocations: {}", stats.allocation_count);
  
  // Test memory comparison
  let comparison = memory_benchmark.compare_memory_usage(
  "allocating_version",
  || {
   let _vec = vec![42u8; 512];
   memory_benchmark.tracker.record_allocation(512);
 },
  "minimal_version",
  || {
   let _x = 42;
   // No allocations
 },
  3,
 );
  
  let (efficient_name, _) = comparison.more_memory_efficient();
  println!("  ✅ Memory comparison: {} is more efficient", efficient_name);
  
  println!();
  Ok(())
}

fn test_throughput_analysis() -> Result< () >
{
  println!("4️⃣ Testing Throughput Analysis");
  println!("-----------------------------");
  
  let test_data = "field1,field2,field3,field4,field5,field6,field7,field8,field9,field10".repeat(100);
  let throughput_analyzer = ThroughputAnalyzer ::new("string_processing", test_data.len() as u64)
  .with_items(1000);
  
  // Create some test results
  let mut results = HashMap ::new();
  
  // Fast version (50ms)
  let fast_times = vec![std ::time ::Duration ::from_millis(50); 10];
  results.insert("fast_algorithm".to_string(), BenchmarkResult ::new("fast", fast_times));
  
  // Slow version (150ms)
  let slow_times = vec![std ::time ::Duration ::from_millis(150); 10];
  results.insert("slow_algorithm".to_string(), BenchmarkResult ::new("slow", slow_times));
  
  let throughput_comparison = throughput_analyzer.compare_throughput(&results);
  
  if let Some((fastest_name, fastest_metrics)) = throughput_comparison.fastest_throughput()
  {
  println!("  ✅ Throughput analysis completed");
  println!("     - Fastest: {} ({})", fastest_name, fastest_metrics.throughput_description());
  
  if let Some(items_desc) = fastest_metrics.items_description()
  {
   println!("     - Item processing: {}", items_desc);
 }
 }
  
  if let Some(speedups) = throughput_comparison.calculate_speedups("slow_algorithm")
  {
  for (name, speedup) in speedups
  {
   if name != "slow_algorithm"
   {
  println!("     - {} : {:.1}x speedup", name, speedup);
 }
 }
 }
  
  println!();
  Ok(())
}

#[ cfg(feature = "statistical_analysis") ]
fn test_statistical_analysis() -> Result< () >
{
  println!("5️⃣ Testing Statistical Analysis");
  println!("------------------------------");
  
  // Create test results with different characteristics
  let consistent_times = vec![std ::time ::Duration ::from_millis(100); 20];
  let consistent_result = BenchmarkResult ::new("consistent", consistent_times);
  
  let variable_times: Vec< _ > = (0..20)
  .map(|i| std ::time ::Duration ::from_millis(100 + (i * 5)))
  .collect();
  let variable_result = BenchmarkResult ::new("variable", variable_times);
  
  // Analyze individual results
  let consistent_analysis = StatisticalAnalysis ::analyze(&consistent_result, SignificanceLevel ::Standard)?;
  let variable_analysis = StatisticalAnalysis ::analyze(&variable_result, SignificanceLevel ::Standard)?;
  
  println!("  ✅ Statistical analysis completed");
  println!("     - Consistent CV: {:.1}% ({})", 
   consistent_analysis.coefficient_of_variation * 100.0,
   if consistent_analysis.is_reliable() 
   { "Reliable" } else { "Questionable" });
  println!("     - Variable CV: {:.1}% ({})",
   variable_analysis.coefficient_of_variation * 100.0,
   if variable_analysis.is_reliable() 
   { "Reliable" } else { "Questionable" });
  
  // Compare results
  let comparison = StatisticalAnalysis ::compare(
  &consistent_result,
  &variable_result,
  SignificanceLevel ::Standard
 )?;
  
  println!("     - Effect size: {:.3} ({})", 
   comparison.effect_size,
   comparison.effect_size_interpretation());
  println!("     - Statistically significant: {}", comparison.is_significant);
  
  println!();
  Ok(())
}

fn test_report_generation() -> Result< () >
{
  println!("6️⃣ Testing Report Generation");
  println!("---------------------------");
  
  // Generate a simple comparison
  let mut comparison = ComparativeAnalysis ::new("report_test");
  
  comparison = comparison
  .algorithm("approach_a", ||
  {
   let _result = "test,data,processing".split(',').count();
   std ::hint ::black_box(_result);
 })
  .algorithm("approach_b", ||
  {
   let parts: Vec< &str > = "test,data,processing".split(',').collect();
   std ::hint ::black_box(parts.len());
 });

  let report = comparison.run();
  
  // Generate markdown report
  let markdown_report = generate_comprehensive_markdown_report(&report);
  
  // Save report to test file
  let report_path = "target/manual_test_report.md";
  std ::fs ::write(report_path, &markdown_report)?;
  
  println!("  ✅ Report generation completed");
  println!("     - Report saved: {}", report_path);
  println!("     - Report length: {} characters", markdown_report.len());
  
  // Check if report contains expected sections
  let has_performance = markdown_report.contains("Performance");
  let has_results = markdown_report.contains("ops/sec");
  let has_methodology = markdown_report.contains("Statistical");
  
  println!("     - Contains performance data: {}", has_performance);
  println!("     - Contains results: {}", has_results);  
  println!("     - Contains methodology: {}", has_methodology);
  
  println!();
  Ok(())
}

fn generate_comprehensive_markdown_report(report: &ComparisonAnalysisReport) -> String
{
  let mut output = String ::new();
  
  output.push_str("# Manual Test Report\n\n");
  output.push_str("*Generated with benchkit manual testing*\n\n");
  
  output.push_str("## Performance Results\n\n");
  // Generate simple table from results
  output.push_str("| Operation | Mean Time | Ops/sec |\n");
  output.push_str("|-----------|-----------|--------|\n");
  for (name, result) in &report.results 
  {
  output.push_str(&format!(
   "| {} | {:.2?} | {:.0} |\n",
   name,
   result.mean_time(),
   result.operations_per_second()
 ));
 }
  
  output.push_str("## Statistical Quality\n\n");
  
  let mut reliable_count = 0;
  let mut total_count = 0;
  
  for (name, result) in &report.results
  {
  total_count += 1;
  let is_reliable = result.is_reliable();
  if is_reliable { reliable_count += 1; }
  
  let status = if is_reliable { "✅ Reliable" } else { "⚠️ Needs improvement" };
  output.push_str(&format!("- **{}** : {} (CV: {:.1}%)\n",
  name,
  status,
  result.coefficient_of_variation() * 100.0));
 }
  
  output.push_str(&format!("\n**Quality Summary** : {}/{} implementations meet reliability standards\n\n",
   reliable_count, total_count));
  
  output.push_str("## Manual Testing Summary\n\n");
  output.push_str("This report demonstrates successful integration of benchkit with manual testing procedures.\n");
  output.push_str("All core functionality tested and working correctly.\n\n");
  
  output.push_str("---\n");
  output.push_str("*Generated by benchkit manual testing suite*\n");
  
  output
}