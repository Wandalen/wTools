# CLI Best Practices

Recommended patterns, guidelines, and anti-patterns for `claude_runner` CLI development and extension.

## Core Principles

### 1. Semantic Clarity Over Brevity

**Principle:** Command and parameter names should clearly indicate their purpose, even if slightly longer.

**Good:**
```bash
claude_runner .run --message "Fix the bug"
```

**Avoid:**
```bash
# Too terse
claude_runner -m "Fix"          # What does -m mean?
```

**Rationale:** Clear names reduce cognitive load and documentation needs.

### 2. Consistent Naming Conventions

**Principle:** Follow established patterns across all commands and parameters.

**Patterns:**
- Commands: Dot-prefixed, noun-verb or verb (`.help`, `.run`, `.status`)
- Parameters: Kebab-case, double-word (e.g., `--max-tokens`, `--skip-permissions`)
- Short flags: Single letter for common options (`-m`, `-d`, `-c`, `-v`)

**Good:**
```bash
claude_runner --max-tokens 50000
claude_runner --session-dir /path
```

**Avoid:**
```bash
# Inconsistent casing
claude_runner --MaxTokens 50000
claude_runner --session_dir /path

# Inconsistent separator
claude_runner --max_tokens 50000
claude_runner --session-dir /path
```

### 3. Backward Compatibility With Deprecation Path

**Principle:** Never introduce breaking changes without a clear migration path and deprecation period.

**Good:**
```rust
// Phase 1: Add new feature, keep old
fn parse_with_compat(argv: &[String]) -> Result<Vec<String>> {
    if argv.contains(&"--help") {
        // Old behavior
        return Ok(vec![".help".to_string()]);
    }
    if argv.contains(&".help") {
        // New behavior
        return Ok(vec![".help".to_string()]);
    }
    // ... rest of parsing
}

// Phase 2: Add deprecation notice
if contains_deprecated_flag(&argv) {
    eprintln!("DEPRECATION: --help is deprecated. Use '.help' instead.");
}
```

**Avoid:**
```rust
// Breaking change without migration
fn parse_new_only(argv: &[String]) -> Result<Vec<String>> {
    // --help no longer works!
    match argv.first() {
        Some("--help") => Err(Error::msg("unknown argument")),
        _ => { /* ... */ }
    }
}
```

### 4. Fail Fast With Clear Messages

**Principle:** Detect errors early and provide actionable, specific error messages.

**Good:**
```rust
if let Some(val) = argv.get(i) {
    if val.parse::<u32>().is_err() {
        return Err(Error::msg(
            format!("invalid --max-tokens value: {val} (must be positive integer)")
        ));
    }
}
```

**Avoid:**
```rust
// Vague error
if let Some(val) = argv.get(i) {
    if val.parse::<u32>().is_err() {
        return Err(Error::msg("Error parsing value"));
    }
}

// Error at wrong level (late detection)
parse_all_arguments(&argv)?;
build_command(&args)?;
execute_command(&cmd)?;  // Error should have been caught earlier
```

### 5. Composable Commands and Parameters

**Principle:** Design commands and parameters to work together seamlessly.

**Good:**
```bash
# Commands compose cleanly
claude_runner .run --message "Fix bug" --dry-run

# Parameters don't interfere
claude_runner --verbose --dry-run
```

**Avoid:**
```rust
// Commands that conflict
if has_message_flag && has_message_positional() {
    return Err(Error::msg("Cannot specify message both ways"));
}

// Parameters with hidden dependencies
if has_dry_flag && !has_verbose_flag {
    // Dry-run implies verbose output but not flagged
    // User may not understand expected behavior
}
```

### 6. Type Safety at CLI Boundary

**Principle:** Validate types early, convert safely, provide clear error on type mismatch.

**Good:**
```rust
// Parse with type validation
let max_tokens: u32 = argv.get(i)
    .ok_or_else(|| Error::msg("--max-tokens requires a value"))?
    .and_then(|v| v.parse::<u32>()
    .map_err(|_| Error::msg(format!("invalid --max-tokens value: {v}")))?;

// Use validated type
builder.with_max_output_tokens(max_tokens);
```

