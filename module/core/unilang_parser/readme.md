# unilang_parser

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/unilang_parser.svg)](https://crates.io/crates/unilang_parser)
[![Documentation](https://docs.rs/unilang_parser/badge.svg)](https://docs.rs/unilang_parser)

A high-performance, spec-compliant parser for the Unilang CLI instruction syntax. This crate transforms CLI-like instruction strings into structured `GenericInstruction` objects, enabling developers to build sophisticated command-line interfaces with consistent parsing behavior.

## Why unilang_parser?

Building robust CLI parsers from scratch is complex and error-prone. The `unilang_parser` solves this by providing:

- **🎯 Consistent Syntax**: Follows the formal Unilang specification for predictable parsing behavior
- **⚡ High Performance**: Leverages `strs_tools` for efficient tokenization with minimal allocations
- **🔧 Flexible Configuration**: Customizable parsing rules through `UnilangParserOptions`
- **📍 Precise Error Reporting**: Detailed error messages with exact source locations
- **🌐 Universal Design**: Works across CLI, GUI, TUI, and Web API modalities
- **🚫 `no_std` Support**: Can be used in embedded and resource-constrained environments

## Key Features

### Core Parsing Capabilities
- **Command Paths**: Single and multi-segment paths (`cmd`, `namespace.command`, `deep.nested.path`)  
- **Arguments**: Both positional (`arg1 arg2`) and named (`key::value`) arguments
- **Quoting & Escaping**: Handles quoted strings (`"value"`, `'value'`) with escape sequences (`\"`, `\\`, `\n`, etc.)
- **Help Operator**: Built-in support for `?` help requests
- **Multiple Instructions**: Parse command sequences separated by `;;`

### Advanced Features
- **Configurable Parsing**: Control duplicate argument handling, positional vs named argument order
- **Location-Aware Errors**: `ParseError` with `ErrorKind` and precise `SourceLocation` information
- **Robust Error Handling**: Comprehensive error categorization for better user experience
- **Memory Efficient**: Built on `strs_tools` for optimal performance

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
unilang_parser = "0.2"
```

For `no_std` environments:

```toml
[dependencies]
unilang_parser = { version = "0.2", default-features = false, features = ["no_std"] }
```

## Quick Start

```rust
use unilang_parser::{Parser, UnilangParserOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    // Create parser with default options
    let parser = Parser::new(UnilangParserOptions::default());
    
    // Parse a single instruction
    let instruction = parser.parse_single_instruction(
        "file.copy src::\"/path/to/source.txt\" dest::\"/path/to/dest.txt\" --overwrite"
    )?;
    
    println!("Command: {:?}", instruction.command_path_slices);
    println!("Arguments: {:?}", instruction.arguments);
    
    Ok(())
}
```

## Running Examples

The `examples/` directory contains comprehensive, runnable examples demonstrating all parser features:

```bash
# Run the basic usage example
cargo run --example unilang_parser_basic

# Run specific feature examples
cargo run --example 01_basic_command_parsing
cargo run --example 02_named_arguments_quoting
cargo run --example 03_complex_argument_patterns
cargo run --example 04_multiple_instructions
cargo run --example 05_help_operator_usage
cargo run --example 06_advanced_escaping_quoting
cargo run --example 07_error_handling_diagnostics
cargo run --example 08_custom_parser_configuration
cargo run --example 09_integration_command_frameworks
cargo run --example 10_performance_optimization_patterns
```

Each example file includes:
- Clear documentation of what it demonstrates
- Practical, real-world usage scenarios  
- Detailed comments explaining the code
- Expected output and behavior

## Comprehensive Examples

### 1. Basic Command Parsing

```rust
use unilang_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());

// Simple command
let cmd = parser.parse_single_instruction("system.info")?;
assert_eq!(cmd.command_path_slices, ["system", "info"]);

// Command with positional arguments
let cmd = parser.parse_single_instruction("log.write \"Error occurred\" 5")?;
assert_eq!(cmd.command_path_slices, ["log", "write"]);
assert_eq!(cmd.arguments.len(), 2);
```

### 2. Named Arguments and Quoting

```rust
use unilang_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());

// Named arguments with quoting
let cmd = parser.parse_single_instruction(
    r#"database.query sql::"SELECT * FROM users WHERE name = 'John'" timeout::30"#
)?;

println!("SQL: {}", cmd.named_arguments.get("sql").unwrap());
println!("Timeout: {}", cmd.named_arguments.get("timeout").unwrap());
```

### 3. Complex Argument Patterns

```rust
use unilang_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());

