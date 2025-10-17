# Test Examples and Pattern Demonstrations

This directory contains comprehensive examples demonstrating best practices for each test category in the systematic organization structure. These examples serve as both documentation and templates for writing high-quality tests.

## Example Files Overview

### [`unit_test_example.rs`](unit_test_example.rs)
**Component isolation and focused testing patterns**

Demonstrates:
- ✅ **Single responsibility principle** - Each test verifies one behavior
- ✅ **Component isolation** - Testing individual components with mocked dependencies
- ✅ **Boundary testing** - Edge cases and limits
- ✅ **Error condition testing** - Comprehensive error scenarios
- ✅ **Property-based testing** - Robustness with generated inputs
- ✅ **Mock and dependency injection** - Proper mocking patterns
- ✅ **Test helper functions** - Reusable utilities

**Key Patterns:**
```rust
#[test]
fn test_semantic_analyzer_validates_required_arguments()
{
  // Arrange - Set up test data and dependencies
  let mut registry = CommandRegistry::new();
  // ... setup code

  // Act - Exercise the component under test
  let result = analyzer.analyze();

  // Assert - Verify expected behavior
  assert!(result.is_err(), "Should reject command missing required argument");
}
```

### [`integration_test_example.rs`](integration_test_example.rs)
**Component interaction and data flow testing patterns**

Demonstrates:
- ✅ **End-to-end component interaction** - Multiple components working together
- ✅ **Data flow validation** - Information flow through pipeline
- ✅ **Component contract testing** - Interface compliance
- ✅ **State transition testing** - Stateful workflow validation
- ✅ **Error propagation** - Error handling across boundaries
- ✅ **Performance integration** - Performance impact of interactions
- ✅ **Configuration integration** - Configuration effects on components

**Key Patterns:**
```rust
#[test]
fn test_complete_command_processing_pipeline()
{
  // 1. Parse command
  let instruction = parser.parse_single_instruction(input_command).expect(...);

  // 2. Semantic analysis
  let verified_commands = analyzer.analyze().expect(...);

  // 3. Command execution
  let output = interpreter.execute(verified_command, context).expect(...);

  // Verify complete data flow
  assert_eq!(output.format, "json");
}
```

### [`acceptance_test_example.rs`](acceptance_test_example.rs)
**User scenario and CLI interaction testing patterns**

Demonstrates:
- ✅ **User workflow simulation** - Complete user journeys
- ✅ **CLI interface testing** - Real command-line interactions
- ✅ **Help system user experience** - Documentation usability
- ✅ **Error handling UX** - User-friendly error messages
- ✅ **Interactive sessions** - Multi-command workflows
- ✅ **Edge case scenarios** - Unusual but valid user inputs
- ✅ **Performance from user perspective** - Responsiveness testing
- ✅ **Cross-platform experience** - Platform compatibility

**Key Patterns:**
```rust
#[test]
fn test_user_workflow_file_processing()
{
  let cli = TestCLI::new();

  // Set up user environment
  cli.create_file("config1.json", r#"{"name": "app1"}"#);

  // User executes command
  let result = cli.run(&[".process", "file::config1.json"]);

  // Verify user expectations
  assert!(result.success, "Command should succeed");
  assert!(result.stdout.contains("processed"));
}
```

### [`regression_test_example.rs`](regression_test_example.rs)
**Bug prevention and compatibility testing patterns**

Demonstrates:
- ✅ **Exact bug reproduction** - Specific failing scenarios
- ✅ **Backward compatibility** - Ensuring old patterns work
- ✅ **Performance regression** - Performance baseline protection
- ✅ **Edge case robustness** - Previously problematic edge cases
- ✅ **Configuration compatibility** - Config stability
- ✅ **API stability** - Public interface protection
- ✅ **Golden master testing** - Reference output comparison

**Key Patterns:**
```rust
#[test]
fn regression_task_024_multiple_parameter_collection_exact_reproduction()
{
  // BUG CONTEXT: Task 024 - Multiple parameter collection was failing when...
  // ORIGINAL FAILING SCENARIO: Command with multiple same-name parameters
  // FIX IMPLEMENTED: Modified collection logic for backward compatibility

  // Execute the EXACT failing scenario from Task 024
  let instruction = parser.parse_single_instruction(
    r#".run command::"cargo build" command::"echo hello" command::"cargo clippy""#
  ).expect("Should parse the exact Task 024 command");

  // Verify the bug is fixed
  assert_eq!(command_values.len(), 3, "Should collect all three commands");
}
```

## Testing Best Practices Illustrated

### 1. Test Structure and Organization

**Arrange-Act-Assert Pattern** (Consistent across all examples)
```rust
#[test]
fn test_example()
{
  // Arrange - Set up test conditions
  let setup = create_test_setup();

  // Act - Execute the behavior being tested
  let result = setup.perform_action();

  // Assert - Verify the expected outcome
  assert!(result.is_ok(), "Should succeed");
}
```

**Descriptive Test Names**
```rust
// ✅ Good - Describes what the test verifies
#[test]
fn test_semantic_analyzer_rejects_unknown_commands() { }

#[test]
fn test_multiple_parameter_collection_preserves_order() { }

// ❌ Bad - Unclear purpose
#[test]
fn test_parser() { }

#[test]
fn test_edge_case() { }
```

### 2. Error Testing Patterns

**Comprehensive Error Coverage**
```rust
let error_cases = vec![
  ("missing_arg", "required argument"),
  ("wrong_type", "type mismatch"),
  ("unknown_cmd", "unknown command"),
];

for (input, expected_error) in error_cases {
  let result = analyzer.analyze_input(input);
  assert!(result.is_err());
  assert!(error_contains(result, expected_error));
}
```

