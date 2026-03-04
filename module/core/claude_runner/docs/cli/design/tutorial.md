# CLI Tutorial

**Version:** 1.0.0
**Target Audience:** Developers integrating or extending `claude_runner`
**Prerequisites:** Rust familiarity, basic CLI knowledge
**Time to Complete:** 30-45 minutes (all 3 lessons)

---

## Overview

This tutorial guides you through `claude_runner` CLI architecture, integration patterns, and extensibility mechanisms. By the end, you'll understand:

1. **Lesson 1:** Adapter layer — How `--flag value` maps to unilang
2. **Lesson 2:** Command routing — From user argv to execution
3. **Lesson 3:** Extending the CLI — Adding new commands

**Learning Path:** Each lesson builds on previous concepts. Complete them in order for best results.

---

## Lesson 1: Adapter Layer (10 minutes)

### What You'll Learn

- How `argv_to_unilang_tokens()` converts user input
- Flag parsing logic and parameter extraction
- Token format conversion to unilang syntax

### Concepts

**What is the Adapter Layer?**

The adapter layer translates between user-friendly `--flag value` syntax and unilang's `key::value` format. This allows users to type familiar CLI syntax while leveraging unilang's command routing and validation.

**Adapter Flow:**
```
User Input: claude_runner --message "Fix bug" --dir /project
                ↓ argv_to_unilang_tokens()
Tokens:     [".run", "message::Fix bug", "dir::/project"]
                ↓ Parser → SemanticAnalyzer
Verified:    ClaudeCommand with message="Fix bug", dir="/project"
```

### Exercise 1.1: Basic Flag Parsing

**Goal:** Understand how flags are converted to tokens.

**Code walkthrough:**

```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    let mut message: Option<String> = None;
    let mut dir: Option<String> = None;
    let mut dry = false;
    let mut help = false;

    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "-m" | "--message" => {
                i += 1;
                let val = argv.get(i).ok_or_else(|| Error::msg("--message requires a value"))?;
                message = Some(val.clone());
            }
            "-d" | "--dir" => {
                i += 1;
                let val = argv.get(i).ok_or_else(|| Error::msg("--dir requires a value"))?;
                dir = Some(val.clone());
            }
            "--dry-run" => {
                dry = true;
            }
            "-h" | "--help" => {
                help = true;
            }
            other => {
                if !other.starts_with('-') && message.is_none() {
                    message = Some(other.to_string());
                }
            }
        }
        i += 1;
    }

    // Route based on flags
    if help { return Ok(vec![".help".to_string()]); }

    // Build unilang tokens
    let mut tokens = vec![".run".to_string()];
    if let Some(msg) = message { tokens.insert(1, format!("message::{msg}")); }
    if let Some(d) = dir { tokens.push(format!("dir::{d}")); }
    if dry { tokens.push("dry::1".to_string()); }

    Ok(tokens)
}
```

**What happens?**
1. Sequentially parse each argument
2. Track flag states in local variables
3. Convert to unilang token format (`key::value`)
4. Route to `.help` or `.run` command

### Exercise 1.2: Token Format

**Goal:** Understand unilang token structure.

**Input:**
```bash
claude_runner --message "Fix the bug" --dir /workspace --dry-run
```

**Adapter Output:**
```rust
[
    ".run",                           // Command
    "message::Fix the bug",          // String parameter
    "dir::/workspace",               // Path parameter
    "dry::1"                          // Boolean parameter
]
```

**Parser sees:**
```bash
.run message::Fix the bug dir::/workspace dry::1
```

**SemanticAnalyzer validates:**
- `.run` command exists in registry
- `message`, `dir`, `dry` are valid arguments
- Types match argument definitions

**Checkpoint:** You understand the adapter layer!

---

## Lesson 2: Command Routing (10 minutes)

### What You'll Learn

- How commands are discovered and routed
- The pipeline from argv to execution
- `.help` vs `.run` command handling

### Concepts

**Command Routing Flow:**
```
User argv
    ↓ argv_to_unilang_tokens()
Tokens [".run", "message::hi"]
    ↓ Parser::parse_from_argv()
GenericInstruction
    ↓ SemanticAnalyzer::analyze()
VerifiedCommand
    ↓ Interpreter::run()
Output
```

### Exercise 2.1: Help Command

**Goal:** Understand help command routing.

**Code walkthrough:**

