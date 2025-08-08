//! ## Test Matrix for Data Generation Functionality
//!
//! This test suite validates data generation utilities for benchmarking.
//!
//! ### Test Factors
//! - Data Size: Small (10), Medium (100), Large (1000), Huge (10000), Custom
//! - Data Type: Lists, Maps, Strings, Nested structures, File paths, URLs
//! - Generation Method: Static patterns, Seeded random, Parsing test data
//!
//! ### Test Combinations
//! | ID   | Data Type    | Size   | Method       | Expected Behavior                    |
//! |------|--------------|--------|--------------|--------------------------------------|
//! | G1.1 | List         | Small  | Static       | 10 comma-separated items             |
//! | G1.2 | List         | Custom | Static       | Exact count specified                |
//! | G1.3 | Map          | Medium | Static       | 100 key-value pairs                 |
//! | G1.4 | String       | Custom | Static       | Exact length string                 |
//! | G1.5 | Nested       | Custom | Static       | Controlled depth/width structure    |
//! | G1.6 | Random       | Custom | Seeded       | Reproducible with same seed         |
//! | G1.7 | Parsing      | Small  | Test data    | Command args, CSV, JSON formats     |
//! | G1.8 | File paths   | Large  | Static       | 1000 valid file path strings       |

use super::*;

/// Tests basic list data generation with small size
/// Test Combination: G1.1
#[test]
fn test_small_list_generation()
{
  let data = generate_list_data(DataSize::Small);
  let items: Vec<&str> = data.split(',').collect();
  
  assert_eq!(items.len(), 10, "Small size should generate 10 items");
  assert_eq!(items[0], "item1", "First item should be 'item1'");
  assert_eq!(items[9], "item10", "Last item should be 'item10'");
  assert!(!data.is_empty(), "Generated data should not be empty");
}

/// Tests custom size list generation
/// Test Combination: G1.2
#[test]
fn test_custom_size_list_generation()
{
  let custom_size = DataSize::Custom(25);
  let data = generate_list_data(custom_size);
  let items: Vec<&str> = data.split(',').collect();
  
  assert_eq!(items.len(), 25, "Custom size should generate exact count");
  assert_eq!(items[0], "item1", "First item format should be consistent");
  assert_eq!(items[24], "item25", "Last item should match custom size");
}

/// Tests map data generation with medium size
/// Test Combination: G1.3
#[test]  
fn test_medium_map_generation()
{
  let data = generate_map_data(DataSize::Medium);
  let pairs: Vec<&str> = data.split(',').collect();
  
  assert_eq!(pairs.len(), 100, "Medium size should generate 100 pairs");
  
  // Check first and last pairs format
  assert!(pairs[0].contains("key1=value1"), "First pair should be key1=value1");
  assert!(pairs[99].contains("key100=value100"), "Last pair should be key100=value100");
  
  // Verify all pairs have correct format
  for pair in pairs.iter().take(5) { // Check first 5
    assert!(pair.contains('='), "Each pair should contain '=' separator");
    assert!(pair.starts_with("key"), "Each pair should start with 'key'");
  }
}

/// Tests string generation with custom length
/// Test Combination: G1.4
#[test]
fn test_custom_length_string_generation()
{
  let short_string = generate_string_data(5);
  assert_eq!(short_string.len(), 5, "Should generate exact length string");
  assert_eq!(short_string, "aaaaa", "Should repeat specified character");
  
  let long_string = generate_string_data(1000);
  assert_eq!(long_string.len(), 1000, "Should handle large string lengths");
  
  let empty_string = generate_string_data(0);
  assert!(empty_string.is_empty(), "Should handle zero length");
}

/// Tests nested data structure generation
/// Test Combination: G1.5
#[test]
fn test_nested_structure_generation()
{
  let nested = generate_nested_data(2, 3);
  
  // Should be valid JSON-like structure
  assert!(nested.starts_with('{'), "Should start with opening brace");
  assert!(nested.ends_with('}'), "Should end with closing brace");
  assert!(nested.contains("key0"), "Should contain expected keys");
  assert!(nested.contains("key1"), "Should contain multiple keys");
  assert!(nested.contains("key2"), "Should respect width parameter");
  
  // Test depth = 1 (no nesting)
  let shallow = generate_nested_data(1, 2);
  assert!(shallow.contains("value"), "Depth 1 should contain value strings");
}