**Error Recovery Testing**
```rust
// Cause an error
let error_result = system.process_invalid_input();
assert!(error_result.is_err());

// System should still be usable
let recovery_result = system.process_valid_input();
assert!(recovery_result.is_ok(), "System should recover from errors");
```

### 3. Mock and Test Double Patterns

**Dependency Injection for Testing**
```rust
let mock_routine = Box::new(|cmd: VerifiedCommand, _ctx: ExecutionContext| {
  // Mock implementation that can be verified
  Ok(OutputData { content: "mock_response".to_string(), format: "text".to_string() })
});

registry.command_add_runtime(&cmd, mock_routine).unwrap();
```

**Interaction Verification**
```rust
let call_count = Arc::new(Mutex::new(0));
let mock_routine = Box::new(move |cmd: VerifiedCommand, _ctx: ExecutionContext| {
  *call_count_clone.lock().unwrap() += 1;
  // Verify parameters passed correctly
  assert_eq!(cmd.definition.name, ".expected_command");
  Ok(OutputData { content: "success".to_string(), format: "text".to_string() })
});
```

### 4. Performance Testing Patterns

**Baseline Performance Testing**
```rust
let start_time = Instant::now();
let result = system.process_large_input(large_data);
let duration = start_time.elapsed();

assert!(result.is_ok(), "Should handle large input");
assert!(duration.as_millis() < 1000, "Should complete within reasonable time");
```

**Performance Regression Detection**
```rust
// Measure before and after
let baseline_duration = measure_baseline_performance();
let current_duration = measure_current_performance();

assert!(current_duration <= baseline_duration * 1.5,
  "Performance should not degrade significantly");
```

### 5. Data-Driven Testing

**Parameterized Test Cases**
```rust
let test_cases = vec![
  (r#".test count::42"#, Kind::Integer, true),
  (r#".test count::"not_a_number""#, Kind::Integer, false),
  (r#".test flag::true"#, Kind::Boolean, true),
];

for (input, expected_type, should_succeed) in test_cases {
  let result = validate_input(input, expected_type);
  assert_eq!(result.is_ok(), should_succeed, "Failed for: {}", input);
}
```

## Anti-Patterns Demonstrated

### ❌ What NOT to Do

**Testing Implementation Details**
```rust
// Bad - Tests internal structure that could change
assert_eq!(parser.internal_state.position, 5);

// Good - Tests observable behavior
assert_eq!(result.unwrap().command_name, ".test");
```

**Overly Complex Setup**
```rust
// Bad - Setup obscures test intent
let builder = ComplexBuilder::new()
  .with_database().with_network().with_custom_config()
  // 50 lines of setup...

// Good - Simple, focused setup
let parser = Parser::new(UnilangParserOptions::default());
```

**Test Interdependence**
```rust
// Bad - Tests depend on execution order
static mut COUNTER: i32 = 0;
#[test] fn test_first() { unsafe { COUNTER += 1; } }
#[test] fn test_second() { assert_eq!(unsafe { COUNTER }, 1); }

// Good - Independent tests
#[test] fn test_independent_a() { let state = create_fresh_state(); }
#[test] fn test_independent_b() { let state = create_fresh_state(); }
```

## Helper Utilities and Patterns

### Test Data Factories
```rust
fn create_simple_test_command(name: &str) -> CommandDefinition {
  CommandDefinition::former()
    .name(name)
    .description("Simple test command")
    .arguments(vec![/* standard test arguments */])
    .end()
}
```

### CLI Testing Utilities
```rust
struct TestCLI {
  temp_dir: TempDir,
  binary_path: String,
}

impl TestCLI {
  fn run(&self, args: &[&str]) -> CLIResult { /* execute CLI */ }
  fn create_file(&self, name: &str, content: &str) { /* setup files */ }
  fn file_exists(&self, name: &str) -> bool { /* verify outputs */ }
}
```

### Property-Based Testing Setup
```rust
#[cfg(feature = "proptest")]
proptest! {
  #[test]
  fn test_parser_robustness(input in "\\.[a-zA-Z_]+.*") {
    let result = parser.parse(input);
    // Should either succeed or fail gracefully (no panics)
  }
}
```

## Integration with Development Workflow

### Running Examples
```bash
# Run all examples
cargo test --test "*example*"

# Run specific category examples
cargo test --test unit_test_example
cargo test --test integration_test_example
cargo test --test acceptance_test_example
cargo test --test regression_test_example
```

### Using as Templates
1. **Copy relevant example** for your test category
2. **Adapt the pattern** to your specific component
3. **Maintain the structure** (Arrange-Act-Assert)
4. **Follow naming conventions** demonstrated
5. **Include proper documentation** as shown

### IDE Integration
- Examples work with standard Rust tooling
- Tests can be run individually in IDEs
- Debug breakpoints work normally
- Test output is captured properly

## Maintenance Guidelines

### Keeping Examples Current
1. **Update examples** when patterns evolve
2. **Add new patterns** as they're discovered
3. **Remove deprecated patterns** when appropriate
4. **Maintain consistency** across all examples

### Quality Standards
- ✅ Examples should always compile and pass
- ✅ Patterns should reflect current best practices
- ✅ Documentation should be clear and complete
- ✅ Code should be self-explanatory

## Benefits of Following These Patterns

✅ **Consistent test quality** across the codebase
✅ **Easier onboarding** for new team members
✅ **Reduced test maintenance** burden
✅ **Better test reliability** and stability
✅ **Improved debugging** when tests fail
✅ **Higher confidence** in refactoring
✅ **Better documentation** of expected behavior

These examples provide a solid foundation for maintaining high-quality tests in the systematic organization structure. By following these patterns, the test suite remains valuable as the codebase evolves, providing confidence in both new features and refactoring efforts.