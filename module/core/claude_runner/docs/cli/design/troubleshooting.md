# CLI Troubleshooting

Common issues, solutions, and debugging techniques for `claude_runner` CLI.

## Table of Contents

- [Common Issues](#common-issues)
- [Parameter Issues](#parameter-issues)
- [Execution Issues](#execution-issues)
- [Help/Documentation Issues](#helppocumentation-issues)
- [Debugging Techniques](#debugging-techniques)
- [Performance Issues](#performance-issues)

---

## Common Issues

### Issue: Command Not Found

**Symptom:**
```bash
$ claude_runner .mycommand
Error: unknown command: .mycommand
```

**Cause:** Command name not recognized or typo.

**Solutions:**
1. Check spelling: `claude_runner .help` to list available commands
2. Check dot prefix: Commands must start with `.`
3. Verify command is registered in `build_registry()`

### Issue: Parameter Not Recognized

**Symptom:**
```bash
$ claude_runner --myparam value
Error: unknown argument: --myparam
```

**Cause:** Parameter flag name not in parsing logic.

**Solutions:**
1. Check spelling: `claude_runner .help` to see available options
2. Verify parameter is added to `argv_to_unilang_tokens()` parser
3. Check if short form exists: `-m` for `--message`

### Issue: Invalid Parameter Value

**Symptom:**
```bash
$ claude_runner --max-tokens invalid
Error: invalid --max-tokens value: invalid
```

**Cause:** Value doesn't match expected type or validation rules.

**Solutions:**
1. Check parameter type (Integer, Boolean, String, Path)
2. Verify value format (no spaces, valid range)
3. Check validation rules in docs

### Issue: Missing Required Value

**Symptom:**
```bash
$ claude_runner --message
Error: --message requires a value
```

**Cause:** Flag expects value but none provided.

**Solutions:**
1. Add value after flag: `--message "my text"`
2. Use short form: `-m "my text"`
3. Use positional: `claude_runner "my text"`

### Issue: Conflicting Parameters

**Symptom:**
```bash
$ claude_runner "positional" --message "explicit"
Error: --message conflicts with a previously set message (positional or duplicate --message)
```

**Cause:** Message specified both positionally and via flag.

**Solutions:**
1. Use only positional: `claude_runner "my text"`
2. Use only flag: `claude_runner --message "my text"`
3. Don't mix: Avoid specifying both

---

## Parameter Issues

### Issue: Boolean Flag Not Working

**Symptom:**
```bash
$ claude_runner --verbose
# Verbose mode not enabled
```

**Cause:** Handler not checking for boolean flag.

**Solutions:**
1. Verify flag in adapter parsing: `-v | "--verbose" => { verbose = true; }`
2. Verify parameter in command definition: `ArgumentDefinition::new("verbose", Kind::Boolean)`
3. Verify handler extraction: `matches!(cmd.arguments.get("verbose"), Some(Value::Boolean(true)))`

### Issue: Integer Parameter Out of Range

**Symptom:**
```bash
$ claude_runner --max-tokens 4294967296
Error: invalid --max-tokens value: 4294967296
```

**Cause:** Value exceeds u32 maximum (4,294,967,295).

**Solutions:**
1. Use valid range: 1-4,294,967,295
2. Check for overflow in other values
3. Consider using larger type if range insufficient

### Issue: Negative Integer Rejected

**Symptom:**
```bash
$ claude_runner --max-tokens -1
Error: invalid --max-tokens value: -1
```

**Cause:** Type is u32 (unsigned), negative values invalid.

**Solutions:**
1. Use positive value: `--max-tokens 1`
2. Consider if signed integer type needed

### Issue: Path Parameter Not Found

**Symptom:**
```bash
$ claude_runner --dir /nonexistent/path
Error: [Claude execution error about path not found]
```

**Cause:** Directory doesn't exist on system.

**Solutions:**
1. Verify path exists: `ls -la /path`
2. Use absolute path if relative path ambiguous
3. Check current working directory

---

## Execution Issues

### Issue: Claude Not Found

**Symptom:**
```bash
$ claude_runner --message "test"
Error: Failed to execute Claude: No such file or directory (os error 2)
```

**Cause:** `claude` binary not in PATH.

**Solutions:**
1. Install Claude Code CLI
2. Check PATH: `echo $PATH`
3. Use absolute path if needed
4. Verify Claude installation: `claude --version`

### Issue: Claude Execution Failed

**Symptom:**
```bash
$ claude_runner --message "test"
# [Claude error output]
Error: Claude exited with code 1
```

**Cause:** Claude execution failed (authentication, network, etc.).

**Solutions:**
1. Check Claude authentication: `claude auth status`
2. Check network connectivity
3. Check Claude subscription/API status
4. Review Claude error messages for root cause

### Issue: Dry Run Shows Wrong Output

**Symptom:**
```bash
$ claude_runner --dry-run --message "test"
# Missing expected env vars or command
```

**Cause:** Adapter not building tokens correctly or handler not outputting.

**Solutions:**
1. Verify adapter returns correct tokens: check `argv_to_unilang_tokens()`
2. Verify handler uses `builder.describe_env()` and `builder.describe()`
3. Check for missing parameters in output

### Issue: Verbose Not Printing to Stderr

**Symptom:**
```bash
$ claude_runner --verbose --message "test"
# No preview on stderr
```

**Cause:** Handler not checking `verbose` flag.

**Solutions:**
1. Verify flag parsing: `-v | "--verbose" => { verbose = true; }`
2. Verify handler: `if is_verbose { eprintln!("{preview}"); }`
3. Check that preview prints before `builder.execute()`

---

## Help/Documentation Issues

### Issue: Help Text Doesn't Show All Options

**Symptom:**
```bash
$ claude_runner .help
# Some options missing from output
```

**Cause:** `print_help()` function missing option or out of sync with code.

**Solutions:**
1. Compare `print_help()` output with adapter flags
2. Add missing options to help text
3. Keep options in sync with code changes

### Issue: Examples in Help Don't Work

**Symptom:**
```bash
$ claude_runner .help
# Example in help doesn't work when tried
```

**Cause:** Example has typo or outdated syntax.

**Solutions:**
1. Test each example before documenting
2. Keep examples up to date with code changes
3. Verify examples produce expected output

### Issue: Documentation Outdated

**Symptom:**
```bash
# Documentation says X works
# But it doesn't
```

**Cause:** Code changed but docs not updated.

**Solutions:**
1. Update docs with each code change
2. Run tests to verify documentation accuracy
3. Consider automated docs generation

---

## Debugging Techniques

### Technique 1: Print Parsed Tokens

**Add debug output to adapter:**
```rust
fn argv_to_unilang_tokens(argv: &[String]) -> Result<Vec<String>> {
    let tokens = /* ... parsing ... */;

    // DEBUG: Print what we're passing to unilang
    eprintln!("DEBUG: Tokens: {:?}", tokens);

    Ok(tokens)
}
```

**Usage:**
```bash
$ claude_runner --message "test" --dry-run
DEBUG: Tokens: [".run", "message::test", "dry::1"]
```

### Technique 2: Print Verified Command

**Add debug output to handler:**
```rust
let handler: CommandRoutine = Box::new(|cmd, _ctx| {
    // DEBUG: Print what unilang parsed
    eprintln!("DEBUG: Command: {}", cmd.command);
    eprintln!("DEBUG: Args: {:?}", cmd.arguments);

    // ... handler logic ...
});
```

**Usage:**
```bash
$ claude_runner .run --message "test"
DEBUG: Command: .run
DEBUG: Args: {"message": String("test"), "dry": Boolean(false)}
```

### Technique 3: Print Builder State

**Add debug output before execution:**
```rust
let builder = ClaudeCommand::new()
    .with_message("test")
    .with_dry(true);

// DEBUG: Print what would execute
eprintln!("DEBUG: Env:\n{}", builder.describe_env());
eprintln!("DEBUG: Command:\n{}", builder.describe());

if dry {
    return Ok(OutputData { /* ... */ });
}

// ... execute ...
```

### Technique 4: Test Adapter in Isolation

**Create test to verify token conversion:**
```rust
#[cfg(test)]
fn test_argv_to_tokens() {
    let argv = vec![
        "--message".to_string(),
        "test".to_string(),
        "--dry-run".to_string(),
    ];

    let tokens = argv_to_unilang_tokens(&argv)
        .expect("conversion should succeed");

    assert_eq!(tokens, vec![
        ".run".to_string(),
        "message::test".to_string(),
        "dry::1".to_string(),
    ]);
}
```

### Technique 5: Test Handler in Isolation

**Create test to verify handler logic:**
```rust
#[cfg(test)]
fn test_handler_with_params() {
    let mut cmd = VerifiedCommand::new();
    cmd.command = ".run".to_string();
    cmd.arguments.insert("message".to_string(), Value::String("test".to_string()));

    let ctx = ExecutionContext::default();
    let result = handler(&cmd, &mut ctx);

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.content.contains("test"));
}
```

---

## Performance Issues

### Issue: Slow Startup

**Symptom:**
```bash
$ time claude_runner --message "test"
# Takes >1 second for simple invocation
```

**Cause:** Expensive operations in initialization path.

**Solutions:**
1. Profile startup: identify bottleneck
2. Lazy initialization: defer work until needed
3. Cache expensive results

**Profiling example:**
```bash
$ cargo flamegraph --bin claude_runner
$ flamegraph --out=flamegraph.svg
```

### Issue: Memory Growth

**Symptom:**
```bash
# Memory usage grows with repeated invocations
```

**Cause:** Static state not being cleaned.

**Solutions:**
1. Avoid static mut state if possible
2. Reset state between invocations
3. Use arena allocation for temporary data

---

## Build/Installation Issues

### Issue: Binary Not Found

**Symptom:**
```bash
$ claude_runner --version
command not found: claude_runner
```

**Cause:** Binary not installed or not in PATH.

**Solutions:**
1. Build from source: `cargo install --path .` (development)
2. Install from release: `cargo install --path ~/.local/bin claude_runner`
3. Add to PATH: `export PATH="$HOME/.local/bin:$PATH"`

### Issue: Permission Denied

**Symptom:**
```bash
$ claude_runner --message "test"
permission denied
```

**Cause:** Execute permission not set on binary.

**Solutions:**
1. Set execute permission: `chmod +x target/release/claude_runner`
2. Reinstall: `cargo install --reinstall claude_runner`
3. Check file ownership: `ls -l target/release/claude_runner`

---

## Error Message Quick Reference

| Error Message | Likely Cause | Fix |
|---------------|---------------|-----|
| `unknown argument: X` | Typo or unsupported flag | Check spelling, run `.help` |
| `X requires a value` | Flag without value | Add value after flag |
| `invalid X value: Y` | Type mismatch | Check expected type, fix value |
| `unknown command: X` | Command not registered | Use `.help` to list |
| `conflicts with previously set X` | Duplicate specification | Remove duplicate |
| `command not found: X` | Not implemented | Check docs for available commands |
| `Failed to execute Claude` | Binary not in PATH | Install Claude CLI |
| `Claude exited with code 1` | Claude execution error | Check Claude status |
| `Claude exited with code 2` | Claude runtime error | Review Claude logs |

---

## Common Mistakes

### Mistake 1: Using Wrong Command Format

**Wrong:**
```bash
claude_runner run        # Missing dot prefix
claude_runner help       # Missing dot prefix
```

**Correct:**
```bash
claude_runner .run        # Dot prefix
claude_runner .help       # Dot prefix
```

### Mistake 2: Mixing Positional and Flag

**Wrong:**
```bash
claude_runner "positional message" --message "flag message"
```

**Correct:**
```bash
claude_runner "positional message"    # Only positional
# OR
claude_runner --message "flag message"  # Only flag
```

### Mistake 3: Spaces Around Parameters

**Wrong:**
```bash
claude_runner --message " message with spaces "
```

**Correct:**
```bash
claude_runner --message "message with spaces"  # No spaces around value
```

### Mistake 4: Wrong Value Type

**Wrong:**
```bash
claude_runner --max-tokens "50000"  # String, not integer
```

**Correct:**
```bash
claude_runner --max-tokens 50000  # Integer value
```

### Mistake 5: Missing Dot Prefix

**Wrong:**
```bash
claude_runner mycommand --param value
```

**Correct:**
```bash
claude_runner .mycommand --param value  # Dot prefix
```

---

## Getting Help

### When to Ask for Help

1. **Before reporting bug:** Check if issue is documented here
2. **Before proposing change:** Review best practices
3. **When confused:** Check `claude_runner .help` and documentation
4. **Extension issues:** See [Implementation Guide](implementation_guide.md) for adding commands

### Support Channels

- **Documentation:** This guide and docs/cli/ directory
- **Examples:** See [Tutorial](tutorial.md)
- **API Reference:** See [API Reference](api_reference.md)
- **Unilang Docs:** Published on crates.io

### Reporting Bugs

If you find a bug or issue not covered here:

1. Verify it's not a usage error
2. Check existing issues in this document
3. Create minimal reproducer
4. Report with details:
   - Expected behavior
   - Actual behavior
   - Steps to reproduce
   - Environment details

---

## Diagnostic Commands

### Version Check

```bash
$ claude_runner .version
# Or (if version command exists)
$ claude_runner --version
```

### Dry Run Verification

```bash
$ claude_runner .run --message "test" --dry-run
# Shows what would execute without running
```

### Verbose Mode

```bash
$ claude_runner --verbose --message "test"
# Shows command assembly before execution
```

---

## Summary

| Category | Key Techniques |
|----------|----------------|
| Common Issues | Check spelling, verify registration, use `.help` |
| Parameter Issues | Type validation, range checking, path verification |
| Execution Issues | Check PATH, verify Claude install, check error codes |
| Debugging | Print tokens, print commands, test in isolation |
| Performance | Profile startup, avoid allocations, lazy init |
| Build/Install | Set execute permissions, add to PATH |
| Common Mistakes | Dot prefix, no duplicate specs, right types |

---

## References

- [Architecture](architecture.md) — System diagrams and data flow
- [API Reference](api_reference.md) — Complete API documentation
- [Best Practices](best_practices.md) — Patterns and guidelines
- [Migration Guide](migration_guide.md) — Upgrade path
- [Quick Reference](quick_reference.md) — Fast lookup card