/// Tests seeded random generation reproducibility
/// Test Combination: G1.6
#[test]
fn test_seeded_random_reproducibility()
{
  let mut gen1 = SeededGenerator::new(42);
  let mut gen2 = SeededGenerator::new(42);
  
  // Same seed should produce identical sequences
  assert_eq!(
    gen1.random_string(10), 
    gen2.random_string(10),
    "Same seed should produce identical strings"
  );
  
  assert_eq!(
    gen1.random_int(1, 100),
    gen2.random_int(1, 100),
    "Same seed should produce identical integers"
  );
  
  let vec1 = gen1.random_vec(5, 1, 100);
  let vec2 = gen2.random_vec(5, 1, 100);
  assert_eq!(vec1, vec2, "Same seed should produce identical vectors");
}

/// Tests parsing test data generation
/// Test Combination: G1.7
#[test]
fn test_parsing_test_data_generation()
{
  // Test command arguments format
  let args = ParsingTestData::command_args(DataSize::Small);
  assert!(args.contains("--arg1 value1"), "Should contain first argument");
  assert!(args.contains("--arg10 value10"), "Should contain last argument");
  assert_eq!(args.matches("--arg").count(), 10, "Should have correct number of arguments");
  
  // Test configuration format
  let config = ParsingTestData::config_pairs(DataSize::Small);
  let lines: Vec<&str> = config.lines().collect();
  assert_eq!(lines.len(), 10, "Should have 10 configuration lines");
  assert!(lines[0].contains("setting1=value1"), "First line should be setting1=value1");
  
  // Test CSV format
  let csv = ParsingTestData::csv_data(3, 4);
  let lines: Vec<&str> = csv.lines().collect();
  assert_eq!(lines.len(), 4, "Should have header + 3 rows");
  assert_eq!(lines[0], "column1,column2,column3,column4", "Header should match column count");
  assert!(lines[1].contains("row1col1"), "Data rows should match format");
  
  // Test JSON objects
  let json = ParsingTestData::json_objects(DataSize::Small);
  assert!(json.starts_with('['), "Should be JSON array");
  assert!(json.ends_with(']'), "Should close JSON array");
  assert!(json.contains(r#""id": 1"#), "Should contain first object");
  assert!(json.contains(r#""id": 10"#), "Should contain last object");
}

/// Tests file path generation with large size
/// Test Combination: G1.8
#[test]
fn test_file_path_generation()
{
  let paths = generate_file_paths(DataSize::Large);
  
  assert_eq!(paths.len(), 1000, "Large size should generate 1000 paths");
  assert_eq!(paths[0], "/path/to/file1.txt", "First path should match format");
  assert_eq!(paths[999], "/path/to/file1000.txt", "Last path should match format");
  
  // All paths should be valid format
  for (i, path) in paths.iter().take(10).enumerate() {
    assert!(path.starts_with("/path/to/file"), "Path should start with expected prefix");
    assert!(path.ends_with(".txt"), "Path should end with .txt extension");
    assert!(path.contains(&(i + 1).to_string()), "Path should contain sequence number");
  }
}

/// Tests URL generation
#[test]
fn test_url_generation()
{
  let urls = generate_urls(DataSize::Medium);
  
  assert_eq!(urls.len(), 100, "Medium size should generate 100 URLs");
  assert!(urls[0].starts_with("https://"), "Should generate HTTPS URLs");
  assert!(urls[0].contains("example1.com"), "Should include domain with sequence");
  
  // Check URL format consistency
  for url in urls.iter().take(5) {
    assert!(url.starts_with("https://example"), "Should have consistent HTTPS prefix");
    assert!(url.contains(".com/path"), "Should have domain and path");
  }
}

/// Tests data size enumeration and standard sizes
#[test]
fn test_data_size_enumeration()
{
  assert_eq!(DataSize::Small.size(), 10);
  assert_eq!(DataSize::Medium.size(), 100);
  assert_eq!(DataSize::Large.size(), 1000);
  assert_eq!(DataSize::Huge.size(), 10000);
  assert_eq!(DataSize::Custom(42).size(), 42);
  
  let standard = DataSize::standard_sizes();
  assert_eq!(standard.len(), 4, "Should have 4 standard sizes");
  assert!(matches!(standard[0], DataSize::Small));
  assert!(matches!(standard[3], DataSize::Huge));
}

/// Tests custom delimiter support in generation
#[test]
fn test_custom_delimiters()
{
  let pipe_delimited = generate_list_data_with_delimiter(DataSize::Custom(3), "|");
  assert_eq!(pipe_delimited, "item1|item2|item3", "Should use custom delimiter");
  
  let map_with_custom = generate_map_data_with_delimiters(DataSize::Custom(2), ";", ":");
  assert_eq!(map_with_custom, "key1:value1;key2:value2", "Should use custom delimiters");
}

/// Tests numeric list generation
#[test]
fn test_numeric_list_generation()
{
  let numbers = generate_numeric_list(DataSize::Custom(5));
  assert_eq!(numbers, "1,2,3,4,5", "Should generate numeric sequence");
  
  let large_numbers = generate_numeric_list(DataSize::Small);
  let parts: Vec<&str> = large_numbers.split(',').collect();
  assert_eq!(parts.len(), 10, "Should generate correct count of numbers");
  assert_eq!(parts[0], "1", "Should start with 1");
  assert_eq!(parts[9], "10", "Should end with size");
}

/// Tests enum data generation
#[test]
fn test_enum_data_generation()
{
  let enums = generate_enum_data(DataSize::Custom(3));
  assert_eq!(enums, "choice1,choice2,choice3", "Should generate enum choices");
}

/// Tests variable string generation
#[test]
fn test_variable_string_generation()
{
  let strings = generate_variable_strings(5, 2, 10);
  assert_eq!(strings.len(), 5, "Should generate requested count");
  
  // Strings should vary in length
  assert_eq!(strings[0].len(), 2, "First string should be minimum length");
  assert_eq!(strings[4].len(), 10, "Last string should be maximum length");
  
  // All strings should use same character
  for s in &strings {
    assert!(s.chars().all(|c| c == 'x'), "All characters should be 'x'");
  }
}

/// Tests seeded random generator statistical properties
#[test]
fn test_random_generator_properties()
{
  let mut gen = SeededGenerator::new(123);
  
  // Test random string properties
  let random_str = gen.random_string(100);
  assert_eq!(random_str.len(), 100, "Should generate exact length");
  
  // Should use alphanumeric characters
  for c in random_str.chars() {
    assert!(c.is_alphanumeric(), "Should only contain alphanumeric characters");
  }
  
  // Test integer range
  for _ in 0..20 {
    let val = gen.random_int(10, 20);
    assert!(val >= 10 && val <= 20, "Integer should be in specified range");
  }
}

/// Tests convenience random vector generation
#[test]
fn test_convenience_random_vec()
{
  let vec = generate_random_vec(10);
  assert_eq!(vec.len(), 10, "Should generate requested size");
  
  for &val in &vec {
    assert!(val >= 1 && val <= 1000, "Values should be in expected range");
  }
}

/// Tests all data size variants with all generators
#[test]
fn test_all_generators_with_all_sizes()
{
  let sizes = DataSize::standard_sizes();
  
  for size in sizes {
    let expected_count = size.size();
    
    // Test list generation
    let list = generate_list_data(size);
    let list_count = if list.is_empty() { 0 } else { list.matches(',').count() + 1 };
    assert_eq!(list_count, expected_count, "List should have correct item count for {:?}", size);
    
    // Test map generation  
    let map = generate_map_data(size);
    let map_count = if map.is_empty() { 0 } else { map.matches(',').count() + 1 };
    assert_eq!(map_count, expected_count, "Map should have correct pair count for {:?}", size);
    
    // Test file paths
    let paths = generate_file_paths(size);
    assert_eq!(paths.len(), expected_count, "File paths should have correct count for {:?}", size);
  }
}

/// Tests parsing test data with different row/column configurations
#[test]
fn test_csv_generation_configurations()
{
  let csv_2x3 = ParsingTestData::csv_data(2, 3);
  let lines: Vec<&str> = csv_2x3.lines().collect();
  assert_eq!(lines.len(), 3, "Should have header + 2 rows");
  
  let header_cols = lines[0].matches(',').count() + 1;
  assert_eq!(header_cols, 3, "Header should have 3 columns");
  
  let csv_1x1 = ParsingTestData::csv_data(1, 1);
  let single_lines: Vec<&str> = csv_1x1.lines().collect();
  assert_eq!(single_lines.len(), 2, "Should have header + 1 row");
  assert_eq!(single_lines[0], "column1", "Single column header");
  assert_eq!(single_lines[1], "row1col1", "Single cell data");
}