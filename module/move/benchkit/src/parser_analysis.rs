//! Parser-specific analysis utilities
//!
//! This module provides specialized analysis capabilities for parser benchmarking,
//! including command throughput metrics, parsing pipeline analysis, and 
//! parser-specific quality metrics.

use crate::measurement::BenchmarkResult;
use std::time::Duration;
use std::collections::HashMap;

/// Parser performance analyzer with command-specific metrics
#[derive(Debug, Clone)]
pub struct ParserAnalyzer
{
  /// Name of the parser being analyzed
  pub parser_name: String,
  /// Number of commands processed
  pub command_count: u64,
  /// Total characters in input
  pub character_count: u64,
  /// Average command complexity (arguments, nesting, etc.)
  pub average_complexity: f64,
}

impl ParserAnalyzer
{
  /// Create a new parser analyzer
  pub fn new(parser_name: impl Into<String>, command_count: u64, character_count: u64) -> Self
  {
    Self
    {
      parser_name: parser_name.into(),
      command_count,
      character_count,
      average_complexity: 1.0,
    }
  }
  
  /// Set the average command complexity
  pub fn with_complexity(mut self, complexity: f64) -> Self
  {
    self.average_complexity = complexity;
    self
  }
  
  /// Analyze parser performance metrics
  pub fn analyze(&self, result: &BenchmarkResult) -> ParserMetrics
  {
    let mean_duration = result.mean_time();
    let mean_seconds = mean_duration.as_secs_f64();
    
    let commands_per_second = if mean_seconds > 0.0 
    {
      self.command_count as f64 / mean_seconds
    }
    else
    {
      0.0
    };
    
    let characters_per_second = if mean_seconds > 0.0
    {
      self.character_count as f64 / mean_seconds  
    }
    else
    {
      0.0
    };
    
    let tokens_per_second = commands_per_second * self.average_complexity;
    
    ParserMetrics
    {
      parser_name: self.parser_name.clone(),
      processing_time: mean_duration,
      commands_per_second,
      characters_per_second,
      tokens_per_second,
      throughput_mb_per_second: characters_per_second / 1_048_576.0,
      command_count: self.command_count,
      character_count: self.character_count,
      average_complexity: self.average_complexity,
    }
  }
  
  /// Compare multiple parser implementations
  pub fn compare_parsers(&self, results: &HashMap<String, BenchmarkResult>) -> ParserComparison
  {
    let mut metrics = HashMap::new();
    
    for (name, result) in results
    {
      let parser_metrics = self.analyze(result);
      metrics.insert(name.clone(), parser_metrics);
    }
    
    ParserComparison
    {
      parser_name: self.parser_name.clone(),
      metrics,
    }
  }
}

/// Parser performance metrics
#[derive(Debug, Clone)]
pub struct ParserMetrics
{
  /// Parser name
  pub parser_name: String,
  /// Processing time
  pub processing_time: Duration,
  /// Commands processed per second
  pub commands_per_second: f64,
  /// Characters processed per second  
  pub characters_per_second: f64,
  /// Tokens processed per second (estimated)
  pub tokens_per_second: f64,
  /// Throughput in MB/s
  pub throughput_mb_per_second: f64,
  /// Total commands processed
  pub command_count: u64,
  /// Total characters processed
  pub character_count: u64,
  /// Average command complexity
  pub average_complexity: f64,
}

impl ParserMetrics
{
  /// Get human-readable commands per second description
  pub fn commands_description(&self) -> String
  {
    if self.commands_per_second >= 1_000_000.0
    {
      format!("{:.1}M cmd/s", self.commands_per_second / 1_000_000.0)
    }
    else if self.commands_per_second >= 1_000.0
    {
      format!("{:.1}K cmd/s", self.commands_per_second / 1_000.0)
    }
    else
    {
      format!("{:.0} cmd/s", self.commands_per_second)
    }
  }
  
