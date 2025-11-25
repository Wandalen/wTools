# Specification: wca

## Overview

**wca** (wTools Command Aggregator) is a comprehensive CLI framework for building complex command-line applications with type-safe argument parsing, subcommand routing, help generation, and fluent API construction. It aggregates both Rust functions and external binary applications into a unified command interface.

**Version:** 0.39.0
**Status:** Experimental
**Category:** CLI Framework
**Dependents:** 2 workspace crates (willbe, genfile)

### Scope

#### Responsibility

Provide a complete CLI framework with type-safe argument parsing, subcommand routing, automatic help generation, and fluent API for building command-line applications that can aggregate both Rust functions and external binaries.

#### In-Scope

1. **Command Aggregation**
   - `CommandsAggregator` - Main entry point for CLI applications
   - Fluent API for command definition using former pattern
   - Support for both Rust function routines and external binary execution
   - Command chaining and composition

2. **Argument Parsing**
   - Type-safe argument parsing with `Type` system
   - Subject (positional arguments) parsing
   - Property (named arguments) parsing with `--key value` or `--key=value` syntax
   - Type coercion and validation
   - `Parser` component for command-line string parsing

3. **Type System**
   - `Type` enum: String, Number, Path, List, etc.
   - `Value` enum for runtime type-checked values
   - `TryCast` trait for type conversions
   - Type validation at parse time

4. **Command Grammar**
   - `Command` structure defining command syntax
   - `Dictionary` for command lookup and management
   - Subject and property definitions with hints and types
   - Optional vs required arguments

5. **Verification System**
   - `Verifier` component validating parsed commands against grammar
   - `VerifiedCommand` containing validated arguments and properties
   - Type checking and constraint validation
   - Unknown argument detection

6. **Execution System**
   - `Executor` component running verified commands
   - `Routine` trait for command handlers
   - Support for closures, functions, and external binaries
   - `Context` for execution state
   - Error handling and result propagation

7. **Help Generation**
   - Automatic help text generation from command definitions
   - `HelpVariants` for different help formats
   - Hint system for user-friendly descriptions
   - Command usage examples

8. **Input Abstraction**
   - `Input` trait for various input sources
   - `IntoInput` conversion trait
   - Support for `Vec<String>`, iterators, etc.

9. **Formatting and Display**
   - `Formatter` for consistent CLI output
   - Table formatting for structured data
   - Error message formatting

10. **Feature Gating**
    - `enabled`: Master feature switch
    - `full`: All features including suggestions
    - `on_unknown_suggest`: Fuzzy command matching with textdistance

11. **mod_interface Architecture**
    - Uses `mod_interface!` macro for module organization
    - Exposed modules: grammar, parser, verifier, executor, input, tool, aggregator, help, formatter
    - Controlled visibility and clean public API

#### Out-of-Scope

1. **NOT Shell Completion**
   - Does not generate shell completion scripts
   - Does not integrate with bash/zsh/fish completion systems
   - **Rationale:** Completion is a separate concern, delegates to specialized crates

2. **NOT ANSI Color Output**
   - Does not provide colored terminal output
   - Does not handle terminal styling
   - **Rationale:** Styling delegates to other crates (colored, ansi_term, etc.)

3. **NOT Interactive CLI**
   - Does not provide REPL (Read-Eval-Print Loop)
   - Does not handle interactive prompts
   - Batch/command-driven only
   - **Rationale:** Interactive features are a different paradigm

4. **NOT Configuration File Parsing**
   - Does not parse config files (TOML, YAML, JSON)
   - Command-line arguments only
   - **Rationale:** Config parsing is a separate concern

5. **NOT Argument Validation Logic**
   - Does not provide custom validation rules
   - Type checking only, not business logic validation
   - **Rationale:** Application-specific validation should be in routines

6. **NOT Terminal UI Framework**
   - Does not provide TUI widgets or layout
   - Does not handle terminal drawing
   - **Rationale:** TUI is handled by dedicated crates (tui-rs, cursive, etc.)

7. **NOT Process Management**
   - Does not manage process lifecycle beyond execution
   - Does not provide process monitoring or supervision
   - **Rationale:** Process management is process_tools responsibility

8. **NOT Logging Framework**
   - Uses log crate for logging, does not implement logger
   - Does not provide log configuration
   - **Rationale:** Logging implementation is application responsibility

#### Boundaries

- **wca vs clap/structopt**: wca provides aggregation of multiple commands with fluent API, clap focuses on single-binary argument parsing
- **wca vs process_tools**: wca executes commands, process_tools manages subprocess lifecycle
- **wca vs former**: wca uses former for fluent command building API
- **wca vs error_tools**: wca uses error_tools for error handling

