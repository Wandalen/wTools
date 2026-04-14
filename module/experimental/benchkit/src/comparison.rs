//! Framework and algorithm comparison utilities
//!
//! This module provides specialized tools for comparing multiple frameworks,
//! libraries, or algorithm implementations against each other with detailed
//! analysis and insights.

use crate ::prelude :: *;
use std ::collections ::HashMap;

/// Multi-framework comparison configuration
#[ derive(Debug, Clone) ]
pub struct ComparisonConfig
{
  /// Name of the comparison study
  pub study_name: String,
  /// Scale factors to test each framework at
  pub scale_factors: Vec< usize >,
  /// Skip slow frameworks at large scales
  pub skip_slow_at_large_scale: bool,
  /// Threshold for "slow" (ops/sec below this value)
  pub slow_threshold: f64,
  /// Large scale threshold (skip slow frameworks above this scale)
  pub large_scale_threshold: usize,
}

impl Default for ComparisonConfig
{
  fn default() -> Self
  {
  Self
  {
   study_name: "Framework Comparison".to_string(),
   scale_factors: vec![10, 100, 1000, 10000],
   skip_slow_at_large_scale: true,
   slow_threshold: 1000.0, // ops/sec
   large_scale_threshold: 50000,
 }
 }
}

/// Framework comparison results
#[ derive(Debug) ]
pub struct FrameworkComparison
{
  /// Configuration used for comparison
  pub config: ComparisonConfig,
  /// Benchmark results organized by framework and scale
  pub results: HashMap< String, HashMap<usize, BenchmarkResult >>,
  /// Analyzed characteristics of each framework
  pub framework_characteristics: HashMap< String, FrameworkCharacteristics >,
}

/// Characteristics of a framework
#[ derive(Debug, Clone) ]
pub struct FrameworkCharacteristics
{
  /// Framework name
  pub name: String,
  /// Estimated algorithmic complexity
  pub estimated_complexity: String,
  /// Optimal scale range for this framework
  pub best_scale_range: String,
  /// Performance category classification
  pub performance_category: PerformanceCategory,
  /// Framework strengths
  pub strengths: Vec< String >,
  /// Framework weaknesses
  pub weaknesses: Vec< String >,
}

/// Performance category classification for frameworks
#[ derive(Debug, Clone) ]
pub enum PerformanceCategory
{
  /// Consistently fast across all scales
  HighPerformance,
  /// Gets better at larger scales
  ScalableOptimal,
  /// Good for small scales only
  SmallScaleOptimal,
  /// Decent across all scales
  GeneralPurpose,
  /// Consistently slow performance
  Poor,
}

impl FrameworkComparison
{
  /// Create new framework comparison
  pub fn new(config: ComparisonConfig) -> Self
  {
  Self
  {
   config,
   results: HashMap ::new(),
   framework_characteristics: HashMap ::new(),
 }
 }
  
  /// Add framework benchmark results
  pub fn add_framework_results(
  &mut self,
  framework_name: &str,
  results: HashMap< usize, BenchmarkResult >,
 )
  {
  // Analyze characteristics
  let characteristics = self.analyze_framework_characteristics(framework_name, &results);
  
  self.results.insert(framework_name.to_string(), results);
  self.framework_characteristics.insert(framework_name.to_string(), characteristics);
 }
  
