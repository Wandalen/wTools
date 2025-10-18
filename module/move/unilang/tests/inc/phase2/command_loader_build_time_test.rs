//! Tests for build-time command loading from YAML/JSON files.
//!
//! This module tests the complete YAML/JSON parity for build-time PHF generation:
//! - YAML single-file → build-time PHF (#1)
//! - YAML multi-file → build-time PHF (#2)
//! - JSON single-file → build-time PHF (#4)
//! - JSON multi-file → build-time PHF (#5)
//!
//! **Test Matrix for Build-Time Command Loading**
//!
//! | Test | Variant | Format | Files | Status |
//! |------|---------|--------|-------|--------|
//! | BT1.1 | Single-file YAML | .yaml | 1 | ✅ Tested |
//! | BT1.2 | Single-file YAML | .yml | 1 | ✅ Tested |
//! | BT2.1 | Multi-file YAML | .yaml | 2 | ✅ Tested |
//! | BT2.2 | Multi-file YAML | .yml | 2 | ✅ Tested |
//! | BT3.1 | Single-file JSON | .json | 1 | ✅ Tested |
//! | BT4.1 | Multi-file JSON | .json | 2 | ✅ Tested |
//! | BT5.1 | Multi-file Mixed | .yaml+.json | 4 | ✅ Tested |
//!
//! **Note**: These tests verify the parsing logic. Full build-time PHF generation
//! is tested via environment variables:
//! ```bash
//! # Test single-file YAML
//! UNILANG_STATIC_COMMANDS_PATH=tests/test_data/build_time/test_commands.yaml cargo build
//!
//! # Test single-file JSON
//! UNILANG_STATIC_COMMANDS_PATH=tests/test_data/build_time/test_commands.json cargo build
//!
//! # Test multi-file discovery
//! UNILANG_YAML_DISCOVERY_PATHS=tests/test_data/build_time/multi cargo build
//! ```

use std::path::Path;

// BT1.1: Single-file YAML with .yaml extension
#[ test ]
fn test_yaml_single_file_yaml_extension()
{
  let test_file = Path::new("tests/test_data/build_time/test_commands.yaml");
  assert!( test_file.exists(), "Test file does not exist: {}", test_file.display() );

  // In a real build, build.rs would call parse_command_file
  // Here we verify the file is valid YAML
  let content = std::fs::read_to_string( test_file ).unwrap();
  let commands: Vec< serde_yaml::Value > = serde_yaml::from_str( &content ).unwrap();

  assert_eq!( commands.len(), 1, "Expected 1 command in YAML file" );
  assert_eq!( commands[ 0 ][ "name" ].as_str().unwrap(), "build_yaml_cmd" );
  assert_eq!( commands[ 0 ][ "namespace" ].as_str().unwrap(), ".test" );
  assert_eq!( commands[ 0 ][ "description" ].as_str().unwrap(), "Command loaded from YAML at build-time" );
}

// BT1.2: Single-file YAML with .yml extension
#[ test ]
fn test_yaml_single_file_yml_extension()
{
  // Test data uses .yaml, but build.rs supports .yml
  // Create temporary .yml file for testing
  let temp_yml = std::env::temp_dir().join( "test_commands_temp.yml" );

  let yaml_content = r#"
---
- name: "yml_test"
  namespace: ".test"
  description: "Command with .yml extension"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
"#;

  std::fs::write( &temp_yml, yaml_content ).unwrap();

  // Verify parsing
  let content = std::fs::read_to_string( &temp_yml ).unwrap();
  let commands: Vec< serde_yaml::Value > = serde_yaml::from_str( &content ).unwrap();

  assert_eq!( commands.len(), 1 );
  assert_eq!( commands[ 0 ][ "name" ].as_str().unwrap(), "yml_test" );

  // Cleanup
  std::fs::remove_file( &temp_yml ).ok();
}