  /// Get human-readable tokens per second description
  pub fn tokens_description(&self) -> String
  {
    if self.tokens_per_second >= 1_000_000.0
    {
      format!("{:.1}M tokens/s", self.tokens_per_second / 1_000_000.0)
    }
    else if self.tokens_per_second >= 1_000.0
    {
      format!("{:.1}K tokens/s", self.tokens_per_second / 1_000.0)
    }
    else
    {
      format!("{:.0} tokens/s", self.tokens_per_second)
    }
  }
  
  /// Get human-readable throughput description
  pub fn throughput_description(&self) -> String
  {
    if self.throughput_mb_per_second >= 1.0
    {
      format!("{:.1} MB/s", self.throughput_mb_per_second)
    }
    else
    {
      format!("{:.0} KB/s", self.characters_per_second / 1024.0)
    }
  }
  
  /// Generate markdown report for parser metrics
  pub fn to_markdown(&self) -> String
  {
    let mut report = String::new();
    
    report.push_str(&format!("### {} Parser Analysis\n\n", self.parser_name));
    
    report.push_str(&format!("- **Commands processed**: {} ({:.1} avg complexity)\n", 
                             self.command_count, self.average_complexity));
    report.push_str(&format!("- **Characters processed**: {} ({:.1} chars/cmd)\n", 
                             self.character_count, 
                             self.character_count as f64 / self.command_count as f64));
    report.push_str(&format!("- **Processing time**: {:.3?}\n", self.processing_time));
    
    report.push_str(&format!("- **Performance**:\n"));
    report.push_str(&format!("  - Commands: {}\n", self.commands_description()));
    report.push_str(&format!("  - Tokens: {}\n", self.tokens_description()));
    report.push_str(&format!("  - Throughput: {}\n", self.throughput_description()));
    
    report.push('\n');
    report
  }
}

/// Comparison of parser performance across implementations
#[derive(Debug, Clone)]
pub struct ParserComparison
{
  /// Parser name being compared
  pub parser_name: String,
  /// Parser metrics for each implementation
  pub metrics: HashMap<String, ParserMetrics>,
}

impl ParserComparison
{
  /// Get the fastest parser by commands per second
  pub fn fastest_parser(&self) -> Option<(&String, &ParserMetrics)>
  {
    self.metrics
      .iter()
      .max_by(|a, b| a.1.commands_per_second.partial_cmp(&b.1.commands_per_second).unwrap())
  }
  
  /// Get the highest throughput parser
  pub fn highest_throughput(&self) -> Option<(&String, &ParserMetrics)>
  {
    self.metrics
      .iter()
      .max_by(|a, b| a.1.throughput_mb_per_second.partial_cmp(&b.1.throughput_mb_per_second).unwrap())
  }
  
  /// Calculate performance speedups relative to baseline
  pub fn calculate_speedups(&self, baseline: &str) -> Option<HashMap<String, f64>>
  {
    let baseline_rate = self.metrics.get(baseline)?.commands_per_second;
    
    if baseline_rate <= 0.0
    {
      return None;
    }
    
    let mut speedups = HashMap::new();
    
    for (name, metrics) in &self.metrics
    {
      let speedup = metrics.commands_per_second / baseline_rate;
      speedups.insert(name.clone(), speedup);
    }
    
    Some(speedups)
  }
  
