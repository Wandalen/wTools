//! Data generators for benchmarking
//!
//! This module provides common data generation patterns based on learnings
//! from unilang and strs_tools benchmarking. It focuses on realistic test
//! data with configurable parameters.

/// Common data size patterns for benchmarking
#[derive(Debug, Clone, Copy)]
pub enum DataSize {
  /// Small dataset (typically 10 items)
  Small,
  /// Medium dataset (typically 100 items) 
  Medium,
  /// Large dataset (typically 1000 items)
  Large,
  /// Huge dataset (typically 10000 items)
  Huge,
  /// Custom size
  Custom(usize),
}

impl DataSize {
  /// Get the actual size value
  pub fn size(&self) -> usize {
    match self {
      DataSize::Small => 10,
      DataSize::Medium => 100, 
      DataSize::Large => 1000,
      DataSize::Huge => 10000,
      DataSize::Custom(size) => *size,
    }
  }

  /// Get standard size variants for iteration
  pub fn standard_sizes() -> Vec<DataSize> {
    vec![DataSize::Small, DataSize::Medium, DataSize::Large, DataSize::Huge]
  }
}

/// Generate list data with configurable size and delimiter
pub fn generate_list_data(size: DataSize) -> String {
  generate_list_data_with_delimiter(size, ",")
}

/// Generate list data with custom delimiter
pub fn generate_list_data_with_delimiter(size: DataSize, delimiter: &str) -> String {
  (1..=size.size())
    .map(|i| format!("item{}", i))
    .collect::<Vec<_>>()
    .join(delimiter)
}

/// Generate numeric list data
pub fn generate_numeric_list(size: DataSize) -> String {
  (1..=size.size())
    .map(|i| i.to_string())
    .collect::<Vec<_>>()
    .join(",")
}

/// Generate map/dictionary data with key-value pairs
pub fn generate_map_data(size: DataSize) -> String {
  generate_map_data_with_delimiters(size, ",", "=")
}

/// Generate map data with custom delimiters
pub fn generate_map_data_with_delimiters(size: DataSize, entry_delimiter: &str, kv_delimiter: &str) -> String {
  (1..=size.size())
    .map(|i| format!("key{}{kv_delimiter}value{}", i, i, kv_delimiter = kv_delimiter))
    .collect::<Vec<_>>()
    .join(entry_delimiter)
}

/// Generate enum choices data
pub fn generate_enum_data(size: DataSize) -> String {
  (1..=size.size())
    .map(|i| format!("choice{}", i))
    .collect::<Vec<_>>()
    .join(",")
}

/// Generate string data with controlled length
pub fn generate_string_data(length: usize) -> String {
  "a".repeat(length)
}

/// Generate string data with varying lengths
pub fn generate_variable_strings(count: usize, min_len: usize, max_len: usize) -> Vec<String> {
  let mut strings = Vec::with_capacity(count);
  let step = if count > 1 { (max_len - min_len) / (count - 1) } else { 0 };
  
  for i in 0..count {
    let len = min_len + (i * step);
    strings.push("x".repeat(len));
  }
  
  strings
}

/// Generate nested data structure (JSON-like)
pub fn generate_nested_data(depth: usize, width: usize) -> String {
  fn generate_level(current_depth: usize, max_depth: usize, width: usize) -> String {
    if current_depth >= max_depth {
      return format!("\"value{}\"", current_depth);
    }
    
    let items: Vec<String> = (0..width)
      .map(|i| {
        let key = format!("key{}", i);
        let value = generate_level(current_depth + 1, max_depth, width);
        format!("\"{}\": {}", key, value)
      })
      .collect();
    
    format!("{{{}}}", items.join(", "))
  }
  
  generate_level(0, depth, width)
}

/// Generate file path data
pub fn generate_file_paths(size: DataSize) -> Vec<String> {
  (1..=size.size())
    .map(|i| format!("/path/to/file{}.txt", i))
    .collect()
}

/// Generate URL data
pub fn generate_urls(size: DataSize) -> Vec<String> {
  (1..=size.size())
    .map(|i| format!("https://example{}.com/path", i))
    .collect()
}

/// Seeded random data generator using simple LCG
#[derive(Debug)]
pub struct SeededGenerator {
  seed: u64,
}

impl SeededGenerator {
  /// Create new seeded generator
  pub fn new(seed: u64) -> Self {
    Self { seed }
  }

  /// Generate next random number
  fn next(&mut self) -> u64 {
    // Simple Linear Congruential Generator
    self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
    self.seed
  }

  /// Generate random string of given length
  pub fn random_string(&mut self, length: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    (0..length)
      .map(|_| {
        let idx = (self.next() as usize) % CHARS.len();
        CHARS[idx] as char
      })
      .collect()
  }

  /// Generate random integer in range
  pub fn random_int(&mut self, min: i32, max: i32) -> i32 {
    let range = (max - min) as u64;
    min + ((self.next() % range) as i32)
  }

  /// Generate random vector of integers  
  pub fn random_vec(&mut self, size: usize, min: i32, max: i32) -> Vec<i32> {
    (0..size)
      .map(|_| self.random_int(min, max))
      .collect()
  }
}

/// Convenience function to generate random vector with default seed
pub fn generate_random_vec(size: usize) -> Vec<i32> {
  let mut gen = SeededGenerator::new(42);
  gen.random_vec(size, 1, 1000)
}

/// Generate test data for common parsing scenarios (based on unilang experience)
#[derive(Debug)]
pub struct ParsingTestData;

impl ParsingTestData {
  /// Generate command-line argument style data
  pub fn command_args(size: DataSize) -> String {
    (1..=size.size())
      .map(|i| format!("--arg{} value{}", i, i))
      .collect::<Vec<_>>()
      .join(" ")
  }

  /// Generate configuration file style data
  pub fn config_pairs(size: DataSize) -> String {
    (1..=size.size())
      .map(|i| format!("setting{}=value{}", i, i))
      .collect::<Vec<_>>()
      .join("\n")
  }

  /// Generate CSV-like data
  pub fn csv_data(rows: usize, cols: usize) -> String {
    let header = (1..=cols)
      .map(|i| format!("column{}", i))
      .collect::<Vec<_>>()
      .join(",");
    
    let mut lines = vec![header];
    
    for row in 1..=rows {
      let line = (1..=cols)
        .map(|col| format!("row{}col{}", row, col))
        .collect::<Vec<_>>()
        .join(",");
      lines.push(line);
    }
    
    lines.join("\n")
  }

  /// Generate JSON-like object data
  pub fn json_objects(size: DataSize) -> String {
    let objects: Vec<String> = (1..=size.size())
      .map(|i| format!(r#"{{"id": {}, "name": "object{}", "value": {}}}"#, i, i, i * 10))
      .collect();
    
    format!("[{}]", objects.join(", "))
  }
}

