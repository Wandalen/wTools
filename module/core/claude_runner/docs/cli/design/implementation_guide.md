# CLI Implementation Guide

Step-by-step guide for implementing new commands and modifying existing CLI behavior in `claude_runner`.

## Quick Reference

| Task | File | Key Function |
|------|------|---------------|
| Add new command | `src/main.rs` | `argv_to_unilang_tokens()`, `build_registry()` |
| Add new parameter | `src/main.rs` | Parse in adapter, add to `ArgumentDefinition` |
| Update help text | `src/main.rs` | `print_help()` function |
| Add parameter handling | `src/main.rs` | Handler routine logic |
| Add tests | `tests/*.rs` | Integration tests for new functionality |

---

## Command Addition Checklist

### Step 1: Add to Adapter Layer

**Location:** `src/main.rs` — `argv_to_unilang_tokens()` function

```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    // ... existing parsing

    match argv[i].as_str() {
        // Add new command match
        ".mycommand" => {
            return Ok(vec![".mycommand".to_string()]);
        }
        // ... existing matches
    }
}
```

### Step 2: Register Command

**Location:** `src/main.rs` — `build_registry()` function

```rust
fn build_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // ... existing registrations

    let mycommand_def = CommandDefinition::new(
        CommandName::new(".mycommand").expect("valid command name"),
        "Command description".to_string(),
    )
    .with_arguments(vec![
        ArgumentDefinition::new("param1", Kind::String)
            .with_description("First parameter"),
        ArgumentDefinition::new("param2", Kind::Integer)
            .with_description("Second parameter")
            .with_optional(Some("1")),
    ]);

    let mycommand_routine: CommandRoutine = Box::new(|cmd, _ctx| {
        // Handler implementation
        Ok(OutputData {
            content: "result".to_string(),
            format: "text".to_string(),
            execution_time_ms: None,
        })
    });

    registry.command_add_runtime(&mycommand_def, mycommand_routine)
        .expect("internal error: failed to register .mycommand command");

    registry
}
```

### Step 3: Update Help Text

**Location:** `src/main.rs` — `print_help()` function

```rust
fn print_help() {
    // ... existing help text

    println!("COMMANDS:");
    println!("  .help       Print this help");
    println!("  .run        Execute Claude Code (default command)");
    println!("  .mycommand  My custom command");
    println!();

    println!("OPTIONS (for .run, .mycommand):");
    // ... options
}
```

### Step 4: Add Tests

**Location:** `tests/cli_args_test.rs` or new test file

```rust
#[test]
fn mycommand_basic_execution() {
    let out = run_cli(&[".mycommand", "param1::test", "param2::2"]);
    assert!(out.status.success(), ".mycommand should exit 0");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("result"), "Output should contain result");
}

#[test]
fn mycommand_missing_required_param() {
    let out = run_cli(&[".mycommand"]);
    assert!(!out.status.success(), "Should fail without required param");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("required"), "Error should mention required parameter");
}
```

### Step 5: Update Documentation

**Locations:**
- `docs/cli/readme.md` — Update command count
- `docs/cli/commands.md` — Add command entry (if exists)
- `docs/cli/params.md` — Add new parameters (if any)

---

## Parameter Addition Checklist

### Step 1: Add to Adapter Parsing

**Location:** `src/main.rs` — `argv_to_unilang_tokens()` function

```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    let mut myparam: Option<String> = None;

    // ... existing parsing

    match argv[i].as_str() {
        "--myparam" => {
            i += 1;
            let val = argv.get(i).ok_or_else(|| Error::msg("--myparam requires a value"))?;
            myparam = Some(val.clone());
        }
        // ... existing matches
    }

    // Build tokens
    let mut tokens = vec![".run".to_string()];
    if let Some(p) = myparam {
        tokens.push(format!("myparam::{p}"));
    }
    // ... other parameters

    Ok(tokens)
}
```

### Step 2: Add to Command Definition

**Location:** `src/main.rs` — `build_registry()` function