**Avoid:**
```rust
// Unsafe conversion
let max_tokens: u32 = argv.get(i)
    .unwrap()
    .parse()
    .unwrap();  // May panic on invalid input

// Late type checking
let builder = ClaudeCommand::new();
let tokens: Vec<i64> = parse_all_integers(&argv)?;
builder.with_max_output_tokens(tokens[0] as u32);  // Could overflow
```

### 7. Help is Documentation

**Principle:** Help text should be comprehensive enough to serve as primary documentation.

**Good:**
```bash
$ claude_runner .help
claude_runner — Execute Claude Code with configurable parameters

USAGE:
  claude_runner [COMMAND] [OPTIONS] [MESSAGE]

COMMANDS:
  .help    Print this help
  .run      Execute Claude Code (default command)

OPTIONS (for .run):
  -m, --message <MSG>        Prompt message for Claude
  -d, --dir <PATH>           Working directory
  ...

EXAMPLES:
  claude_runner --message "Fix the bug" --dir /project
  claude_runner .run --message "Fix the bug" --dir /project
```

**Avoid:**
```bash
# Minimal help (no examples, no context)
$ claude_runner .help
USAGE: claude_runner [OPTIONS]
OPTIONS: -m, -d, -c, --max-tokens, ...

# Vague parameter descriptions
  -m   message
  -d   directory
```

---

## Command Design Patterns

### Pattern 1: Single Responsibility Commands

**Definition:** Each command should do one thing well.

**Good:**
```bash
claude_runner .run                    # Execute Claude
claude_runner .status                # Show status
claude_runner .list                  # List sessions
```

**Avoid:**
```bash
# Multiple responsibilities
claude_runner .execute              # Does what? Run? Status? List?
claude_runner .manage               # Manage what? Config? Sessions? Files?
```

**Rationale:** Focused commands are easier to understand, test, and extend.

### Pattern 2: Verb-First or Action-First Commands

**Definition:** Commands should start with a verb indicating the action.

**Good:**
```bash
.run          # Run (execute)
.list          # List (show)
.add           # Add (create)
.remove        # Remove (delete)
.switch        # Switch (select)
.load          # Load (read)
.save          # Save (write)
.export        # Export (output)
.import        # Import (input)
```

**Avoid:**
```bash
# Noun-first (less clear)
.session        # What to do with session?
.config         # What to do with config?
.setting        # What to do with setting?
```

**Exception:** `.status` is acceptable as a query command.

### Pattern 3: Logical Parameter Grouping

**Definition:** Related parameters should share a prefix or be grouped in documentation.

**Good:**
```bash
# Session-related parameters
claude_runner .session.new name::"project"
claude_runner .session.save path::"session.json"
claude_runner .session.load path::"session.json"
claude_runner .session.switch name::"prod"

# Or shared prefix
claude_runner --session-new "project"
claude_runner --session-save "session.json"
claude_runner --session-load "session.json"
```

**Avoid:**
```bash
# Unrelated parameter names
claude_runner .create name::"session"
claude_runner .save path::"session.json"    # What's being saved?
claude_runner .load path::"session.json"    # What's being loaded?
```

### Pattern 4: Optional Parameters Have Sensible Defaults

**Definition:** When a parameter is optional, provide a useful default.

**Good:**
```bash
# Automation-friendly defaults
claude_runner .run --max-tokens 50000
# Default: 200000 (reasonable for most use)

# Boolean flags default to false
claude_runner .run --verbose
# Default: false (quiet mode)
```

**Avoid:**
```bash
# No default (forces users to specify always)
claude_runner .run --max-tokens REQUIRED

# Default requires understanding
claude_runner .run --mode some-complex-value
# What does "some-complex-value" mean? Users must read docs.
```

---

## Parameter Design Patterns

### Pattern 1: Kebab-Case for Long Flags

**Definition:** Multi-word flags use hyphens between words.

**Good:**
```bash
--max-tokens
--skip-permissions
--session-dir
```

**Avoid:**
```bash
--max_tokens          # Underscore
--skipPermissions      # CamelCase
--sessiondir          # Lowercase concat
```

