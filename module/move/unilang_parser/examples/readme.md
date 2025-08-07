# unilang_parser Examples

This directory contains comprehensive, runnable examples demonstrating all features of the `unilang_parser` crate. Each example is self-contained and includes detailed comments explaining the concepts being demonstrated.

## 🚀 Quick Start

To run any example:

```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example unilang_parser_basic
```

## 📚 Example Index

### Core Examples

| Example | File | Description | Concepts |
|---------|------|-------------|----------|
| **Basic Usage** | [`unilang_parser_basic.rs`](unilang_parser_basic.rs) | Comprehensive introduction to all parser features | Parser creation, instruction parsing, argument access |
| **1. Basic Commands** | [`01_basic_command_parsing.rs`](01_basic_command_parsing.rs) | Simple command path parsing | Command paths, positional arguments |
| **2. Named Arguments** | [`02_named_arguments_quoting.rs`](02_named_arguments_quoting.rs) | Named arguments with quotes | `key::value` syntax, single/double quotes |
| **3. Complex Patterns** | [`03_complex_argument_patterns.rs`](03_complex_argument_patterns.rs) | Mixed argument types | Positional + named args, flag-like arguments |
| **4. Multiple Instructions** | [`04_multiple_instructions.rs`](04_multiple_instructions.rs) | Command sequences | `;;` separator, workflow patterns |
| **5. Help Operator** | [`05_help_operator_usage.rs`](05_help_operator_usage.rs) | Help requests | `?` operator, contextual help |

### Advanced Examples

| Example | File | Description | Concepts |
|---------|------|-------------|----------|
| **6. Advanced Escaping** | [`06_advanced_escaping_quoting.rs`](06_advanced_escaping_quoting.rs) | Complex string handling | Escape sequences, regex patterns, JSON |
| **7. Error Handling** | [`07_error_handling_diagnostics.rs`](07_error_handling_diagnostics.rs) | Comprehensive error handling | Error types, location info, diagnostics |
| **8. Configuration** | [`08_custom_parser_configuration.rs`](08_custom_parser_configuration.rs) | Parser customization | Strict vs permissive parsing |
| **9. Integration** | [`09_integration_command_frameworks.rs`](09_integration_command_frameworks.rs) | Framework integration | Command dispatch, validation, aliasing |
| **10. Performance** | [`10_performance_optimization_patterns.rs`](10_performance_optimization_patterns.rs) | Performance optimization | Instance reuse, batch processing |

## 🎯 Learning Path

### 1. Start Here - Fundamentals
```bash
# Get familiar with basic parser usage
cargo run --example unilang_parser_basic

# Learn simple command parsing
cargo run --example 01_basic_command_parsing

# Understand named arguments
cargo run --example 02_named_arguments_quoting
```

### 2. Core Features
```bash
# Master complex argument patterns
cargo run --example 03_complex_argument_patterns

# Learn command sequences
cargo run --example 04_multiple_instructions

# Understand help system
cargo run --example 05_help_operator_usage
```

### 3. Advanced Topics
```bash
# Handle complex strings and escaping
cargo run --example 06_advanced_escaping_quoting

# Master error handling
cargo run --example 07_error_handling_diagnostics

# Configure parser behavior
cargo run --example 08_custom_parser_configuration
```

### 4. Real-World Usage
```bash
# Integrate with existing systems
cargo run --example 09_integration_command_frameworks

# Optimize for performance
cargo run --example 10_performance_optimization_patterns
```

## 🔍 Example Categories

### By Difficulty Level

**🟢 Beginner**
- `unilang_parser_basic.rs` - Start here!
- `01_basic_command_parsing.rs`
- `02_named_arguments_quoting.rs`

**🟡 Intermediate**
- `03_complex_argument_patterns.rs`
- `04_multiple_instructions.rs`
- `05_help_operator_usage.rs`
- `07_error_handling_diagnostics.rs`

**🔴 Advanced**
- `06_advanced_escaping_quoting.rs`
- `08_custom_parser_configuration.rs`
- `09_integration_command_frameworks.rs`
- `10_performance_optimization_patterns.rs`

### By Use Case

**📝 CLI Development**
- `01_basic_command_parsing.rs` - Command structure
- `03_complex_argument_patterns.rs` - Argument handling
- `05_help_operator_usage.rs` - Help system
- `07_error_handling_diagnostics.rs` - User-friendly errors

**🔧 Framework Integration**
- `09_integration_command_frameworks.rs` - Building command systems
- `08_custom_parser_configuration.rs` - Customizing behavior
- `10_performance_optimization_patterns.rs` - Scaling considerations

**🎨 Advanced String Processing**
- `02_named_arguments_quoting.rs` - Basic quoting
- `06_advanced_escaping_quoting.rs` - Complex strings
- `04_multiple_instructions.rs` - Command chaining