```rust
let run_def = CommandDefinition::new(
    CommandName::new(".run").expect("valid command name"),
    "Execute Claude Code with configurable parameters".to_string(),
)
.with_arguments(vec![
    // ... existing arguments
    ArgumentDefinition::new("myparam", Kind::String)
        .with_description("My new parameter"),
        .with_optional(None::<String>),  // Required
    // ... or with_optional(Some("default")) for optional
]);
```

### Step 3: Add to Handler Logic

**Location:** `src/main.rs` — `.run` command handler

```rust
let run_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    // ... existing parameter extraction

    if let Some(Value::String(s)) = cmd.arguments.get("myparam") {
        builder = builder.with_myparam(s.clone());
    }

    // ... rest of handler
});
```

### Step 4: Update Help Text

**Location:** `src/main.rs` — `print_help()` function

```rust
fn print_help() {
    // ... existing help text

    println!("OPTIONS (for .run):");
    println!("  -m, --message <MSG>        Prompt message for Claude");
    println!("      --myparam <VALUE>       My new parameter");
    // ... rest of options
}
```

### Step 5: Add Tests

**Location:** `tests/cli_args_test.rs`

```rust
#[test]
fn myparam_with_value() {
    let out = run_cli(&["--myparam", "value", "--dry-run"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("value"), "Output should include myparam value");
}

#[test]
fn myparam_missing_value() {
    let out = run_cli(&["--myparam"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("requires a value"), "Error should mention missing value");
}
```

### Step 6: Update Documentation

**Locations:**
- `docs/cli/params.md` — Add parameter entry
- `docs/cli/types.md` — Add type if new kind
- `docs/cli/parameter_groups.md` — Add to group if related

---

## Handler Implementation Patterns

### String Parameter

```rust
if let Some(Value::String(s)) = cmd.arguments.get("message") {
    builder = builder.with_message(s.clone());
}
```

### Integer Parameter

```rust
if let Some(Value::Integer(n)) = cmd.arguments.get("max_tokens") {
    builder = builder.with_max_output_tokens(u32::try_from(*n).unwrap_or(0));
}
```

### Boolean Parameter

```rust
if matches!(cmd.arguments.get("dry"), Some(Value::Boolean(true))) {
    // Dry-run mode
}
if matches!(cmd.arguments.get("verbose"), Some(Value::Boolean(true))) {
    // Verbose mode
}
```

### Optional Parameter

```rust
let value = cmd.arguments.get("optional_param")
    .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
    .unwrap_or("default_value");  // Or handle None case
```

### Multiple Parameters

```rust
// Note: unilang doesn't support multiple values natively
// Design alternatives: repeat flag (last wins) or comma-separated values

if let Some(Value::String(s)) = cmd.arguments.get("files") {
    for file in s.split(',') {
        // Process each file
    }
}
```

---

## Error Handling Patterns

### Missing Required Parameter

```rust
let name = cmd.arguments.get("name")
    .ok_or_else(|| ErrorData::new(
        ErrorCode::ValidationError,
        "name parameter is required".to_string(),
    ))?;

if let Value::String(s) = name {
    // Use name
} else {
    // Should never happen due to ok_or_else above
    return Err(ErrorData::new(
        ErrorCode::ValidationError,
        "name parameter is required".to_string(),
    ));
}
```

### Type Conversion Error

```rust
if let Some(Value::Integer(n)) = cmd.arguments.get("count") {
    let count = u32::try_from(*n).map_err(|_| ErrorData::new(
        ErrorCode::ValidationError,
        "count must fit in u32 range".to_string(),
    ))?;
    // Use count
}
```

### File Operation Error

```rust
builder.execute().map_err(|e| ErrorData::new(
    ErrorCode::InternalError,
    format!("Failed to execute Claude: {e}"),
))?;
```

---

## Testing Patterns

### Basic Execution Test

```rust
#[test]
fn command_basic_execution() {
    let out = run_cli(&["--message", "test"]);
    assert!(out.status.success());
}
```

