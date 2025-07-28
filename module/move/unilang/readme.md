# Unilang

Define your command-line utility interface once and get consistent interaction across multiple modalities — CLI, GUI, TUI, AUI, Web APIs, and more—essentially for free.

## Overview

Unilang is a command framework that allows you to define commands declaratively and execute them across different modalities. It provides a complete pipeline from command definition to execution, including parsing, semantic analysis, validation, and help generation.

## Core Features

- **📝 Declarative Command Definition**: Define commands using structured data with comprehensive metadata
- **🏪 Centralized Registry**: Manage all commands in a single registry with namespace organization
- **📄 External Configuration**: Load command definitions from YAML and JSON files
- **🔍 Robust Parsing**: Convert text input to structured instructions using the unilang_parser
- **🧠 Semantic Analysis**: Validate commands against registry with type checking and constraint enforcement
- **⚡ Flexible Execution**: Execute commands with proper context and error handling
- **📚 Automatic Help**: Generate comprehensive help documentation from command definitions
- **🛡️ Error Management**: Structured error handling with meaningful error codes and messages

## Data Types & Validation

### Supported Argument Types
- **Basic Types**: String, Integer, Float, Boolean, Path, File, Directory, URL, DateTime, Pattern
- **Choice Types**: Enum with predefined options
- **Collection Types**: List and Map with custom delimiters
- **Complex Types**: JSON strings and objects

### Validation Rules
- **Numeric**: min/max value constraints
- **String**: length constraints and regex pattern matching
- **Collection**: minimum item count requirements

### Argument Attributes
- **Optional**: Arguments that are not required
- **Multiple**: Arguments that can accept multiple values
- **Default**: Arguments with default values
- **Sensitive**: Arguments containing sensitive data
- **Interactive**: Arguments that may require user interaction

## Step-by-Step Tutorial

The `examples/` directory contains comprehensive tutorials demonstrating all capabilities:

### 01. Basic Command Registration
**File**: [`examples/01_basic_command_registration.rs`](examples/01_basic_command_registration.rs)

Learn the fundamentals of command registration:
- Creating a command registry
- Defining commands with metadata
- Implementing command routines
- Registering commands with their execution logic

```bash
cargo run --example 01_basic_command_registration
```

### 02. Argument Types
**File**: [`examples/02_argument_types.rs`](examples/02_argument_types.rs)

Explore all supported argument types:
- String, Integer, Float, Boolean types
- Path, File, Directory, URL types
- DateTime, Pattern, Enum types
- Type validation and conversion

```bash
cargo run --example 02_argument_types
```

### 03. Collection Types
**File**: [`examples/03_collection_types.rs`](examples/03_collection_types.rs)

Master collection types and custom delimiters:
- List types with various delimiters
- Map types with key-value pairs
- Custom delimiter configuration
- Nested collection structures

```bash
cargo run --example 03_collection_types
```

### 04. Validation Rules
**File**: [`examples/04_validation_rules.rs`](examples/04_validation_rules.rs)

Implement robust input validation:
- Numeric constraints (min/max)
- String length validation
- Pattern matching with regex
- Argument attribute configuration

```bash
cargo run --example 04_validation_rules
```

### 05. Namespaces and Aliases
**File**: [`examples/05_namespaces_and_aliases.rs`](examples/05_namespaces_and_aliases.rs)

Organize commands with namespaces and aliases:
- Namespace-based command organization
- Command and argument aliases
- Hierarchical command structure
- Alias resolution system

```bash
cargo run --example 05_namespaces_and_aliases
```

### 06. Help System
**File**: [`examples/06_help_system.rs`](examples/06_help_system.rs)

Generate comprehensive command documentation:
- Automatic help generation
- Command listing and detailed help
- Documentation best practices
- Interactive help access

```bash
cargo run --example 06_help_system
```

### 07. YAML and JSON Loading
**File**: [`examples/07_yaml_json_loading.rs`](examples/07_yaml_json_loading.rs)

