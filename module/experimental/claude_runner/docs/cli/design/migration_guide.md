# CLI Migration Guide

Complete guide for migrating from current design to recommended explicit dot-prefixed commands.

## Overview

This guide walks through the migration path from implicit command routing (`--help` flag) to explicit dot-prefixed commands (`.help`, `.run`).

## Migration Goals

1. **Add explicit commands** — `.help` and `.run` as first-class citizens
2. **Maintain backward compatibility** — No breaking changes in Phase 1
3. **Clear user guidance** — Help users transition smoothly
4. **Deprecate gradually** — Multi-phase deprecation of `--help` flag
5. **Enable extensibility** — Clean path for adding new commands

---

## Phase 1: Dual Support (Current Release)

**Objective:** Add explicit commands while keeping `--help` flag fully functional.

### Implementation Steps

#### Step 1.1: Update Adapter Routing

**Location:** `src/main.rs` — `argv_to_unilang_tokens()`

**Add explicit command detection:**
```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    // Phase 1: Check for explicit dot command FIRST
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

**Key changes:**
- Check for dot-prefixed tokens before any other parsing
- Return immediately on match (don't continue parsing)
- Preserve existing implicit routing for non-dot invocations

#### Step 1.2: Create Helper Function

**Add parameter parsing helper:**
```rust
fn parse_parameters(
    argv: &[String],
    tokens: &mut Vec<String>,
    matched_token: &str,
) -> Result<(), Error> {
    let mut i = argv.iter().position(|t| t == matched_token).unwrap() + 1;

    while i < argv.len() {
        match argv[i].as_str() {
            "-m" | "--message" => {
                i += 1;
                let val = argv.get(i).ok_or_else(|| Error::msg("--message requires a value"))?;
                tokens.insert(1, format!("message::{val}"));
            }
            "-d" | "--dir" => {
                i += 1;
                let val = argv.get(i).ok_or_else(|| Error::msg("--dir requires a value"))?;
                tokens.push(format!("dir::{val}"));
            }
            // ... all other parameters
            other => {
                if !other.starts_with('-') && tokens.iter().all(|t| !t.starts_with("message::")) {
                    tokens.insert(1, format!("message::{other}"));
                } else if !other.starts_with('-') {
                    return Err(Error::msg(format!("unknown argument: {other}")));
                }
            }
        }
        i += 1;
    }
    Ok(())
}
```

#### Step 1.3: Update Help Text

**Location:** `src/main.rs` — `print_help()`

**Add COMMANDS section:**
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
    println!("  # Explicit .run");
    println!("  claude_runner .run --message \"Fix the bug\" --dir /project");
    println!();
    println!("  # Help");
    println!("  claude_runner .help");
}
```

#### Step 1.4: Add Tests

**Location:** `tests/cli_args_test.rs`

**Test explicit .help:**
```rust
#[test]
fn help_explicit_command() {
    let out = run_cli(&[".help"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("USAGE:"), ".help should print usage");
}
```

**Test explicit .run:**
```rust
#[test]
fn run_explicit_command() {
    let out = run_cli(&[".run", "--message", "test"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("test"), ".run with params should work");
}
```

**Update existing tests:**
```rust
// Replace --help tests with .help equivalents
// All existing functionality preserved
```

### User Impact

**Changes:** Zero breaking changes — all existing invocations continue to work.

**New capabilities:**
```bash
# Now supported
claude_runner .help               # Explicit help
claude_runner .run --message "hi"  # Explicit run

# Still supported (backward compatible)
claude_runner --help               # Flag help still works
claude_runner --message "hi"       # Implicit .run still works
```

**Documentation update:**
- Add COMMANDS section to help text
- Document both invocation styles
- Show examples of each

---

## Phase 2: Deprecation (Next Release)

**Objective:** Mark `--help` flag as deprecated, guide users to `.help`.

### Implementation Steps

#### Step 2.1: Add Deprecation Notice

**Location:** `src/main.rs` — Adapter parsing

**Add deprecation on `--help` flag:**
```rust
"-h" | "--help" => {
    // Print deprecation notice
    eprintln!("DEPRECATION: --help flag is deprecated. Use '.help' command instead.");
    eprintln!("  This will be removed in a future release.");

    help = true;
}
```

**Alternative: Suppress notice if explicit command used:**
```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    // Check for explicit command first
    for token in argv {
        if token.starts_with('.') {
            // No deprecation notice needed
            return match token.as_str() {
                ".help" => Ok(vec![".help".to_string()]),
                ".run" => { /* ... */ },
                other => Err(Error::msg(format!("unknown command: {other}"))),
            };
        }
    }

    // No explicit command → implicit routing
    // ... (may print deprecation if --help found)
}
```