```rust
fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    // Phase 1: Convert to unilang tokens
    let tokens = match argv_to_unilang_tokens(&argv) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Run with --help for usage.");
            std::process::exit(1);
        }
    };

    // Phase 2: Check for help command
    if tokens.first().map(String::as_str) == Some(".help") {
        print_help();
        return;
    }

    // Phase 3: Build registry
    let registry = build_registry();

    // Phase 4: Parse tokens
    let parser = Parser::new(UnilangParserOptions::default());
    let instruction = match parser.parse_from_argv(&tokens) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Run with --help for usage.");
            std::process::exit(1);
        }
    };

    // Phase 5: Semantic analysis
    let instructions = [instruction];
    let analyzer = SemanticAnalyzer::new(&instructions, &registry);
    let commands = match analyzer.analyze() {
        Ok(cmds) => cmds,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    // Phase 6: Execute
    let interpreter = Interpreter::new(&commands, &registry);
    let mut ctx = ExecutionContext::default();
    match interpreter.run(&mut ctx) {
        Ok(outputs) => {
            for output in outputs {
                print!("{}", output.content);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
```

**What happens with `.help`?**
1. Adapter detects `--help` flag → returns `[.help]`
2. First token check finds `.help` → calls `print_help()`
3. Function returns early (no parsing, no execution)

**What happens with `.run`?**
1. Adapter detects no `--help` → returns `[.run, message::hi]`
2. First token check doesn't match `.help` → continues
3. Registry built, parsing proceeds
4. Execution flows through complete pipeline

### Exercise 2.2: Command Registry

**Goal:** Understand how commands are registered.

**Code walkthrough:**

```rust
fn build_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    let run_def = CommandDefinition::new(
        CommandName::new(".run").expect("valid command name"),
        "Execute Claude Code with configurable parameters".to_string(),
    )
    .with_arguments(vec![
        ArgumentDefinition::new("message", Kind::String)
            .with_optional(None::<String>),
        ArgumentDefinition::new("dir", Kind::String)
            .with_optional(None::<String>),
        ArgumentDefinition::new("dry", Kind::Boolean)
            .with_optional(None::<String>),
        // ... all 9 arguments
    ]);

    let run_routine: CommandRoutine = Box::new(|cmd, _ctx| {
        // Extract parameters from cmd.arguments
        // Build ClaudeCommand
        // Execute and return OutputData
    });

    registry.command_add_runtime(&run_def, run_routine)
        .expect("internal error: failed to register .run command");

    registry
}
```

**What happens?**
1. Create `CommandDefinition` with metadata
2. Define all arguments with types
3. Create handler routine (closure)
4. Register with `registry.command_add_runtime()`
5. Unilang now knows about `.run` command

**Checkpoint:** You understand command routing!

---

## Lesson 3: Extending the CLI (15 minutes)

### What You'll Learn

- How to add new commands
- Command definition patterns
- Handler implementation best practices

### Concepts

**Current Commands:**
- `.help` — Print usage (special case)
- `.run` — Execute Claude Code (primary command)

### Exercise 3.1: Adding a Status Command

**Goal:** Add `.status` command to show current state.

**Step 1: Define command in adapter:**

```rust
// In argv_to_unilang_tokens()
match argv[i].as_str() {
    "--help" | "-h" => { help = true; }
    ".status" => {
        // New explicit command
        return Ok(vec![".status".to_string()]);
    }
    // ... rest of parsing
}
```

**Step 2: Register command:**

```rust
fn build_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // Existing .run command
    register_run_command(&mut registry)?;

    // New .status command
    let status_def = CommandDefinition::new(
        CommandName::new(".status").expect("valid command name"),
        "Show current session status".to_string(),
    )
    .with_arguments(vec![
        ArgumentDefinition::new("verbosity", Kind::Integer)
            .with_description("Output verbosity level (0-5)")
            .with_optional(Some("1")),
    ]);

    let status_routine: CommandRoutine = Box::new(|cmd, _ctx| {
        let verbosity = cmd.arguments.get("verbosity")
            .and_then(|v| if let Value::Integer(n) = v { Some(n) } else { None })
            .unwrap_or(&1);

        let output = if *verbosity >= 2 {
            "Status:\n  Session: Active\n  Working dir: /project\n  Last command: --message \"hi\""
        } else {
            "Active session in /project"
        };

        Ok(OutputData {
            content: output.to_string(),
            format: "text".to_string(),
            execution_time_ms: None,
        })
    });

    registry.command_add_runtime(&status_def, status_routine)
        .expect("internal error: failed to register .status command");

    registry
}
```