// BT2.1: Multi-file YAML discovery
#[ test ]
fn test_yaml_multi_file_discovery()
{
  let multi_dir = Path::new( "tests/test_data/build_time/multi" );
  assert!( multi_dir.exists(), "Multi-file test directory does not exist" );

  // Verify YAML files exist and are valid
  let yaml_files = [ "commands_a.yaml", "commands_b.yml" ];

  for yaml_file in &yaml_files
  {
    let file_path = multi_dir.join( yaml_file );
    assert!( file_path.exists(), "YAML file missing: {}", file_path.display() );

    let content = std::fs::read_to_string( &file_path ).unwrap();
    let commands: Vec< serde_yaml::Value > = serde_yaml::from_str( &content ).unwrap();

    assert_eq!( commands.len(), 1, "Expected 1 command in {yaml_file}" );
  }

  // In build.rs, walkdir would discover and merge these files
  // Verify both would be found by walkdir
  use walkdir::WalkDir;
  let yaml_count = WalkDir::new( multi_dir )
    .into_iter()
    .filter_map( Result::ok )
    .filter( | e | e.file_type().is_file() )
    .filter( | e | {
      if let Some( ext ) = e.path().extension()
      {
        ext == "yaml" || ext == "yml"
      }
      else
      {
        false
      }
    })
    .count();

  assert_eq!( yaml_count, 2, "Expected 2 YAML files in multi directory" );
}

// BT3.1: Single-file JSON
#[ test ]
fn test_json_single_file()
{
  let test_file = Path::new( "tests/test_data/build_time/test_commands.json" );
  assert!( test_file.exists(), "Test file does not exist: {}", test_file.display() );

  // Verify the file is valid JSON
  let content = std::fs::read_to_string( test_file ).unwrap();
  let json_value: serde_json::Value = serde_json::from_str( &content ).unwrap();

  // Convert to array
  let commands = json_value.as_array().unwrap();

  assert_eq!( commands.len(), 1, "Expected 1 command in JSON file" );
  assert_eq!( commands[ 0 ][ "name" ].as_str().unwrap(), "build_json_cmd" );
  assert_eq!( commands[ 0 ][ "namespace" ].as_str().unwrap(), ".test" );
  assert_eq!( commands[ 0 ][ "description" ].as_str().unwrap(), "Command loaded from JSON at build-time" );
}

// BT4.1: Multi-file JSON discovery
#[ test ]
fn test_json_multi_file_discovery()
{
  let multi_dir = Path::new( "tests/test_data/build_time/multi" );

  // Verify JSON files exist and are valid
  let json_files = [ "commands_c.json", "commands_d.json" ];

  for json_file in &json_files
  {
    let file_path = multi_dir.join( json_file );
    assert!( file_path.exists(), "JSON file missing: {}", file_path.display() );

    let content = std::fs::read_to_string( &file_path ).unwrap();
    let json_value: serde_json::Value = serde_json::from_str( &content ).unwrap();
    let commands = json_value.as_array().unwrap();

    assert_eq!( commands.len(), 1, "Expected 1 command in {json_file}" );
  }

  // Verify walkdir would find all JSON files
  use walkdir::WalkDir;
  let json_count = WalkDir::new( multi_dir )
    .into_iter()
    .filter_map( Result::ok )
    .filter( | e | e.file_type().is_file() )
    .filter( | e | {
      if let Some( ext ) = e.path().extension()
      {
        ext == "json"
      }
      else
      {
        false
      }
    })
    .count();

  assert_eq!( json_count, 2, "Expected 2 JSON files in multi directory" );
}