## Architecture

### Dependency Structure

```
wca
├── Internal Dependencies
│   ├── error_tools (workspace, error handling)
│   ├── mod_interface (workspace, module organization)
│   ├── iter_tools (workspace, iterator utilities)
│   └── former (workspace, fluent API builder)
├── External Dependencies
│   ├── log (logging facade)
│   ├── textdistance (optional, fuzzy matching)
│   └── indexmap (ordered maps)
└── Dev Dependencies
    ├── test_tools (workspace)
    ├── assert_fs (filesystem testing)
    └── criterion (benchmarking)
```

### Module Architecture

```
wca
├── lib.rs (mod_interface! organization)
└── ca/ (command aggregator)
    ├── grammar/ (command syntax definitions)
    │   ├── types.rs (Type, Value, TryCast)
    │   └── dictionary.rs (Command, Dictionary)
    ├── parser/ (CLI string parsing)
    │   ├── parser.rs (Parser implementation)
    │   └── command.rs (parsed command structure)
    ├── verifier/ (command validation)
    │   ├── verifier.rs (Verifier implementation)
    │   └── command.rs (VerifiedCommand)
    ├── executor/ (command execution)
    │   ├── executor.rs (Executor implementation)
    │   ├── routine.rs (Routine trait)
    │   └── context.rs (execution context)
    ├── aggregator.rs (CommandsAggregator)
    ├── input.rs (Input trait)
    ├── help.rs (help generation)
    ├── formatter.rs (output formatting)
    └── tool/ (utilities)
        └── table.rs (table formatting)
```

### Feature Architecture

```
enabled (master switch)
└── full (includes all features)
    └── on_unknown_suggest (fuzzy command matching)
```

**Default Features:** `enabled`

### Command Processing Pipeline

```
Input (CLI args)
↓
Parser (parse strings → ParsedCommand)
↓
Verifier (validate against grammar → VerifiedCommand)
↓
Executor (run routine → Result)
```

## Public API

### Core Types

```rust
/// Main CLI application builder
pub struct CommandsAggregator {
  // Internal dictionary of commands
}

/// Command definition with grammar
pub struct Command {
  phrase: String,
  subjects: Vec<Subject>,
  properties: Vec<Property>,
  routine: Box<dyn Routine>,
}

/// Type-safe value system
pub enum Type {
  String,
  Number,
  Path,
  List,
  // ...
}

pub enum Value {
  String(String),
  Number(i64),
  Path(PathBuf),
  List(Vec<Value>),
  // ...
}

/// Verified command ready for execution
pub struct VerifiedCommand {
  pub args: Vec<Value>,
  pub props: HashMap<String, Value>,
}
```

### Traits

```rust
/// Command routine handler
pub trait Routine {
  fn call(&self, args: VerifiedCommand) -> Result<(), String>;
}

/// Input source abstraction
pub trait Input {
  fn to_vec(&self) -> Vec<String>;
}

pub trait IntoInput {
  fn into_input(self) -> Box<dyn Input>;
}

/// Type conversion
pub trait TryCast {
  fn try_cast<T>(&self) -> Result<T, Error>;
}
```

### Main API

```rust
impl CommandsAggregator {
  /// Create new aggregator with former pattern
  pub fn former() -> CommandsAggregatorFormer;

  /// Execute command from input
  pub fn perform(&self, input: impl IntoInput) -> Result<(), Error>;
}

/// Fluent API for building commands
impl CommandsAggregatorFormer {
  pub fn command(self, name: &str) -> CommandFormer;
}

impl CommandFormer {
  pub fn hint(self, hint: &str) -> Self;
  pub fn subject(self) -> SubjectFormer;
  pub fn property(self, name: &str) -> PropertyFormer;
  pub fn routine<F>(self, routine: F) -> Self
  where F: Routine + 'static;
  pub fn end(self) -> CommandsAggregatorFormer;
}
```

## Usage Patterns

### Pattern 1: Basic Command Definition

```rust
use wca::{CommandsAggregator, VerifiedCommand, Type};

let ca = CommandsAggregator::former()
  .command("echo")
    .hint("prints all subjects and properties")
    .subject().hint("Subject").kind(Type::String).optional(true).end()
    .property("property").hint("simple property").kind(Type::String).optional(true).end()
    .routine(|o: VerifiedCommand| {
      println!("Args: {:?}\nProps: {:?}", o.args, o.props);
      Ok(())
    })
    .end()
  .perform();

let args: Vec<String> = std::env::args().skip(1).collect();
ca.perform(args).unwrap();
```