Load commands from external configuration files:
- YAML command definitions
- JSON command specifications
- External file loading
- Configuration management

```bash
cargo run --example 07_yaml_json_loading
```

### 08. Semantic Analysis
**File**: [`examples/08_semantic_analysis.rs`](examples/08_semantic_analysis.rs)

Understand command validation and verification:
- Parsing to semantic analysis pipeline
- Command existence validation
- Argument binding and type checking
- Error detection and reporting

```bash
cargo run --example 08_semantic_analysis
```

### 09. Command Execution
**File**: [`examples/09_command_execution.rs`](examples/09_command_execution.rs)

Execute verified commands with proper context:
- Command routine implementation
- Execution context usage
- Error handling patterns
- Batch command processing

```bash
cargo run --example 09_command_execution
```

### 10. Full Pipeline
**File**: [`examples/10_full_pipeline.rs`](examples/10_full_pipeline.rs)

See the complete Unilang pipeline in action:
- End-to-end command processing
- Registry setup and management
- Interactive command processing
- Real-world usage patterns

```bash
cargo run --example 10_full_pipeline
```

## Quick Start Example

```rust
use unilang::registry::CommandRegistry;
use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, OutputData};

// Create registry
let mut registry = CommandRegistry::new();

// Define command
let greet_cmd = CommandDefinition::former()
    .name("greet")
    .description("Greets a person")
    .arguments(vec![
        ArgumentDefinition::former()
            .name("name")
            .kind(Kind::String)
            .hint("Person to greet")
            .end()
    ])
    .end();

// Define routine
let routine = Box::new(|cmd, _ctx| {
    let name = cmd.arguments.get("name").unwrap();
    println!("Hello, {}!", name);
    Ok(OutputData {
        content: format!("Hello, {}!", name),
        format: "text".to_string(),
    })
});

// Register command
registry.command_add_runtime(&greet_cmd, routine)?;
```

## CLI Usage

The included CLI demonstrates practical usage:

```bash
# Build the CLI
cargo build --bin unilang_cli

# Show available commands
./target/debug/unilang_cli help

# Execute commands
./target/debug/unilang_cli math.add 10 20
./target/debug/unilang_cli greet Alice
./target/debug/unilang_cli system.echo "Hello, World!"

# Get command help
./target/debug/unilang_cli help math.add
```

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Command         │    │ External Config  │    │ Help            │
│ Registration    │────│ (YAML/JSON)      │────│ Generation      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                       │
         ▼                        ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Command Registry                             │
└─────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Text Input      │────│ Parser           │────│ Instructions    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                       │
         ▼                        ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Semantic        │────│ Verified         │────│ Interpreter     │
│ Analysis        │    │ Commands         │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                       │
         ▼                        ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Error           │    │ Execution        │────│ Output Data     │
│ Reports         │    │ Context          │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Key Components

### Data Structures
- **CommandDefinition**: Complete command specification with metadata
- **ArgumentDefinition**: Argument specification with validation rules
- **Kind**: Type system for arguments with validation support
- **VerifiedCommand**: Validated command ready for execution

### Core Systems
- **CommandRegistry**: Central command storage and management
- **SemanticAnalyzer**: Command validation and verification
- **Interpreter**: Command execution engine
- **HelpGenerator**: Automatic documentation generation

### Error Handling
- **Structured Errors**: Machine-readable error codes with human descriptions
- **Validation Errors**: Constraint violation reporting
- **Execution Errors**: Runtime error handling

## Testing

Run the test suite to verify functionality:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test phase1
cargo test phase2
cargo test phase3

# Run integration tests
cargo test --test cli_integration_test
```

## Contributing

1. Study the examples to understand the architecture
2. Check existing tests for patterns
3. Add tests for new functionality
4. Follow the established code style
5. Update documentation as needed

## License

MIT License - see LICENSE file for details.