  /// Analyze framework characteristics
  fn analyze_framework_characteristics(
  &self,
  framework_name: &str,
  results: &HashMap< usize, BenchmarkResult >,
 ) -> FrameworkCharacteristics
  {
  if results.is_empty()
  {
   return FrameworkCharacteristics
   {
  name: framework_name.to_string(),
  estimated_complexity: "Unknown".to_string(),
  best_scale_range: "Unknown".to_string(),
  performance_category: PerformanceCategory ::Poor,
  strengths: vec![],
  weaknesses: vec!["No benchmark data".to_string()],
 };
 }
  
  // Find performance at different scales
  let mut sorted_scales: Vec< _ > = results.keys().collect();
  sorted_scales.sort();
  
  let min_scale = *sorted_scales.first().unwrap();
  let max_scale = *sorted_scales.last().unwrap();
  
  let min_ops = results[&min_scale].operations_per_second();
  let max_ops = results[&max_scale].operations_per_second();
  
  // Estimate complexity
  let complexity = if results.len() > 1
  {
   let scale_ratio = *max_scale as f64 / *min_scale as f64;
   let perf_ratio = min_ops / max_ops; // Higher means better scaling
   
   if perf_ratio < 2.0
   {
  "O(1) - Constant".to_string()
 }
   else if perf_ratio < scale_ratio * 2.0
   {
  "O(n) - Linear".to_string()
 }
   else
   {
  "O(n²) or worse".to_string()
 }
 }
  else
  {
   "Unknown".to_string()
 };
  
  // Determine best scale range
  let best_scale = sorted_scales.iter()
   .max_by(|&&a, &&b| results[&a].operations_per_second()
  .partial_cmp(&results[&b].operations_per_second())
  .unwrap_or(std ::cmp ::Ordering ::Equal))
   .unwrap();
  
  let best_scale_range = if **best_scale < 100
  {
   "Small scales (< 100)".to_string()
 }
  else if **best_scale < 10000
  {
   "Medium scales (100-10K)".to_string()
 }
  else
  {
   "Large scales (> 10K)".to_string()
 };
  
  // Categorize performance
  let avg_ops = results.values()
   .map(|r| r.operations_per_second())
   .sum :: < f64 >() / results.len() as f64;
  
  let performance_category = if avg_ops > 100_000.0
  {
   PerformanceCategory ::HighPerformance
 }
  else if max_ops > min_ops * 2.0
  {
   PerformanceCategory ::ScalableOptimal
 }
  else if min_ops > max_ops * 2.0
  {
   PerformanceCategory ::SmallScaleOptimal
 }
  else if avg_ops > 1000.0
  {
   PerformanceCategory ::GeneralPurpose
 }
  else
  {
   PerformanceCategory ::Poor
 };
  
  // Generate strengths and weaknesses
  let mut strengths = Vec ::new();
  let mut weaknesses = Vec ::new();
  
  match performance_category
  {
   PerformanceCategory ::HighPerformance =>
   {
  strengths.push("Excellent performance across all scales".to_string());
  strengths.push("Suitable for high-throughput applications".to_string());
 }
   PerformanceCategory ::ScalableOptimal =>
   {
  strengths.push("Scales well with input size".to_string());
  strengths.push("Good choice for large-scale applications".to_string());
  weaknesses.push("May have overhead at small scales".to_string());
 }
   PerformanceCategory ::SmallScaleOptimal =>
   {
  strengths.push("Excellent performance at small scales".to_string());
  strengths.push("Low overhead for simple use cases".to_string());
  weaknesses.push("Performance degrades at larger scales".to_string());
 }
   PerformanceCategory ::GeneralPurpose =>
   {
  strengths.push("Consistent performance across scales".to_string());
  strengths.push("Good balance of features and performance".to_string());
 }
   PerformanceCategory ::Poor =>
   {
  weaknesses.push("Below-average performance".to_string());
  weaknesses.push("May not be suitable for performance-critical applications".to_string());
 }
 }
  
  FrameworkCharacteristics
  {
   name: framework_name.to_string(),
   estimated_complexity: complexity,
   best_scale_range,
   performance_category,
   strengths,
   weaknesses,
 }
 }
  
  /// Generate comprehensive comparison report
  pub fn generate_report( &self ) -> String
  {
  let mut output = String ::new();
  
  output.push_str(&format!("# {} Report\n\n", self.config.study_name));
  
  // Executive summary
  output.push_str("## Executive Summary\n\n");
  output.push_str(&self.generate_executive_summary());
  output.push_str("\n\n");
  
  // Performance comparison table
  output.push_str("## Performance Comparison\n\n");
  output.push_str(&self.generate_performance_table());
  output.push_str("\n\n");
  
  // Framework analysis
  output.push_str("## Framework Analysis\n\n");
  output.push_str(&self.generate_framework_analysis());
  output.push_str("\n\n");
  
  // Recommendations
  output.push_str("## Recommendations\n\n");
  output.push_str(&self.generate_recommendations());
  
  output
 }
  
  fn generate_executive_summary( &self ) -> String
  {
  let mut summary = String ::new();
  
  let total_frameworks = self.results.len();
  let total_tests = self.results.values()
   .map(|results| results.len())
   .sum :: < usize >();
  
  summary.push_str(&format!("Tested **{}** frameworks across **{}** different scales.\n\n", 
   total_frameworks, self.config.scale_factors.len()));
  
  // Find overall winner
  if let Some(winner) = self.find_overall_winner()
  {
   summary.push_str(&format!("**🏆 Overall Winner** : {} ", winner.0));
   summary.push_str(&format!("(avg {:.0} ops/sec)\n\n", winner.1));
 }
  
  summary.push_str(&format!("Total benchmark operations: {}\n", total_tests));
  
  summary
 }
  
