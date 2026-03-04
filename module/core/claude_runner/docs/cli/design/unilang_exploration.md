# Unilang Framework Exploration

Comprehensive analysis of unilang crate (`~0.48.0`) architecture, patterns, and integration strategies for CLI utilities.

## Overview

**unilang** is a command-line argument parsing and processing framework published on crates.io. It provides a complete pipeline for building type-safe, extensible CLIs with semantic validation.

### Core Components

| Component | Purpose | Key Types |
|-----------|-----------|-------------|
| `CommandDefinition` | Command metadata (name, description, arguments) | `CommandName`, `CommandStatus`, `VersionType` |
| `ArgumentDefinition` | Parameter metadata (kind, optional, default) | `Kind` (String, Path, Integer, Boolean, Enum) |
| `CommandRegistry` | Runtime command registration and lookup | `command_add_runtime()` |
| `Parser` | Token-to-instruction conversion | `parse_from_argv()`, `parse()` |
| `SemanticAnalyzer` | Instruction validation against registry | `analyze()` → `Vec<VerifiedCommand>` |
| `Interpreter` | Verified command execution | `run(&mut ExecutionContext)` |
| `Pipeline` | End-to-end processing (orchestrator) | `process_command()`, `process_command_from_argv()` |
| `Value` enum | Runtime value representation | `String`, `Integer`, `Boolean`, `Map`, `List` |
| `ErrorData`/`ErrorCode` | Structured error handling | `InternalError`, `ValidationError` |
| `OutputData` | Command output encapsulation | `content`, `format`, `execution_time_ms` |

---

## Command Naming Convention

### Dot-Prefixed Pattern

**All unilang commands use dot prefix:**

```bash
.archive.new          # Archive namespace, new action
.file.add            # File namespace, add action
.materialize          # Top-level command
.info                # Top-level command
```

### Why Dot Prefix?

1. **Visual Distinction** — Commands (`.name`) vs parameters (`key::value`)
2. **Namespace Hierarchy** — `.namespace.action` pattern enables grouping
3. **Parser Recognition** — Tokens starting with `.` are quickly identified as commands
4. **System vs User** — Distinguishes built-in framework commands from user-defined commands

### Real-World Examples (from genfile)

```bash
# Archive lifecycle commands
.archive.new name::"my-template"
.archive.load path::"template.yaml"
.archive.save path::"output.json"
.archive.from_directory source::"./src"

# File operation commands
.file.add path::"main.rs" content::"code"
.file.remove path::"old.rs"
.file.list verbosity::2
.file.show path::"readme.md"

# Core operations
.materialize destination::"./output"
.pack input::"./templates"
.status
.analyze
```

---

## Parameter Syntax: `key::value`

### Format Specification

All parameters use double-colon syntax after parsing:

```bash
name::"value"           # String literal
path::"/to/file"        # Path value
verbosity::2             # Integer value
dry::1                  # Boolean value
mode::inline            # Enum value selection
```

### Key Rules

1. **No Spaces Around `::`** — `key::value`, not `key :: value`
2. **Spaces Preserved in Values** — Each token treated as complete (no re-splitting)
3. **Quotes for Strings** — Optional in unilang, typically added by adapter layer

### Value Types

| Kind | Example | Rust Type | Validation |
|------|---------|-------------|-------------|
| `String` | `name::"my-app"` | `String` | Non-empty |
| `Path` | `path::"/to/file"` | `String` + path validation | File exists check (optional) |
| `Directory` | `dir::"./src"` | `String` + dir validation | Directory exists (optional) |
| `Integer` | `verbosity::2` | `i64` | Numeric parsing |
| `Boolean` | `dry::1` | `bool` | `0`/`1` or `true`/`false` |
| `Enum` | `mode::inline` | `String` | Must match one of defined variants |

---

## Architecture Patterns

### Genfile Pattern: Direct Unilang Syntax

**User types unilang syntax directly:**

```bash
# Command invocation
genfile .archive.new name::"test" verbosity::2
genfile .file.add path::"src/main.rs"

# REPL mode
genfile> .materialize destination::"./output"
genfile> .value.set name::project_name value::"MyApp"
```

**Architecture:**
```
User Input (String/Vec<String>)
    ↓ Pipeline::process_command_from_argv()
Tokens (already in unilang format)
    ↓ Parser → SemanticAnalyzer
VerifiedCommand
    ↓ Interpreter
OutputData
```

**No Adapter Needed** — Direct mapping between user input and internal model.

### Claude Runner Pattern: Flag Adapter Layer

**User types Claude-style flags:**

```bash
claude_runner --message "hi" --dir /path --dry-run
```

**Adapter converts to unilang:**
```
[".run", "message::hi", "dir::/path", "dry::1"]
```

**Architecture:**
```
User Input (--flag value)
    ↓ argv_to_unilang_tokens()
Tokens (unilang format)
    ↓ Parser → SemanticAnalyzer
VerifiedCommand
    ↓ Interpreter
OutputData
```

**Adapter Layer Complexity** — Extra step to translate between formats.

---

## Pipeline Flow

### Complete Processing Pipeline