### Pattern 2: Consistent Short Flags

**Definition:** Short flags use first letter of primary word; avoid conflicts.

**Good:**
```bash
-m  # --message
-d  # --dir
-c  # --continue
-v  # --verbose
```

**Avoid:**
```bash
-msg   # Redundant
-mesg  # Not obvious
-dm    # Wrong order
```

### Pattern 3: Boolean Flags Are Idempotent

**Definition:** Specifying a boolean flag multiple times should have no additional effect.

**Good:**
```bash
# Specifying once
claude_runner --verbose

# Specifying twice (same effect)
claude_runner --verbose --verbose

# No error, no warning (idempotent)
```

**Avoid:**
```bash
# Duplicate flag error
claude_runner --verbose --verbose
# Error: --verbose specified multiple times

# Cumulative effect
claude_runner --verbose --verbose --verbose
# Verbosity level 3 (unexpected behavior)
```

### Pattern 4: Last-Wins for Value Parameters

**Definition:** When a value parameter is specified multiple times, the last value wins.

**Good:**
```bash
# Last value used
claude_runner --dir /first --dir /second
# Working directory: /second

# No error or warning
```

**Avoid:**
```bash
# First wins (unexpected)
claude_runner --dir /first --dir /second
# Working directory: /first (users may not expect)

# Error on duplicate
claude_runner --dir /first --dir /second
# Error: --dir specified multiple times
```

### Pattern 5: Conflict Detection for Mutually Exclusive Options

**Definition:** Detect and error on mutually exclusive parameter combinations.

**Good:**
```rust
if has_positional_message && has_message_flag {
    return Err(Error::msg(
        "Cannot specify message both as positional argument and --message flag"
    ));
}
```

**Avoid:**
```rust
// No validation
// User specifies both, unexpected behavior
parse_positional_and_flag();  // Both used, last wins silently

// Silent priority
parse_all();
// Positional ignored if flag present (users confused)
```

---

## Error Handling Patterns

### Pattern 1: Specific Error Messages

**Definition:** Error messages should name the specific problem, not just say "error".

**Good:**
```rust
Err(Error::msg(
    format!("invalid --max-tokens value: {val} (must be positive integer)")
))

Err(Error::msg(
    format!("unknown command: {cmd} (valid commands: .help, .run)")
))
```

**Avoid:**
```rust
Err(Error::msg("Invalid value"))

Err(Error::msg("Unknown command"))

Err(Error::msg("Error"))
```

### Pattern 2: Include Recovery Suggestion

**Definition:** When possible, suggest how to fix the error.

**Good:**
```rust
Err(Error::msg(
    format!("invalid --max-tokens value: {val} (must be 1-4294967295)")
))

Err(Error::msg(
    format!("unknown command: {cmd} (run '.help' for available commands)")
))
```

**Avoid:**
```rust
Err(Error::msg("Invalid value"))

Err(Error::msg("Unknown command (run '.help')"))
```

### Pattern 3: Contextual Help References

**Definition:** Include help command reference in error messages.

**Good:**
```rust
Err(Error::msg("Run with '.help' for usage information"))
```

**Avoid:**
```rust
Err(Error::msg("See documentation"))

Err(Error::msg("Run --help"))  // Deprecated
```

---

## Testing Patterns

### Pattern 1: Test All Error Paths

**Definition:** Every error condition should have a corresponding test.

**Good:**
```rust
#[test]
fn missing_required_parameter() {
    let out = run_cli(&["--max-tokens"]);
    assert!(!out.status.success());
    assert!(out.stderr.contains("requires a value"));
}

#[test]
fn invalid_integer_value() {
    let out = run_cli(&["--max-tokens", "-1"]);
    assert!(!out.status.success());
    assert!(out.stderr.contains("invalid --max-tokens value"));
}

#[test]
fn unknown_command() {
    let out = run_cli(&[".unknown"]);
    assert!(!out.status.success());
    assert!(out.stderr.contains("unknown command"));
}
```

**Avoid:**
```rust
// Only testing happy path
#[test]
fn happy_path_only() {
    let out = run_cli(&["--message", "test"]);
    assert!(out.status.success());
}
```

