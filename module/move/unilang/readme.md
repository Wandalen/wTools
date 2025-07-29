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
- **🚀 High-Level Pipeline API**: Simplified workflow helpers for common usage patterns
- **📚 Automatic Help**: Generate comprehensive help documentation from command definitions
- **🛡️ Error Management**: Structured error handling with meaningful error codes and messages
- **📦 Batch Processing**: Execute multiple commands with comprehensive result tracking
- **🔐 Command Validation**: Validate commands without execution for safety checks

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

### 11. High-Level Pipeline API
**File**: [`examples/11_pipeline_api.rs`](examples/11_pipeline_api.rs)

Master the high-level Pipeline API for simplified workflows:
- Single command processing with error handling
- Batch processing with success tracking
- Sequence processing with fail-fast behavior
- Command validation without execution
- Performance optimization through component reuse

```bash
cargo run --example 11_pipeline_api
```

## Quick Start Examples

### Basic Command Registration
```rust
use unilang::registry::CommandRegistry;
use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, OutputData};
use unilang::types::Value;

// Create registry
let mut registry = CommandRegistry::new();

// Define command
let greet_cmd = CommandDefinition::former()
    .name("greet")
    .namespace("".to_string())
    .description("Greets a person".to_string())
    .hint("Simple greeting")
    .status("stable")
    .version("1.0.0")
    .aliases(vec![])
    .tags(vec![])
    .permissions(vec![])
    .idempotent(true)
    .deprecation_message("".to_string())
    .http_method_hint("GET".to_string())
    .examples(vec!["greet name::\"Alice\"".to_string()])
    .arguments(vec![
        ArgumentDefinition::former()
            .name("name")
            .kind(Kind::String)
            .hint("Person to greet")
            .description("Name of person to greet".to_string())
            .attributes(Default::default())
            .validation_rules(vec![])
            .aliases(vec![])
            .tags(vec![])
            .end()
    ])
    .end();

// Define routine
let routine = Box::new(|cmd, _ctx| {
    let name = cmd.arguments.get("name")
        .and_then(|v| if let Value::String(s) = v { Some(s) } else { None })
        .unwrap_or(&"World".to_string());
    println!("Hello, {}!", name);
    Ok(OutputData {
        content: format!("Hello, {}!", name),
        format: "text".to_string(),
    })
});

// Register command
registry.command_add_runtime(&greet_cmd, routine)?;
```

### High-Level Pipeline API
```rust
use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;

// Create pipeline with registry
let pipeline = Pipeline::new(registry);

// Process single command
let result = pipeline.process_command_simple("greet name::\"Alice\"");
if result.success {
    println!("Output: {}", result.outputs[0].content);
}

// Process batch of commands
let commands = vec!["greet name::\"Alice\"", "greet name::\"Bob\"", "greet name::\"Charlie\""];
let batch_result = pipeline.process_batch(&commands, ExecutionContext::default());
println!("Success rate: {:.1}%", batch_result.success_rate());

// Validate command without execution
match pipeline.validate_command("greet name::\"Alice\"") {
    Ok(()) => println!("Command is valid"),
    Err(e) => println!("Invalid command: {}", e),
}
```

### External Configuration Loading
```rust
use unilang::registry::CommandRegistry;

let yaml_config = r#"
- name: "hello"
  namespace: ""
  description: "Hello world command"
  arguments:
    - name: "target"
      kind: "String"
      hint: "Who to greet"
"#;

let registry = CommandRegistry::builder()
    .load_from_yaml_str(yaml_config)?
    .build();
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
./target/debug/unilang_cli greet name::\"Alice\"
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
- **OutputData**: Structured command execution results
- **ErrorData**: Structured error information with codes and messages

### Core Systems
- **CommandRegistry**: Central command storage and management
- **SemanticAnalyzer**: Command validation and verification
- **Interpreter**: Command execution engine
- **HelpGenerator**: Automatic documentation generation
- **Pipeline**: High-level workflow orchestration

### Pipeline API
- **Pipeline**: Main high-level interface for command processing
- **CommandResult**: Single command execution result with success/error info
- **BatchResult**: Batch processing results with statistics
- **Convenience Functions**: One-off processing helpers

### Processing Modes
- **Single Command**: Process one command with full error handling
- **Batch Processing**: Process multiple commands independently
- **Sequence Processing**: Process commands with fail-fast behavior
- **Validation Only**: Validate commands without execution

### Error Handling
- **Structured Errors**: Machine-readable error codes with human descriptions
- **Validation Errors**: Constraint violation reporting
- **Execution Errors**: Runtime error handling
- **Pipeline Errors**: High-level workflow error management

## API Reference

### Pipeline API

The Pipeline API provides high-level interfaces for common Unilang workflows:

#### Pipeline Structure
```rust
pub struct Pipeline {
    // Internal parser and registry
}