```
┌─────────────────────────────────────────────────────┐
│ User Input                                      │
│ (String or Vec<String>)                      │
└──────────────┬──────────────────────────────────────┘
               │
               ▼
        ┌───────────────┐
        │   Pipeline     │
        │   - process_command()      │
        │   - process_command_from_argv() │
        └───────┬───────┘
                  │
                  ▼
           ┌─────────────┐
           │   Parser     │
           │   - parse_from_argv()    │
           │   - parse()             │
           └───────┬─────┘
                  │
                  ▼
     GenericInstruction
                  │
                  ▼
           ┌──────────────────┐
           │ SemanticAnalyzer  │
           │ - analyze()      │
           │ - validate       │
           └───────┬────────┘
                  │
                  ▼
     Vec<VerifiedCommand>
                  │
                  ▼
           ┌─────────────┐
           │ Interpreter  │
           │ - run()     │
           └───────┬─────┘
                  │
                  ▼
      Vec<OutputData>
```

### Key Pipeline Characteristics

1. **Token Preservation** — `parse_from_argv()` treats each element as complete token
2. **Semantic Validation** — `SemanticAnalyzer` validates instructions against registry
3. **Handler Execution** — `Interpreter` routes to registered routines
4. **Context Propagation** — `ExecutionContext` carries execution state

---

## CommandDefinition Pattern

### Rust-Based Registration

```rust
use unilang::data::{
    CommandDefinition,
    ArgumentDefinition,
    Kind,
    CommandName,
    CommandStatus,
    VersionType,
};

let cmd = CommandDefinition::new(
    CommandName::new(".archive.new").expect("valid command name"),
    "Create new empty template archive".to_string(),
)
.with_namespace(String::new())
.with_status(CommandStatus::Active)
.with_version(VersionType::new("0.1.0").expect("valid version"))
.with_tags(vec!["archive".to_string(), "create".to_string()])
.with_aliases(vec![])
.with_examples(vec![
    ".archive.new name::\"my-template\"".to_string(),
    ".archive.new name::\"api-scaffold\" description::\"REST API template\"".to_string(),
])
.with_auto_help(true)
.with_arguments(vec![
    ArgumentDefinition::new("name", Kind::String)
        .with_description("Archive name"),
    ArgumentDefinition::new("verbosity", Kind::Integer)
        .with_description("Output verbosity level (0-5)")
        .with_optional(Some("1")),
]);

registry.command_add_runtime(&cmd, Box::new(handler))?;
```

### CommandDefinition Fields

| Field | Type | Purpose |
|--------|------|-----------|
| `name` | `CommandName` | Dot-prefixed command name (`.archive.new`) |
| `description` | `String` | Human-readable description for help |
| `namespace` | `String` | Optional namespace prefix for grouping |
| `status` | `CommandStatus` | `Active`/`Deprecated`/`Experimental` |
| `version` | `VersionType` | Semantic version for API compatibility |
| `tags` | `Vec<String>` | Discoverability tags for categorization |
| `aliases` | `Vec<String>` | Alternative command names |
| `arguments` | `Vec<ArgumentDefinition>` | Parameter definitions |
| `auto_help` | `bool` | Enable built-in help generation |

---

## ArgumentDefinition Pattern

### Parameter Definition

```rust
ArgumentDefinition::new("path", Kind::Path)
    .with_description("File path within archive")
    .with_optional(false)              // Required
    .with_default(null)               // No default value
    .with_sensitive(false)             // Not sensitive in output
    .with_multiple(false)              // Single value only
```

### ArgumentDefinition Fields

| Field | Type | Purpose |
|--------|------|-----------|
| `name` | `String` | Parameter name (used in `key::value` syntax) |
| `description` | `String` | Human-readable description for help |
| `kind` | `Kind` | Type constraint (String, Path, Integer, Boolean, Enum) |
| `optional` | `bool` | Whether parameter is required |
| `default` | `Option<String>` | Default value when not provided |
| `sensitive` | `bool` | Whether to mask in output |
| `interactive` | `bool` | Whether to prompt user for value |
| `multiple` | `bool` | Whether to accept multiple values |

---

## YAML-Based Command Definitions

### Genfile's Hybrid Approach

**Authoritative YAML** (`commands/*.yaml`):
```yaml
- name: ".file.add"
  namespace: ""
  description: "Add file to current archive"
  hint: "Add file"
  status: "stable"
  version: "0.1.0"
  auto_help_enabled: true
  examples:
    - ".file.add path::src/main.rs content::\"fn main() {}\""
  arguments:
    - name: "path"
      kind: "Path"
      hint: "Archive file path"
      attributes:
        optional: false
        default: null
```

**Rust Duplication** (`commands/file.rs`):
```rust
let cmd = CommandDefinition::new(
    CommandName::new(".file.add").expect("valid command name"),
    "Add file to current archive".to_string(),
)
// ... duplicate all YAML fields as builder calls
```

### Why Hybrid Pattern?

1. **YAML is Authoritative** — Single source of truth for command metadata
2. **Rust Duplication Required** — unilang doesn't export external Multi-YAML support
3. **Future Migration** — When unilang adds external consumer support, Rust boilerplate can be removed

