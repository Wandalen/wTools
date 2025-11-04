# Unilang Usage Guide

Quick reference for using the unilang command framework.

---

## Core Principle

**"Define Once, Use Everywhere"** - Write command definitions once, deploy across CLI, REPL, and Web APIs.

---

## Quick Start

### Default Approach (Recommended)

Unilang uses **Approach #2 (Multi-YAML Build-Time Static)** by default:
- Define commands in YAML files at compile-time
- Zero-overhead lookups (~80ns vs ~4,200ns runtime)
- 50x faster than runtime HashMap approach

```toml
[dependencies]
unilang = "0.35"  # Default: Multi-YAML + SIMD + enhanced REPL
```

### Alternative Approaches

Enable via feature flags when needed:

| Approach | Feature Flag | Performance | Use Case |
|----------|--------------|-------------|----------|
| Multi-YAML Build-time | `approach_yaml_multi_build` (default) | ~80ns | Production CLI |
| YAML Runtime | `approach_yaml_runtime` | ~4,200ns | Plugins, dynamic configs |
| Rust DSL | Always available | ~4,200ns | Tests, full control |
| Hybrid | `approach_hybrid` | Mixed | Base CLI + plugins |

---

## Command Naming (CRITICAL)

### Absolute Rule

**All command names MUST start with dot (`.`)**

```rust
// ✅ CORRECT
.greet
.math.add
.db.migrate.status

// ❌ INCORRECT
greet         // Missing dot prefix
math.add      // Missing leading dot
```

### YAML Formats (Both Valid)

**Format 1** (Recommended for clarity):
```yaml
- name: ".session.list"
  namespace: ""
```

**Format 2** (Valid for namespacing):
```yaml
- name: "list"
  namespace: ".session"  # Namespace MUST include dot prefix
```

### Namespace Hierarchy

```
.database         → Database operations
.db.migrate       → Migrations
.db.migrate.up    → Specific migration
.fs.copy          → File operations
.net.ping         → Network operations
```

---

## Command Structure

### Required Fields

```rust
CommandDefinition {
  name: ".command_name",        // MUST start with dot
  description: "...",           // Required
  arguments: vec![...],         // Define all parameters
}
```

### Recommended Fields

```rust
CommandDefinition {
  hint: "Short one-liner",      // Shown in lists
  status: "stable",             // stable, beta, experimental, deprecated
  version: "1.0.0",             // Track evolution
  aliases: vec![".short"],      // Alternative names
  examples: vec!["..."],        // Real usage patterns
  auto_help_enabled: true,      // Default: true. Auto-generates .command.help
}
```

**Note:** `auto_help_enabled` defaults to `true`, creating `.command.help` commands automatically. Set to `false` to prevent auto-generation while keeping `?` and `??` help operators.

---

## Arguments

### Supported Types

**Basic**: `String`, `Integer`, `Float`, `Boolean`
**Paths**: `Path`, `File`, `Directory`
**Specialized**: `Url`, `DateTime`, `Pattern`, `Enum`
**Collections**: `List`, `Map`
**Complex**: `JsonString`, `Object`

### Common Patterns

**String with validation:**
```rust
ArgumentDefinition {
  name: "email",
  kind: Kind::String,
  validation_rules: vec![
    ValidationRule::Pattern(r"^[^@]+@[^@]+\.[^@]+$".to_string())
  ],
}
```

**Integer with range:**
```rust
ArgumentDefinition {
  name: "age",
  kind: Kind::Integer,
  validation_rules: vec![
    ValidationRule::Min(18.0),
    ValidationRule::Max(120.0),
  ],
}
```

**Enum with choices:**
```rust
ArgumentDefinition {
  name: "format",
  kind: Kind::Enum(vec!["json".to_string(), "csv".to_string()]),
  attributes: ArgumentAttributes {
    optional: true,
    default: Some("json".to_string()),
    ..Default::default()
  },
}
```

**Sensitive data (passwords, API keys):**
```rust
ArgumentDefinition {
  name: "api_key",
  kind: Kind::String,
  attributes: ArgumentAttributes {
    sensitive: true,      // Prevents display in logs
    interactive: true,    // May prompt for input
    ..Default::default()
  },
}
```

### Common Pitfalls (YAML Definitions)

When defining commands in YAML, avoid these type mismatches that violate unilang's type system:

**❌ WRONG:**
```yaml
arguments:
  - name: "dry"
    kind: "Boolean"
    attributes:
      default: 'false'     # Quoted string - type mismatch

  - name: "verbosity"
    kind: "Integer"
    attributes:
      default: '2'         # Quoted string - type mismatch
```