// BT5.1: Multi-file mixed YAML+JSON discovery
#[ test ]
fn test_mixed_yaml_json_multi_file_discovery()
{
  let multi_dir = Path::new( "tests/test_data/build_time/multi" );

  // Verify all files would be discovered by build.rs
  use walkdir::WalkDir;
  let all_count = WalkDir::new( multi_dir )
    .into_iter()
    .filter_map( Result::ok )
    .filter( | e | e.file_type().is_file() )
    .filter( | e | {
      if let Some( ext ) = e.path().extension()
      {
        ext == "yaml" || ext == "yml" || ext == "json"
      }
      else
      {
        false
      }
    })
    .count();

  assert_eq!( all_count, 4, "Expected 4 total files (2 YAML + 2 JSON) in multi directory" );

  // Verify all commands can be parsed
  let mut all_commands = Vec::new();

  for entry in WalkDir::new( multi_dir )
    .into_iter()
    .filter_map( Result::ok )
    .filter( | e | e.file_type().is_file() )
    .filter( | e | {
      if let Some( ext ) = e.path().extension()
      {
        ext == "yaml" || ext == "yml" || ext == "json"
      }
      else
      {
        false
      }
    })
  {
    let extension = entry.path().extension().and_then( | s | s.to_str() ).unwrap();

    match extension
    {
      "yaml" | "yml" =>
      {
        let content = std::fs::read_to_string( entry.path() ).unwrap();
        let mut commands: Vec< serde_yaml::Value > = serde_yaml::from_str( &content ).unwrap();
        all_commands.append( &mut commands );
      }
      "json" =>
      {
        let content = std::fs::read_to_string( entry.path() ).unwrap();
        let json_value: serde_json::Value = serde_json::from_str( &content ).unwrap();

        // Convert JSON to YAML Value (same as build.rs does)
        let json_str = serde_json::to_string( &json_value ).unwrap();
        let mut commands: Vec< serde_yaml::Value > = serde_yaml::from_str( &json_str ).unwrap();
        all_commands.append( &mut commands );
      }
      _ => {}
    }
  }

  assert_eq!( all_commands.len(), 4, "Expected 4 total commands from mixed YAML+JSON files" );

  // Verify command names
  let names: Vec< &str > = all_commands
    .iter()
    .map( | cmd | cmd[ "name" ].as_str().unwrap() )
    .collect();

  assert!( names.contains( &"multi_yaml_a" ) );
  assert!( names.contains( &"multi_yaml_b" ) );
  assert!( names.contains( &"multi_json_c" ) );
  assert!( names.contains( &"multi_json_d" ) );
}

// BT6.1: Verify JSON→YAML conversion preserves all fields
#[ test ]
fn test_json_to_yaml_conversion_fidelity()
{
  let test_file = Path::new( "tests/test_data/build_time/test_commands.json" );
  let content = std::fs::read_to_string( test_file ).unwrap();

  // Parse as JSON
  let json_value: serde_json::Value = serde_json::from_str( &content ).unwrap();

  // Convert to YAML Value (same as build.rs does)
  let json_str = serde_json::to_string( &json_value ).unwrap();
  let yaml_commands: Vec< serde_yaml::Value > = serde_yaml::from_str( &json_str ).unwrap();

  // Verify all fields are preserved
  let cmd = &yaml_commands[ 0 ];

  assert_eq!( cmd[ "name" ].as_str().unwrap(), "build_json_cmd" );
  assert_eq!( cmd[ "namespace" ].as_str().unwrap(), ".test" );
  assert_eq!( cmd[ "description" ].as_str().unwrap(), "Command loaded from JSON at build-time" );
  assert_eq!( cmd[ "status" ].as_str().unwrap(), "stable" );
  assert_eq!( cmd[ "version" ].as_str().unwrap(), "1.0.0" );
  assert!( cmd[ "idempotent" ].as_bool().unwrap() );
  assert_eq!( cmd[ "http_method_hint" ].as_str().unwrap(), "GET" );

  // Verify arguments array
  let args = cmd[ "arguments" ].as_sequence().unwrap();
  assert_eq!( args.len(), 1 );
  assert_eq!( args[ 0 ][ "name" ].as_str().unwrap(), "param" );
  assert_eq!( args[ 0 ][ "kind" ].as_str().unwrap(), "String" );

  // Verify tags array
  let tags = cmd[ "tags" ].as_sequence().unwrap();
  assert_eq!( tags.len(), 2 );
  assert_eq!( tags[ 0 ].as_str().unwrap(), "build-time" );
  assert_eq!( tags[ 1 ].as_str().unwrap(), "json" );

  // Verify examples array
  let examples = cmd[ "examples" ].as_sequence().unwrap();
  assert_eq!( examples.len(), 1 );
  assert_eq!( examples[ 0 ].as_str().unwrap(), ".test.build_json_cmd param::value" );
}