**Note**: `claude_runner` uses pure Rust `CommandDefinition` (no YAML) due to simpler command set.

---

## Command Registry Pattern

### Runtime Registration

```rust
use unilang::registry::CommandRegistry;

let mut registry = CommandRegistry::new();

// Register commands with handlers
registry.command_add_runtime(&cmd_def, Box::new(handler))?;

// Register more commands
registry.command_add_runtime(&another_cmd_def, Box::new(another_handler))?;
```

### Registration API

| Method | Purpose |
|---------|-----------|
| `new()` | Create empty registry |
| `command_add_runtime()` | Register command with handler routine |
| `find_command()` | Lookup command by name |

### Handler Signature

```rust
type CommandRoutine = Box<dyn Fn(&VerifiedCommand, &mut ExecutionContext) -> Result<OutputData, ErrorData>>;

fn handler(cmd: &VerifiedCommand, ctx: &mut ExecutionContext) -> Result<OutputData, ErrorData> {
    // Extract parameters
    if let Some(Value::String(s)) = cmd.arguments.get("path") {
        // Use parameter value
    }

    // Execute logic
    let output = /* ... */;

    // Return formatted output
    Ok(OutputData {
        content: output,
        format: "text".to_string(),
        execution_time_ms: None,
    })
}
```

---

## Execution Context

### ExecutionContext Structure

```rust
use unilang::interpreter::ExecutionContext;

let ctx = ExecutionContext::default();
// Extend with application-specific state when API supports
```

**Current Limitation**: `ExecutionContext` is minimal; application state (e.g., genfile's `ArchiveState`) must use module-level access or thread-local storage.

**Future Enhancement**: `ExecutionContext` could support custom state injection for cleaner handler implementations.

---

## REPL Mode

### Interactive Command Processing

```rust
use unilang::pipeline::Pipeline;

let pipeline = Pipeline::new(registry);
let ctx = ExecutionContext::default();

loop {
    print!("prompt> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let result = pipeline.process_command(input.trim(), ctx);

    if result.success {
        for output in result.outputs {
            println!("{}", output.content);
        }
    } else {
        eprintln!("{}", result.error.unwrap_or_default());
    }
}
```

**REPL Characteristics**:
- Direct string input (no argv parsing)
- State preserved across commands
- Error handling per command
- Graceful exit on EOF or quit command

---

## Error Handling

### ErrorData Structure

```rust
use unilang::data::{ErrorData, ErrorCode};

Err(ErrorData::new(
    ErrorCode::InternalError,      // Or ValidationError, etc.
    "Descriptive error message".to_string(),
))
```

### Error Codes

| ErrorCode | When Used |
|-----------|-------------|
| `InternalError` | Application-level errors (handler failures) |
| `ValidationError` | Parameter validation failures |
| `CommandNotFoundError` | Command not found in registry |
| `ArgumentRequired` | Required argument missing |

---

## Design Implications

### Pattern 1: Direct Unilang (genfile)

**Pros:**
- No adapter layer needed
- Direct mapping to internal model
- Clean command hierarchy (`.namespace.action`)
- REPL-friendly

**Cons:**
- Users must learn `::` syntax
- Non-standard for simple CLIs
- Less familiar to `git`/`npm` users

### Pattern 2: Flag Adapter (claude_runner)

**Pros:**
- Familiar `--flag value` syntax
- Works like other CLIs (docker, npm, cargo)
- Short flags (`-m`, `-d`) supported
- Brevity for common use

**Cons:**
- Adapter layer complexity
- Implicit command routing (help as flag vs command)
- Mixing concerns (help flag vs command selector)
- Extensibility friction for new commands

---

## Comparison Summary

| Aspect | genfile (Direct) | claude_runner (Adapter) |
|---------|-------------------|------------------------|
| User syntax | `.archive.new name::test` | `--message "hi" --dir /path` |
| Adapter layer | None | `argv_to_unilang_tokens()` |
| Command selection | Explicit (first token) | Implicit (`--help` flag or default) |
| Parameter syntax | `key::value` | `--flag value` → `key::value` |
| CLI conventions | unilang-native | Claude-style flags |
| REPL support | Yes | No |
| Total commands | 24 | 2 (`.run`, `.help`) |

---

## Key Takeaways

1. **Dot Prefix is Universal** — All unilang commands use `.name` pattern
2. **Namespace Hierarchy** — `.namespace.action` enables logical grouping
3. **Parameter Syntax** — `key::value` is the universal unilang format
4. **Pipeline Architecture** — Parser → Semantic → Interpreter is standard flow
5. **Registry-Based** — Commands registered at runtime with handlers
6. **YAML Authoritative** — genfile pattern (Rust duplicates it)
7. **Adapter Layer Tradeoff** — User familiarity vs direct unilang mapping

---

## References

- [genfile CLI](../../../../genfile/docs/cli/readme.md) - Full genfile CLI documentation
- [unilang on crates.io](https://docs.rs/unilang) - Published API documentation
- [claude_runner CLI](../cli/readme.md) - User-facing CLI reference
- [Command Design](command_design.md) - Design recommendations for claude_runner
