# CLI Architecture

Complete architectural overview of `claude_runner` CLI binary (`claude_runner_cli` package),
showing data flow, component relationships, and integration points.

## Architectural Context

`claude_runner` here refers to the **CLI binary** (`claude_runner_cli` crate). This binary
is invoked as a **subprocess** by `dream_agent` (willbe). The binary has no knowledge of
session management or context injection — it purely translates CLI flags to `ClaudeCommand`
builder calls.

**Invocation chain:**
```
dream_agent (willbe, orchestrator)
  → spawns subprocess: claude_runner --message X --dir Y --continue ...
      → claude_runner_cli (wtools binary, THIS binary)
          → claude_runner_core ClaudeCommand builder
              → Command::new("claude")
```

**Key boundary:** `dream_agent` owns session management, context injection, and parameter
routing. `claude_runner` binary owns flag-to-builder translation and process spawning.

## System Overview

```
┌──────────────────────────────────────────────────────────────────────┐
│                         User Interface                         │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Terminal     │  │ REPL (TBD)  │  │ Library API  │    │
│  │ --flags     │  │ .cmd syntax  │  │ direct call  │    │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘    │
└─────────┼──────────────────┼──────────────────┼───────────────────┘
          │                  │                  │
          ▼                  │                  ▼
    ┌─────────┐             │          ┌─────────────┐
    │ Adapter  │             │          │ Parser       │
    │ Layer    │             │          │ (unilang)   │
    │           │             │          └──────┬──────┘
    │           │             │                 │
    │           │             │                 ▼
    └───────┬───┘             ┌─────────────┐
              │                 │ Semantic    │
              │                 │ Analyzer    │
              │                 └──────┬──────┘
              │                        │
              ▼                        ▼
        ┌─────────────┐       ┌─────────────┐
        │ Token Vec   │       │ Verified     │
        │ .run msg::hi│       │ Command(s)  │
        └──────┬──────┘       └──────┬──────┘
               │                      │
               │                      │
               │                      │
               ▼                      ▼
        ┌─────────────┐       ┌─────────────┐
        │ Command     │       │ Command     │
        │ Registry    │       │ Definitions │
        │ .run        │       │ Arg Types   │
        │ .help       │       │ Handlers    │
        │ (future)   │       └──────┬──────┘
        └──────┬──────┘              │
               │                     │
               │                     │
               │                     │
               ▼                     ▼
        ┌─────────────┐       ┌─────────────┐
        │ Claude      │       │ Value       │
        │ Command     │       │ Types       │
        │ Builder     │       │ Map/List    │
        │ (core)     │       │ Boolean     │
        └──────┬──────┘       └──────┬──────┘
               │                     │
               │                     │
               │                     │
               ▼                     ▼
        ┌───────────────────────────────────┐
        │      Execution                │
        │  (Interpreter → Handler)       │
        └──────┬───────────────────────┘
               │
               │
               ▼
        ┌─────────────┐
        │ Claude      │
        │ Process    │
        │ (executed)  │
        └──────┬──────┘
               │
               │
               ▼
          ┌────────┐
          │ stdout │
          │ stderr │
          │ exit   │
          └────────┘
```

---

## Component Layers

### Layer 1: User Interface

**Components:**
- Terminal input (`--flag value` syntax)
- Potential REPL (`.command` syntax)
- Library API (direct function calls)

**Responsibilities:**
- Accept user input
- Provide user feedback
- Handle graceful exits

### Layer 2: Adapter Layer

**File:** `src/main.rs` — `argv_to_unilang_tokens()`

**Components:**
- Flag parser (`--message`, `--dir`, etc.)
- Command router (implicit/explicit detection)
- Token builder (`.run`, `.help`, `key::value`)

**Responsibilities:**
- Parse user argv
- Convert to unilang token format
- Route to correct command

### Layer 3: Parsing Layer

**Framework:** `unilang::parser::Parser`

**Components:**
- Token parser
- Instruction builder
- Syntax validator

**Responsibilities:**
- Parse token vec
- Build `GenericInstruction`
- Validate syntax

### Layer 4: Semantic Analysis Layer

**Framework:** `unilang::semantic::SemanticAnalyzer`

**Components:**
- Command matcher
- Argument validator
- Type checker

**Responsibilities:**
- Match commands against registry
- Validate arguments
- Check types

### Layer 5: Command Registry

**Framework:** `unilang::registry::CommandRegistry`

**Components:**
- Command definitions (`.run`, `.help`)
- Argument definitions (9 parameters)
- Handler routines (closures)

**Responsibilities:**
- Store command metadata
- Route to handlers
- Provide help data

### Layer 6: Execution Layer

**Framework:** `unilang::interpreter::Interpreter`

**Components:**
- Handler executor
- Context manager
- Output formatter

**Responsibilities:**
- Execute handler routines
- Manage execution context
- Return formatted outputs

### Layer 7: Core Library

**Library:** `claude_runner_core::ClaudeCommand`

**Components:**
- Builder pattern
- Process executor
- Output descriptors

**Responsibilities:**
- Build Claude invocation
- Execute subprocess
- Capture stdout/stderr

---

## Data Flow

### Happy Path

```
User: claude_runner --message "Fix bug" --dry-run
  ↓
Adapter: [".run", "message::Fix bug", "dry::1"]
  ↓
Parser: GenericInstruction { command: "run", args: {...} }
  ↓
Semantic: VerifiedCommand { args: {"message": String("Fix bug"), "dry": Boolean(true)} }
  ↓
Registry: Found handler for ".run"
  ↓
Interpreter: Execute handler
  ↓
Handler: preview = builder.with_message("Fix bug").with_dry(true).describe()
  ↓
Output: OutputData { content: preview, format: "text" }
  ↓
User: env var block + "claude \"Fix bug\"" (on stdout)
```

