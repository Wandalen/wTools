//! Test Suite: Argv-Based API for CLI Integration
//!
//! This test suite validates the argv-based parsing API (Task 080) which provides proper
//! CLI integration by preserving the original argv structure from the operating system.
//!
//! ## Test Matrix
//!
//! | Test Name | Purpose | Input | Expected Output |
//! |-----------|---------|-------|-----------------|
//! | `test_argv_with_dashes` | Verify values with dashes are combined | `["command::ls", "-la"]` | command = "ls -la" |
//! | `test_argv_multiple_params` | Verify multiple parameters | `["cmd", "k1::v1", "k2::v2"]` | k1="v1", k2="v2" |
//! | `test_argv_multi_word_value` | Verify multi-word values | `["cmd", "cmd::cargo", "build"]` | cmd="cargo build" |
//! | `test_argv_empty` | Verify empty argv | `[]` | Empty instruction |
//! | `test_argv_value_with_spaces` | Verify preservation of spaces | `["cmd", "text::hello", "world"]` | text="hello world" |
//! | `test_argv_stop_at_next_param` | Verify stopping at next :: | `["c", "a::x", "y", "b::z"]` | a="x y", b="z" |
//! | `test_pipeline_argv_integration` | Full pipeline with argv | `["msg::test"]` | Executes correctly |
//!
//! ## Coverage
//!
//! - [x] Values with dashes (primary use case from MRE)
//! - [x] Multiple parameters with proper boundaries
//! - [x] Multi-word value combining
//! - [x] Empty argv handling
//! - [x] Space preservation in values
//! - [x] Stopping at parameter boundaries
//! - [x] Full pipeline integration
//!
//! ## References
//!
//! - Task 080: Argv-Based API Request (`task/080_argv_based_api_request.md`)
//! - MRE: `task/080_argv_api_mre.rs`

use unilang::prelude::*;
use unilang_parser::{Parser, UnilangParserOptions};

/// Test the primary use case from the MRE: values with dashes
///
/// Shell: `./app .run command::ls -la`
/// OS provides: `[".run", "command::ls", "-la"]`
/// Expected: command = "ls -la" (correctly combined)
#[test]
fn test_argv_with_dashes()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv = vec![".run".to_string(), "command::ls".to_string(), "-la".to_string()];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  // Verify the command was parsed
  assert_eq!(instruction.command_path_slices, vec!["run"], "Should have 'run' as command");

  // Verify the parameter was combined correctly
  let command_args = instruction.named_arguments.get("command").expect("Should have 'command' parameter");
  assert_eq!(command_args.len(), 1, "Should have exactly one argument");
  assert_eq!(command_args[0].value, "ls -la", "Should combine 'ls' and '-la' into a single value");
}

/// Test multiple parameters with proper boundaries
///
/// Shell: `./app .crates.for.each command::cargo build filter::w*`
/// Expected: command="cargo build", filter="w*"
#[test]
fn test_argv_multiple_params()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv = vec![
    ".crates.for.each".to_string(),
    "command::cargo".to_string(),
    "build".to_string(),
    "filter::w*".to_string(),
  ];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  // Verify command path
  assert_eq!(instruction.command_path_slices, vec!["crates", "for", "each"]);

  // Verify parameters
  let command_args = instruction.named_arguments.get("command").expect("Should have 'command' parameter");
  assert_eq!(command_args[0].value, "cargo build", "Should combine 'cargo' and 'build'");

  let filter_args = instruction.named_arguments.get("filter").expect("Should have 'filter' parameter");
  assert_eq!(filter_args[0].value, "w*", "Filter should be 'w*'");
}

/// Test multi-word value combining
#[test]
fn test_argv_multi_word_value()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv = vec![
    ".test".to_string(),
    "message::hello".to_string(),
    "beautiful".to_string(),
    "world".to_string(),
  ];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  let message_args = instruction.named_arguments.get("message").expect("Should have 'message' parameter");
  assert_eq!(message_args[0].value, "hello beautiful world", "Should combine all words");
}

/// Test empty argv
#[test]
fn test_argv_empty()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv: Vec<String> = vec![];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  assert!(instruction.command_path_slices.is_empty());
  assert!(instruction.named_arguments.is_empty());
  assert!(instruction.positional_arguments.is_empty());
}

/// Test value with spaces preservation
#[test]
fn test_argv_value_with_spaces()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv = vec![
    ".greet".to_string(),
    "name::John".to_string(),
    "Smith".to_string(),
  ];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  let name_args = instruction.named_arguments.get("name").expect("Should have 'name' parameter");
  assert_eq!(name_args[0].value, "John Smith", "Should preserve space between John and Smith");
}