  /// Generate comprehensive parser comparison report
  pub fn to_markdown(&self) -> String
  {
    let mut report = String::new();
    
    report.push_str(&format!("## {} Parser Comparison\n\n", self.parser_name));
    
    // Executive summary
    if let Some((fastest_name, fastest_metrics)) = self.fastest_parser()
    {
      report.push_str(&format!("**Best performing parser**: {} ({})\n\n", 
                               fastest_name,
                               fastest_metrics.commands_description()));
    }
    
    // Detailed results table
    report.push_str("| Implementation | Commands/sec | Tokens/sec | Throughput | Avg Latency | Complexity |\n");
    report.push_str("|----------------|--------------|------------|------------|-------------|------------|\n");
    
    // Sort by commands per second (fastest first)
    let mut sorted_metrics: Vec<_> = self.metrics.iter().collect();
    sorted_metrics.sort_by(|a, b| b.1.commands_per_second.partial_cmp(&a.1.commands_per_second).unwrap());
    
    for (name, metrics) in &sorted_metrics
    {
      let avg_latency = if metrics.commands_per_second > 0.0 
      {
        format!("{:.1} μs", 1_000_000.0 / metrics.commands_per_second)
      }
      else
      {
        "N/A".to_string()
      };
      
      report.push_str(&format!(
        "| {} | {} | {} | {} | {} | {:.1} |\n",
        name,
        metrics.commands_description(),
        metrics.tokens_description(),
        metrics.throughput_description(),
        avg_latency,
        metrics.average_complexity
      ));
    }
    
    report.push('\n');
    
    // Speedup analysis
    if self.metrics.len() > 1
    {
      let slowest_name = sorted_metrics.last().unwrap().0;
      if let Some(speedups) = self.calculate_speedups(slowest_name)
      {
        report.push_str("### Performance Speedups\n\n");
        report.push_str(&format!("*Relative to {} (baseline)*\n\n", slowest_name));
        
        for (name, _metrics) in &sorted_metrics
        {
          if let Some(speedup) = speedups.get(*name)
          {
            if *name != slowest_name
            {
              report.push_str(&format!("- **{}**: {:.1}x faster\n", name, speedup));
            }
          }
        }
        report.push('\n');
      }
    }
    
    report
  }
}

/// Parser pipeline stage analysis
#[derive(Debug, Clone)]
pub struct ParserPipelineAnalyzer
{
  /// Pipeline stages and their names
  stage_names: Vec<String>,
  /// Results for each pipeline stage
  stage_results: HashMap<String, BenchmarkResult>,
}

impl ParserPipelineAnalyzer
{
  /// Create a new pipeline analyzer
  pub fn new() -> Self
  {
    Self
    {
      stage_names: Vec::new(),
      stage_results: HashMap::new(),
    }
  }
  
  /// Add a pipeline stage result
  pub fn add_stage(&mut self, name: impl Into<String>, result: BenchmarkResult) -> &mut Self
  {
    let stage_name = name.into();
    self.stage_names.push(stage_name.clone());
    self.stage_results.insert(stage_name, result);
    self
  }
  
  /// Analyze pipeline bottlenecks
  pub fn analyze_bottlenecks(&self) -> PipelineAnalysis
  {
    let mut stage_times = HashMap::new();
    let mut total_time = Duration::ZERO;
    
    for (name, result) in &self.stage_results
    {
      let mean_time = result.mean_time();
      stage_times.insert(name.clone(), mean_time);
      total_time += mean_time;
    }
    
    // Find bottleneck (slowest stage)
    let bottleneck = stage_times
      .iter()
      .max_by(|a, b| a.1.cmp(b.1))
      .map(|(name, time)| (name.clone(), *time));
    
    // Calculate stage percentages
    let mut stage_percentages = HashMap::new();
    if total_time > Duration::ZERO
    {
      for (name, time) in &stage_times
      {
        let percentage = time.as_secs_f64() / total_time.as_secs_f64() * 100.0;
        stage_percentages.insert(name.clone(), percentage);
      }
    }
    
    PipelineAnalysis
    {
      stage_times,
      stage_percentages,
      bottleneck,
      total_time,
      stage_count: self.stage_results.len(),
    }
  }
  