### Pattern 2: Idempotency Tests

**Definition:** Verify that operations produce the same result when repeated.

**Good:**
```rust
#[test]
fn boolean_flag_duplicate_is_idempotent() {
    let out1 = run_cli(&["--continue", "--continue"]);
    let out2 = run_cli(&["--continue"]);
    assert_eq!(out1.stdout, out2.stdout, "Duplicate flag should be idempotent");
}
```

### Pattern 3: Boundary Value Tests

**Definition:** Test at type boundaries and edge cases.

**Good:**
```rust
#[test]
fn max_tokens_boundary_tests() {
    // Valid: minimum
    assert!(run_cli(&["--max-tokens", "1"]).is_success());

    // Valid: typical
    assert!(run_cli(&["--max-tokens", "50000"]).is_success());

    // Valid: maximum
    assert!(run_cli(&["--max-tokens", "4294967295"]).is_success());

    // Invalid: negative
    assert!(!run_cli(&["--max-tokens", "-1"]).is_success());

    // Invalid: overflow
    assert!(!run_cli(&["--max-tokens", "4294967296"]).is_success());
}
```

---

## Anti-Patterns

### Anti-Pattern 1: Implicit State

**Problem:** Commands have hidden state that affects subsequent invocations.

**Example:**
```rust
// Bad: Global mutable state
static mut CURRENT_DIR: Option<String> = None;

fn cd_handler(cmd: &VerifiedCommand) {
    if let Some(dir) = extract_dir(cmd) {
        CURRENT_DIR = Some(dir);  // Hidden state
    }
}

fn run_handler(cmd: &VerifiedCommand) {
    if let Some(ref dir) = CURRENT_DIR {
        // Uses hidden state
        // User may not expect this behavior
    }
}
```

**Solution:** Either make state explicit or avoid hidden state.

### Anti-Pattern 2: Magic Numbers

**Problem:** Unclear numeric constants without semantic names.

**Example:**
```rust
// Bad: Magic number
if n > 1000 {
    return Err(Error::msg("Value too large"));
}
```

**Solution:** Use named constants.

```rust
// Good: Named constant
const MAX_MESSAGE_LENGTH: usize = 1000;

if n > MAX_MESSAGE_LENGTH {
    return Err(Error::msg(
        format!("Message too long (max: {MAX_MESSAGE_LENGTH} characters)")
    ));
}
```

### Anti-Pattern 3: Silent Failures

**Problem:** Operations fail without clear error messages.

**Example:**
```rust
// Bad: Silent failure
if invalid_input(&input) {
    return Ok(OutputData { content: "".to_string(), ... });
}

// Or return empty error
return Err(ErrorData::new(ErrorCode::InternalError, "".to_string()));
```

**Solution:** Always provide clear error messages.

### Anti-Pattern 4: Inconsistent Exit Codes

**Problem:** Different errors use the same exit code.

**Example:**
```rust
// Bad: Always exit 1
match error_type {
    ParseError => std::process::exit(1),
    ValidationError => std::process::exit(1),
    ExecutionError => std::process::exit(1),
}
```

**Solution:** Use distinct exit codes or categorization.

```rust
// Good: Categorize exit codes
match error_type {
    ParseError => std::process::exit(2),        // Argument error
    ValidationError => std::process::exit(2),     // Validation error
    ExecutionError => std::process::exit(3),    // Runtime error
}
```

### Anti-Pattern 5: Overly Permissive Parsing

**Problem:** Accepting invalid input without error, leading to unexpected behavior.

**Example:**
```rust
// Bad: Accepts anything
if token.starts_with("--") {
    value = Some(rest_of_token);  // No validation
}
```

**Solution:** Validate values.

```rust
// Good: Validate against known patterns
if token.starts_with("--") {
    value = Some(rest_of_token);
    if !is_valid_value(&value) {
        return Err(Error::msg(format!("invalid value for {token}: {value}")));
    }
}
```

---

## Documentation Patterns

### Pattern 1: Examples in Help

**Definition:** Include concrete examples in help text for common use cases.