/// Test stopping at next parameter boundary
#[test]
fn test_argv_stop_at_next_param()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv = vec![
    ".cmd".to_string(),
    "first::value1".to_string(),
    "value2".to_string(),
    "second::value3".to_string(),
  ];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  // First parameter should combine value1 and value2
  let first_args = instruction.named_arguments.get("first").expect("Should have 'first' parameter");
  assert_eq!(first_args[0].value, "value1 value2", "Should combine until next parameter");

  // Second parameter should be standalone
  let second_args = instruction.named_arguments.get("second").expect("Should have 'second' parameter");
  assert_eq!(second_args[0].value, "value3", "Second parameter standalone");
}

/// Test full pipeline integration with argv
#[test]
fn test_pipeline_argv_integration()
{
  // Create a test registry with a simple command
  let mut registry = CommandRegistry::new();

  let test_cmd = CommandDefinition::former()
    .name(".echo")
    .namespace(String::new())
    .description("Echo command".to_string())
    .hint("Echo a message")
    .status("stable")
    .version("1.0.0")
    .aliases(vec![])
    .tags(vec![])
    .permissions(vec![])
    .idempotent(true)
    .deprecation_message(String::new())
    .http_method_hint("GET".to_string())
    .examples(vec![])
    .arguments(vec![
      ArgumentDefinition::former()
        .name("message")
        .description("Message to echo".to_string())
        .kind(Kind::String)
        .hint("The message")
        .attributes(ArgumentAttributes {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        })
        .validation_rules(vec![])
        .aliases(vec![])
        .tags(vec![])
        .end()
    ])
    .end();

  let test_routine = Box::new(|cmd: VerifiedCommand, _ctx| {
    let message = cmd.arguments.get("message")
      .and_then(|v| if let Value::String(s) = v { Some(s) } else { None })
      .expect("message is required");

    Ok(OutputData {
      content: message.clone(),
      format: "text".to_string(),
      execution_time_ms : None,
    })
  });

  registry.command_add_runtime(&test_cmd, test_routine).unwrap();

  // Create pipeline and test argv API
  let pipeline = Pipeline::new(registry);
  let argv = vec![".echo".to_string(), "message::Hello".to_string(), "World".to_string()];

  let result = pipeline.process_command_from_argv_simple(&argv);

  assert!(result.is_success(), "Command should succeed");
  assert_eq!(result.outputs.len(), 1, "Should have one output");
  assert_eq!(result.outputs[0].content, "Hello World", "Should combine 'Hello' and 'World'");
}

/// Test that value combining respects parameter boundaries
///
/// Note: In real CLI usage, multiple dot-prefixed commands in one argv isn't typical.
/// This test verifies that when we have path-like values, they're handled correctly.
#[test]
fn test_argv_path_like_values()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let argv = vec![
    ".copy".to_string(),
    "src::file1.txt".to_string(),
    "dest::file2.txt".to_string(),
  ];

  let instruction = parser.parse_from_argv(&argv).expect("Should parse successfully");

  // Verify command path
  assert_eq!(instruction.command_path_slices, vec!["copy"]);

  // Verify parameters are separate
  let src_args = instruction.named_arguments.get("src").expect("Should have 'src' parameter");
  assert_eq!(src_args[0].value, "file1.txt");

  let dest_args = instruction.named_arguments.get("dest").expect("Should have 'dest' parameter");
  assert_eq!(dest_args[0].value, "file2.txt");
}

/// Test comparison: String API vs Argv API
///
/// This test demonstrates the problem that argv API solves
#[test]
fn test_comparison_string_vs_argv()
{
  let parser = Parser::new(UnilangParserOptions::default());

  // Include a command path to make this valid unilang syntax
  let argv = vec![".run".to_string(), "command::ls".to_string(), "-la".to_string()];

  // String API: May not handle multi-word values optimally
  let joined_string = argv.join(" "); // ".run command::ls -la"
  let _string_result = parser.parse_single_instruction(&joined_string);

  // Argv API: Works correctly by combining consecutive argv elements
  let argv_result = parser.parse_from_argv(&argv);
  assert!(argv_result.is_ok(), "Argv API should work");

  let instruction = argv_result.unwrap();
  assert_eq!(instruction.command_path_slices, vec!["run"]);
  let command_args = instruction.named_arguments.get("command").expect("Should have command parameter");
  assert_eq!(command_args[0].value, "ls -la", "Argv API correctly combines the value");
}
