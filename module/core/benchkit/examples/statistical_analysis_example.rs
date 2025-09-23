//! Example demonstrating benchkit's research-grade statistical analysis
//! 
//! Run with: `cargo run --example statistical_analysis_example --features statistical_analysis`

#[ cfg(feature = "statistical_analysis") ]
use benchkit ::prelude :: *;

#[ cfg(feature = "statistical_analysis") ]
type Result< T > = core ::result ::Result< T, Box<dyn core ::error ::Error >>;

#[ cfg(feature = "statistical_analysis") ]
fn main() -> Result< () >
{
  use core ::time ::Duration;
  use std ::collections ::HashMap;
  
  println!("📊 Benchkit Research-Grade Statistical Analysis Example");
  println!("=======================================================");
  
  // Create sample benchmark results with different statistical quality
  
  // High quality result: low variation, sufficient samples
  let high_quality_times: Vec< Duration > = (0..20)
  .map(|i| Duration ::from_millis(100 + (i % 3))) // 100-102ms range
  .collect();
  let high_quality_result = BenchmarkResult ::new("high_quality_algorithm", high_quality_times);
  
  // Poor quality result: high variation, fewer samples
  let poor_quality_times: Vec< Duration > = vec![
  Duration ::from_millis(95),
  Duration ::from_millis(180), // Outlier
  Duration ::from_millis(105),
  Duration ::from_millis(110),
  Duration ::from_millis(200), // Another outlier
 ];
  let poor_quality_result = BenchmarkResult ::new("poor_quality_algorithm", poor_quality_times);
  
  // Medium quality result
  let medium_quality_times: Vec< Duration > = (0..15)
  .map(|i| Duration ::from_millis(150 + (i * 2) % 10)) // 150-159ms range
  .collect();
  let medium_quality_result = BenchmarkResult ::new("medium_quality_algorithm", medium_quality_times);
  
  println!("1️⃣ Statistical Analysis of Individual Results");
  println!("============================================\n");
  
  // Analyze each result individually
  for result in [&high_quality_result, &medium_quality_result, &poor_quality_result] 
  {
  println!("📈 Analyzing: {}", result.name);
  let analysis = StatisticalAnalysis ::analyze(result, SignificanceLevel ::Standard)?;
  
  println!("  Mean: {:.2?} ± {:.2?} (95% CI)", 
  analysis.mean_confidence_interval.point_estimate,
  analysis.mean_confidence_interval.margin_of_error);
  println!("  CV: {:.1}%", analysis.coefficient_of_variation * 100.0);
  println!("  Statistical Power: {:.3}", analysis.statistical_power);
  println!("  Outliers: {}", analysis.outlier_count);
  println!("  Quality: {}", if analysis.is_reliable() { "✅ Research-grade" } else { "⚠️ Needs improvement" });
  
  if !analysis.is_reliable() 
  {
   println!("  📋 Full Report: ");
   println!("{}", analysis.generate_report());
 }
  println!();
 }
  
  println!("2️⃣ Statistical Comparison Between Algorithms");
  println!("==========================================\n");
  
  // Compare high quality vs medium quality
  let comparison = StatisticalAnalysis ::compare(
  &high_quality_result,
  &medium_quality_result, 
  SignificanceLevel ::Standard
 )?;
  
  println!("Comparing: {} vs {}", high_quality_result.name, medium_quality_result.name);
  println!("  Test statistic: {:.4}", comparison.test_statistic);
  println!("  P-value: {:.4}", comparison.p_value);  
  println!("  Effect size: {:.4} ({})", comparison.effect_size, comparison.effect_size_interpretation());
  println!("  Significant: {}", if comparison.is_significant { "Yes" } else { "No" });
  println!("  Conclusion: {}", comparison.conclusion());
  println!();
  
  println!("3️⃣ Comprehensive Statistical Report Generation");
  println!("============================================\n");
  
  // Create comprehensive report with all results
  let mut results = HashMap ::new();
  results.insert(high_quality_result.name.clone(), high_quality_result);
  results.insert(medium_quality_result.name.clone(), medium_quality_result); 
  results.insert(poor_quality_result.name.clone(), poor_quality_result);
  
  let report_generator = ReportGenerator ::new("Statistical Analysis Demo", results);
  
  // Generate research-grade statistical report
  let statistical_report = report_generator.generate_statistical_report();
  println!("{statistical_report}");
  
  // Save report to file
  let report_path = "target/statistical_analysis_report.md";
  std ::fs ::write(report_path, &statistical_report)?;
  println!("📝 Full statistical report saved to: {report_path}");
  
  println!("\n🎓 Key Research-Grade Features Demonstrated: ");
  println!("  ✅ Confidence intervals with proper t-distribution");
  println!("  ✅ Effect size calculation (Cohen's d)");
  println!("  ✅ Statistical significance testing (Welch's t-test)"); 
  println!("  ✅ Normality testing for data validation");
  println!("  ✅ Outlier detection using IQR method");
  println!("  ✅ Statistical power analysis"); 
  println!("  ✅ Coefficient of variation for reliability assessment");
  println!("  ✅ Research methodology documentation");
  
  Ok(())
}

#[ cfg(not(feature = "statistical_analysis")) ]
fn main() 
{
  println!("⚠️  Statistical analysis disabled - enable 'statistical_analysis' feature");
}