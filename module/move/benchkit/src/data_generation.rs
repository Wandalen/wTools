//! Advanced data generation utilities for benchmarking
//!
//! This module provides sophisticated data generators that create realistic
//! test datasets for benchmarking. Supports pattern-based generation,
//! scaling, and various data complexity levels.

use crate::generators::DataSize;
use std::collections::HashMap;

/// Advanced data generator with pattern-based generation capabilities
#[derive(Debug, Clone)]
pub struct DataGenerator
{
  /// Pattern template for data generation (e.g., "item{},field{}")
  pub pattern: Option<String>,
  /// Target size 
  pub size: Option<DataSize>,
  /// Target size in bytes (alternative to size)
  pub size_bytes: Option<usize>,
  /// Number of repetitions for pattern-based generation
  pub repetitions: Option<usize>,
  /// Complexity level affecting data characteristics
  pub complexity: DataComplexity,
  /// Random seed for reproducible generation
  pub seed: Option<u64>,
  /// Custom parameters for pattern substitution
  pub parameters: HashMap<String, String>,
}

/// Data complexity levels affecting generation characteristics
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataComplexity
{
  /// Simple patterns with minimal variation
  Simple,
  /// Moderate patterns with some complexity
  Medium,
  /// Complex patterns with high variation and nested structures
  Complex,
  /// Full complexity with maximum variation and realistic edge cases
  Full,
}

impl Default for DataGenerator
{
  fn default() -> Self
  {
    Self
    {
      pattern: None,
      size: None,
      size_bytes: None,
      repetitions: None,
      complexity: DataComplexity::Medium,
      seed: None,
      parameters: HashMap::new(),
    }
  }
}

impl DataGenerator
{
  /// Create a new data generator
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set the pattern template for generation
  pub fn pattern(mut self, pattern: &str) -> Self
  {
    self.pattern = Some(pattern.to_string());
    self
  }

  /// Set target size for generated data
  pub fn size(mut self, size: usize) -> Self
  {
    self.size = Some(DataSize::Custom(size));
    self
  }

  /// Set target size in bytes
  pub fn size_bytes(mut self, bytes: usize) -> Self
  {
    self.size_bytes = Some(bytes);
    self
  }

  /// Set number of pattern repetitions
  pub fn repetitions(mut self, repetitions: usize) -> Self
  {
    self.repetitions = Some(repetitions);
    self
  }

  /// Set data complexity level
  pub fn complexity(mut self, complexity: DataComplexity) -> Self
  {
    self.complexity = complexity;
    self
  }

  /// Set random seed for reproducible generation
  pub fn seed(mut self, seed: u64) -> Self
  {
    self.seed = Some(seed);
    self
  }

  /// Add custom parameter for pattern substitution
  pub fn parameter(mut self, key: &str, value: &str) -> Self
  {
    self.parameters.insert(key.to_string(), value.to_string());
    self
  }

  /// Generate string data based on configuration
  pub fn generate_string(&self) -> String
  {
    match (&self.pattern, &self.size, &self.size_bytes, &self.repetitions) 
    {
      // Pattern-based generation with repetitions
      (Some(pattern), _, _, Some(reps)) => self.generate_pattern_string(pattern, *reps),
      
      // Pattern-based generation with size target
      (Some(pattern), Some(size), _, _) => self.generate_sized_pattern_string(pattern, size.size()),
      
      // Pattern-based generation with byte size target
      (Some(pattern), _, Some(bytes), _) => self.generate_sized_pattern_string_bytes(pattern, *bytes),
      
      // Size-based generation without pattern
      (None, Some(size), _, _) => self.generate_sized_string_items(size.size()),
      
      // Byte size-based generation without pattern
      (None, _, Some(bytes), _) => self.generate_sized_string_bytes(*bytes),
      
      // Default generation
      _ => self.generate_default_string(),
    }
  }

  /// Generate vector of strings
  pub fn generate_strings(&self, count: usize) -> Vec<String>
  {
    (0..count).map(|i| 
    {
      // Add variation by modifying seed
      let mut generator = self.clone();
      if let Some(base_seed) = self.seed 
      {
        generator.seed = Some(base_seed + i as u64);
      }
      generator.generate_string()
    }).collect()
  }

