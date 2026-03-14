# CLI Design Recommendations

Comprehensive analysis of CLI design patterns and recommendations for `claude_runner` based on unilang framework conventions and industry best practices.

## Current Design Analysis

### Architecture Overview

```
User: claude_runner --message "hi" --dir /path --dry-run
         ↓ argv_to_unilang_tokens()
Tokens: [".run", "message::hi", "dir::/path", "dry::1"]
         ↓ Parser → SemanticAnalyzer → Interpreter
Execution: ClaudeCommand::execute()
```

### Current Command Routing (Implicit)

| Trigger | Command | Mechanism |
|---------|----------|------------|
| `--help` or `-h` | `.help` | Flag sets help=true, routes after parse |
| Anything else | `.run` | Default implicit route |

### Current Parameter Syntax

```
-m, --message <MSG>      - Prompt message
-d, --dir <PATH>         - Working directory
-c, --continue            - Continue conversation
    --max-tokens <N>      - Max output tokens
    --skip-permissions    - Skip tool prompts
    --dry-run             - Print without execute
-v, --verbose             - Print to stderr, then execute
    --session-dir <PATH>   - Session directory
    --model <NAME>         - Claude model
-h, --help               - Show help
```

---

## Design Problems

### 1. Implicit Command Routing

**Issue**: `--help` is a **command selector** not a parameter, but parsed as a flag
**Impact**:
- User sees flags and implicit behavior; no first-class command concept
- Not consistent with CLI conventions (`git status`, `npm run`, `docker build`)
- Confusing semantics: help as "flag" but functions as "command"

### 2. `--help` Semantic Mismatch

**Issue**: `--help` takes priority over `.run` but is parsed alongside parameters
**Impact**:
- Mixed concerns: command selection (`--help`) vs configuration (`--dir`, `--message`)
- Help is orthogonal to other parameters, but routing treats them equally
- Sequential parse order matters: `--unknown --help` errors before help reaches

### 3. Extensibility Friction

**Issue**: Adding new commands requires special routing logic in `argv_to_unilang_tokens()`
**Impact**:
- Every new command needs its own flag-to-command mapping
- The "implicit default" pattern becomes confusing with >2 commands
- Routing logic becomes increasingly complex

### 4. Ambiguity in Token Format

**Issue**: Internal tokens use `.run`, `.help` (dot-prefixed) but user interface doesn't
**Impact**:
- Disconnect between internal model and user-facing API
- Users see `--help` but code uses `.help` command
- Debugging: confusing to map user input to internal tokens

---

## Recommended Design: Dot-Prefixed Explicit Commands

### Command Routing (Explicit + Backward Compatible)

```bash
# Explicit dot-prefixed command (new)
claude_runner .help
claude_runner .run --message "hi"

# Implicit default .run (backward compatible)
claude_runner --message "hi"          # .run assumed
claude_runner --dir /path --dry-run   # .run assumed
```

### Routing Rules

| Condition | Action |
|-----------|--------|
| First token starts with `.` | Explicit command routing |
| Token is `.help` | Help mode only (no parameters) |
| Token is `.run` | Run mode with remaining parameters |
| No dot token | Check for `--help` flag (Phase 1) → else `.run` default (Phase 2+) |

### Command Table

| Command | Syntax | Purpose |
|---------|---------|----------|
| `.help` | `claude_runner .help` | Print usage and exit |
| `.run` | `claude_runner .run [OPTIONS] [MESSAGE]` | Execute Claude (default) |

---

## Proposed Help Text

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
  -d, --dir <PATH>           Working directory (default: current dir)
  -c, --continue             Continue existing conversation
      --max-tokens <N>       Max output tokens (default: 200000)
      --skip-permissions     Skip tool permission prompts
      --dry-run              Print command without executing
  -v, --verbose              Print command to stderr, then execute
      --session-dir <PATH>   Session storage directory
      --model <NAME>         Claude model to use

EXAMPLES:
  # Implicit .run (backward compatible)
  claude_runner --message "Fix the bug" --dir /project

  # Explicit .run (future-proof)
  claude_runner .run --message "Fix the bug" --dir /project

  # Help
  claude_runner .help