// Mixed positional and named arguments
let cmd = parser.parse_single_instruction(
    "server.deploy production config::\"/etc/app.conf\" replicas::3 --verbose --dry-run"
)?;

assert_eq!(cmd.arguments[0], "production"); // positional
assert_eq!(cmd.named_arguments.get("config").unwrap(), "/etc/app.conf");
assert_eq!(cmd.named_arguments.get("replicas").unwrap(), "3");
```

### 4. Multiple Instructions

```rust
use unilang_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());

// Parse command sequence
let instructions = parser.parse_multiple_instructions(
    "backup.create name::daily ;; cloud.upload file::daily.tar.gz ;; notify.send \"Backup complete\""
)?;

assert_eq!(instructions.len(), 3);
assert_eq!(instructions[0].command_path_slices, ["backup", "create"]);
assert_eq!(instructions[1].command_path_slices, ["cloud", "upload"]);
assert_eq!(instructions[2].command_path_slices, ["notify", "send"]);
```

### 5. Help Operator Usage

```rust
use unilang_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());

// Command help
let cmd = parser.parse_single_instruction("file.copy ?")?;
assert!(cmd.help_invoked);

// Contextual help with arguments
let cmd = parser.parse_single_instruction("database.migrate version::1.2.0 ?")?;
assert!(cmd.help_invoked);
assert_eq!(cmd.named_arguments.get("version").unwrap(), "1.2.0");
```

### 6. Advanced Escaping and Quoting

```rust
use unilang_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());

// Complex escaping scenarios
let cmd = parser.parse_single_instruction(
    r#"log.message text::"Line 1\nLine 2\tTabbed" pattern::"\\d+\\.\\d+""#
)?;

// The parser handles escape sequences
assert_eq!(cmd.named_arguments.get("text").unwrap(), "Line 1\nLine 2\tTabbed");
assert_eq!(cmd.named_arguments.get("pattern").unwrap(), r"\d+\.\d+");
```

### 7. Error Handling and Diagnostics

```rust
use unilang_parser::{Parser, UnilangParserOptions, ErrorKind};

let parser = Parser::new(UnilangParserOptions::default());

// Handle parsing errors
match parser.parse_single_instruction("invalid..command") {
    Ok(_) => unreachable!(),
    Err(error) => {
        match error.kind {
            ErrorKind::InvalidCommandPath => {
                println!("Invalid command path at position {}", error.location.start());
            },
            _ => println!("Other error: {}", error),
        }
    }
}
```

### 8. Custom Parser Configuration

```rust
use unilang_parser::{Parser, UnilangParserOptions};

// Configure strict parsing rules
let options = UnilangParserOptions {
    error_on_duplicate_named_arguments: true,
    error_on_positional_after_named: true,
};

let parser = Parser::new(options);

// This will error due to duplicate arguments
let result = parser.parse_single_instruction("cmd arg1::val1 arg1::val2");
assert!(result.is_err());
```

### 9. Integration with Command Frameworks

```rust
use unilang_parser::{Parser, UnilangParserOptions, GenericInstruction};

// Example: Converting to your application's command structure
#[derive(Debug)]
struct AppCommand 
{
    name: String,
    args: std::collections::HashMap<String, String>,
}

fn convert_instruction(instruction: GenericInstruction) -> AppCommand 
{
    AppCommand {
        name: instruction.command_path_slices.join("."),
        args: instruction.named_arguments,
    }
}

let parser = Parser::new(UnilangParserOptions::default());
let instruction = parser.parse_single_instruction("user.create name::john email::john@example.com")?;
let app_cmd = convert_instruction(instruction);

println!("App command: {:?}", app_cmd);
```

### 10. Performance Optimization Patterns

```rust
use unilang_parser::{Parser, UnilangParserOptions};

// Reuse parser instance for better performance
let parser = Parser::new(UnilangParserOptions::default());

let commands = vec![
    "system.status",
    "user.list active::true",
    "report.generate format::pdf output::\"/tmp/report.pdf\"",
];