**✅ CORRECT:**
```yaml
arguments:
  - name: "dry"
    kind: "Boolean"
    attributes:
      default: false       # Unquoted boolean

  - name: "verbosity"
    kind: "Integer"
    attributes:
      default: 2           # Unquoted integer
```

**Type Hint System:** Build-time analysis detects these issues and emits non-blocking hints during `cargo build`. Audit found these mistakes in 124 instances across production codebases. To suppress hints for intentional cases (e.g., `default: "1.0.0"` for version strings), add `suppress_type_hint: true` to the argument's attributes.

---

## Parameter Syntax

### Named Parameters

```bash
.command arg_name::arg_value
```

**Examples:**
```bash
.greet name::Alice age::30
.file.copy source::/home/user/file.txt dest::/tmp/file.txt
```

### Quoting Rules (IMPORTANT)

**Values with spaces MUST be quoted:**
```bash
# ✅ CORRECT
.command path::"/home/user/my files"

# ❌ INCORRECT
.command path::/home/user/my files  # Loses space info
```

### Argv API (CLI Applications)

**Use argv-based API for CLI apps:**
```rust
// ✅ RECOMMENDED
let result = pipeline.process_command_from_argv(&std::env::args().collect());

// ⚠️ Fallback for string parsing
let result = pipeline.process_command_simple(".command arg::value");
```

**Why argv is better:**
- Preserves argument boundaries from OS shell
- No information loss with spaces
- Natural shell syntax

**Example:**
```rust
// Shell: my_cli .greet name "Alice Smith" age 30
// argv:  [".greet", "name::Alice", "Smith", "age::30"]
// Result: name="Alice Smith", age="30" ✅
```

### Unknown Parameter Detection

**Always enforced** (cannot be disabled):
```bash
.command arg::value typo::value  # ❌ REJECTED: unknown parameter 'typo'
```

Error includes:
- Machine-readable code: `UNILANG_UNKNOWN_PARAMETER`
- "Did you mean...?" suggestions (Levenshtein distance ≤ 2)
- Help reference: "Use '.command ??' for help"

---

## Help System

### Three Ways to Access Help

**Method 1: Traditional `?` Operator**
```bash
.command ?
```

**Method 2: Modern `??` Parameter**
```bash
.command ??
.command arg::value ??
```

**Method 3: Auto-Generated `.help` Command**
```bash
.command.help
.namespace.command.help
```

### Verbosity Levels (0-4)

```bash
UNILANG_HELP_VERBOSITY=0  # Minimal - single line
UNILANG_HELP_VERBOSITY=1  # Basic - add parameters
UNILANG_HELP_VERBOSITY=2  # Standard (default) - usage + examples
UNILANG_HELP_VERBOSITY=3  # Detailed - full metadata
UNILANG_HELP_VERBOSITY=4  # Comprehensive - extensive docs
```

---

## Error Handling

### Standard Error Codes

| Code | Meaning |
|------|---------|
| `UNILANG_COMMAND_NOT_FOUND` | Command doesn't exist |
| `UNILANG_ARGUMENT_MISSING` | Required argument not provided |
| `UNILANG_ARGUMENT_TYPE_MISMATCH` | Wrong type for argument |
| `UNILANG_UNKNOWN_PARAMETER` | Parameter not recognized |
| `UNILANG_VALIDATION_RULE_FAILED` | Validation constraint violated |
| `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` | Interactive input needed (REPL) |

### Command Routine Signature

```rust
fn my_command(cmd: VerifiedCommand, ctx: ExecutionContext)
    -> Result<OutputData, ErrorData>
{
    // Extract arguments (type-safe, concise)
    let name = cmd.require_string("name")?;
    let age = cmd.get_integer("age").unwrap_or(0);

    // Do work
    let result = do_something(name, age)?;

    // Return success
    Ok(OutputData {
        content: result,
        format: "text".to_string(),
        execution_time_ms: None,  // Auto-populated
    })
}
```

### Convenient Extraction Methods

```rust
// Type-safe, eliminates ~90% of boilerplate
let age = cmd.require_integer("age")?;
let name = cmd.get_string("name").unwrap_or("World");
let path = cmd.require_path("file")?;
let items = cmd.get_list("items")?;
```

---

## REPL Implementation

### Stateless Operation

```rust
// Create pipeline ONCE
let pipeline = Pipeline::new(registry);

// Reuse for multiple commands (stateless)
loop {
    let cmd = read_user_input();
    let result = pipeline.process_command_simple(&cmd);

    // No state accumulation
    // Memory usage constant
}
```