  /// Generate test data for CSV-like workloads
  pub fn generate_csv_data(&self, rows: usize, columns: usize) -> String
  {
    let mut csv = String::new();
    
    for row in 0..rows 
    {
      let mut row_data = Vec::new();
      for col in 0..columns 
      {
        let cell_data = match self.complexity 
        {
          DataComplexity::Simple => format!("field{}_{}", col, row),
          DataComplexity::Medium => format!("data_{}_{}_value", col, row),
          DataComplexity::Complex => format!("complex_field_{}_{}_with_special_chars@#$%", col, row),
          DataComplexity::Full => format!("full_complexity_field_{}_{}_with_unicode_🦀_and_escapes\\\"quotes\\\"", col, row),
        };
        row_data.push(cell_data);
      }
      csv.push_str(&row_data.join(","));
      csv.push('\n');
    }
    
    csv
  }

  /// Generate realistic unilang command data
  pub fn generate_unilang_commands(&self, count: usize) -> Vec<String>
  {
    let namespaces = ["math", "string", "file", "network", "system"];
    let commands = ["process", "parse", "transform", "validate", "execute"];
    let args = ["input", "output", "config", "flags", "options"];
    
    (0..count).map(|i|
    {
      let ns = namespaces[i % namespaces.len()];
      let cmd = commands[i % commands.len()];
      let arg = args[i % args.len()];
      
      match self.complexity
      {
        DataComplexity::Simple => format!("{}.{}", ns, cmd),
        DataComplexity::Medium => format!("{}.{} {}::value", ns, cmd, arg),
        DataComplexity::Complex => format!("{}.{} {}::value,flag::true,count::{}", ns, cmd, arg, i),
        DataComplexity::Full => format!("{}.{} {}::complex_value_with_specials@#$,flag::true,count::{},nested::{{key::{},array::[1,2,3]}}", ns, cmd, arg, i, i),
      }
    }).collect()
  }

  /// Generate data for memory allocation testing
  pub fn generate_allocation_test_data(&self, base_size: usize, fragment_count: usize) -> Vec<String>
  {
    (0..fragment_count).map(|i|
    {
      let size = base_size + (i * 17) % 100; // Vary sizes for realistic allocation patterns
      match self.complexity
      {
        DataComplexity::Simple => "a".repeat(size),
        DataComplexity::Medium => {
          let pattern = format!("data_{}_", i).repeat(size / 10 + 1);
          pattern[..size.min(pattern.len())].to_string()
        },
        DataComplexity::Complex => {
          let pattern = format!("complex_data_{}_{}", i, "x".repeat(i % 50)).repeat(size / 30 + 1);
          pattern[..size.min(pattern.len())].to_string()
        },
        DataComplexity::Full => {
          let pattern = format!("full_complexity_{}_{}_unicode_🦀_{}", i, "pattern".repeat(i % 10), "end").repeat(size / 50 + 1);
          pattern[..size.min(pattern.len())].to_string()
        },
      }
    }).collect()
  }

  // Private helper methods

  fn generate_pattern_string(&self, pattern: &str, repetitions: usize) -> String
  {
    let mut result = String::new();
    
    for i in 0..repetitions 
    {
      let expanded = self.expand_pattern(pattern, i);
      result.push_str(&expanded);
    }
    
    result
  }

  fn generate_sized_pattern_string(&self, pattern: &str, target_items: usize) -> String
  {
    let target_bytes = target_items * 10; // Estimate 10 bytes per item
    self.generate_sized_pattern_string_bytes(pattern, target_bytes)
  }
  
  fn generate_sized_pattern_string_bytes(&self, pattern: &str, target_bytes: usize) -> String
  {
    let mut result = String::new();
    let mut counter = 0;
    
    while result.len() < target_bytes 
    {
      let expanded = self.expand_pattern(pattern, counter);
      result.push_str(&expanded);
      counter += 1;
      
      // Safety valve to prevent infinite loops
      if counter > 1_000_000 
      {
        break;
      }
    }
    
    // Truncate to exact size if needed
    if result.len() > target_bytes 
    {
      result.truncate(target_bytes);
    }
    
    result
  }

  fn generate_sized_string_items(&self, items: usize) -> String
  {
    let target_bytes = items * 10; // Estimate 10 bytes per item
    self.generate_sized_string_bytes(target_bytes)
  }

