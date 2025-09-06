//! Throughput calculation and analysis utilities
//!
//! This module provides tools for measuring and analyzing throughput metrics
//! including bytes/second, items/second, and various bandwidth calculations.

use crate::measurement::BenchmarkResult;
use std::time::Duration;
use std::collections::HashMap;

/// Throughput measurement and calculation utilities
#[derive(Debug, Clone)]
pub struct ThroughputAnalyzer
{
  /// Data size being processed (in bytes)
  pub data_size: u64,
  /// Number of items being processed
  pub item_count: Option<u64>,
  /// Operation description
  pub operation: String,
}

impl ThroughputAnalyzer
{
  /// Create a new throughput analyzer
  pub fn new(operation: impl Into<String>, data_size: u64) -> Self
  {
    Self
    {
      operation: operation.into(),
      data_size,
      item_count: None,
    }
  }

  /// Set the number of items processed
  pub fn with_items(mut self, item_count: u64) -> Self
  {
    self.item_count = Some(item_count);
    self
  }

  /// Calculate throughput metrics from benchmark result
  pub fn analyze(&self, result: &BenchmarkResult) -> ThroughputMetrics
  {
    let mean_duration = result.mean_time();
    let mean_seconds = mean_duration.as_secs_f64();
    
    let bytes_per_second = if mean_seconds > 0.0 
    {
      self.data_size as f64 / mean_seconds
    } 
    else 
    {
      0.0
    };
    
    let items_per_second = if let Some(items) = self.item_count 
    {
      if mean_seconds > 0.0 
      {
        Some(items as f64 / mean_seconds)
      } 
      else 
      {
        Some(0.0)
      }
    } 
    else 
    {
      None
    };

    ThroughputMetrics
    {
      operation: self.operation.clone(),
      data_size_bytes: self.data_size,
      item_count: self.item_count,
      processing_time: mean_duration,
      bytes_per_second,
      items_per_second,
      megabytes_per_second: bytes_per_second / (1024.0 * 1024.0),
      gigabytes_per_second: bytes_per_second / (1024.0 * 1024.0 * 1024.0),
    }
  }

  /// Analyze multiple benchmark results and compare throughput
  pub fn compare_throughput(&self, results: &HashMap<String, BenchmarkResult>) -> ThroughputComparison
  {
    let mut metrics = HashMap::new();
    
    for (name, result) in results 
    {
      let throughput = self.analyze(result);
      metrics.insert(name.clone(), throughput);
    }
    
    ThroughputComparison
    {
      operation: self.operation.clone(),
      metrics,
    }
  }
}

/// Throughput metrics for a single benchmark result
#[derive(Debug, Clone)]
pub struct ThroughputMetrics
{
  /// Operation being measured
  pub operation: String,
  /// Data size in bytes
  pub data_size_bytes: u64,
  /// Number of items processed (optional)
  pub item_count: Option<u64>,
  /// Processing time
  pub processing_time: Duration,
  /// Bytes processed per second
  pub bytes_per_second: f64,
  /// Items processed per second (if available)
  pub items_per_second: Option<f64>,
  /// Megabytes per second
  pub megabytes_per_second: f64,
  /// Gigabytes per second  
  pub gigabytes_per_second: f64,
}

impl ThroughputMetrics
{
  /// Get human-readable throughput description
  pub fn throughput_description(&self) -> String
  {
    if self.gigabytes_per_second >= 1.0 
    {
      format!("{:.2} GB/s", self.gigabytes_per_second)
    } 
    else if self.megabytes_per_second >= 1.0 
    {
      format!("{:.1} MB/s", self.megabytes_per_second)
    } 
    else if self.bytes_per_second >= 1024.0 
    {
      format!("{:.1} KB/s", self.bytes_per_second / 1024.0)
    } 
    else 
    {
      format!("{:.0} B/s", self.bytes_per_second)
    }
  }

  /// Get items per second description
  pub fn items_description(&self) -> Option<String>
  {
    self.items_per_second.map(|ips|
    {
      if ips >= 1_000_000.0 
      {
        format!("{:.1}M items/s", ips / 1_000_000.0)
      } 
      else if ips >= 1_000.0 
      {
        format!("{:.1}K items/s", ips / 1_000.0)
      } 
      else 
      {
        format!("{:.0} items/s", ips)
      }
    })
  }

  /// Generate markdown report for this throughput measurement
  pub fn to_markdown(&self) -> String
  {
    let mut report = String::new();
    
    report.push_str(&format!("### {} Throughput Analysis\n\n", self.operation));
    report.push_str(&format!("- **Data processed**: {} bytes ({:.1} KB)\n", 
                             self.data_size_bytes,
                             self.data_size_bytes as f64 / 1024.0));
    
    if let Some(items) = self.item_count 
    {
      report.push_str(&format!("- **Items processed**: {}\n", items));
    }
    
    report.push_str(&format!("- **Processing time**: {:.3?}\n", self.processing_time));
    report.push_str(&format!("- **Throughput**: {}\n", self.throughput_description()));
    
    if let Some(items_desc) = self.items_description() 
    {
      report.push_str(&format!("- **Item throughput**: {}\n", items_desc));
    }
    
    report.push('\n');
    report
  }
}

/// Comparison of throughput metrics across multiple implementations
#[derive(Debug, Clone)]
pub struct ThroughputComparison
{
  /// Operation being compared
  pub operation: String,
  /// Throughput metrics for each implementation
  pub metrics: HashMap<String, ThroughputMetrics>,
}