  /// Generate pipeline analysis report
  pub fn to_markdown(&self) -> String
  {
    let analysis = self.analyze_bottlenecks();
    let mut report = String::new();
    
    report.push_str("## Parser Pipeline Analysis\n\n");
    
    if let Some((bottleneck_name, bottleneck_time)) = &analysis.bottleneck
    {
      report.push_str(&format!("**Primary bottleneck**: {} ({:.2?})\n", 
                               bottleneck_name, bottleneck_time));
      
      if let Some(percentage) = analysis.stage_percentages.get(bottleneck_name)
      {
        report.push_str(&format!("**Bottleneck impact**: {:.1}% of total processing time\n\n", percentage));
      }
    }
    
    // Pipeline breakdown table
    report.push_str("| Pipeline Stage | Time | Percentage | Reliability |\n");
    report.push_str("|----------------|------|------------|-------------|\n");
    
    // Sort stages by order they were added (pipeline order)
    for stage_name in &self.stage_names
    {
      if let (Some(time), Some(percentage), Some(result)) = (
        analysis.stage_times.get(stage_name),
        analysis.stage_percentages.get(stage_name),
        self.stage_results.get(stage_name)
      ) {
        let reliability = if result.is_reliable() { "✅ Reliable" } else { "⚠️ Variable" };
        
        report.push_str(&format!(
          "| {} | {:.2?} | {:.1}% | {} |\n",
          stage_name, time, percentage, reliability
        ));
      }
    }
    
    report.push('\n');
    
    // Performance recommendations
    report.push_str("### Optimization Recommendations\n\n");
    
    if let Some((bottleneck_name, _)) = &analysis.bottleneck
    {
      if let Some(percentage) = analysis.stage_percentages.get(bottleneck_name)
      {
        if *percentage > 50.0
        {
          report.push_str(&format!("🎯 **High Priority**: Optimize {} stage ({:.1}% of total time)\n", 
                                   bottleneck_name, percentage));
        }
        else if *percentage > 25.0
        {
          report.push_str(&format!("⚡ **Medium Priority**: Consider optimizing {} stage ({:.1}% of total time)\n", 
                                   bottleneck_name, percentage));
        }
      }
    }
    
    // Check for unreliable stages
    for (stage_name, result) in &self.stage_results
    {
      if !result.is_reliable()
      {
        let cv = result.coefficient_of_variation() * 100.0;
        report.push_str(&format!("📊 **Reliability Issue**: {} stage has high variability (CV: {:.1}%)\n", 
                                 stage_name, cv));
      }
    }
    
    report.push('\n');
    report
  }
}

/// Analysis results for parser pipeline
#[derive(Debug, Clone)]
pub struct PipelineAnalysis
{
  /// Time taken by each stage
  pub stage_times: HashMap<String, Duration>,
  /// Percentage of total time for each stage
  pub stage_percentages: HashMap<String, f64>,
  /// Primary bottleneck (stage name and time)
  pub bottleneck: Option<(String, Duration)>,
  /// Total pipeline time
  pub total_time: Duration,
  /// Number of stages analyzed
  pub stage_count: usize,
}

impl Default for ParserPipelineAnalyzer
{
  fn default() -> Self
  {
    Self::new()
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use std::time::Duration;

  fn create_test_result(time_ms: u64) -> BenchmarkResult
  {
    let times = vec![Duration::from_millis(time_ms); 5];
    BenchmarkResult::new("test", times)
  }

  #[test]
  fn test_parser_analyzer()
  {
    let analyzer = ParserAnalyzer::new("test_parser", 100, 5000);
    let result = create_test_result(100); // 100ms
    
    let metrics = analyzer.analyze(&result);
    
    assert_eq!(metrics.command_count, 100);
    assert_eq!(metrics.character_count, 5000);
    assert!(metrics.commands_per_second > 0.0);
    assert!(metrics.characters_per_second > 0.0);
  }

  #[test]
  fn test_parser_comparison()
  {
    let analyzer = ParserAnalyzer::new("comparison_test", 50, 2500);
    
    let mut results = HashMap::new();
    results.insert("fast_parser".to_string(), create_test_result(50));
    results.insert("slow_parser".to_string(), create_test_result(200));
    
    let comparison = analyzer.compare_parsers(&results);
    
    assert_eq!(comparison.metrics.len(), 2);
    
    let (fastest_name, _) = comparison.fastest_parser().unwrap();
    assert_eq!(fastest_name, "fast_parser");
  }

  #[test]
  fn test_pipeline_analyzer()
  {
    let mut analyzer = ParserPipelineAnalyzer::new();
    
    analyzer
      .add_stage("tokenization", create_test_result(50))
      .add_stage("parsing", create_test_result(100))
      .add_stage("ast_build", create_test_result(25));
    
    let analysis = analyzer.analyze_bottlenecks();
    
    assert_eq!(analysis.stage_count, 3);
    assert!(analysis.bottleneck.is_some());
    
    let (bottleneck_name, _) = analysis.bottleneck.unwrap();
    assert_eq!(bottleneck_name, "parsing");
  }
}