for cmd_str in commands {
    match parser.parse_single_instruction(cmd_str) {
        Ok(instruction) => {
            // Process instruction
            println!("Processing: {:?}", instruction.command_path_slices);
        },
        Err(e) => eprintln!("Parse error in '{}': {}", cmd_str, e),
    }
}
```

## API Reference

### Core Types

- **`Parser`**: Main parsing engine
- **`GenericInstruction`**: Parsed instruction with command path and arguments  
- **`UnilangParserOptions`**: Configuration for parsing behavior
- **`ParseError`**: Detailed error information with source location
- **`Argument`**: Individual argument representation

### Key Methods

- **`Parser::new(options)`**: Create parser with configuration
- **`parse_single_instruction(input)`**: Parse one command
- **`parse_multiple_instructions(input)`**: Parse `;;`-separated commands

## Integration with the Unilang Ecosystem

This parser is part of the larger Unilang framework:

- **`unilang`**: Core framework for building multi-modal command interfaces
- **`unilang_meta`**: Procedural macros for compile-time command definitions
- **`unilang_parser`** (this crate): Dedicated instruction parsing

The parser outputs `GenericInstruction` objects that are consumed by the `unilang` framework for semantic analysis and execution.

## Performance Characteristics

- **Zero-copy parsing** where possible using string slices
- **Minimal allocations** through efficient use of `strs_tools`
- **Linear time complexity** O(n) relative to input length
- **Suitable for real-time applications** with microsecond parsing times

## Error Categories

The parser provides detailed error classification:

- `InvalidCommandPath`: Malformed command paths
- `InvalidArgument`: Malformed argument syntax  
- `UnterminatedQuotedString`: Missing closing quotes
- `InvalidEscapeSequence`: Malformed escape sequences
- `DuplicateNamedArgument`: Duplicate argument names (when configured)
- `PositionalAfterNamed`: Positional args after named (when configured)

## Specification Compliance

This parser implements the official Unilang CLI syntax specification, ensuring consistent behavior across all Unilang-based applications. See `spec.md` for complete syntax rules and grammar.

## Examples Directory

All code examples shown in this README are available as complete, runnable programs in the [`examples/`](examples/) directory:

| Example File | Description | Key Features Demonstrated |
|--------------|-------------|---------------------------|
| [`unilang_parser_basic.rs`](examples/unilang_parser_basic.rs) | Comprehensive basic usage | Parser creation, single/multiple instructions, argument access |
| [`01_basic_command_parsing.rs`](examples/01_basic_command_parsing.rs) | Simple command parsing | Command paths, positional arguments |
| [`02_named_arguments_quoting.rs`](examples/02_named_arguments_quoting.rs) | Named arguments | Named args with `::`, single/double quotes |
| [`03_complex_argument_patterns.rs`](examples/03_complex_argument_patterns.rs) | Mixed argument types | Positional + named args, flag-like arguments |
| [`04_multiple_instructions.rs`](examples/04_multiple_instructions.rs) | Command sequences | `;;` separated commands, workflow patterns |
| [`05_help_operator_usage.rs`](examples/05_help_operator_usage.rs) | Help requests | `?` operator, contextual help |
| [`06_advanced_escaping_quoting.rs`](examples/06_advanced_escaping_quoting.rs) | Complex strings | Escape sequences, regex patterns, JSON content |
| [`07_error_handling_diagnostics.rs`](examples/07_error_handling_diagnostics.rs) | Error handling | Error types, location info, diagnostics |
| [`08_custom_parser_configuration.rs`](examples/08_custom_parser_configuration.rs) | Parser configuration | Strict vs permissive parsing options |
| [`09_integration_command_frameworks.rs`](examples/09_integration_command_frameworks.rs) | Framework integration | Command dispatch, validation, aliasing |
| [`10_performance_optimization_patterns.rs`](examples/10_performance_optimization_patterns.rs) | Performance optimization | Instance reuse, batch processing, streaming |

**To run any example:**
```bash
cargo run --example <filename_without_extension>
```

**To run all examples:**
```bash
for example in examples/*.rs; do
  echo "Running $example..."
  cargo run --example $(basename "$example" .rs)
done
```

## Contributing

We welcome contributions! Please see our [contribution guidelines](https://github.com/Wandalen/wTools/blob/master/CONTRIBUTING.md) for details on:

- Reporting bugs
- Suggesting features  
- Submitting pull requests
- Code style guidelines

## License

Licensed under the [MIT License](license).