impl ThroughputComparison
{
  /// Get the fastest implementation by bytes per second
  pub fn fastest_throughput(&self) -> Option<(&String, &ThroughputMetrics)>
  {
    self.metrics
      .iter()
      .max_by(|a, b| a.1.bytes_per_second.partial_cmp(&b.1.bytes_per_second).unwrap())
  }

  /// Get the fastest implementation by items per second
  pub fn fastest_items(&self) -> Option<(&String, &ThroughputMetrics)>
  {
    self.metrics
      .iter()
      .filter_map(|(name, metrics)| metrics.items_per_second.map(|ips| (name, metrics, ips)))
      .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
      .map(|(name, metrics, _)| (name, metrics))
  }

  /// Calculate speedup factors relative to baseline
  pub fn calculate_speedups(&self, baseline: &str) -> Option<HashMap<String, f64>>
  {
    let baseline_throughput = self.metrics.get(baseline)?.bytes_per_second;
    
    if baseline_throughput <= 0.0 
    {
      return None;
    }
    
    let mut speedups = HashMap::new();
    
    for (name, metrics) in &self.metrics 
    {
      let speedup = metrics.bytes_per_second / baseline_throughput;
      speedups.insert(name.clone(), speedup);
    }
    
    Some(speedups)
  }

  /// Generate comprehensive throughput comparison report
  pub fn to_markdown(&self) -> String
  {
    let mut report = String::new();
    
    report.push_str(&format!("## {} Throughput Comparison\n\n", self.operation));
    
    // Executive summary
    if let Some((fastest_name, fastest_metrics)) = self.fastest_throughput() 
    {
      report.push_str(&format!("**Best performing**: {} ({})\n\n", 
                               fastest_name,
                               fastest_metrics.throughput_description()));
    }
    
    // Detailed results table
    report.push_str("| Implementation | Throughput | Items/sec | Processing Time | Efficiency |\n");
    report.push_str("|---------------|------------|-----------|-----------------|------------|\n");
    
    // Sort by throughput (fastest first)
    let mut sorted_metrics: Vec<_> = self.metrics.iter().collect();
    sorted_metrics.sort_by(|a, b| b.1.bytes_per_second.partial_cmp(&a.1.bytes_per_second).unwrap());
    
    for (name, metrics) in &sorted_metrics 
    {
      let items_desc = metrics.items_description().unwrap_or_else(|| "N/A".to_string());
      let efficiency = if metrics.data_size_bytes > 0 
      {
        format!("{:.1} MB/s per GB input", 
                metrics.megabytes_per_second / (metrics.data_size_bytes as f64 / 1_000_000_000.0))
      } 
      else 
      {
        "N/A".to_string()
      };
      
      report.push_str(&format!("| {} | {} | {} | {:.3?} | {} |\n",
                               name,
                               metrics.throughput_description(),
                               items_desc,
                               metrics.processing_time,
                               efficiency));
    }
    
    report.push('\n');
    
    // Speedup analysis if we have multiple implementations
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

/// Bandwidth analysis for different data types and patterns
#[derive(Debug, Clone)]
pub struct BandwidthAnalyzer;

impl BandwidthAnalyzer
{
  /// Analyze memory bandwidth utilization
  pub fn analyze_memory_bandwidth(
    data_size: u64,
    processing_time: Duration,
    read_passes: u32,
    write_passes: u32,
  ) -> MemoryBandwidthMetrics
  {
    let time_seconds = processing_time.as_secs_f64();
    let total_bytes_read = data_size * u64::from(read_passes);
    let total_bytes_written = data_size * u64::from(write_passes);
    let total_bytes = total_bytes_read + total_bytes_written;
    
    let bandwidth = if time_seconds > 0.0 
    {
      total_bytes as f64 / time_seconds
    } 
    else 
    {
      0.0
    };
    
    MemoryBandwidthMetrics
    {
      data_size,
      processing_time,
      read_passes,
      write_passes,
      total_bytes_accessed: total_bytes,
      bandwidth_bytes_per_second: bandwidth,
      bandwidth_gb_per_second: bandwidth / (1024.0 * 1024.0 * 1024.0),
    }
  }
}

/// Memory bandwidth utilization metrics
#[derive(Debug, Clone)]
pub struct MemoryBandwidthMetrics
{
  /// Size of data being processed
  pub data_size: u64,
  /// Time taken for processing
  pub processing_time: Duration,
  /// Number of read passes over the data
  pub read_passes: u32,
  /// Number of write passes over the data
  pub write_passes: u32,
  /// Total bytes accessed (reads + writes)
  pub total_bytes_accessed: u64,
  /// Memory bandwidth in bytes per second
  pub bandwidth_bytes_per_second: f64,
  /// Memory bandwidth in GB/s
  pub bandwidth_gb_per_second: f64,
}

impl MemoryBandwidthMetrics
{
  /// Get human-readable bandwidth description
  pub fn bandwidth_description(&self) -> String
  {
    if self.bandwidth_gb_per_second >= 1.0 
    {
      format!("{:.2} GB/s", self.bandwidth_gb_per_second)
    } 
    else 
    {
      format!("{:.0} MB/s", self.bandwidth_bytes_per_second / (1024.0 * 1024.0))
    }
  }

  /// Calculate memory efficiency percentage (vs theoretical peak)
  pub fn efficiency_vs_peak(&self, theoretical_peak_gb_s: f64) -> f64
  {
    if theoretical_peak_gb_s > 0.0 
    {
      (self.bandwidth_gb_per_second / theoretical_peak_gb_s) * 100.0
    } 
    else 
    {
      0.0
    }
  }
}

