# claude_runner

> **Workspace:** [wtools](https://github.com/Wandalen/wTools) — `module/core/claude_runner`

Claude Code process execution with builder pattern and single execution point.

### Responsibility Table

| Entity | Responsibility | Input→Output | Scope | Out of Scope |
|--------|---------------|--------------|-------|--------------|
| claude_runner | Claude Code process execution | ClaudeCommand Config → Process Output | Command building, process spawning, output capture, token limits | ❌ Session storage paths → `claude_session`<br>❌ Continuation detection → `claude_session`<br>❌ Context injection → `dream_agent`<br>❌ Parameter parsing → `dream_agent`<br>❌ Session strategy → `dream_agent` |

### Scope

**Responsibility:**
- Claude Code process execution (Command::new("claude"))
- Builder pattern API (ClaudeCommand::new().with_*())
- Token limit configuration (200K default)
- Process output capture (stdout/stderr)
- Single execution point (duplication = 1x)

**In Scope:**
- ClaudeCommand::new() builder entry point
- with_working_directory(), with_max_output_tokens(), with_continue_conversation(), etc. (40+ methods)
- execute() terminal method with process spawning
- stdout/stderr capture and parsing
- Exit code handling and error mapping

**Out of Scope:**
- ❌ Session storage path resolution → delegated to `claude_session` crate
- ❌ Continuation detection → delegated to `claude_session` crate
- ❌ Context injection from wplan → delegated to `dream_agent` crate
- ❌ Parameter parsing from CLI → delegated to `dream_agent` crate
- ❌ Session lifecycle strategy → delegated to `dream_agent` crate

## Features

- **Builder Pattern**: Fluent API with method chaining (NO deprecated factories)
- **Token Limit Fix**: Explicit 200K token default (prevents "exceeded maximum" errors)
- **Single Execution Point**: Consolidates duplicate Command::new("claude") calls
- **Type Safety**: Builder pattern enforces correct configuration
- **Minimal Dependencies**: Only error_tools + standard library

## Usage

```rust
use claude_runner::ClaudeCommand;

// Basic execution
let result = ClaudeCommand::new()
  .with_working_directory("/home/user/project")
  .with_max_output_tokens(200_000)
  .with_continue_conversation(true)
  .execute()?;

println!("Output: {}", result);

// Advanced configuration
let result = ClaudeCommand::new()
  .with_working_directory("/tmp/work")
  .with_max_output_tokens(200_000)
  .with_model("claude-opus-4-5")
  .with_verbose(true)
  .with_system_prompt("You are a helpful coding assistant")
  .with_message("Fix the bug in main.rs")
  .execute()?;
```

## Architecture

```
Builder Pattern Flow:

ClaudeCommand::new()
  └→ with_working_directory()      (fluent method chaining)
  └→ with_max_output_tokens()
  └→ with_continue_conversation()
  └→ execute()                     ← SINGLE execution point
      └→ CommandBuilder::build()   (construct std::process::Command)
      └→ Command::new("claude")    ← ONLY location in entire codebase
      └→ ProcessExecutor::run()    (spawn, capture output)
      └→ Return ExecutionResult
```

## Migration from Old API

**Before (DEPRECATED - DO NOT USE):**
```rust
// Factory method (DEPRECATED)
ClaudeCommand::generate(/* 40 parameters */)

// Mixed execution (DEPRECATED)
session.execute_interactive()
session.execute_non_interactive()

// Duplicate execution points (2x)
Command::new("claude")  // Location 1
Command::new("claude")  // Location 2
```

**After (THIS CRATE):**
```rust
// Builder pattern (CORRECT)
ClaudeCommand::new()
  .with_*()
  .execute()

// Single execution point (1x)
Command::new("claude")  // ONLY in claude_runner::execute()
```

## Token Limit Bug Fix

**Problem:** Default Claude Code token limit is 32K, causing "exceeded maximum" errors

**Solution:** Set explicit max_output_tokens to 200K:

```rust
ClaudeCommand::new()
  .with_max_output_tokens(200_000)  // Explicit token limit
  .execute()?
```

## Dependencies

- **error_tools**: Workspace-standard error handling (Result, Error types)

Total: 1 workspace dependency (wtools), 0 external direct dependencies

## Testing

```bash
cargo nextest run
```

## Critical Execution Rule

**Command::new("claude") MUST appear exactly once:**
- ✅ Single occurrence in claude_runner::execute()
- ❌ Zero occurrences in dream_agent
- ❌ Zero occurrences in claude_session

Verification: `grep -r "Command::new.*claude" src/` should find exactly 1 match.