### Error Path

```
User: claude_runner --max-tokens -1
  ↓
Adapter: Parse --max-tokens
  ↓
Adapter: Parse "-1" as i64 → u32 conversion fails
  ↓
Adapter: Return Err("invalid --max-tokens value: -1")
  ↓
Main: eprintln!("Error: invalid --max-tokens value: -1")
  ↓
Main: eprintln!("Run with --help for usage.")
  ↓
Main: std::process::exit(1)
  ↓
User: Error message + exit code 1
```

---

## Integration Points

### Point 1: User → Adapter

**Interface:** CLI argv
**Format:** `Vec<String>`
**Example:** `["--message", "hi", "--dry-run"]`

### Point 2: Adapter → Parser

**Interface:** Unilang tokens
**Format:** `Vec<String>`
**Example:** `[".run", "message::hi", "dry::1"]`

### Point 3: Parser → Semantic

**Interface:** GenericInstruction
**Format:** Struct with command, args
**Example:** `{ command: "run", args: {"message": "hi", "dry": "1"} }`

### Point 4: Semantic → Registry

**Interface:** Command lookup
**Format:** Command name string
**Example:** ".run" → returns command definition + handler

### Point 5: Registry → Handler

**Interface:** Handler routine
**Format:** Closure
**Signature:** `Fn(&VerifiedCommand, &mut ExecutionContext) -> Result<OutputData, ErrorData>`

### Point 6: Handler → Core Library

**Interface:** Builder pattern
**Format:** Method chain
**Example:** `builder.with_message("hi").with_dry(true).describe()`

### Point 7: Core → System

**Interface:** Process spawn
**Format:** Subprocess execution
**Example:** `std::process::Command::new("claude").args([...]).spawn()`

---

## Component Dependencies

### Unilang Dependencies

```
claude_runner
  ├─> unilang (external crate, ~0.48.0)
  │   ├─> parser
  │   ├─> semantic
  │   ├─> interpreter
  │   └─> registry
  └─> error_tools (via unilang re-export)
```

### Core Library Dependency

```
claude_runner
  └─> claude_runner_core (workspace member)
      ├─> error_tools
      └─> (no other deps - thin wrapper)
```

### Error Flow

```
Error sources:
  ├─> Adapter (parse errors)
  ├─> Parser (syntax errors)
  ├─> Semantic (validation errors)
  ├─> Interpreter (handler errors)
  └─> Core Library (execution errors)

All converge to:
  └─> error_tools::Error (user-facing)
  └─> std::process::exit(1) (CLI exit)
```

---

## Performance Characteristics

### Startup Path

```
1. Parse argv (O(n)) — n = number of arguments
2. Build tokens (O(n)) — create token vec
3. Parser invocation (O(n)) — parse tokens
4. Semantic analysis (O(n*m)) — n commands, m args each
5. Handler lookup (O(1)) — single command
6. Handler execution (O(1)) — execute once

Total: O(n + n + n*m) ≈ O(n) for typical use
```

### Memory Profile

```
Static:
  - Command definitions (constant size)
  - Argument definitions (constant size)
  - Handler closures (constant size)

Per-invocation:
  - Token vec: O(n) where n = number of tokens
  - GenericInstruction: O(m) where m = number of arguments
  - VerifiedCommand: O(m)
  - ExecutionContext: minimal (few fields)
  - OutputData: O(o) where o = output size

No persistent state between invocations (current design).
```

---

## Extension Points

### Adding New Commands

**1. Add to adapter routing:**
```rust
match argv[i].as_str() {
    ".mycommand" => return Ok(vec![".mycommand".to_string()]),
    // ... existing
}
```

**2. Register in registry:**
```rust
let cmd = CommandDefinition::new(
    CommandName::new(".mycommand").expect("valid name"),
    "Description".to_string(),
)
.with_arguments(vec![
    // Define arguments
]);

registry.command_add_runtime(&cmd, Box::new(handler))?;
```

**3. Update help text:**
```rust
println!("COMMANDS:");
println!("  .run      Execute Claude Code (default command)");
println!("  .mycommand My custom command");
```

### Adding New Parameters

**1. Add to adapter parsing:**
```rust
match argv[i].as_str() {
    "--myparam" => {
        i += 1;
        let val = argv.get(i)?;
        myparam = Some(val.clone());
    }
    // ... existing
}
```

**2. Add to command definition:**
```rust
ArgumentDefinition::new("myparam", Kind::String)
    .with_description("My new parameter")
    .with_optional(Some("default")),
```

**3. Add to handler:**
```rust
if let Some(Value::String(s)) = cmd.arguments.get("myparam") {
    // Use parameter
}
```

---

## Security Boundaries

### Input Sanitization

**Adapter layer:**
- No arbitrary code execution
- All input parsed before interpretation
- No command injection possible

**Unilang layer:**
- Token-based parsing (no eval)
- Strict type checking
- No implicit value expansion

### Process Execution

**Core library:**
- `std::process::Command` (safe subprocess API)
- Arguments passed as array (no shell injection)
- No shell involvement

---

## Summary

| Layer | Responsibility | Key Types |
|--------|---------------|-------------|
| User Interface | Input/Output | `Vec<String>`, `OutputData` |
| Adapter | Translation | `Vec<String>` → `Vec<String>` |
| Parser | Token parsing | `GenericInstruction` |
| Semantic | Validation | `VerifiedCommand` |
| Registry | Command metadata | `CommandDefinition`, `CommandRegistry` |
| Interpreter | Execution | `CommandRoutine`, `ExecutionContext` |
| Core Library | External execution | `ClaudeCommand`, `ProcessOutput` |

**Total:** 7 layers, 3 integration points (external → unilang, unilang → core, core → system)