## 🛠️ Running Examples

### Individual Examples
```bash
# Run a specific example
cargo run --example 01_basic_command_parsing

# Run with output capture
cargo run --example 02_named_arguments_quoting > output.txt
```

### Batch Execution
```bash
# Run all examples (Unix/Linux/macOS)
for example in examples/*.rs; do
  name=$(basename "$example" .rs)
  echo "=== Running $name ==="
  cargo run --example "$name"
  echo
done

# Run all examples (Windows PowerShell)
Get-ChildItem examples\*.rs | ForEach-Object {
  $name = $_.BaseName
  Write-Host "=== Running $name ==="
  cargo run --example $name
  Write-Host
}
```

### With Different Configurations
```bash
# Run with release optimizations (faster execution)
cargo run --release --example 10_performance_optimization_patterns

# Run with debugging info
RUST_LOG=debug cargo run --example 07_error_handling_diagnostics
```

## 📖 Understanding the Examples

### Code Structure
Each example follows a consistent structure:

```rust
//! Example Title
//!
//! This example demonstrates:
//! - Feature 1
//! - Feature 2
//! - Feature 3
//!
//! Run with: cargo run --example example_name

use unilang_parser::{Parser, UnilangParserOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example implementation with detailed comments
    println!("=== Example Title ===");
    
    // ... example code ...
    
    println!("✓ Example completed successfully!");
    Ok(())
}
```

### Key Concepts Explained

**Parser Creation**
```rust
let parser = Parser::new(UnilangParserOptions::default());
```

**Single Instruction Parsing**
```rust
let instruction = parser.parse_single_instruction("command arg::value")?;
```

**Multiple Instruction Parsing**
```rust
let instructions = parser.parse_multiple_instructions("cmd1 ;; cmd2")?;
```

**Accessing Results**
```rust
println!("Command: {:?}", instruction.command_path_slices);
println!("Args: {:?}", instruction.arguments);
println!("Named: {:?}", instruction.named_arguments);
println!("Help: {}", instruction.help_invoked);
```

## 🚦 Common Patterns

### Error Handling Pattern
```rust
match parser.parse_single_instruction(input) {
    Ok(instruction) => {
        // Process successful parse
        println!("Parsed: {:?}", instruction.command_path_slices);
    }
    Err(error) => {
        // Handle parse error
        eprintln!("Error: {} at position {}", error, error.location.start());
    }
}
```

### Batch Processing Pattern
```rust
let commands = vec!["cmd1", "cmd2", "cmd3"];
for cmd in commands {
    match parser.parse_single_instruction(cmd) {
        Ok(instruction) => process_instruction(instruction),
        Err(e) => eprintln!("Failed to parse '{}': {}", cmd, e),
    }
}
```

### Configuration Pattern
```rust
let options = UnilangParserOptions {
    error_on_duplicate_named_arguments: true,
    error_on_positional_after_named: false,
};
let parser = Parser::new(options);
```

## 🔗 Related Documentation

- **Main README**: [`../readme.md`](../readme.md) - Complete crate documentation
- **Specification**: [`../spec.md`](../spec.md) - Formal language specification
- **API Docs**: Run `cargo doc --open` for detailed API documentation
- **Tests**: [`../tests/`](../tests/) - Additional test cases and edge cases

## 💡 Tips for Learning

1. **Start Simple**: Begin with `unilang_parser_basic.rs` to understand the fundamentals
2. **Run Examples**: Execute each example to see the output and behavior
3. **Modify Code**: Try changing inputs and configurations to see different results
4. **Read Comments**: Each example has detailed explanations of what's happening
5. **Check Tests**: Look at the test files for additional usage patterns
6. **Experiment**: Create your own variations based on the examples

## 🐛 Troubleshooting

### Common Issues

**Example won't compile:**
```bash
# Ensure you're in the correct directory
cd /path/to/unilang_parser

# Update dependencies
cargo update

# Try a clean build
cargo clean && cargo build
```

**Example runs but produces errors:**
- Check that you're using the correct command syntax
- Review the example comments for expected behavior
- Some examples (like error handling) intentionally show error cases

**Performance seems slow:**
- Run with `--release` flag for optimized builds
- See `10_performance_optimization_patterns.rs` for optimization techniques

### Getting Help

1. **Read the source**: Examples are heavily commented
2. **Check the main README**: [`../README.md`](../README.md)
3. **Review tests**: [`../tests/`](../tests/) directory
4. **Open an issue**: [GitHub Issues](https://github.com/Wandalen/wTools/issues)

---

**Happy parsing! 🎉**

*These examples demonstrate the full power and flexibility of the unilang_parser crate. Each example is designed to be educational, practical, and immediately useful in your own projects.*