```

---

## Implementation Changes Required

### 1. `argv_to_unilang_tokens()` — New Routing Logic

```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    // Phase 1: Check for explicit dot command
    for token in argv {
        if token.starts_with('.') {
            return match token.as_str() {
                ".help" => Ok(vec![".help".to_string()]),
                ".run" => {
                    let mut tokens = vec![".run".to_string()];
                    // Parse remaining tokens as parameters
                    parse_parameters(&argv, &mut tokens, token)?;
                    Ok(tokens)
                }
                other => Err(Error::msg(format!("unknown command: {other}"))),
            };
        }
    }

    // Phase 2: No explicit command → implicit routing
    parse_implicit_routing(argv)
}
```

### 2. Remove `--help` Flag (Phase 2)

```rust
// DELETE these lines from the match:
"-h" | "--help" => { help = true; }

// DELETE help=true state variable
// let mut help = false;

// DELETE help routing at end
// if help { return Ok(vec![".help".to_string()]); }
```

### 3. Update `print_help()`

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
    println!();
    println!("OPTIONS (for .run):");
    println!("  -m, --message <MSG>        Prompt message for Claude");
    println!("  -d, --dir <PATH>           Working directory (default: current dir)");
    println!("  -c, --continue             Continue existing conversation");
    println!("      --max-tokens <N>       Max output tokens (default: 200000)");
    println!("      --skip-permissions     Skip tool permission prompts");
    println!("      --dry-run              Print command without executing");
    println!("  -v, --verbose              Print command to stderr, then execute");
    println!("      --session-dir <PATH>   Session storage directory");
    println!("      --model <NAME>         Claude model to use");
    println!();
    println!("EXAMPLES:");
    println!("  # Implicit .run (backward compatible)");
    println!("  claude_runner --message \"Fix the bug\" --dir /project");
    println!();
    println!("  # Explicit .run (future-proof)");
    println!("  claude_runner .run --message \"Fix the bug\" --dir /project");
    println!();
    println!("  # Help");
    println!("  claude_runner .help");
}
```

### 4. Test Updates

**Replace `--help` tests with `.help` tests:**
- `--help` exits with code 0 → `.help` exits with code 0
- `--help` prints usage → `.help` prints usage

**Add explicit `.run` invocation tests:**
- `.run --message "hi"` works
- `.run` with parameters correctly parses

**Update help output verification:**
- COMMANDS section present
- `.help` and `.run` both documented
- Examples show both implicit and explicit forms

---

## Migration Path

### Phase 1: Dual Support (Backward Compatible)

**Goal**: Add explicit commands while keeping `--help` flag

**Changes:**
- Support both `.help` command AND `--help` flag
- Support both `.run` command AND implicit `.run`
- Update help text to show COMMANDS section
- Deprecate `--help` flag (add deprecation notice)

**User Impact**: Zero breaking changes; users can transition gradually

### Phase 2: Deprecation (Next Release)

**Goal**: Mark `--help` flag as deprecated, guide to `.help`

**Changes:**
- Add deprecation notice when `--help` is used
- Update all documentation to prefer `.help`
- Add migration guide

**User Impact**: Warning encourages migration; still works

### Phase 3: Removal (Following Release)

**Goal**: Remove `--help` flag entirely, enforce dot commands

**Changes:**
- Remove `--help` flag from parsing
- Remove `-h` short form
- Update all docs to use `.help` only
- `.run` optional (can be explicit or implicit)

**User Impact**: Breaking change; migration path already provided

---

## Pros vs Cons

### Recommended Design (Explicit Dot Commands)

**Pros:**
- ✅ **Clear separation**: Commands (`.`) vs parameters (`--`)
- ✅ **Extensibility**: New commands added cleanly (`.status`, `.list`, etc.)
- ✅ **Semantic clarity**: No more "help flag" confusion
- ✅ **Convention aligned**: Matches unilang patterns (genfile)
- ✅ **Explicit when needed**: Scripts can use `.run` for clarity
- ✅ **Brevity preserved**: `.run` optional for common use