#### Step 2.2: Update Help Text

**Location:** `src/main.rs` — `print_help()`

**Add deprecation section:**
```rust
fn print_help() {
    println!("claude_runner — Execute Claude Code with configurable parameters");
    println!();
    println!("DEPRECATION NOTICE:");
    println!("  The --help flag is deprecated and will be removed in a future release.");
    println!("  Please use the '.help' command instead.");
    println!();
    println!("USAGE:");
    println!("  claude_runner [COMMAND] [OPTIONS] [MESSAGE]");
    println!();
    println!("COMMANDS:");
    println!("  .help    Print this help (recommended)");
    println!("  .run      Execute Claude Code (default command)");
    println!();
    println!("DEPRECATED (will be removed):");
    println!("  -h, --help     Show this help (use '.help' command instead)");
    println!();
    println!("OPTIONS (for .run):");
    // ... rest of options
}
```

#### Step 2.3: Update Documentation

**Update all docs to prefer `.help`:**
- `docs/cli/readme.md` — Recommend `.help` over `--help`
- `docs/cli/commands.md` — Note deprecation in examples
- `readme.md` — Add deprecation notice

**Example documentation update:**
```markdown
## Usage

```bash
# Recommended (new style)
claude_runner .help

# Still supported but deprecated
claude_runner --help  # Will be removed in future release
```
```

### User Impact

**Changes:** Warning on deprecated usage, no functionality removed.

**Migration encouragement:**
```bash
$ claude_runner --help
DEPRECATION: --help flag is deprecated. Use '.help' command instead.
  This will be removed in a future release.

claude_runner — Execute Claude Code with configurable parameters

USAGE:
  claude_runner [COMMAND] [OPTIONS] [MESSAGE]
...
```

---

## Phase 3: Removal (Following Release)

**Objective:** Remove `--help` flag entirely, enforce explicit commands.

### Implementation Steps

#### Step 3.1: Remove --help Flag

**Location:** `src/main.rs` — `argv_to_unilang_tokens()`

**Remove help flag parsing:**
```rust
// DELETE these lines:
"-h" | "--help" => { help = true; }

// DELETE help state variable
// let mut help = false;

// DELETE help routing at end
// if help { return Ok(vec![".help".to_string()]); }
```

#### Step 3.2: Remove -h Short Form

**Also remove `-h` from help text:**
```rust
fn print_help() {
    println!("COMMANDS:");
    println!("  .help    Print this help");
    println!("  .run      Execute Claude Code (default command)");

    // No -h mentioned
}
```

#### Step 3.3: Update Routing Logic

**Simplified routing:**
```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    // Check for explicit dot command
    for token in argv {
        if token.starts_with('.') {
            return match token.as_str() {
                ".help" => Ok(vec![".help".to_string()]),
                ".run" => {
                    let mut tokens = vec![".run".to_string()];
                    parse_parameters(&argv, &mut tokens, token)?;
                    Ok(tokens)
                }
                other => Err(Error::msg(format!("unknown command: {other}"))),
            };
        }
    }

    // No explicit command → assume .run with parameters
    let mut tokens = vec![".run".to_string()];
    parse_parameters(&argv, &mut tokens, "")?;
    Ok(tokens)
}
```

#### Step 3.4: Update Tests

**Remove --help tests:**
```rust
// DELETE or UPDATE tests
// All --help tests should now use .help
```

**Add error tests for removed flag:**
```rust
#[test]
fn help_flag_removed() {
    let out = run_cli(&["--help"]);
    assert!(!out.status.success(), "--help flag should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown argument"), "Should error on --help");
}
```

#### Step 3.5: Update Documentation

**Remove all --help references:**
```markdown
## Usage

```bash
# Only supported syntax
claude_runner .help
claude_runner .run --message "task"