### Pattern 2: Multiple Commands

```rust
let ca = CommandsAggregator::former()
  .command("echo")
    .hint("prints arguments")
    .subject().hint("text").kind(Type::String).optional(false).end()
    .routine(|o: VerifiedCommand| {
      println!("{}", o.args.get::<String>(0)?);
      Ok(())
    })
    .end()
  .command("exit")
    .hint("exit the application")
    .routine(|| {
      std::process::exit(0);
    })
    .end()
  .perform();
```

### Pattern 3: Error Handling

```rust
let ca = CommandsAggregator::former()
  .command("error")
    .hint("demonstrates error handling")
    .subject().hint("error message").kind(Type::String).optional(false).end()
    .routine(|o: VerifiedCommand| {
      let msg = o.args.get_owned::<String>(0)?;
      Err(format!("Error: {}", msg))
    })
    .end()
  .perform();

match ca.perform(args) {
  Ok(_) => println!("Success"),
  Err(e) => eprintln!("Command failed: {}", e),
}
```

### Pattern 4: Type-Safe Properties

```rust
use wca::{CommandsAggregator, VerifiedCommand, Type};

let ca = CommandsAggregator::former()
  .command("config")
    .property("port").kind(Type::Number).optional(false).end()
    .property("host").kind(Type::String).optional(true).end()
    .routine(|o: VerifiedCommand| {
      let port: i64 = o.props.get("port")?.try_cast()?;
      let host: String = o.props.get("host")
        .map(|v| v.try_cast())
        .unwrap_or(Ok("localhost".to_string()))?;

      println!("Server: {}:{}", host, port);
      Ok(())
    })
    .end()
  .perform();

// Usage: app config --port=8080 --host=example.com
```

### Pattern 5: Fuzzy Command Matching

```rust
// With on_unknown_suggest feature
let ca = CommandsAggregator::former()
  .command("deploy")
    .hint("deploy application")
    .routine(|| {
      println!("Deploying...");
      Ok(())
    })
    .end()
  .perform();

// User types: app deploi
// Output: Unknown command 'deploi'. Did you mean 'deploy'?
```

### Pattern 6: Help Generation

```rust
let ca = CommandsAggregator::former()
  .command("serve")
    .hint("start the server")
    .subject().hint("port number").kind(Type::Number).optional(true).end()
    .property("host").hint("hostname").kind(Type::String).optional(true).end()
    .routine(|_| Ok(()))
    .end()
  .perform();

// User types: app help serve
// Output:
// serve - start the server
//   Arguments:
//     <port> (Number, optional) - port number
//   Properties:
//     --host (String, optional) - hostname
```

## Dependencies and Consumers

### Direct Dependencies

**Internal:**
- `error_tools` (workspace) - Typed and untyped error handling
- `mod_interface` (workspace) - Module organization pattern
- `iter_tools` (workspace) - Iterator utilities
- `former` (workspace) - Fluent API builder pattern

**External:**
- `log` - Logging facade
- `textdistance` (optional) - Fuzzy string matching for command suggestions
- `indexmap` - Ordered hash maps for command storage

**Dev:**
- `test_tools` (workspace) - Testing utilities
- `assert_fs` - Filesystem assertion utilities
- `criterion` - Benchmarking framework

### Consumers (2 workspace crates)

1. **willbe** - Workspace automation tool using wca for CLI
2. **genfile** - File generation tool using wca for command interface

## Design Rationale

### Why Command Aggregation?

Traditional CLI frameworks focus on single-binary argument parsing. wca enables:

1. **Multi-Command Applications**: Single binary with many subcommands
2. **External Binary Integration**: Aggregate Rust functions AND external programs
3. **Modular CLI Design**: Each module can register its own commands

**Use Case:** Workspace automation tools (willbe), code generators (genfile)

### Why Three-Stage Pipeline (Parse → Verify → Execute)?

**Separation of Concerns:**
1. **Parser**: Syntax (string → structure)
2. **Verifier**: Semantics (structure → validated)
3. **Executor**: Action (validated → result)

**Benefits:**
- Clear error messages at each stage
- Testable independently
- Extensible (custom verifiers possible)

### Why Fluent API with former?

```rust
// Traditional approach (verbose)
let mut cmd = Command::new("echo");
cmd.add_subject(Subject::new().hint("text").kind(Type::String));
cmd.set_routine(Box::new(|_| Ok(())));

// wca approach (fluent)
.command("echo")
  .subject().hint("text").kind(Type::String).end()
  .routine(|_| Ok(()))
  .end()
```