**Cons:**
- ⚠️ **Breaking change**: `--help` flag removed (migration path available)
- ⚠️ **Docs update**: All docs need command section update
- ⚠️ **Test changes**: Help tests need update

---

## Alternative: Keep `--help` Flag (Minimal Change)

### Dual Support Pattern

```bash
claude_runner --help           # Still works (backward compat)
claude_runner .help            # New explicit form
claude_runner --message "hi"   # Still works (implicit .run)
claude_runner .run --message "hi"  # New explicit form
```

### Routing Logic

```
If first arg is `.help` → help command
If first arg is `.run` → run command (and don't look for --help)
Else: check for --help flag → help (backward compat)
Else: .run with parameters (default)
```

**Tradeoffs:**
- Preserves backward compatibility
- Enables new extensibility
- Users can transition gradually
- Slightly more complex routing logic

---

## Design Decision Framework

### Criteria for Choosing Approach

| Criteria | Explicit Commands | Dual Support | Status Quo |
|----------|------------------|--------------|-------------|
| **Breaking changes** | High (Phase 3) | Low | None |
| **Backward compat** | Low (Phase 1+2) | High | High |
| **Extensibility** | Excellent | Good | Poor |
| **Semantic clarity** | Excellent | Good | Poor |
| **Migration effort** | Medium (3 phases) | Low | None |
| **Future maintenance** | Low | Low | Medium |

### Recommendation: **Phase 1 → Phase 2 → Phase 3**

1. **Immediate (Phase 1)**: Add explicit `.help`/`.run` support while keeping `--help` flag
2. **Next release (Phase 2)**: Deprecate `--help` flag in help text
3. **Following release (Phase 3)**: Remove `--help` flag entirely

This gives you the best of both worlds: immediate extensibility + graceful migration path.

---

## Future Command Examples

With dot-prefixed explicit commands, future extensibility becomes clean:

```bash
# Session management
claude_runner .status              # Show current session status
claude_runner .list                 # List available sessions
claude_runner .switch name::"prod"  # Switch to session

# Configuration
claude_runner .config set key::"model" value::"claude-opus-4-6"
claude_runner .config get key::"model"
claude_runner .config list

# Export/Import
claude_runner .export format::"json" output::"session.json"
claude_runner .import file::"session.json"
```

All commands follow the same pattern without special routing logic needed.

---

## Alignment with Unilang Patterns

### Namespace Hierarchy

```bash
# Potential future organization
claude_runner .session.new name::"project"
claude_runner .session.load path::"./session.json"
claude_runner .session.save path::"./session.json"
claude_runner .session.status
claude_runner .session.close

claude_runner .config.set key::value
claude_runner .config.get key
claude_runner .config.list
claude_runner .config.reset

claude_runner .export format::json output::file.json
claude_runner .import file::session.json
```

This matches genfile's `.namespace.action` pattern.

### Parameter Syntax Consistency

```bash
# All commands use same parameter syntax
claude_runner .run message::"Fix bug" dir::/path verbosity::2
claude_runner .session.new name::"project"
claude_runner .config.set key::"model" value::"claude-opus-4-6"
```

No adapter layer needed once users adopt `.command` syntax.

---

## Key Takeaways

1. **Dot Prefix is Unilang Standard** — `.run`, `.help` match framework patterns
2. **Explicit Commands Enable Extensibility** — New commands added cleanly without routing changes
3. **Phase Migration Reduces Risk** — Dual support → Deprecation → Removal
4. **Backward Compatibility Achievable** — Phase 1 maintains all existing behavior
5. **Semantic Clarity Improves UX** — Commands vs parameters clearly distinguished

---

## References

- [Unilang Exploration](unilang_exploration.md) - Deep dive into unilang architecture
- [Genfile CLI Reference](../../../../genfile/docs/cli/readme.md) - Reference implementation
- [Current CLI](../cli/readme.md) - Existing user-facing documentation