**Good:**
```bash
$ claude_runner .help
...
EXAMPLES:
  # Basic usage
  claude_runner --message "Fix the bug"

  # With directory
  claude_runner --message "Fix the bug" --dir /project

  # Dry run preview
  claude_runner --message "Fix the bug" --dry-run

  # Explicit command
  claude_runner .run --message "Fix the bug"
```

### Pattern 2: Cross-Reference Documentation

**Definition:** Help text should reference detailed documentation where appropriate.

**Good:**
```bash
$ claude_runner .help
...
For complete documentation, see:
  https://github.com/...

OPTIONS:
  --max-tokens <N>       See: docs/parameters.md#max-tokens
  --session-dir <PATH>   See: docs/parameters.md#session-dir
```

---

## Performance Patterns

### Pattern 1: Lazy Evaluation

**Definition:** Don't do expensive work until necessary.

**Example:**
```rust
// Good: Check help flag early
if argv.contains(&"--help") {
    print_help();
    return;  // Exit before any expensive parsing
}

// Bad: Parse everything first
let tokens = parse_all_arguments(&argv);  // Expensive!
if tokens.contains(&".help") {
    print_help();
}
```

### Pattern 2: Minimal Allocations

**Definition:** Avoid unnecessary heap allocations in hot paths.

**Example:**
```rust
// Good: Single allocation
let mut tokens = Vec::with_capacity(argv.len());
tokens.push(".run".to_string());
for token in argv {
    tokens.push(token.clone());
}

// Bad: Many allocations
let mut tokens: vec![];
for token in argv {
    tokens = vec![tokens.clone(), token.clone()];  // Reallocates each iteration
}
```

---

## Security Patterns

### Pattern 1: No Shell Injection

**Definition:** Use process spawning APIs that don't involve shell interpretation.

**Good:**
```rust
// Good: std::process::Command (no shell)
let cmd = std::process::Command::new("claude");
cmd.args(vec![message, dir]);
cmd.spawn()?;
```

**Avoid:**
```rust
// Bad: shell execution (vulnerable)
use std::process::Command;
cmd.arg(format!("claude --message \"{message}\""));  // Shell interpolation possible
```

### Pattern 2: Validate Paths

**Definition:** Validate file paths before use.

**Good:**
```rust
let path = get_path_arg()?;
if !Path::new(&path).exists() {
    return Err(Error::msg(format!("Path does not exist: {path}")));
}
```

---

## Checklist for New Commands

### Before Implementation

- [ ] Command name follows dot-prefix convention
- [ ] Command is verb-first or action-first
- [ ] Single responsibility clearly defined
- [ ] Parameters grouped logically
- [ ] Sensible defaults for optional parameters
- [ ] Conflict detection for mutually exclusive options

### During Implementation

- [ ] Clear error messages for all failure modes
- [ ] Help text includes command description
- [ ] Help text includes at least one example
- [ ] Type validation at CLI boundary
- [ ] Tests for happy path
- [ ] Tests for all error paths
- [ ] Tests for boundary values
- [ ] Tests for idempotency (if applicable)

### After Implementation

- [ ] Documentation updated
- [ ] Cross-references added between docs
- [ ] Release notes updated
- [ ] Backward compatibility verified (if needed)
- [ ] Deprecation path documented (if breaking change)

---

## Summary

| Category | Key Principle |
|----------|----------------|
| Naming | Semantic clarity over brevity, consistent conventions |
| Compatibility | Graceful deprecation with migration path |
| Errors | Fail fast with specific, actionable messages |
| Design | Single responsibility, type safety at boundary |
| Parameters | Kebab-case, logical grouping, sensible defaults |
| Testing | All error paths, boundaries, idempotency |
| Documentation | Examples, cross-references |
| Performance | Lazy evaluation, minimal allocations |
| Security | No shell injection, validate paths |

---

## References

- [API Reference](api_reference.md) — Complete API documentation
- [Migration Guide](migration_guide.md) — Step-by-step migration path
- [Command Design](command_design.md) — Design recommendations
- [Architecture](architecture.md) — System overview
- [Tutorial](tutorial.md) — Hands-on lessons
