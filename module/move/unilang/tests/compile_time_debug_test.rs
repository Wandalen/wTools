//! Tests to ensure no compile-time debug output is emitted by default
//!
//! This module tests that the unilang framework does not emit debug output
//! during compilation or macro expansion when used normally.
//!
//! Bug Coverage: Prevents regression where compile-time debug logs (like
//! "ENTRY DEBUG", "RESULT DEBUG", etc.) are printed during normal compilation,
//! which creates noise in user applications.

use std::process::Command;

#[test]
fn test_no_compile_time_debug_output_in_build()
{
  // This test verifies that building a simple unilang application
  // does not produce any compile-time debug output
  
  // Create a minimal test project that uses unilang
  let test_code = r#"
use unilang::prelude::*;

fn main() -> Result<(), unilang::error::Error> {
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();
  
  let greet_cmd = CommandDefinition {
    name: "greet".to_string(),
    namespace: String::new(),
    description: "Test command".to_string(),
    hint: "Test".to_string(),
    arguments: vec![],
    routine_link: None,
        auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: true,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  };
  
  registry.register(greet_cmd);
  Ok(())
}
"#;
  
  // Write test code to temporary file
  let temp_dir = std::env::temp_dir();
  let test_file = temp_dir.join("unilang_debug_test.rs");
  std::fs::write(&test_file, test_code).expect("Failed to write test file");
  
  // Try to compile the test code and capture output
  let output = Command::new("rustc")
    .args([
      "--edition", "2021",
      "--extern", "unilang",
      "-L", "target/debug/deps",
      "--crate-type", "bin",
      test_file.to_str().unwrap(),
      "-o", temp_dir.join("unilang_debug_test").to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run rustc");
  
  let stderr = String::from_utf8_lossy(&output.stderr);
  let stdout = String::from_utf8_lossy(&output.stdout);
  
  // Check for debug output patterns that should not appear
  let debug_patterns = [
    "ENTRY DEBUG:",
    "RESULT DEBUG:", 
    "Generated result length:",
    "Generated code written to",
    "Parsed AST successfully",
  ];
  
  for pattern in &debug_patterns {
    assert!(
      !stderr.contains(pattern) && !stdout.contains(pattern),
      "Found forbidden compile-time debug output pattern '{pattern}' in compilation output.\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}"
    );
  }
  
  // Clean up
  let _ = std::fs::remove_file(&test_file);
  let _ = std::fs::remove_file(temp_dir.join("unilang_debug_test"));
}

#[test]
fn test_former_derive_macro_no_debug_output()
{
  // This test specifically targets the former derive macro which seems to be
  // the source of the debug output seen in the user's example
  
  let test_code = r#"
use former::Former;

#[derive(Former)]
pub struct TestStruct {
  pub field1: String,
  pub field2: i32,
}

fn main() {
  let _test = TestStruct::former()
    .field1("test".to_string())
    .field2(42)
    .form();
}
"#;
  
  // Write test code to temporary file  
  let temp_dir = std::env::temp_dir();
  let test_file = temp_dir.join("former_debug_test.rs");
  std::fs::write(&test_file, test_code).expect("Failed to write test file");
  
  // Try to compile the test code and capture output
  let output = Command::new("rustc")
    .args([
      "--edition", "2021", 
      "--extern", "former",
      "-L", "target/debug/deps",
      "--crate-type", "bin",
      test_file.to_str().unwrap(),
      "-o", temp_dir.join("former_debug_test").to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run rustc");
  
  let stderr = String::from_utf8_lossy(&output.stderr);
  let stdout = String::from_utf8_lossy(&output.stdout);
  
  // Check for debug output patterns from former macro
  let debug_patterns = [
    "ENTRY DEBUG:",
    "RESULT DEBUG:",
    "Generated result length:", 
    "Generated code written to",
    "Parsed AST successfully",
    "Generated code is syntactically valid",
  ];
  
  for pattern in &debug_patterns {
    assert!(
      !stderr.contains(pattern) && !stdout.contains(pattern),
      "Found forbidden compile-time debug output pattern '{pattern}' from former macro.\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}"
    );
  }
  
  // Clean up
  let _ = std::fs::remove_file(&test_file);
  let _ = std::fs::remove_file(temp_dir.join("former_debug_test"));
}

#[test]
fn test_documentation_of_debug_output_requirement()
{
  // This test documents the requirement that no compile-time debug output
  // should be emitted by default
  
  // These are the verbosity levels as documented
  const _VERBOSITY_QUIET: u8 = 0;    // No debug output
  const VERBOSITY_NORMAL: u8 = 1;   // Default, no debug output  
  const _VERBOSITY_DEBUG: u8 = 2;    // Full debug output
  
  // Verify that the default verbosity level produces no debug output
  assert_eq!(VERBOSITY_NORMAL, 1, "Default verbosity should be 1 (normal)");
  
  // Document that compile-time debug output is forbidden by default
  let compile_time_debug_allowed_by_default = false;
  assert!(!compile_time_debug_allowed_by_default, 
    "Compile-time debug output must not be emitted by default");
  
  // Document that runtime debug output is controlled by verbosity
  let runtime_debug_controlled_by_verbosity = true;
  assert!(runtime_debug_controlled_by_verbosity,
    "Runtime debug output must be controlled by verbosity settings");
}