# DEPRECATED (no longer supported)
# claude_runner --help  # ERROR: unknown argument
# claude_runner -h         # ERROR: unknown argument
```

**Remove deprecation notices:**
- All deprecation sections from help text
- All deprecation warnings from documentation

### User Impact

**Breaking changes:**
- `--help` flag no longer recognized (error: "unknown argument")
- `-h` short form no longer recognized
- Only `.help` command accepted

**Migration path:**
- Previous phase provided guidance (deprecated warnings)
- Documentation updated to show new syntax
- Users had one release cycle to adapt

---

## Testing Strategy

### Phase 1 Test Coverage

**Existing tests:** Preserve all (backward compatible)

**New tests:**
- Explicit `.help` invocation
- Explicit `.run` invocation
- Both styles produce identical output

**Test matrix:**
| Invocation | Expected | Status |
|-----------|----------|--------|
| `--help` | Help output | ✅ Still works |
| `.help` | Help output | ✅ New support |
| `--message "hi"` | Run execution | ✅ Still works |
| `.run --message "hi"` | Run execution | ✅ New support |
| Mixed (`.run` + flags) | Run execution | ✅ Both work |

### Phase 2 Test Coverage

**Deprecation tests:**
- Verify deprecation notice printed with `--help`
- Verify no notice with `.help`
- Verify functionality unchanged

### Phase 3 Test Coverage

**Removal tests:**
- Verify `--help` fails with error
- Verify `.help` works
- Verify `.run` works
- All existing tests pass (with updates)

---

## Rollback Plan

### If Issues Discovered

**Phase 1 rollback:** Remove explicit command detection, restore old logic
**Phase 2 rollback:** Remove deprecation notices, restore Phase 1 behavior
**Phase 3 rollback:** Restore `--help` flag handling

### Rollback Steps

1. Revert code changes (git revert or manual)
2. Run full test suite
3. Verify all previous functionality restored
4. Document rollback reason

---

## Migration Checklist

### Phase 1: Dual Support

- [ ] Add explicit command detection in adapter
- [ ] Implement `parse_parameters()` helper function
- [ ] Update `print_help()` with COMMANDS section
- [ ] Add explicit `.help` test
- [ ] Add explicit `.run` test
- [ ] Verify all existing tests still pass
- [ ] Update help text with COMMANDS section
- [ ] Document both invocation styles

### Phase 2: Deprecation

- [ ] Add deprecation notice on `--help` flag
- [ ] Update `print_help()` with deprecation section
- [ ] Update documentation with deprecation notice
- [ ] Verify deprecation printed correctly
- [ ] Verify functionality unchanged

### Phase 3: Removal

- [ ] Remove `--help` flag from parser
- [ ] Remove `-h` short form from help
- [ ] Simplify routing logic (remove help state)
- [ ] Update `print_help()` (remove deprecation section)
- [ ] Update all documentation (remove --help references)
- [ ] Add error test for removed flag
- [ ] Update existing tests (convert --help → .help)
- [ ] Run full test suite
- [ ] Verify all tests pass

---

## User Communication

### Release Notes Template

```markdown
## Version X.Y.Z

### Added

- Explicit `.help` command for printing usage
- Explicit `.run` command for executing Claude Code
- Both commands work with all existing parameters

### Changed

- Help text now includes COMMANDS section
- Improved extensibility for future commands

### Migration Notes

No breaking changes — all existing invocations continue to work. Users can optionally use new `.help` and `.run` commands for clarity.
```

---

## Version X.Y.Z+1 (Deprecation)

### Deprecated

- `--help` flag is deprecated; use `.help` command instead
- `-h` short form is deprecated

### Migration Path

1. Current invocation: `claude_runner --help`
2. Recommended: `claude_runner .help`
3. Timeline: Flag will be removed in next major release

### Changes

- Deprecation notice displayed when `--help` is used
- Documentation updated to recommend `.help` command

---

## Version X.Y.Z+2 (Removal)

### Breaking Changes

- `--help` flag removed (use `.help` command instead)
- `-h` short form removed

### Migration Required

Update all invocations:
```bash
# Old (no longer works)
claude_runner --help

# New
claude_runner .help
```

### Scripts and CI

Update any scripts or CI configurations:
```bash
# Old
#!/bin/bash
claude_runner --help

# New
#!/bin/bash
claude_runner .help
```

---

## Summary

| Phase | Changes | Breaking? | Tests Needed |
|--------|----------|------------|---------------|
| Phase 1 | Add explicit commands, keep `--help` flag | No | Add `.help`, `.run` tests |
| Phase 2 | Add deprecation notice for `--help` | No | Verify deprecation printed |
| Phase 3 | Remove `--help` flag entirely | Yes | Update all tests, add error tests |

**Total migration time:** 3 releases with graceful transition path.

---

## References

- [Command Design](command_design.md) — Design recommendations and rationale
- [API Reference](api_reference.md) — Complete API documentation
- [Architecture](architecture.md) — System diagrams and data flow
- [Best Practices](best_practices.md) — Patterns and guidelines
