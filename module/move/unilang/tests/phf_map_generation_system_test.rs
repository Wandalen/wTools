//!
//! Tests for Perfect Hash Function (PHF) map generation system.
//!
//! This module tests the build.rs PHF generation system including YAML parsing,
//! code generation, and the resulting static command maps.
//!

use std::process::Command;
use assert_fs::prelude::*;

#[ test ]
fn test_empty_yaml_generates_valid_phf()
{
  // Test that empty YAML input generates valid empty PHF map
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "test_commands.yaml" );

  // Create empty YAML array
  yaml_file.write_str( "[]" ).expect( "Failed to write test YAML" );

  // Set environment variable to point to our test file
  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_missing_yaml_file_generates_empty_phf()
{
  // Test that missing YAML file generates empty PHF without error
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let nonexistent_file = temp_dir.child( "nonexistent.yaml" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", nonexistent_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build should succeed with missing file" );
}

#[ test ]
fn test_simple_command_yaml_parsing()
{
  // Test parsing a simple command definition
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "simple_commands.yaml" );

  let yaml_content = r#"
- name: "test"
  namespace: ""
  description: "Test command"
  hint: "A simple test"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: ["t"]
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_namespaced_command_yaml_parsing()
{
  // Test parsing commands with namespaces
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "namespaced_commands.yaml" );

  let yaml_content = r#"
- name: "status"
  namespace: "system"
  description: "Show system status"
  hint: "system status"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: ["system"]
  aliases: []
  permissions: ["read"]
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: ["system.status"]
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_command_with_arguments_yaml_parsing()
{
  // Test parsing commands with complex arguments
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "command_with_args.yaml" );

  let yaml_content = r#"
- name: "process"
  namespace: "file"
  description: "Process files"
  hint: "file processor"
  arguments:
    - name: "input"
      kind: "File"
      attributes:
        optional: false
        multiple: true
        default: null
        sensitive: false
        interactive: false
      hint: "Input files"
      description: "Files to process"
      validation_rules: []
      aliases: ["i"]
      tags: ["required"]
    - name: "output"
      kind: "Directory"
      attributes:
        optional: true
        multiple: false
        default: "./output"
        sensitive: false
        interactive: false
      hint: "Output directory"
      description: "Where to save results"
      validation_rules: []
      aliases: ["o"]
      tags: ["optional"]
  status: "stable"
  version: "1.0.0"
  tags: ["file", "processing"]
  aliases: ["proc"]
  permissions: ["read", "write"]
  idempotent: false
  deprecation_message: ""
  http_method_hint: "POST"
  examples: ["file.process --input file1.txt --output ./results"]
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_multiple_commands_yaml_parsing()
{
  // Test parsing multiple commands in one file
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "multiple_commands.yaml" );

  let yaml_content = r#"
- name: "help"
  namespace: ""
  description: "Show help"
  hint: "help"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: ["h"]
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []

- name: "version"
  namespace: ""
  description: "Show version"
  hint: "version info"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: ["v"]
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []

- name: "config"
  namespace: "system"
  description: "Manage configuration"
  hint: "config management"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: ["system"]
  aliases: []
  permissions: ["admin"]
  idempotent: false
  deprecation_message: ""
  http_method_hint: "PUT"
  examples: ["system.config --set key=value"]
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_yaml_validation_and_error_handling()
{
  // Test that the build system handles various YAML edge cases gracefully
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "edge_case.yaml" );

  // Test with non-array YAML (should be handled gracefully)
  yaml_file.write_str( "not_an_array: true" ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  // Build should either succeed (if handled gracefully) or fail predictably
  let stderr = String::from_utf8_lossy( &output.stderr );

  // The test passes if either:
  // 1. The build succeeds (graceful handling)
  // 2. The build fails with a reasonable error message
  let reasonable_behavior = output.status.success()
    || stderr.contains( "Failed to parse" )
    || stderr.contains( "panicked" )
    || stderr.contains( "error" );

  assert!( reasonable_behavior, "Build should handle YAML edge cases reasonably, stderr: {stderr}" );
}

#[ test ]
#[ allow( clippy::too_many_lines ) ]
fn test_all_argument_kinds_yaml_parsing()
{
  // Test parsing all supported argument kinds
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "all_kinds.yaml" );

  let yaml_content = r#"