impl Pipeline {
    pub fn new(registry: CommandRegistry) -> Self;
    pub fn with_parser_options(registry: CommandRegistry, options: UnilangParserOptions) -> Self;
    
    // Single command processing
    pub fn process_command(&self, command_str: &str, context: ExecutionContext) -> CommandResult;
    pub fn process_command_simple(&self, command_str: &str) -> CommandResult;
    
    // Batch processing
    pub fn process_batch(&self, commands: &[&str], context: ExecutionContext) -> BatchResult;
    pub fn process_sequence(&self, commands: &[&str], context: ExecutionContext) -> BatchResult;
    
    // Validation
    pub fn validate_command(&self, command_str: &str) -> Result<(), Error>;
    pub fn validate_batch(&self, commands: &[&str]) -> Vec<Result<(), Error>>;
    
    // Registry access
    pub fn registry(&self) -> &CommandRegistry;
    pub fn registry_mut(&mut self) -> &mut CommandRegistry;
}
```

#### Result Types
```rust
pub struct CommandResult {
    pub command: String,           // Original command string
    pub outputs: Vec<OutputData>,  // Command outputs
    pub success: bool,             // Whether command succeeded
    pub error: Option<String>,     // Error message if failed
}

pub struct BatchResult {
    pub results: Vec<CommandResult>,  // Individual command results
    pub total_commands: usize,        // Total commands processed
    pub successful_commands: usize,   // Number that succeeded
    pub failed_commands: usize,       // Number that failed
}

impl BatchResult {
    pub fn all_succeeded(&self) -> bool;
    pub fn any_failed(&self) -> bool;
    pub fn success_rate(&self) -> f64;
}
```

#### Convenience Functions
```rust
// Process single command without creating Pipeline
pub fn process_single_command(
    command_str: &str,
    registry: &CommandRegistry,
    context: ExecutionContext,
) -> CommandResult;

// Validate single command without creating Pipeline
pub fn validate_single_command(
    command_str: &str,
    registry: &CommandRegistry,
) -> Result<(), Error>;
```

### Usage Patterns

#### Basic Command Processing
```rust
use unilang::pipeline::Pipeline;

let pipeline = Pipeline::new(registry);
let result = pipeline.process_command_simple("command arg1 arg2");

if result.success {
    println!("Success: {}", result.outputs[0].content);
} else {
    eprintln!("Error: {}", result.error.unwrap());
}
```

#### Batch Processing with Error Handling
```rust
let commands = vec!["cmd1", "cmd2", "cmd3"];
let batch_result = pipeline.process_batch(&commands, ExecutionContext::default());

println!("Processed {}/{} commands successfully", 
    batch_result.successful_commands, batch_result.total_commands);

for result in &batch_result.results {
    if let Some(error) = &result.error {
        eprintln!("Command '{}' failed: {}", result.command, error);
    }
}
```

#### Command Validation
```rust
// Validate before execution
match pipeline.validate_command("risky_command --dangerous-flag") {
    Ok(()) => {
        let result = pipeline.process_command_simple("risky_command --dangerous-flag");
        // Process result
    }
    Err(e) => {
        eprintln!("Invalid command: {}", e);
    }
}
```

#### Performance-Optimized Processing
```rust
// Reuse pipeline for multiple commands (faster)
let pipeline = Pipeline::new(registry);
for cmd in commands {
    let result = pipeline.process_command_simple(cmd);
    // Handle result
}

// vs. one-off processing (slower due to repeated setup)
for cmd in commands {
    let result = process_single_command(cmd, &registry, ExecutionContext::default());
    // Handle result
}
```

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

# Run pipeline API tests
cargo test pipeline

# Run examples as tests
cargo test --examples
```

## Contributing

1. Study the examples to understand the architecture
2. Check existing tests for patterns
3. Add tests for new functionality
4. Follow the established code style
5. Update documentation as needed

## License

MIT License - see LICENSE file for details.