  fn generate_performance_table( &self ) -> String
  {
  let mut output = String ::new();
  
  // Create table header
  output.push_str("| Framework |");
  for &scale in &self.config.scale_factors
  {
   let scale_display = if scale >= 1000
   {
  format!(" {}K |", scale / 1000)
 }
   else
   {
  format!(" {} |", scale)
 };
   output.push_str(&scale_display);
 }
  output.push_str(" Category |\n");
  
  output.push_str("|-----------|");
  for _ in &self.config.scale_factors
  {
   output.push_str("---------|");
 }
  output.push_str("----------|\n");
  
  // Fill table rows
  for framework_name in self.results.keys()
  {
   output.push_str(&format!("| **{}** |", framework_name));
   
   for &scale in &self.config.scale_factors
   {
  if let Some(result) = self.results[framework_name].get(&scale)
  {
   output.push_str(&format!(" {:.0} |", result.operations_per_second()));
 }
  else
  {
   output.push_str(" N/A |");
 }
 }
   
   if let Some(characteristics) = self.framework_characteristics.get(framework_name)
   {
  let category = match characteristics.performance_category
  {
   PerformanceCategory ::HighPerformance => "🚀 High Perf",
   PerformanceCategory ::ScalableOptimal => "📈 Scalable",
   PerformanceCategory ::SmallScaleOptimal => "⚡ Small Scale",
   PerformanceCategory ::GeneralPurpose => "⚖️ Balanced",
   PerformanceCategory ::Poor => "🐌 Needs Work",
 };
  output.push_str(&format!(" {} |\n", category));
 }
   else
   {
  output.push_str(" Unknown |\n");
 }
 }
  
  output
 }
  
  fn generate_framework_analysis( &self ) -> String
  {
  let mut output = String ::new();
  
  for (framework_name, characteristics) in &self.framework_characteristics
  {
   output.push_str(&format!("### {} Analysis\n\n", framework_name));
   output.push_str(&format!("- **Estimated Complexity** : {}\n", characteristics.estimated_complexity));
   output.push_str(&format!("- **Best Scale Range** : {}\n", characteristics.best_scale_range));
   
   if !characteristics.strengths.is_empty()
   {
  output.push_str("\n**Strengths** : \n");
  for strength in &characteristics.strengths
  {
   output.push_str(&format!("- ✅ {}\n", strength));
 }
 }
   
   if !characteristics.weaknesses.is_empty()
   {
  output.push_str("\n**Weaknesses** : \n");
  for weakness in &characteristics.weaknesses
  {
   output.push_str(&format!("- ⚠️ {}\n", weakness));
 }
 }
   
   output.push_str("\n");
 }
  
  output
 }
  
  fn generate_recommendations( &self ) -> String
  {
  let mut recommendations = String ::new();
  
  // Performance-based recommendations
  if let Some((winner_name, avg_perf)) = self.find_overall_winner()
  {
   recommendations.push_str("### For Maximum Performance\n\n");
   recommendations.push_str(&format!("Choose **{}** for the best overall performance ({:.0} ops/sec average).\n\n", 
  winner_name, avg_perf));
 }
  
  // Scale-specific recommendations
  recommendations.push_str("### Scale-Specific Recommendations\n\n");
  
  for &scale in &self.config.scale_factors
  {
   if let Some(best_at_scale) = self.find_best_at_scale(scale)
   {
  let scale_desc = if scale < 100 { "small" } else if scale < 10000 { "medium" } else { "large" };
  recommendations.push_str(&format!("- **{} scale ({})** : {} ({:.0} ops/sec)\n", 
   scale_desc, scale, best_at_scale.0, best_at_scale.1));
 }
 }
  
  recommendations
 }
  
  fn find_overall_winner( &self ) -> Option< (String, f64) >
  {
  let mut best_framework = None;
  let mut best_avg_performance = 0.0;
  
  for (framework_name, results) in &self.results
  {
   let avg_perf: f64 = results.values()
  .map(|r| r.operations_per_second())
  .sum :: < f64 >() / results.len() as f64;
   
   if avg_perf > best_avg_performance
   {
  best_avg_performance = avg_perf;
  best_framework = Some(framework_name.clone());
 }
 }
  
  best_framework.map(|name| (name, best_avg_performance))
 }
  
  fn find_best_at_scale(&self, scale: usize) -> Option< (String, f64) >
  {
  let mut best_framework = None;
  let mut best_performance = 0.0;
  
  for (framework_name, results) in &self.results
  {
   if let Some(result) = results.get(&scale)
   {
  let ops_per_sec = result.operations_per_second();
  if ops_per_sec > best_performance
  {
   best_performance = ops_per_sec;
   best_framework = Some(framework_name.clone());
 }
 }
 }
  
  best_framework.map(|name| (name, best_performance))
 }
}

