//! Tests for help system formatting improvements
//!
//! This module tests that help output follows improved formatting principles
//! for better readability and user experience.
//!
//! Bug Coverage: Prevents regression where help output is cramped, hard to read,
//! or contains redundant information that makes it difficult for users to quickly
//! understand command usage.

#![ allow( deprecated ) ]

use unilang::prelude::*;

#[test]
#[allow(clippy::too_many_lines)]
fn test_help_formatting_is_readable()
{
  // This test ensures help output follows the improved formatting specification
  
  // Create a command with multiple arguments to test formatting
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  let test_cmd = CommandDefinition {
    name: "run_file".to_string(),
    namespace: String::new(),
    description: "Execute prompts from structured or plain text files".to_string(),
    hint: "Run prompts from a file (text, YAML, JSON, or TOML)".to_string(),
    arguments: vec![
      ArgumentDefinition {
        name: "file".to_string(),
        description: "Path to prompt file".to_string(),
        kind: Kind::File,
        hint: "Path to prompt file".to_string(),
        attributes: ArgumentAttributes {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        aliases: vec![],
        tags: vec!["automation".to_string(), "file".to_string()],
      },
      ArgumentDefinition {
        name: "working_dir".to_string(),
        description: "Directory to run commands in".to_string(),
        kind: Kind::Directory,
        hint: "Directory to run commands in".to_string(),
        attributes: ArgumentAttributes {
          optional: true,
          ..Default::default()
        },
        validation_rules: vec![],
        aliases: vec![],
        tags: vec![],
      },
      ArgumentDefinition {
        name: "simple".to_string(),
        description: "Simple mode without session management".to_string(),
        kind: Kind::Boolean,
        hint: "Simple mode without session management".to_string(),
        attributes: ArgumentAttributes {
          optional: true,
          ..Default::default()
        },
        validation_rules: vec![],
        aliases: vec![],
        tags: vec![],
      },
    ],
    routine_link: None,
        auto_help_enabled: false,
    status: "stable".to_string(),
    version: "0.1.0".to_string(),
    tags: vec!["automation".to_string(), "file".to_string()],
    aliases: vec![],
    permissions: vec![],
    idempotent: true,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  };
  
  registry.register(test_cmd);
  
  let help_gen = HelpGenerator::new(&registry);
  let help_output = help_gen.command("run_file").expect("Command should exist");
  
  // Test formatting requirements from specification section 9.5
  
  // 1. Should not have overly long lines (no single line over 100 chars for readability)
  for line in help_output.lines() {
    assert!(
      line.len() <= 100,
      "Help line too long ({}): '{}'", line.len(), line
    );
  }
  
  // 2. Should not have redundant "Hint:" prefix when context is clear in arguments section
  let lines = help_output.lines().collect::<Vec<_>>();
  let in_arguments_section = lines.iter().any(|line| line.contains("Arguments:"));
  if in_arguments_section {
    // Find lines in arguments section (after "Arguments:" line)
    let mut found_arguments_section = false;
    for line in &lines {
      if line.contains("Arguments:") {
        found_arguments_section = true;
        continue;
      }
      if found_arguments_section && !line.trim().is_empty() {
        // Arguments section lines should not have redundant "Hint:" when description is clear
        if line.contains(" - Hint: ") {
          // Check if the hint is identical or very similar to what comes before "Hint:"
          let parts: Vec<&str> = line.split(" - Hint: ").collect();
          if parts.len() == 2 {
            let before_hint = parts[0];
            let hint_text = parts[1].split(',').next().unwrap_or("");
            
            // If the hint is redundant with information already present, fail the test
            assert!(!before_hint.contains(hint_text), "Redundant hint text found: '{before_hint}' already contains '{hint_text}'");
          }
        }
      }
    }
  }
  
  // 3. Should have proper visual hierarchy
  assert!(help_output.contains("Usage:"), "Should have Usage header");
  assert!(help_output.contains("Arguments:"), "Should have Arguments section");
  assert!(help_output.contains("Status:"), "Should have Status information");
  
  // 4. Arguments should be clearly separated and readable
  // This test will initially fail with current formatting, then pass after improvement
  let argument_lines = lines.iter()
    .skip_while(|line| !line.contains("Arguments:"))
    .skip(1) // Skip "Arguments:" line itself
    .take_while(|line| !line.trim().is_empty() && !line.starts_with("Status"))
    .collect::<Vec<_>>();
  
  // Each argument should be well-formatted
  for arg_line in argument_lines {
    // Verify improved formatting - should NOT have the old cramped format
    // Old bad: "file (Kind: File) - Hint: Path to prompt file"
    // New good: "file (Type: File)" followed by indented description
    
    // Should not contain the old cramped patterns
    assert!(
      !arg_line.contains("(Kind:"),
      "Found old 'Kind:' format, should use 'Type:': '{arg_line}'"
    );
    assert!(
      !(arg_line.contains("- Hint:") && arg_line.len() > 60),
      "Found old cramped 'Hint:' format: '{arg_line}'"
    );
    
    // Should use improved patterns
    if arg_line.contains("(Type:") {
      // Main argument lines should be reasonably short
      assert!(
        arg_line.len() <= 80,
        "Argument header line too long: '{arg_line}'"
      );
    }
  }
}

#[test]
fn test_help_formatting_visual_hierarchy()
{
  // This test verifies that help output has clear visual hierarchy
  
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  let test_cmd = CommandDefinition {
    name: "test_command".to_string(),
    namespace: String::new(),
    description: "A test command for formatting verification".to_string(),
    hint: "Tests help formatting".to_string(),
    arguments: vec![
      ArgumentDefinition {
        name: "required_arg".to_string(),
        description: "A required argument".to_string(),
        kind: Kind::String,
        hint: "Required string input".to_string(),
        attributes: ArgumentAttributes {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        aliases: vec![],
        tags: vec![],
      },
    ],
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
  
  registry.register(test_cmd);
  
  let help_gen = HelpGenerator::new(&registry);
  let help_output = help_gen.command("test_command").expect("Command should exist");
  
  // Verify section headers are properly spaced
  let lines: Vec<&str> = help_output.lines().collect();
  
  // Find the Arguments section
  let args_index = lines.iter().position(|line| line.contains("Arguments:"))
    .expect("Should have Arguments section");
  
  // There should be proper spacing around sections
  if args_index > 0 && args_index < lines.len() - 1 {
    // Check that there's visual separation (empty line or clear distinction)
    let line_before = lines[args_index - 1];
    let _line_after = if args_index + 1 < lines.len() { lines[args_index + 1] } else { "" };
    
    // Arguments section should be well-separated from other content
    assert!(
      line_before.trim().is_empty() || !line_before.starts_with("  "),
      "Arguments section should be properly separated from previous content"
    );
  }
}

#[test]
fn test_documentation_of_improved_formatting_requirements()
{
  // This test documents the expected improvements to help formatting
  
  // These are the formatting principles that should be followed
  const MAX_LINE_LENGTH: usize = 80;
  let requires_multiline_format = true;
  let eliminates_redundant_hints = true;
  let provides_visual_hierarchy = true;
  
  // Verify that formatting requirements are understood
  assert_eq!(MAX_LINE_LENGTH, 80, "Lines should not exceed 80 characters when possible");
  assert!(requires_multiline_format, "Help should use multi-line format for clarity");
  assert!(eliminates_redundant_hints, "Redundant hint text should be eliminated");
  assert!(provides_visual_hierarchy, "Help should have clear visual hierarchy");
  
  // Document the problem with current formatting
  let current_bad_example = "file (Kind: File) - Hint: Path to prompt file, Optional";
  assert!(current_bad_example.len() > 50, "Current format crams too much info on one line");
  
  // Document what improved formatting should look like
  let improved_format_example = vec![
    "file",
    "  Type: File",
    "  Path to prompt file",
  ];
  
  // Improved format separates concerns and is more readable
  for line in improved_format_example {
    assert!(line.len() <= MAX_LINE_LENGTH, "Improved format should have reasonable line lengths");
  }
}