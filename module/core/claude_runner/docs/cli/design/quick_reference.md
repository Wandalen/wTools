# CLI Quick Reference

Fast lookup for common CLI patterns, command syntax, and implementation details.

## Command Syntax

```bash
# Explicit commands (recommended for scripts)
claude_runner .help
claude_runner .run --message "task" --dir /path

# Implicit .run (backward compatible)
claude_runner --message "task" --dir /path
```

## Parameter Syntax

```bash
# String parameters
claude_runner --message "my task"
claude_runner -m "my task"

# Path parameters
claude_runner --dir /workspace
claude_runner -d /workspace

# Integer parameters
claude_runner --max-tokens 50000

# Boolean flags
claude_runner --dry-run
claude_runner --verbose
claude_runner -c  # --continue
```

## All Parameters

| Flag | Short | Type | Default | Description |
|-------|--------|------|----------|-------------|
| `--message` | `-m` | String | N/A | Prompt message for Claude |
| `--dir` | `-d` | Path | Current dir | Working directory |
| `--continue` | `-c` | Boolean | false | Continue conversation |
| `--max-tokens` | — | Integer | 200000 | Max output tokens |
| `--skip-permissions` | — | Boolean | false | Skip tool prompts |
| `--dry-run` | — | Boolean | false | Print without execute |
| `--verbose` | `-v` | Boolean | false | Print to stderr |
| `--session-dir` | — | Path | N/A | Session directory |
| `--model` | — | String | N/A | Claude model |
| `--help` | `-h` | — | — | Print help |

## Token Format

```rust
// Unilang token format after adapter
[
    ".run",                    // Command
    "message::Fix the bug",    // String parameter
    "dir::/workspace",          // Path parameter
    "dry::1"                     // Boolean parameter
    "max_tokens::50000"          // Integer parameter
]
```

## Adapter Flow

```
User: claude_runner --message "hi" --dry-run
         ↓ argv_to_unilang_tokens()
Tokens: [".run", "message::hi", "dry::1"]
         ↓ Parser → SemanticAnalyzer → Interpreter
Execution: ClaudeCommand with preview then execution
```

## Command Registration

```rust
// Define command
let cmd = CommandDefinition::new(
    CommandName::new(".run").expect("valid command name"),
    "Description".to_string(),
)
.with_arguments(vec![
    ArgumentDefinition::new("name", Kind::String)
        .with_description("Parameter name"),
]);

// Register with handler
registry.command_add_runtime(&cmd, Box::new(handler))?;
```

## Handler Pattern

```rust
let routine: CommandRoutine = Box::new(|cmd, _ctx| {
    // Extract parameters
    if let Some(Value::String(s)) = cmd.arguments.get("message") {
        // Use s
    }

    // Execute
    Ok(OutputData {
        content: output.to_string(),
        format: "text".to_string(),
        execution_time_ms: None,
    })
});
```

## Error Handling

```rust
// Parse error
return Err(Error::msg("invalid --max-tokens value: -1"));

// Validation error
Ok(ErrorData::new(
    ErrorCode::ValidationError,
    "Required parameter missing".to_string(),
))

// Execution error
builder.execute().map_err(|e| ErrorData::new(
    ErrorCode::InternalError,
    format!("Execution failed: {e}"),
))?;
```

## Testing Helpers

```rust
// Run CLI with arguments
let out = run_cli(&["--message", "test", "--dry-run"]);

// Check success
assert!(out.status.success());

// Check output
let stdout = String::from_utf8_lossy(&out.stdout);
assert!(stdout.contains("expected"));

// Check error
assert!(!out.status.success());
let stderr = String::from_utf8_lossy(&out.stderr);
assert!(stderr.contains("error message"));
```

## Documentation Links

| Topic | File |
|--------|-------|
| Tutorial | [tutorial.md](tutorial.md) |
| Implementation Guide | [implementation_guide.md](implementation_guide.md) |
| Unilang Exploration | [unilang_exploration.md](unilang_exploration.md) |
| Command Design | [command_design.md](command_design.md) |
| Main Docs | [readme.md](readme.md) |
| User CLI Reference | [../cli/readme.md](../cli/readme.md) |
| Testing | [../cli/testing/readme.md](../cli/testing/readme.md) |