### Error Case Test

```rust
#[test]
fn command_error_on_invalid_value() {
    let out = run_cli(&["--max-tokens", "-1"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("invalid --max-tokens value"));
}
```

### Dry Run Test

```rust
#[test]
fn command_dry_run_shows_preview() {
    let out = run_cli(&["--message", "test", "--dry-run"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("claude"), "Should show claude command");
    assert!(!stdout.contains("executed"), "Should not execute");
}
```

### Output Verification Test

```rust
#[test]
fn command_output_contains_expected() {
    let out = run_cli(&["--message", "test"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("test"), "Output should contain message");
}
```

---

## Documentation Templates

### Command Entry Template

```markdown
### Command :: N. `.mycommand`

- **Purpose:** [One-sentence description]
- **Parameters:** X
- **Exit Codes:** 0 (success) | 1 (error)

## Examples

```bash
claude_runner .mycommand param1::"value"
```

## Implementation Notes

[Implementation details, design decisions, known limitations]
```

### Parameter Entry Template

```markdown
### Parameter :: N. `myparam`

- **Kind:** String | Path | Integer | Boolean
- **Commands:** `.run`, `.mycommand`
- **Required:** true | false
- **Default:** [Default value or N/A]

## Description

[Parameter description]

## Validation Rules

[List of validation rules, edge cases covered]

## Examples

```bash
claude_runner --myparam "value"
claude_runner --myparam "value2" --dry-run
```
```

---

## Common Patterns

### State Management

For commands that maintain state (sessions, configs, etc.):

```rust
// Use module-level state
static CURRENT_SESSION: Mutex<Option<Session>> = Mutex::new(None);

fn session_set_handler(cmd: &VerifiedCommand, _ctx) -> Result<OutputData, ErrorData> {
    let name = get_string_param(cmd, "name")?;

    let mut session = CURRENT_SESSION.lock().unwrap();
    *session = Some(Session::new(name));

    Ok(OutputData {
        content: format!("Session '{name}' set as active").to_string(),
        format: "text".to_string(),
        execution_time_ms: None,
    })
}
```

### File Operations

```rust
use std::path::Path;

fn file_path_handler(cmd: &VerifiedCommand, _ctx) -> Result<OutputData, ErrorData> {
    let path = get_string_param(cmd, "path")?;

    if !Path::new(&path).exists() {
        return Err(ErrorData::new(
            ErrorCode::ValidationError,
            format!("Path does not exist: {path}"),
        ));
    }

    // Process file
    Ok(OutputData { /* ... */ })
}
```

### Command Composition

```rust
fn compose_command_handler(cmd: &VerifiedCommand, ctx: &mut ExecutionContext) -> Result<OutputData, ErrorData> {
    // Extract parameters
    let subcommand = get_string_param(cmd, "subcommand")?;

    // Route to sub-handler
    match subcommand.as_str() {
        "create" => create_handler(cmd, ctx),
        "update" => update_handler(cmd, ctx),
        "delete" => delete_handler(cmd, ctx),
        _ => Err(ErrorData::new(
            ErrorCode::ValidationError,
            format!("Unknown subcommand: {subcommand}"),
        )),
    }
}
```

---

## Quick Decision Guide

| Question | Answer |
|----------|--------|
| Boolean or flag? | Use `Kind::Boolean` (dry run, verbose) |
| With value? | Use `Kind::String`, `Kind::Path`, or `Kind::Integer` |
| Required? | `with_optional(None::<String>)` (no default) |
| Optional? | `with_optional(Some("default"))` |
| Multiple values? | Comma-separated string or repeat flag (last wins) |
| Custom type? | Wrap in `struct` with validation, document in `types.md` |

---

## References

- [Tutorial](tutorial.md) — Hands-on lessons
- [Unilang Exploration](unilang_exploration.md) — Framework details
- [Command Design](command_design.md) — Design recommendations
- [Testing](../cli/testing/readme.md) — Test patterns