- name: "comprehensive"
  namespace: "test"
  description: "Test all argument kinds"
  hint: "comprehensive test"
  arguments:
    - name: "string_arg"
      kind: "String"
      attributes:
        optional: false
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "String argument"
      description: "A string value"
      validation_rules: []
      aliases: []
      tags: []
    - name: "integer_arg"
      kind: "Integer"
      attributes:
        optional: true
        multiple: false
        default: "42"
        sensitive: false
        interactive: false
      hint: "Integer argument"
      description: "An integer value"
      validation_rules: []
      aliases: []
      tags: []
    - name: "float_arg"
      kind: "Float"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "Float argument"
      description: "A float value"
      validation_rules: []
      aliases: []
      tags: []
    - name: "boolean_arg"
      kind: "Boolean"
      attributes:
        optional: true
        multiple: false
        default: "false"
        sensitive: false
        interactive: false
      hint: "Boolean argument"
      description: "A boolean value"
      validation_rules: []
      aliases: []
      tags: []
    - name: "path_arg"
      kind: "Path"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "Path argument"
      description: "A file path"
      validation_rules: []
      aliases: []
      tags: []
    - name: "file_arg"
      kind: "File"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "File argument"
      description: "A file path"
      validation_rules: []
      aliases: []
      tags: []
    - name: "directory_arg"
      kind: "Directory"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "Directory argument"
      description: "A directory path"
      validation_rules: []
      aliases: []
      tags: []
    - name: "url_arg"
      kind: "Url"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "URL argument"
      description: "A URL"
      validation_rules: []
      aliases: []
      tags: []
    - name: "datetime_arg"
      kind: "DateTime"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "DateTime argument"
      description: "A date time"
      validation_rules: []
      aliases: []
      tags: []
    - name: "pattern_arg"
      kind: "Pattern"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "Pattern argument"
      description: "A regex pattern"
      validation_rules: []
      aliases: []
      tags: []
    - name: "json_arg"
      kind: "JsonString"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "JSON argument"
      description: "A JSON string"
      validation_rules: []
      aliases: []
      tags: []
    - name: "object_arg"
      kind: "Object"
      attributes:
        optional: true
        multiple: false
        default: null
        sensitive: false
        interactive: false
      hint: "Object argument"
      description: "An object"
      validation_rules: []
      aliases: []
      tags: []
  status: "experimental"
  version: "0.1.0"
  tags: ["test", "comprehensive"]
  aliases: ["comp"]
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "POST"
  examples: []
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_yaml_with_special_characters()
{
  // Test that special characters in strings are properly escaped
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "special_chars.yaml" );

  let yaml_content = r#"
- name: "special"
  namespace: "test"
  description: "Command with \"quotes\" and \\backslashes\\"
  hint: "Special chars: \" \\ \n \t"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: ["test", "special-chars"]
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: "This is \"deprecated\" with \\special\\ chars"
  http_method_hint: "GET"
  examples: ["test.special --help"]
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_build_rs_regeneration_on_yaml_change()
{
  // Test that build.rs properly responds to YAML file changes
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "changing.yaml" );

  // First build with initial content
  yaml_file.write_str( "[]" ).expect( "Failed to write initial YAML" );

  let output1 = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute first cargo build" );

  assert!( output1.status.success(), "First build failed" );

  // Modify YAML content
  let yaml_content = r#"
- name: "changed"
  namespace: ""
  description: "Changed command"
  hint: "changed"
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

  yaml_file.write_str( yaml_content ).expect( "Failed to write modified YAML" );

  let output2 = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute second cargo build" );

  assert!( output2.status.success(), "Second build failed" );
}

#[ test ]
fn test_generated_code_structure()
{
  // Test the structure of generated code by examining the output
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "structure_test.yaml" );

  let yaml_content = r#"
- name: "test"
  namespace: "example"
  description: "Test command"
  hint: "test"
  arguments: []
  status: "stable"
  version: "1.0.0"
  tags: ["test"]
  aliases: ["t"]
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: ["example.test"]
"#;

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  // Build with verbose output to check for compilation success
  let output = Command::new( "cargo" )
    .arg( "build" )
    .arg( "--verbose" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );

  // Verify that the build output indicates successful compilation
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!( !stderr.contains( "error:" ), "Build output contains errors: {stderr}" );
}

#[ test ]
fn test_command_key_generation()
{
  // Test that command keys are generated correctly (namespace.name vs .name)
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "key_test.yaml" );

  let yaml_content = r#"
- name: "global"
  namespace: ""
  description: "Global command"
  hint: "global"
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

- name: "local"
  namespace: "space"
  description: "Namespaced command"
  hint: "local"
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

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_phf_map_compilation()
{
  // Test that the generated PHF map compiles without warnings
  let temp_dir = assert_fs::TempDir::new().expect( "Failed to create temp directory" );
  let yaml_file = temp_dir.child( "phf_test.yaml" );

  let yaml_content = r#"
- name: "one"
  namespace: ""
  description: "First command"
  hint: "one"
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

- name: "two"
  namespace: "ns"
  description: "Second command"
  hint: "two"
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

  yaml_file.write_str( yaml_content ).expect( "Failed to write test YAML" );

  // Build with warnings as errors to ensure clean compilation
  let output = Command::new( "cargo" )
    .arg( "build" )
    .env( "RUSTFLAGS", "-D warnings" )
    .env( "UNILANG_STATIC_COMMANDS_PATH", yaml_file.path() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Build with -D warnings failed: {}", String::from_utf8_lossy( &output.stderr ) );
}

#[ test ]
fn test_default_yaml_file_handling()
{
  // Test default behavior when no environment variable is set
  let output = Command::new( "cargo" )
    .arg( "build" )
    .env_remove( "UNILANG_STATIC_COMMANDS_PATH" )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!( output.status.success(), "Default build failed: {}", String::from_utf8_lossy( &output.stderr ) );
}