**Step 3: Update help text:**

```rust
fn print_help() {
    println!("claude_runner — Execute Claude Code with configurable parameters");
    println!();
    println!("USAGE:");
    println!("  claude_runner [COMMAND] [OPTIONS] [MESSAGE]");
    println!();
    println!("COMMANDS:");
    println!("  .help    Print this help");
    println!("  .run      Execute Claude Code (default command)");
    println!("  .status   Show current session status");
    println!();
    println!("OPTIONS (for .run, .status):");
    println!("  -v, --verbose              Print command to stderr, then execute");
    // ... rest of options
}
```

**Usage:**
```bash
$ claude_runner .status
Active session in /project

$ claude_runner .status verbosity::2
Status:
  Session: Active
  Working dir: /project
  Last command: --message "hi"
```

### Exercise 3.2: Adding a List Command

**Goal:** Add `.list` command to show available sessions.

**Command definition:**

```rust
let list_def = CommandDefinition::new(
    CommandName::new(".list").expect("valid command name"),
    "List available sessions".to_string(),
)
.with_arguments(vec![
    ArgumentDefinition::new("format", Kind::String)
        .with_description("Output format (json, table)")
        .with_optional(Some("table")),
]);

let list_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    let format = cmd.arguments.get("format")
        .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
        .unwrap_or("table");

    // Load sessions from filesystem
    // Format output
    Ok(OutputData {
        content: formatted_output,
        format: "text".to_string(),
        execution_time_ms: None,
    })
});

registry.command_add_runtime(&list_def, list_routine)?;
```

**Usage:**
```bash
$ claude_runner .list
Available sessions:
  project      2024-03-03 14:30  Active
  staging      2024-03-03 10:15
  production   2024-03-01 09:00

$ claude_runner .list format::json
[{"name":"project","created":"2024-03-03T14:30:00Z","active":true},...]
```

### Exercise 3.3: Adding Namespace Commands

**Goal:** Add session management commands under `.session` namespace.

**Commands:**
```bash
claude_runner .session.new name::"my-session"
claude_runner .session.switch name::"prod"
claude_runner .session.save path::"session.json"
```

**Registration:**

```rust
// In argv_to_unilang_tokens()
match argv[i].as_str() {
    ".session.new" => {
        return Ok(vec![".session.new".to_string()]);
    }
    ".session.switch" => {
        return Ok(vec![".session.switch".to_string()]);
    }
    // ... other session commands
}

// In build_registry()
let session_new_def = CommandDefinition::new(
    CommandName::new(".session.new").expect("valid command name"),
    "Create new session".to_string(),
)
.with_arguments(vec![
    ArgumentDefinition::new("name", Kind::String)
        .with_description("Session name"),
]);

registry.command_add_runtime(&session_new_def, session_new_handler)?;
```

**Handler:**

```rust
fn session_new_handler(cmd: &VerifiedCommand, _ctx: -> Result<OutputData, ErrorData> {
    let name = cmd.arguments.get("name")
        .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
        .ok_or_else(|| ErrorData::new(
            ErrorCode::ValidationError,
            "name parameter is required".to_string(),
        ))?;

    // Create session logic
    Ok(OutputData {
        content: format!("Session '{name}' created").to_string(),
        format: "text".to_string(),
        execution_time_ms: None,
    })
}
```

**Checkpoint:** You can extend the CLI!

---

## Summary

### Key Patterns Learned

1. **Adapter Layer**: Converts user `--flag value` to unilang `key::value`
2. **Command Registry**: Runtime command registration with handlers
3. **Routing Logic**: Explicit vs implicit command selection
4. **Extensibility**: New commands added via registry registration

### Architecture Benefits

| Aspect | Benefit |
|---------|----------|
| **User Familiarity** | `--flag value` syntax matches other CLIs |
| **Unilang Power** | Semantic validation, type safety, extensibility |
| **Clear Separation** | Adapter isolates user-facing concerns from framework |
| **Future-Proof** | Easy to add `.status`, `.list`, `.session.*` commands |

### Next Steps

- Review [Command Design](command_design.md) for CLI design recommendations
- See [Unilang Exploration](unilang_exploration.md) for framework details
- Check [Testing Documentation](../cli/testing/readme.md) for test patterns
- Explore genfile CLI for reference implementation

---

**Tutorial Complete!** You now understand:
- Adapter layer mechanics and token conversion
- Command routing through unilang pipeline
- How to extend the CLI with new commands
