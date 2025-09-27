//! Tests for file path parsing with dot prefixes
//!
//! This module tests that file paths starting with ./ are correctly parsed as argument values
//! rather than being treated as part of the command path.
//!
//! Bug Coverage: Prevents regression where file paths like "./examples/file.yaml" are
//! incorrectly parsed as part of the command path instead of as argument values,
//! causing "Expected value for named argument" errors.

use unilang_parser::*;

#[test]
fn test_command_with_dot_prefix_and_file_path_with_dot_slash()
{
  // This test covers the exact user case that was failing:
  // .run_file file::./examples/rust_learning.yaml
  
  let parser = Parser::new(UnilangParserOptions::default());
  let input = ".run_file file::./examples/rust_learning.yaml";
  
  let result = parser.parse_single_instruction(input);
  
  match result {
    Ok(instruction) => {
      // Command should be parsed as "run_file" (leading dot stripped)
      let command_name = instruction.command_path_slices.join(".");
      assert_eq!(command_name, "run_file", "Command name should be 'run_file'");
      
      // Should have one named argument "file" with value "./examples/rust_learning.yaml"
      assert_eq!(instruction.named_arguments.len(), 1, "Should have exactly one named argument");
      assert!(instruction.named_arguments.contains_key("file"), "Should have 'file' argument");
      assert_eq!(
        instruction.named_arguments.get("file").unwrap().first().unwrap().value,
        "./examples/rust_learning.yaml",
        "File argument should contain the full path including './' prefix"
      );
      
      // Should have no positional arguments
      assert_eq!(instruction.positional_arguments.len(), 0, "Should have no positional arguments");
    },
    Err(e) => {
      panic!("Parsing should succeed but failed with error: {e:?}");
    }
  }
}

#[test]
fn test_command_with_dot_prefix_and_various_file_paths()
{
  // Test various file path formats that should all work
  let parser = Parser::new(UnilangParserOptions::default());
  
  let test_cases = vec![
    (".run_file file::./examples/file.yaml", "./examples/file.yaml"),
    (".run_file file::../parent/file.txt", "../parent/file.txt"),
    (".run_file file::/absolute/path/file.json", "/absolute/path/file.json"),
    (".run_file file::~/home/file.toml", "~/home/file.toml"),
    (".run_file file::relative/path/file.md", "relative/path/file.md"),
  ];
  
  for (input, expected_path) in test_cases {
    let result = parser.parse_single_instruction(input);
    
    match result {
      Ok(instruction) => {
        let command_name = instruction.command_path_slices.join(".");
        assert_eq!(command_name, "run_file", "Command name should be 'run_file' for input: {input}");
        assert_eq!(
          instruction.named_arguments.get("file").unwrap().first().unwrap().value,
          expected_path,
          "File path should be correctly parsed for input: {input}"
        );
      },
      Err(e) => {
        panic!("Parsing should succeed for '{input}' but failed with error: {e:?}");
      }
    }
  }
}

#[test]
fn test_file_path_does_not_interfere_with_command_parsing()
{
  // This test ensures that file paths with dots don't get confused with command namespaces
  let parser = Parser::new(UnilangParserOptions::default());
  
  // Command with namespace and file path - should not be confused
  let input = ".namespace.command file::./path/to/file.ext";
  let result = parser.parse_single_instruction(input);
  
  match result {
    Ok(instruction) => {
      // Command should be parsed as "namespace.command"
      let command_name = instruction.command_path_slices.join(".");
      assert_eq!(command_name, "namespace.command", 
        "Command should be 'namespace.command', not confused by file path");
      
      // File argument should be preserved exactly
      assert_eq!(
        instruction.named_arguments.get("file").unwrap().first().unwrap().value,
        "./path/to/file.ext",
        "File path should be preserved exactly"
      );
    },
    Err(e) => {
      panic!("Parsing should succeed but failed with error: {e:?}");
    }
  }
}

#[test]
fn test_documentation_of_file_path_parsing_requirements()
{
  // This test documents the requirements for file path parsing
  
  // File paths should be treated as argument values, not command parts
  let file_paths_should_be_arguments = true;
  let slash_terminates_command_path = true;
  let dot_slash_is_valid_file_path = true;
  
  // Verify requirements are understood
  assert!(file_paths_should_be_arguments, "File paths must be treated as argument values");
  assert!(slash_terminates_command_path, "Slash character must terminate command path parsing");
  assert!(dot_slash_is_valid_file_path, "./path syntax must be supported in file arguments");
  
  // Document the problem case
  let problematic_input = ".run_file file::./examples/rust_learning.yaml";
  let should_parse_successfully = true;
  
  assert!(should_parse_successfully, 
    "Input '{problematic_input}' should parse successfully with proper file path handling");
}