  fn generate_sized_string_bytes(&self, target_bytes: usize) -> String
  {
    match self.complexity 
    {
      DataComplexity::Simple => "abcd,".repeat(target_bytes / 5 + 1)[..target_bytes].to_string(),
      DataComplexity::Medium => "field:value,".repeat(target_bytes / 12 + 1)[..target_bytes].to_string(),
      DataComplexity::Complex => "complex_field:complex_value;flag!option#tag@host&param%data|pipe+plus-minus=equals_under~tilde^caret*star,".repeat(target_bytes / 80 + 1)[..target_bytes].to_string(),
      DataComplexity::Full => "full_complexity_field:complex_value_with_unicode_🦀_special_chars@#$%^&*()_+-=[]{}|\\:;\"'<>?,./;flag!option#tag@host&param%data|pipe+plus-minus=equals_under~tilde^caret*star/slash\\backslash,".repeat(target_bytes / 150 + 1)[..target_bytes].to_string(),
    }
  }

  fn generate_default_string(&self) -> String
  {
    self.generate_sized_string_items(100)
  }

  fn expand_pattern(&self, pattern: &str, index: usize) -> String
  {
    let mut result = pattern.to_string();
    
    // Replace {} with counter
    result = result.replace("{}", &index.to_string());
    
    // Replace custom parameters
    for (key, value) in &self.parameters 
    {
      result = result.replace(&format!("{{{}}}", key), value);
    }
    
    // Add complexity-based variations
    match self.complexity 
    {
      DataComplexity::Simple => result,
      DataComplexity::Medium => 
      {
        if index % 10 == 0 
        {
          result.push_str("_variant");
        }
        result
      },
      DataComplexity::Complex => 
      {
        if index % 5 == 0 
        {
          result.push_str("_complex@#$");
        }
        result
      },
      DataComplexity::Full => 
      {
        if index % 3 == 0 
        {
          result.push_str("_full_unicode_🦀_special");
        }
        result
      },
    }
  }
}

/// Convenient builder pattern functions for common data generation scenarios
impl DataGenerator
{
  /// Generate CSV benchmark data
  pub fn csv() -> Self
  {
    Self::new().complexity(DataComplexity::Medium)
  }

  /// Generate log file benchmark data
  pub fn log_data() -> Self  
  {
    Self::new()
      .pattern("[{}] INFO: Processing request {} with status OK")
      .complexity(DataComplexity::Medium)
  }

  /// Generate command line parsing data
  pub fn command_line() -> Self
  {
    Self::new().complexity(DataComplexity::Complex)
  }

  /// Generate configuration file data
  pub fn config_file() -> Self
  {
    Self::new()
      .pattern("setting_{}=value_{}\n")
      .complexity(DataComplexity::Medium)
  }

  /// Generate JSON-like data
  pub fn json_like() -> Self
  {
    Self::new()
      .pattern("{{\"key_{}\": \"value_{}\", \"number\": {}}},")
      .complexity(DataComplexity::Complex)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_pattern_generation()
  {
    let generator = DataGenerator::new()
      .pattern("item{},")
      .repetitions(3)
      .complexity(DataComplexity::Simple); // Use simple complexity to avoid variations
    
    let result = generator.generate_string();
    assert_eq!(result, "item0,item1,item2,");
  }

  #[test]  
  fn test_size_based_generation()
  {
    let generator = DataGenerator::new()
      .size_bytes(50)
      .complexity(DataComplexity::Simple);
    
    let result = generator.generate_string();
    assert_eq!(result.len(), 50);
  }

  #[test]
  fn test_complexity_variations()
  {
    let simple = DataGenerator::new()
      .complexity(DataComplexity::Simple)
      .size(10)
      .generate_string();
    
    let complex = DataGenerator::new()
      .complexity(DataComplexity::Full)
      .size(10)
      .generate_string();
    
    // Complex should have more varied content
    assert!(complex.chars().any(|c| !simple.contains(c)));
  }

  #[test]
  fn test_csv_generation()
  {
    let generator = DataGenerator::new().complexity(DataComplexity::Medium);
    let csv_data = generator.generate_csv_data(3, 2);
    
    let lines: Vec<&str> = csv_data.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains(","));
  }

  #[test]
  fn test_unilang_command_generation()
  {
    let generator = DataGenerator::new().complexity(DataComplexity::Complex);
    let commands = generator.generate_unilang_commands(5);
    
    assert_eq!(commands.len(), 5);
    assert!(commands.iter().all(|cmd| cmd.contains(".")));
  }

  #[test]
  fn test_reproducible_generation()
  {
    let gen1 = DataGenerator::new().seed(42).pattern("test{}").repetitions(3);
    let gen2 = DataGenerator::new().seed(42).pattern("test{}").repetitions(3);
    
    assert_eq!(gen1.generate_string(), gen2.generate_string());
  }
}