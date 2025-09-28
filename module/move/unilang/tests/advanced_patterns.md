# Advanced Testing Patterns and Best Practices

This document outlines advanced testing patterns, best practices, and examples for the systematic test organization in the Unilang framework.

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Test Category Patterns](#test-category-patterns)
3. [Advanced Unit Testing Patterns](#advanced-unit-testing-patterns)
4. [Integration Testing Strategies](#integration-testing-strategies)
5. [Acceptance Testing Approaches](#acceptance-testing-approaches)
6. [Regression Testing Patterns](#regression-testing-patterns)
7. [Test Data Management](#test-data-management)
8. [Performance Testing Patterns](#performance-testing-patterns)
9. [Error Testing Strategies](#error-testing-strategies)
10. [Anti-Patterns to Avoid](#anti-patterns-to-avoid)

## Testing Philosophy

### Core Principles

**1. Single Responsibility Principle**
Each test should verify exactly one behavior or requirement.

```rust
// ✅ Good - Single responsibility
#[test]
fn test_argument_parsing_handles_quoted_strings()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let result = parser.parse_single_instruction(r#".test arg::"quoted value""#);

  assert!(result.is_ok());
  let instruction = result.unwrap();
  assert_eq!(instruction.named_arguments["arg"][0].value, "quoted value");
}

// ❌ Bad - Multiple responsibilities
#[test]
fn test_parser_functionality()
{
  // Tests multiple parsing scenarios in one test
  // Tests error handling
  // Tests performance
  // Tests edge cases
}
```

**2. Test Isolation**
Tests should not depend on each other or shared state.

```rust
// ✅ Good - Self-contained test
#[test]
fn test_command_registry_lookup()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command(".test");
  registry.command_add_runtime(&cmd, mock_routine()).unwrap();

  let result = registry.command(".test");
  assert!(result.is_some());
}

// ❌ Bad - Depends on external state
static mut GLOBAL_REGISTRY: Option<CommandRegistry> = None;
#[test]
fn test_command_lookup() {
  // Uses global state that might be modified by other tests
}
```

**3. Readable Test Names**
Test names should clearly describe what they verify.

```rust
// ✅ Good - Descriptive names
#[test]
fn test_multiple_parameter_collection_preserves_order()
#[test]
fn test_semantic_analysis_rejects_unknown_commands()
#[test]
fn test_help_generation_includes_argument_descriptions()

// ❌ Bad - Unclear names
#[test]
fn test_1()
#[test]
fn test_parser()
#[test]
fn test_edge_case()
```

## Test Category Patterns

### Unit Test Patterns

**Pattern: Component Isolation**
```rust
/// Tests a single component in isolation with mocked dependencies
#[test]
fn test_semantic_analyzer_validates_argument_types()
{
  // Arrange - Create isolated component
  let registry = create_mock_registry_with_typed_command();
  let instructions = [create_test_instruction_with_wrong_type()];
  let analyzer = SemanticAnalyzer::new(&instructions, &registry);

  // Act - Exercise single functionality
  let result = analyzer.analyze();

  // Assert - Verify specific behavior
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert!(error.contains("type mismatch"));
}
```

**Pattern: Boundary Testing**
```rust
/// Tests component behavior at boundaries and edge cases
#[test]
fn test_argument_parsing_handles_empty_quoted_strings()
{
  let parser = Parser::new(UnilangParserOptions::default());

  // Test empty quoted string boundary
  let result = parser.parse_single_instruction(r#".test arg::"""#);

  assert!(result.is_ok());
  let instruction = result.unwrap();
  assert_eq!(instruction.named_arguments["arg"][0].value, "");
}
```

### Integration Test Patterns

**Pattern: Component Interaction**
```rust
/// Tests interaction between multiple components
#[test]
fn test_parser_semantic_analyzer_integration()
{
  // Arrange - Set up multiple components
  let parser = Parser::new(UnilangParserOptions::default());
  let mut registry = CommandRegistry::new();
  setup_test_commands(&mut registry);

  // Act - Exercise component interaction
  let instruction = parser.parse_single_instruction(".test arg::value").unwrap();
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new(&instructions, &registry);
  let verified_commands = analyzer.analyze().unwrap();

  // Assert - Verify interaction result
  assert_eq!(verified_commands.len(), 1);
  assert_eq!(verified_commands[0].definition.name, ".test");
}
```

**Pattern: Data Flow Testing**
```rust
/// Tests data flow through multiple components
#[test]
fn test_complete_command_processing_pipeline()
{
  let input = r#".process file::"data.txt" format::"json""#;

  // Test data flow through pipeline
  let pipeline = Pipeline::new();
  let result = pipeline.process_command(input).unwrap();

  // Verify data transformation at each stage
  assert!(result.arguments.contains_key("file"));
  assert!(result.arguments.contains_key("format"));
  assert_eq!(result.output_format, "json");
}
```

### Acceptance Test Patterns

**Pattern: User Scenario Simulation**
```rust
/// Simulates complete user interaction scenarios
#[test]
fn test_user_workflow_command_chaining()
{
  let mut shell = TestShell::new();

  // Simulate user typing multiple commands
  shell.execute(".load config::settings.json");
  shell.execute(".process input::data.csv");
  shell.execute(".export format::json output::result.json");

  // Verify end-to-end workflow
  assert!(shell.file_exists("result.json"));
  assert!(shell.last_output().contains("exported successfully"));
}
```

**Pattern: CLI Interface Testing**
```rust
/// Tests CLI interface from user perspective
#[test]
fn test_help_system_user_experience()
{
  let cli = TestCLI::new();

  // Test various help access patterns users might try
  let global_help = cli.run(&[]).output;
  assert!(global_help.contains("Available Commands"));

  let specific_help = cli.run(&[".test", "help"]).output;
  assert!(specific_help.contains("Usage:"));

  let help_flag = cli.run(&["--help"]).output;
  assert!(help_flag.contains("Available Commands"));
}
```

### Regression Test Patterns

**Pattern: Bug Reproduction**
```rust
/// Reproduces specific bug scenarios to prevent regression
#[test]
fn regression_task_024_multiple_parameter_collection()
{
  // Exact reproduction of Task 024 scenario that was failing
  let input = r#".run command::"cargo build" command::"echo hello" command::"cargo clippy""#;

  let parser = Parser::new(UnilangParserOptions::default());
  let instruction = parser.parse_single_instruction(input).unwrap();

  // Verify all three commands are collected (was failing before fix)
  assert_eq!(instruction.named_arguments["command"].len(), 3);
  assert_eq!(instruction.named_arguments["command"][0].value, "cargo build");
  assert_eq!(instruction.named_arguments["command"][1].value, "echo hello");
  assert_eq!(instruction.named_arguments["command"][2].value, "cargo clippy");
}
```

**Pattern: Compatibility Testing**
```rust
/// Ensures backward compatibility is maintained
#[test]
fn regression_backward_compatibility_single_parameters()
{
  // Ensure old single-parameter usage still works
  let single_param_inputs = vec![
    r#".test param::"single_value""#,
    r#".test param::simple"#,
    r#".test param::123"#,
  ];

  let parser = Parser::new(UnilangParserOptions::default());

  for input in single_param_inputs {
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Single parameter parsing should still work: {}", input);
  }
}
```

## Advanced Unit Testing Patterns

### Pattern: Test Fixture Builder
```rust
/// Builder pattern for creating test fixtures
struct CommandFixtureBuilder {
  name: String,
  description: String,
  arguments: Vec<ArgumentDefinition>,
}

impl CommandFixtureBuilder {
  fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      description: "Test command".to_string(),
      arguments: Vec::new(),
    }
  }

  fn with_argument(mut self, name: &str, kind: Kind) -> Self {
    self.arguments.push(ArgumentDefinition {
      name: name.to_string(),
      kind,
      ..Default::default()
    });
    self
  }

  fn build(self) -> CommandDefinition {
    CommandDefinition::former()
      .name(self.name)
      .description(self.description)
      .arguments(self.arguments)
      .end()
  }
}

#[test]
fn test_complex_command_validation() {
  let command = CommandFixtureBuilder::new(".complex")
    .with_argument("input", Kind::String)
    .with_argument("count", Kind::Integer)
    .with_argument("verbose", Kind::Boolean)
    .build();

  // Test with fixture
  // ...
}
```

### Pattern: Property-Based Testing
```rust
use proptest::prelude::*;

/// Property-based testing for parser robustness
proptest! {
  #[test]
  fn test_parser_handles_arbitrary_quoted_strings(
    content in ".*"
  ) {
    let input = format!(r#".test arg::"{}""#, content.replace('"', r#"\""#));
    let parser = Parser::new(UnilangParserOptions::default());

    let result = parser.parse_single_instruction(&input);
    // Parser should either succeed with correct content or fail gracefully
    match result {
      Ok(instruction) => {
        assert_eq!(instruction.named_arguments["arg"][0].value, content);
      }
      Err(_) => {
        // Graceful failure is acceptable for malformed input
      }
    }
  }
}
```

### Pattern: Parameterized Testing
```rust
/// Parameterized tests for multiple similar scenarios
#[test]
fn test_argument_type_validation() {
  let test_cases = vec![
    (r#".test count::42"#, Kind::Integer, true),
    (r#".test count::"not_a_number""#, Kind::Integer, false),
    (r#".test flag::true"#, Kind::Boolean, true),
    (r#".test flag::"maybe""#, Kind::Boolean, false),
    (r#".test name::"text""#, Kind::String, true),
  ];

  for (input, expected_kind, should_pass) in test_cases {
    let result = validate_argument_type(input, expected_kind);
    assert_eq!(result.is_ok(), should_pass, "Failed for input: {}", input);
  }
}
```

## Integration Testing Strategies

### Pattern: Component Contract Testing
```rust
/// Tests contracts between components
#[test]
fn test_semantic_analyzer_parser_contract() {
  // Arrange - Create parser output that semantic analyzer expects
  let parser = Parser::new(UnilangParserOptions::default());
  let instruction = parser.parse_single_instruction(".test arg::value").unwrap();

  // Verify parser output meets semantic analyzer's contract
  assert!(!instruction.command_name.is_empty());
  assert!(instruction.command_name.starts_with('.'));

  // Test semantic analyzer can process parser output
  let registry = create_test_registry();
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new(&instructions, &registry);

  // Should succeed because contract is satisfied
  assert!(analyzer.analyze().is_ok());
}
```

### Pattern: State Transition Testing
```rust
/// Tests state transitions in complex workflows
#[test]
fn test_command_execution_state_transitions() {
  let mut pipeline = Pipeline::new();

  // Test state transitions
  assert_eq!(pipeline.state(), PipelineState::Ready);

  pipeline.load_command(".test");
  assert_eq!(pipeline.state(), PipelineState::Loaded);

  pipeline.validate();
  assert_eq!(pipeline.state(), PipelineState::Validated);

  pipeline.execute();
  assert_eq!(pipeline.state(), PipelineState::Executed);

  pipeline.reset();
  assert_eq!(pipeline.state(), PipelineState::Ready);
}
```

## Acceptance Testing Approaches

### Pattern: User Story Implementation
```rust
/// Tests implement user stories directly
#[test]
fn test_user_story_multiple_file_processing() {
  // As a user, I want to process multiple files with the same command
  // So that I can apply the same operation to a batch of files

  let cli = TestCLI::new();
  cli.create_files(&["file1.txt", "file2.txt", "file3.txt"]);

  // User executes command with multiple file parameters
  let result = cli.run(&[
    ".process",
    "file::file1.txt",
    "file::file2.txt",
    "file::file3.txt"
  ]);

  // All files should be processed
  assert!(result.success);
  assert!(result.output.contains("3 files processed"));

  // Output files should exist
  assert!(cli.file_exists("file1_processed.txt"));
  assert!(cli.file_exists("file2_processed.txt"));
  assert!(cli.file_exists("file3_processed.txt"));
}
```

### Pattern: Error Recovery Testing
```rust
/// Tests user experience during error conditions
#[test]
fn test_user_friendly_error_recovery() {
  let cli = TestCLI::new();

  // User makes a typo in command name
  let result = cli.run(&[".tset", "arg::value"]);  // typo: "tset" instead of "test"

  // Should provide helpful error message
  assert!(!result.success);
  assert!(result.error.contains("Unknown command"));
  assert!(result.error.contains("Did you mean '.test'?"));

  // User should be able to continue after error
  let recovery_result = cli.run(&[".test", "arg::value"]);
  assert!(recovery_result.success);
}
```

## Regression Testing Patterns

### Pattern: Golden Master Testing
```rust
/// Compares output against known good reference
#[test]
fn regression_help_output_format() {
  let registry = create_standard_test_registry();
  let help_generator = HelpGenerator::new(&registry);
  let help_content = help_generator.command(".test").unwrap();

  // Compare against golden master
  let expected_help = include_str!("golden_masters/test_command_help.txt");
  assert_eq!(help_content.trim(), expected_help.trim());
}
```

### Pattern: Performance Regression Testing
```rust
/// Ensures performance doesn't degrade
#[test]
fn regression_parsing_performance() {
  let large_input = generate_large_command_input(1000); // 1000 arguments
  let parser = Parser::new(UnilangParserOptions::default());

  let start = Instant::now();
  let result = parser.parse_single_instruction(&large_input);
  let duration = start.elapsed();

  // Should still parse successfully
  assert!(result.is_ok());

  // Performance should not degrade (generous threshold for CI)
  assert!(duration.as_millis() < 500, "Parsing 1000 arguments took {:?}", duration);
}
```

## Test Data Management

### Pattern: Test Data Factories
```rust
/// Centralized test data creation
pub mod test_factories {
  use super::*;

  pub fn create_simple_command(name: &str) -> CommandDefinition {
    CommandDefinition::former()
      .name(name)
      .description("Simple test command")
      .arguments(vec![
        ArgumentDefinition {
          name: "input".to_string(),
          kind: Kind::String,
          ..Default::default()
        }
      ])
      .end()
  }

  pub fn create_complex_command(name: &str) -> CommandDefinition {
    CommandDefinition::former()
      .name(name)
      .description("Complex test command")
      .arguments(vec![
        ArgumentDefinition {
          name: "file".to_string(),
          kind: Kind::String,
          attributes: ArgumentAttributes {
            multiple: true,
            ..Default::default()
          },
          ..Default::default()
        },
        ArgumentDefinition {
          name: "count".to_string(),
          kind: Kind::Integer,
          ..Default::default()
        }
      ])
      .end()
  }
}
```

### Pattern: Test Database Setup
```rust
/// Manages test data lifecycle
pub struct TestDataManager {
  temp_dir: tempfile::TempDir,
}

impl TestDataManager {
  pub fn new() -> Self {
    Self {
      temp_dir: tempfile::tempdir().unwrap(),
    }
  }

  pub fn create_test_files(&self, files: &[(&str, &str)]) -> Vec<PathBuf> {
    files.iter().map(|(name, content)| {
      let path = self.temp_dir.path().join(name);
      std::fs::write(&path, content).unwrap();
      path
    }).collect()
  }

  pub fn temp_path(&self) -> &Path {
    self.temp_dir.path()
  }
}

#[test]
fn test_with_managed_data() {
  let data_manager = TestDataManager::new();
  let files = data_manager.create_test_files(&[
    ("input.txt", "test data"),
    ("config.json", r#"{"setting": "value"}"#),
  ]);

  // Use files in test
  // They are automatically cleaned up when data_manager is dropped
}
```

## Performance Testing Patterns

### Pattern: Benchmark Integration
```rust
/// Performance tests integrated with regular test suite
#[test]
fn test_simd_tokenizer_performance_characteristics() {
  use std::time::Instant;

  let simd_tokenizer = SimdTokenizer::new(default_options());
  let fallback_tokenizer = SimdTokenizer::new(fallback_options());

  // Create large input to test performance
  let large_input = create_large_tokenization_input(10000);

  // Warm up both tokenizers
  let _ = simd_tokenizer.tokenize(&large_input);
  let _ = fallback_tokenizer.tokenize(&large_input);

  // Measure SIMD performance
  let start = Instant::now();
  let simd_result = simd_tokenizer.tokenize(&large_input).unwrap();
  let simd_duration = start.elapsed();

  // Measure fallback performance
  let start = Instant::now();
  let fallback_result = fallback_tokenizer.tokenize(&large_input).unwrap();
  let fallback_duration = start.elapsed();

  // Verify results are equivalent
  assert_eq!(simd_result.len(), fallback_result.len());

  // Log performance comparison
  println!("SIMD: {:?}, Fallback: {:?}", simd_duration, fallback_duration);

  // SIMD should be faster or equivalent (don't enforce strict requirements)
  assert!(simd_duration <= fallback_duration * 2,
    "SIMD performance significantly degraded");
}
```

### Pattern: Memory Usage Testing
```rust
/// Tests memory usage patterns
#[test]
fn test_memory_usage_with_large_datasets() {
  let initial_memory = get_memory_usage();

  {
    let mut registry = CommandRegistry::new();

    // Load large number of commands
    for i in 0..10000 {
      let cmd = create_test_command(&format!(".test{}", i));
      registry.command_add_runtime(&cmd, mock_routine()).unwrap();
    }

    let peak_memory = get_memory_usage();
    let memory_growth = peak_memory - initial_memory;

    // Memory usage should be reasonable (allow for some overhead)
    assert!(memory_growth < 100_000_000, // 100MB
      "Memory usage too high: {} bytes", memory_growth);
  }

  // Memory should be released after scope
  std::thread::sleep(std::time::Duration::from_millis(100)); // Allow cleanup
  let final_memory = get_memory_usage();
  assert!(final_memory < initial_memory + 10_000_000, // 10MB overhead acceptable
    "Memory leak detected");
}

fn get_memory_usage() -> usize {
  // Implementation depends on platform
  // Could use system APIs or external tools
  0 // Placeholder
}
```

## Error Testing Strategies

### Pattern: Comprehensive Error Coverage
```rust
/// Tests all error paths systematically
#[test]
fn test_semantic_analysis_error_coverage() {
  let test_cases = vec![
    // Missing required arguments
    (r#".test"#, "missing required argument"),

    // Wrong argument types
    (r#".test count::"not_a_number""#, "type mismatch"),

    // Unknown commands
    (r#".unknown_command"#, "unknown command"),

    // Invalid argument names
    (r#".test invalid_arg::value"#, "unknown argument"),

    // Validation rule violations
    (r#".test name::"a""#, "minimum length"), // Assuming min length > 1
  ];

  let registry = create_test_registry();

  for (input, expected_error_fragment) in test_cases {
    let parser = Parser::new(UnilangParserOptions::default());
    let instruction = parser.parse_single_instruction(input).unwrap();
    let instructions = [instruction];
    let analyzer = SemanticAnalyzer::new(&instructions, &registry);

    let result = analyzer.analyze();
    assert!(result.is_err(), "Expected error for input: {}", input);

    let error = format!("{:?}", result.unwrap_err());
    assert!(error.to_lowercase().contains(&expected_error_fragment.to_lowercase()),
      "Error '{}' should contain '{}'", error, expected_error_fragment);
  }
}
```

### Pattern: Error Recovery Testing
```rust
/// Tests system behavior after errors
#[test]
fn test_error_recovery_and_continuation() {
  let mut pipeline = Pipeline::new();

  // Cause an error
  let error_result = pipeline.process_command(".invalid_command");
  assert!(error_result.is_err());

  // System should still be usable after error
  let recovery_result = pipeline.process_command(".test arg::value");
  assert!(recovery_result.is_ok(), "System should recover from errors");

  // Error state should not affect subsequent operations
  let second_result = pipeline.process_command(".test arg::value2");
  assert!(second_result.is_ok(), "Error state should not persist");
}
```

## Anti-Patterns to Avoid

### ❌ Testing Implementation Details
```rust
// Bad - Tests internal implementation
#[test]
fn test_parser_internal_state() {
  let parser = Parser::new(UnilangParserOptions::default());
  parser.parse_single_instruction(".test");

  // Don't test internal fields that could change
  assert_eq!(parser.internal_state.position, 5);
  assert_eq!(parser.internal_state.tokens.len(), 1);
}

// Good - Tests behavior
#[test]
fn test_parser_produces_correct_instruction() {
  let parser = Parser::new(UnilangParserOptions::default());
  let result = parser.parse_single_instruction(".test");

  // Test observable behavior
  assert!(result.is_ok());
  assert_eq!(result.unwrap().command_name, ".test");
}
```

### ❌ Overly Complex Test Setup
```rust
// Bad - Complex setup obscures test intent
#[test]
fn test_complex_scenario() {
  let mut builder = ComplexTestBuilder::new()
    .with_database_connection()
    .with_mock_file_system()
    .with_network_simulation()
    .with_complex_registry()
    .with_custom_configuration()
    .build();

  // 50 lines of setup...

  // Actual test is lost in complexity
  assert!(builder.some_operation());
}

// Good - Simple, focused test
#[test]
fn test_specific_behavior() {
  let parser = Parser::new(UnilangParserOptions::default());
  let result = parser.parse_single_instruction(".test");

  assert!(result.is_ok());
}
```

### ❌ Fragile Assertions
```rust
// Bad - Fragile assertions that break with minor changes
#[test]
fn test_help_output() {
  let help = generate_help();

  // Exact string matching is fragile
  assert_eq!(help, "Usage: .test (v1.0.0)\n  A test command\n\nArguments:\n  input (Type: String)\n    Input file path\n");
}

// Good - Flexible assertions for essential content
#[test]
fn test_help_contains_essential_information() {
  let help = generate_help();

  // Test essential content, not exact format
  assert!(help.contains("Usage:"));
  assert!(help.contains(".test"));
  assert!(help.contains("Arguments:"));
  assert!(help.contains("input"));
}
```

### ❌ Test Interdependence
```rust
// Bad - Tests depend on execution order
static mut COUNTER: i32 = 0;

#[test]
fn test_first() {
  unsafe { COUNTER += 1; }
  assert_eq!(unsafe { COUNTER }, 1);
}

#[test]
fn test_second() {
  // Depends on test_first running first
  assert_eq!(unsafe { COUNTER }, 1);
  unsafe { COUNTER += 1; }
}

// Good - Independent tests
#[test]
fn test_independent_behavior_a() {
  let state = create_test_state();
  let result = operation_a(&state);
  assert!(result.is_ok());
}

#[test]
fn test_independent_behavior_b() {
  let state = create_test_state();
  let result = operation_b(&state);
  assert!(result.is_ok());
}
```

## Conclusion

These advanced testing patterns provide a solid foundation for maintaining high-quality, maintainable tests in the systematic organization structure. Key takeaways:

1. **Focus on behavior, not implementation**
2. **Keep tests simple and focused**
3. **Use appropriate patterns for each test category**
4. **Maintain test independence and isolation**
5. **Provide clear, descriptive test names**
6. **Test error conditions comprehensively**
7. **Avoid anti-patterns that lead to fragile tests**

Following these patterns ensures that the test suite remains valuable as the codebase evolves, providing confidence in refactoring and new feature development.