### Interactive Arguments

```rust
match semantic_analyzer.analyze() {
    Err(Error::Execution(error_data))
        if error_data.code == "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" => {
        // Prompt user for secure input
        let input = prompt_password(&error_data.message);
        // Retry with input
    },
    Ok(verified) => {
        let result = interpreter.run(verified, context);
    }
}
```

---

## CLI Aggregation

### Multi-Crate Consolidation

```rust
let unified_cli = CliBuilder::new()
  .static_module_with_prefix("database", ".db", db_commands)
  .static_module_with_prefix("filesystem", ".fs", fs_commands)
  .static_module_with_prefix("network", ".net", net_commands)
  .detect_conflicts(true)      // Catch duplicate prefixes
  .build_static();             // Zero-overhead
```

**Benefits:**
- Namespace isolation (no conflicts)
- Consistent interface
- Single binary deployment
- Unified help system

---

## Testing

### Location

**All tests in `tests/` directory, NOT `src/`**

```
tests/
  ├── acceptance.rs
  ├── integration/
  │   └── end_to_end.rs
  └── unit/
      └── command_parsing.rs
```

### Coverage Areas

- Command definition and registration
- Parsing (tokens, quotes, parameters)
- Semantic analysis (types, validation)
- Integration (full pipeline, error recovery)
- Help system (generation, verbosity)
- REPL (stateless, interactive)

---

## Performance

### Compile-Time vs Runtime

| Aspect | Compile-Time | Runtime |
|--------|--------------|---------|
| Lookup | ~80ns (PHF) | ~4,200ns (HashMap) |
| Flexibility | Requires recompile | Dynamic changes |
| Use Case | Production CLI | Plugins, development |

**Recommendation:** Compile-time by default, runtime only when flexibility is critical.

### SIMD Optimization

**Enabled by default** - provides 4-25x parsing performance improvement.

Disable if needed:
```toml
unilang = { version = "0.35", default-features = false, features = [
  "enabled",
  "approach_yaml_multi_build"
]}
```

---

## Security

### Sensitive Arguments

```rust
// NEVER output sensitive values
let api_key = cmd.get_string("api_key");
if let Some(key) = api_key {
    // ✅ CORRECT - hide the key
    content.push_str(&format!("Using API key: {}...{}\n",
                              &key[..2], &key[key.len()-2..]));

    // ❌ WRONG - would expose the key
    // content.push_str(&format!("API Key: {}\n", key));
}
```

---

## Quick Reference Checklists

### Command Definition
- [ ] Name starts with dot (`.command`)
- [ ] Clear description and hint
- [ ] Status and version set
- [ ] Examples provided
- [ ] `auto_help_enabled` configured (defaults to true)

### Argument Definition
- [ ] Descriptive name
- [ ] Kind matches data type
- [ ] Optional/required explicit
- [ ] Validation rules appropriate
- [ ] Sensitive flag for passwords/keys

### Error Handling
- [ ] All inputs validated
- [ ] Error messages actionable
- [ ] Help offered on failure
- [ ] Sensitive data never logged

---

## Key Principles

1. **Define Once, Use Everywhere** - Single source of truth
2. **Opinionated Defaults** - Multi-YAML by default, opt into alternatives
3. **Minimum Implicit Magic** - Explicit is better
4. **Fail-Fast Validation** - Errors detected early
5. **Mandatory Help** - Every command gets `.help` automatically
6. **Zero-Overhead Static** - 50x faster than runtime
7. **Stateless Reusability** - REPL-safe, no state accumulation

---

## Examples

See `examples/` directory:
- `00_quick_start.rs` - Basic runtime usage
- `static_01_basic_compile_time.rs` - YAML + build.rs
- `static_02_yaml_build_integration.rs` - Multi-YAML production
- `12_repl_loop.rs` - Basic REPL
- `practical_cli_aggregation.rs` - Advanced multi-crate

**Full specification:** See `spec.md` for complete API documentation.

---

## Summary

**For most users:** Use default configuration (Multi-YAML Build-Time Static) for zero-overhead production CLI applications with 50x faster lookups than runtime approaches.

**Key rule:** All command names MUST start with dot (`.`). This is enforced at runtime.

**Help generation:** Commands with `auto_help_enabled: true` (default) automatically get `.command.help` commands. All commands support `?` and `??` help operators regardless.

**Use argv API:** For CLI applications, always use `process_command_from_argv()` to preserve argument boundaries.