**Benefits:**
1. **Readability**: Hierarchical structure matches mental model
2. **Type Safety**: Compiler enforces correct builder usage
3. **Discoverability**: IDE autocomplete guides API usage

### Why Type System Instead of Stringly-Typed?

```rust
// Without type system
fn routine(args: Vec<String>) {
  let port: i64 = args[0].parse().unwrap(); // Runtime error!
}

// With type system
fn routine(args: VerifiedCommand) {
  let port: i64 = args.get(0)?; // Compile-time type checking
}
```

**Benefits:**
1. **Early Errors**: Type errors at parse time, not execution time
2. **Self-Documenting**: Types show intent
3. **Validation**: Type system validates input automatically

### Why Optional Fuzzy Matching?

Fuzzy matching (on_unknown_suggest) is optional because:

1. **Dependency Cost**: textdistance adds compile time and binary size
2. **Use Case Specific**: Not all CLIs want suggestions
3. **Feature Gate**: Users opt-in via `features = ["full"]`

### Why mod_interface Architecture?

wca uses `mod_interface!` for:

1. **Controlled Visibility**: Expose only public API, hide implementation
2. **Layered Organization**: Grammar, parser, verifier, executor as separate layers
3. **Ecosystem Consistency**: Same pattern as other wTools crates

## Testing Strategy

### Test Coverage

- **Unit Tests**: Each module (grammar, parser, verifier, executor) tested independently
- **Integration Tests**: Full pipeline tests with test_tools
- **Filesystem Tests**: assert_fs for command I/O testing
- **Benchmarks**: criterion benchmarks for parsing performance

### Test Focus

1. **Parser Correctness**: Various CLI syntax patterns
2. **Verifier Validation**: Type checking, required vs optional
3. **Executor Behavior**: Routine invocation, error handling
4. **Help Generation**: Help text accuracy
5. **Fuzzy Matching**: Suggestion quality (when feature enabled)

## Future Considerations

### Potential Enhancements

1. **Shell Completion**: Generate completion scripts for bash/zsh/fish
2. **ANSI Colors**: Built-in colored output support
3. **Interactive Mode**: REPL for command exploration
4. **Config File Support**: Load commands from TOML/YAML
5. **Subcommand Nesting**: Deeper command hierarchies

### Breaking Changes to Consider

1. **API Simplification**: Reduce fluent API verbosity
2. **Type System**: More type variants (Float, Boolean, etc.)
3. **Error Types**: Custom error types instead of String
4. **Async Routines**: Support for async command handlers

### Known Limitations

1. **No Shell Integration**: Requires external completion scripts
2. **No Interactive Prompts**: Batch mode only
3. **Limited Type System**: Fixed set of types, not extensible
4. **No Subcommand Nesting**: Flat command structure

## Adoption Guidelines

### When to Use wca

**Good Candidates:**
- Multi-command CLI applications
- Workspace automation tools
- Code generators with CLI
- Applications aggregating external binaries

**Poor Candidates:**
- Simple single-command apps (use clap)
- Interactive TUI applications (use cursive/tui-rs)
- Performance-critical parsing (overhead from verification)

### Migration from clap

```rust
// clap approach
#[derive(Parser)]
struct Args {
  #[arg(short, long)]
  port: u16,
}

// wca approach
CommandsAggregator::former()
  .command("app")
    .property("port").kind(Type::Number).optional(false).end()
    .routine(|o: VerifiedCommand| {
      let port: i64 = o.props.get("port")?.try_cast()?;
      // ...
    })
    .end()
```

wca is better for multi-command apps; clap is simpler for single-command.

### Best Practices

1. **Descriptive Hints**: Always provide hints for help generation
2. **Type Validation**: Use appropriate Type for each argument
3. **Error Handling**: Return descriptive error messages from routines
4. **Command Organization**: Group related commands by prefix (e.g., `db.migrate`, `db.seed`)

## Related Crates

- **error_tools**: Error handling (dependency)
- **mod_interface**: Module organization (dependency)
- **iter_tools**: Iterator utilities (dependency)
- **former**: Fluent API builder (dependency)
- **willbe**: Primary consumer (workspace automation)
- **genfile**: Consumer (code generation CLI)
- **process_tools**: Subprocess management (complementary)

## References

- [API Documentation](https://docs.rs/wca)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/wca)
- [readme.md](./readme.md)
- [doc/wca.md](./doc/wca.md) - Additional documentation
