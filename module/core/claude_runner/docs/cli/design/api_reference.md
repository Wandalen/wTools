# CLI API Reference

Complete reference for all public APIs used in `claude_runner` CLI implementation.

## Table of Contents

- [Unilang Types](#unilang-types)
- [Adapter Functions](#adapter-functions)
- [Registry Functions](#registry-functions)
- [Handler Signatures](#handler-signatures)
- [Core Library APIs](#core-library-apis)
- [Error Handling](#error-handling)
- [Type Conversions](#type-conversions)

---

## Unilang Types

### CommandDefinition

**Module:** `unilang::data`

**Purpose:** Define command metadata for registration.

**Full Type Signature:**
```rust
pub struct CommandDefinition {
    pub name: CommandName,
    pub description: String,
    pub namespace: String,
    pub status: CommandStatus,
    pub version: VersionType,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub permissions: Vec<String>,
    pub idempotent: bool,
    pub deprecation_message: String,
    pub http_method_hint: String,
    pub auto_help: bool,
    pub examples: Vec<String>,
    pub arguments: Vec<ArgumentDefinition>,
}
```

**Constructor:**
```rust
pub fn new(name: CommandName, description: String) -> Self
```

**Builder Methods:**
```rust
pub fn with_namespace(self, namespace: String) -> Self
pub fn with_status(self, status: CommandStatus) -> Self
pub fn with_version(self, version: VersionType) -> Self
pub fn with_tags(self, tags: Vec<String>) -> Self
pub fn with_aliases(self, aliases: Vec<String>) -> Self
pub fn with_permissions(self, permissions: Vec<String>) -> Self
pub fn with_idempotent(self, idempotent: bool) -> Self
pub fn with_deprecation_message(self, message: String) -> Self
pub fn with_http_method_hint(self, hint: String) -> Self
pub fn with_auto_help(self, auto_help: bool) -> Self
pub fn with_examples(self, examples: Vec<String>) -> Self
pub fn with_arguments(self, arguments: Vec<ArgumentDefinition>) -> Self
```

**Usage Example:**
```rust
let cmd = CommandDefinition::new(
    CommandName::new(".run").expect("valid command name"),
    "Execute Claude Code with configurable parameters".to_string(),
)
.with_arguments(vec![
    ArgumentDefinition::new("message", Kind::String)
        .with_description("Prompt message for Claude"),
    ArgumentDefinition::new("dry", Kind::Boolean)
        .with_description("Print without executing"),
]);
```

---

### ArgumentDefinition

**Module:** `unilang::data`

**Purpose:** Define parameter metadata.

**Full Type Signature:**
```rust
pub struct ArgumentDefinition {
    pub name: String,
    pub description: String,
    pub kind: Kind,
    pub hint: String,
    pub attributes: ArgumentAttributes,
    pub validation_rules: Vec<ValidationRule>,
    pub aliases: Vec<String>,
    pub tags: Vec<String>,
}
```

**Constructor:**
```rust
pub fn new(name: String, kind: Kind) -> Self
```

**Builder Methods:**
```rust
pub fn with_description(self, description: String) -> Self
pub fn with_hint(self, hint: String) -> Self
pub fn with_attributes(self, attributes: ArgumentAttributes) -> Self
pub fn with_validation_rules(self, rules: Vec<ValidationRule>) -> Self
pub fn with_aliases(self, aliases: Vec<String>) -> Self
pub fn with_tags(self, tags: Vec<String>) -> Self
```

**ArgumentAttributes:**
```rust
pub struct ArgumentAttributes {
    pub optional: bool,
    pub default: Option<String>,
    pub sensitive: bool,
    pub interactive: bool,
    pub multiple: bool,
}
```

**Usage Example:**
```rust
ArgumentDefinition::new("message", Kind::String)
    .with_description("Prompt message for Claude")
    .with_attributes(ArgumentAttributes {
        optional: false,
        default: None,
        sensitive: false,
        interactive: false,
        multiple: false,
    })
```

---

### Kind

**Module:** `unilang::data`

**Purpose:** Parameter type definition.

**Variants:**
```rust
pub enum Kind {
    String,
    Path,
    Directory,
    Integer,
    Float,
    Boolean,
    Map,
    List,
    Struct(Vec<Kind>),
    Enum(Vec<String>),
}
```

**Usage Example:**
```rust
ArgumentDefinition::new("path", Kind::Path)      // File path
ArgumentDefinition::new("count", Kind::Integer)   // Integer
ArgumentDefinition::new("verbose", Kind::Boolean) // Boolean flag
```

---

### CommandName

**Module:** `unilang::data`

**Purpose:** Validated command name with dot prefix enforcement.

**Constructor:**
```rust
pub fn new(name: String) -> Result<Self, String>
```

**Usage Example:**
```rust
CommandName::new(".run").expect("valid command name")?
CommandName::new(".help").expect("valid command name")?
CommandName::new("invalid")?  // Returns Err
```

**Validation:** Enforces command names start with `.`.

---

### CommandStatus

**Module:** `unilang::data`

**Purpose:** Command lifecycle status.

**Variants:**
```rust
pub enum CommandStatus {
    Active,
    Deprecated,
    Experimental,
}
```

**Usage Example:**
```rust
.with_status(CommandStatus::Active)      // Live command
.with_status(CommandStatus::Deprecated) // Will be removed
.with_status(CommandStatus::Experimental) // Unstable
```

---

### VersionType

**Module:** `unilang::data`

**Purpose:** Semantic version for command API compatibility.

**Constructor:**
```rust
pub fn new(version: String) -> Result<Self, String>
```

**Usage Example:**
```rust
VersionType::new("0.1.0").expect("valid version")?
VersionType::new("invalid")?  // Returns Err
```

---

### CommandRegistry

**Module:** `unilang::registry`

**Purpose:** Runtime command storage and lookup.

**Constructor:**
```rust
pub fn new() -> Self
```

**Methods:**

```rust
// Register command with handler
pub fn command_add_runtime(
    &mut self,
    command: &CommandDefinition,
    routine: CommandRoutine,
) -> Result<(), String>

// Find command by name
pub fn find_command(&self, name: &str) -> Option<&CommandDefinition>
```

**Usage Example:**
```rust
let mut registry = CommandRegistry::new();

let cmd = CommandDefinition::new(/* ... */);
let handler: CommandRoutine = Box::new(|cmd, ctx| { /* ... */ });

registry.command_add_runtime(&cmd, handler)
    .expect("failed to register command");

if let Some(found) = registry.find_command(".run") {
    // Use found command
}
```

---

### CommandRoutine

**Module:** `unilang::registry`

**Purpose:** Handler function signature type alias.

**Type Alias:**
```rust
pub type CommandRoutine = Box<dyn Fn(&VerifiedCommand, &mut ExecutionContext) -> Result<OutputData, ErrorData>>;
```

**Usage Example:**
```rust
let handler: CommandRoutine = Box::new(|cmd, _ctx| {
    if let Some(Value::String(msg)) = cmd.arguments.get("message") {
        let builder = ClaudeCommand::new().with_message(msg);
        let output = builder.execute()?;
        Ok(OutputData {
            content: output.stdout,
            format: "text".to_string(),
            execution_time_ms: None,
        })
    } else {
        Err(ErrorData::new(
            ErrorCode::ValidationError,
            "message parameter is required".to_string(),
        ))
    }
});
```

---

### VerifiedCommand

**Module:** `unilang::semantic`

**Purpose:** Parsed and validated command with extracted arguments.

**Key Fields:**

```rust
pub struct VerifiedCommand {
    pub command: String,        // Command name (".run", ".help")
    pub arguments: ArgumentsMap, // Key-value map of arguments
}

// ArgumentsMap access
impl VerifiedCommand {
    pub fn get(&self, key: &str) -> Option<&Value>
    pub fn get_owned(&self, key: &str) -> Option<Value>
}
```

**Usage Example:**
```rust
if let Some(Value::String(s)) = cmd.arguments.get("message") {
    let message = s.as_str();
    // Use message
}
```

---

### Value

**Module:** `unilang::types`

**Purpose:** Runtime value representation.

**Variants:**

```rust
pub enum Value {
    String(String),
    Path(String),
    Directory(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Map(HashMap<String, Value>),
    List(Vec<Value>),
    Struct(Vec<(String, Value)>),
}
```

**Pattern Matching:**
```rust
match cmd.arguments.get("param") {
    Some(Value::String(s)) => { /* handle string */ }
    Some(Value::Integer(n)) => { /* handle integer */ }
    Some(Value::Boolean(true)) => { /* handle boolean */ }
    Some(Value::Boolean(false)) => { /* handle boolean false */ }
    Some(Value::List(items)) => { /* handle list */ }
    Some(Value::Map(map)) => { /* handle map */ }
    None => { /* parameter not provided */ }
}
```

---

### ErrorData

**Module:** `unilang::data`

**Purpose:** Structured error representation.

**Full Type Signature:**
```rust
pub struct ErrorData {
    pub code: ErrorCode,
    pub message: String,
}
```

**Constructor:**
```rust
pub fn new(code: ErrorCode, message: String) -> Self
```

---

### ErrorCode

**Module:** `unilang::data`

**Purpose:** Error categorization.

**Variants:**

```rust
pub enum ErrorCode {
    InternalError,
    ValidationError,
    CommandNotFoundError,
    ArgumentRequired,
    ParseError,
    ExecutionError,
}
```

**Usage Example:**
```rust
Err(ErrorData::new(
    ErrorCode::ValidationError,
    "Required parameter missing".to_string(),
))

Err(ErrorData::new(
    ErrorCode::CommandNotFoundError,
    format!("Unknown command: {name}"),
))
```

---

### OutputData

**Module:** `unilang::data`

**Purpose:** Command output encapsulation.

**Full Type Signature:**
```rust
pub struct OutputData {
    pub content: String,
    pub format: String,
    pub execution_time_ms: Option<u64>,
}
```

**Usage Example:**
```rust
Ok(OutputData {
    content: result_string.to_string(),
    format: "text".to_string(),
    execution_time_ms: None,
})
```

---

### ExecutionContext

**Module:** `unilang::interpreter`

**Purpose:** Execution context passed to handlers.

**Constructor:**
```rust
pub fn default() -> Self
```

**Current Limitation:** Minimal fields; application state must use module-level storage.

---

## Adapter Functions

### argv_to_unilang_tokens()

**Location:** `src/main.rs`

**Signature:**
```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>>
```

**Purpose:** Convert user `--flag value` input to unilang `key::value` tokens.

**Returns:**
- `Ok(Vec<String>)` — Tokens ready for parser
- `Err(error_tools::Error)` — Parse/validation error

**Output Format:**
```rust
// Help command
[".help"]

// Run command
[
    ".run",
    "message::Fix bug",      // String parameter
    "dir::/workspace",          // Path parameter
    "dry::1",                      // Boolean parameter
    "max_tokens::50000",           // Integer parameter
]
```

---

### print_help()

**Location:** `src/main.rs`

**Signature:**
```rust
fn print_help()
```

**Purpose:** Print user-friendly help text to stdout.

**No return value:** Returns `()` and exits program.

---

### build_registry()

**Location:** `src/main.rs`

**Signature:**
```rust
fn build_registry() -> CommandRegistry
```

**Purpose:** Create and populate command registry with all commands and handlers.

**Returns:** Populated `CommandRegistry` ready for parsing and execution.

---

## Registry Functions

### CommandRegistry::new()

**Signature:**
```rust
pub fn new() -> Self
```

**Purpose:** Create empty registry.

---

### CommandRegistry::command_add_runtime()

**Signature:**
```rust
pub fn command_add_runtime(
    &mut self,
    command: &CommandDefinition,
    routine: CommandRoutine,
) -> Result<(), String>
```

**Purpose:** Register command with its handler.

**Returns:**
- `Ok(())` — Registration successful
- `Err(String)` — Duplicate command or invalid name

**Panics:** Only on programmer error (malformed CommandDefinition).

---

### CommandRegistry::find_command()

**Signature:**
```rust
pub fn find_command(&self, name: &str) -> Option<&CommandDefinition>
```

**Purpose:** Lookup command by name.

**Returns:**
- `Some(&CommandDefinition)` — Command found
- `None` — Command not found

---

## Handler Signatures

### Standard Handler

```rust
type CommandRoutine = Box<dyn Fn(&VerifiedCommand, &mut ExecutionContext) -> Result<OutputData, ErrorData>>;
```

**Parameters:**
- `cmd: &VerifiedCommand` — Parsed command with arguments
- `ctx: &mut ExecutionContext` — Execution context (read/write)

**Returns:**
- `Ok(OutputData)` — Success with output
- `Err(ErrorData)` — Error with code and message

### Handler Template

```rust
fn my_command_handler(cmd: &VerifiedCommand, _ctx: &mut ExecutionContext) -> Result<OutputData, ErrorData> {
    // Extract parameters
    let param = match cmd.arguments.get("param") {
        Some(Value::String(s)) => s.as_str().to_string(),
        Some(Value::Integer(n)) => n.to_string(),
        Some(Value::Boolean(b)) => b.to_string(),
        None => return Err(ErrorData::new(
            ErrorCode::ValidationError,
            "param is required".to_string(),
        )),
        _ => return Err(ErrorData::new(
            ErrorCode::ValidationError,
            "param has unexpected type".to_string(),
        )),
    };

    // Execute logic
    let result = execute_my_logic(&param)?;

    // Return output
    Ok(OutputData {
        content: result,
        format: "text".to_string(),
        execution_time_ms: None,
    })
}
```

---

## Core Library APIs

### ClaudeCommand::new()

**Location:** `claude_runner_core`

**Signature:**
```rust
pub fn new() -> Self
```

**Purpose:** Create builder with default automation-friendly settings.

**Defaults:**
- `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000`
- `CLAUDE_CODE_BASH_TIMEOUT=3600000`
- `CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000`
- `CLAUDE_CODE_AUTO_CONTINUE=true`
- `CLAUDE_CODE_TELEMETRY=false`

### Builder Methods

```rust
// Message
pub fn with_message(self, message: String) -> Self

// Working directory
pub fn with_working_directory(self, dir: String) -> Self

// Conversation
pub fn with_continue_conversation(self, continue: bool) -> Self

// Token limit
pub fn with_max_output_tokens(self, tokens: u32) -> Self

// Permissions
pub fn with_skip_permissions(self, skip: bool) -> Self

// Session
pub fn with_session_dir(self, dir: String) -> Self

// Model
pub fn with_model(self, model: String) -> Self
```

### ClaudeCommand::describe()

**Signature:**
```rust
pub fn describe(&self) -> String
```

**Purpose:** Get full command string with arguments.

**Returns:** Complete CLI command as string.

**Example:**
```rust
let builder = ClaudeCommand::new()
    .with_message("Fix bug")
    .with_working_directory("/workspace")
    .with_max_output_tokens(50000);

let command = builder.describe();
// "CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000 CLAUDE_CODE_BASH_TIMEOUT=3600000 ... cd /workspace && claude \"Fix bug\""
```

### ClaudeCommand::describe_env()

**Signature:**
```rust
pub fn describe_env(&self) -> String
```

**Purpose:** Get environment variable block.

**Returns:** Multi-line string with one env var per line.

**Example:**
```rust
let builder = ClaudeCommand::new();
let env = builder.describe_env();
/*
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
CLAUDE_CODE_BASH_TIMEOUT=3600000
CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000
CLAUDE_CODE_AUTO_CONTINUE=true
CLAUDE_CODE_TELEMETRY=false
*/
```

### ClaudeCommand::execute()

**Signature:**
```rust
pub fn execute(&self) -> Result<ProcessOutput, Box<dyn std::error::Error>>
```

**Purpose:** Execute Claude Code subprocess.

**Returns:**
- `Ok(ProcessOutput)` — Execution succeeded with stdout/stderr
- `Err(Box<dyn Error>)` — Execution failed

**ProcessOutput:**
```rust
pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
```

---

## Error Handling

### Error Conversion: Adapter → ErrorData

```rust
Err(Error::msg("invalid --max-tokens value: -1"))
```

**Maps directly to unilang error format.**

### Error Conversion: Handler → ErrorData

```rust
Err(ErrorData::new(
    ErrorCode::InternalError,
    format!("Failed to execute Claude: {e}"),
))
```

**Wraps core library errors.**

### Error Formatting

```rust
eprintln!("Error: {message}");
eprintln!("Run with --help for usage.");
std::process::exit(1);
```

**Standardized error format for user output.**

---

## Type Conversions

### String → Value

```rust
let value = Value::String("my text".to_string());
```

### Integer → Value

```rust
let value = Value::Integer(42i64);
```

### Boolean → Value

```rust
let value = Value::Boolean(true);
```

### Value → String (for extraction)

```rust
if let Some(Value::String(s)) = cmd.arguments.get("message") {
    let message = s.as_str();
}
```

### Value → i64 (for integer extraction)

```rust
if let Some(Value::Integer(n)) = cmd.arguments.get("count") {
    let count = *n;
}
```

### Value → bool (for boolean extraction)

```rust
if matches!(cmd.arguments.get("dry"), Some(Value::Boolean(b))) {
    let is_dry = b;
}
```

---

## Quick Reference Card

| Type | Constructor | Key Methods |
|------|-------------|--------------|
| `CommandDefinition` | `new(name, desc)` | `.with_args()`, `.with_status()` |
| `ArgumentDefinition` | `new(name, kind)` | `.with_desc()`, `.with_optional()` |
| `Kind` | N/A | `String`, `Path`, `Integer`, `Boolean`, etc. |
| `CommandName` | `new(name)?` | N/A |
| `CommandStatus` | N/A | `Active`, `Deprecated`, `Experimental` |
| `VersionType` | `new(version)?` | N/A |
| `CommandRegistry` | `new()` | `.command_add_runtime()`, `.find_command()` |
| `VerifiedCommand` | N/A | `.get()`, `.get_owned()` |
| `Value` | N/A | `String(s)`, `Integer(n)`, `Boolean(b)` |
| `ErrorData` | `new(code, msg)` | N/A |
| `ErrorCode` | N/A | `InternalError`, `ValidationError`, etc. |
| `OutputData` | N/A | `content`, `format`, `execution_time_ms` |
| `ClaudeCommand` | `new()` | `.with_*()` builder methods |
| `ProcessOutput` | N/A | `stdout`, `stderr`, `exit_code` |

---

## References

- [Architecture](architecture.md) — System diagrams and data flow
- [Unilang Exploration](unilang_exploration.md) — Framework deep dive
- [Implementation Guide](implementation_guide.md) — Step-by-step implementation
- [Quick Reference](quick_reference.md) — Fast lookup card
