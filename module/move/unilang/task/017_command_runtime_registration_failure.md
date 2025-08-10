# Task 017: Command Runtime Registration Failure

## Status: Not Started
## Priority: High  
## Responsible: @user
## Created: 2025-08-10
## Category: Bug Fix / Critical Runtime Issue

---

## Problem Summary

**Critical Issue**: Unilang command registry successfully registers commands and displays them in help listings, but runtime command execution fails with "No executable routine found" errors. This affects ALL commands, making the entire command system non-functional despite correct registration code.

## Detailed Problem Description

### Symptoms
1. **Command Definition**: Commands are defined correctly with proper `CommandDefinition` structures
2. **Command Registration**: `registry.command_add_runtime()` calls succeed without errors
3. **Command Discovery**: Commands appear correctly in help listings and command discovery (`.` command)
4. **Command Help**: Individual command help works perfectly (`<command> ?`)
5. **Runtime Execution Failure**: All command execution fails with identical error pattern

### Error Pattern
```bash
❌ Command error: Execution error: Execution Error: Internal Error: No executable routine found for command '<command_name>'. This is a system error, please report it.
```

**Key Observation**: Error message shows command name WITHOUT dot prefix (e.g., "chat" instead of ".chat"), suggesting name resolution mismatch between registration and runtime lookup.

### Evidence of Systematic Failure
Commands tested that ALL fail identically:
- `.chat` → "No executable routine found for command 'chat'"  
- `.version` → "No executable routine found for command 'version'"
- All other dot-prefixed commands exhibit same behavior

## Technical Analysis

### Registration Code (WORKING)
```rust
// Command definition - CORRECT
let chat_cmd = CommandDefinition {
    name: ".chat".to_string(),
    namespace: String::new(),
    description: "Start a multi-agent chat session with Initiative-based turn-taking".to_string(),
    // ... other fields
    routine_link: None,  // This is correct for runtime registration
};

// Runtime registration - APPEARS CORRECT  
registry.command_add_runtime(&chat_cmd, Box::new(handle_chat_command))?;
```

### Handler Function (WORKING)
```rust
fn handle_chat_command(cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result<OutputData, ErrorData> {
    // Complete implementation exists and compiles correctly
    // Function signature matches expected runtime handler signature
}
```

### Discovery Working But Execution Failing
```bash
# Command listing works - shows .chat is registered
$ assistant .
Available commands:
  .chat                Start a multi-agent chat session with Initiative-based turn-taking
  # ... other commands

# Command help works - shows .chat details  
$ assistant .chat ?
Usage: .chat (v0.1.0)
# ... detailed help output

# But execution fails
$ assistant .chat
❌ No executable routine found for command 'chat'
```

## Root Cause Hypothesis

The issue appears to be in **unilang's runtime command resolution mechanism**:

1. **Registration Phase**: Commands are registered with full names (e.g., ".chat")
2. **Discovery Phase**: Registry lookup works correctly with full names
3. **Runtime Execution Phase**: Name resolution strips dot prefix, looks up "chat" instead of ".chat"
4. **Lookup Failure**: Runtime registry lookup fails because it's searching for wrong key

This suggests either:
- Bug in unilang's command name normalization during execution
- Inconsistency between registration and lookup key generation  
- Version mismatch in unilang dependencies
- Runtime registry internal storage issue

## Minimal Reproducible Example (MRE)

### Environment
- **System**: Linux 6.8.0-71-generic
- **Rust**: Latest stable
- **Project**: `/home/user1/pro/lib/llm_tools/module/assistant`
- **unilang**: Workspace dependency (exact version TBD)

### Reproduction Steps
```bash
cd /home/user1/pro/lib/llm_tools/module/assistant

# 1. Build the assistant binary
cargo build --bin assistant

# 2. Verify command is registered (this works)
cargo run -- .
# Expected: Lists .chat in available commands ✅

# 3. Get command help (this works)  
cargo run -- .chat ?
# Expected: Shows detailed .chat command help ✅

# 4. Execute command (this fails)
cargo run -- .chat
# Expected: Starts chat session
# Actual: ❌ No executable routine found for command 'chat'
```

### Expected vs Actual Behavior

| Phase | Expected | Actual | Status |
|-------|----------|---------|--------|
| Registration | Command registered as ".chat" | ✅ Works | ✅ |
| Discovery | ".chat" appears in listings | ✅ Works | ✅ |  
| Help | `.chat ?` shows help | ✅ Works | ✅ |
| Execution | `.chat` executes handler | ❌ Fails | ❌ |

## Impact Assessment

### Severity: **CRITICAL**
- **User Impact**: Complete command system failure - no commands can execute
- **Development Impact**: Cannot test or use any unilang-based CLI functionality
- **Business Impact**: Assistant CLI is completely non-functional despite working implementation

### Affected Components
- All dot-prefixed commands in unilang applications
- Command execution pipeline
- Runtime command resolution system
- User-facing CLI functionality

## Investigation Areas

### 1. Command Name Resolution
- [ ] Investigate unilang's internal command key generation
- [ ] Compare registration keys vs runtime lookup keys
- [ ] Check if dot prefix is being stripped during execution phase

### 2. Registry Internal State  
- [ ] Examine runtime registry storage mechanism
- [ ] Verify commands are actually stored with correct keys
- [ ] Check for key normalization inconsistencies

### 3. Version Compatibility
- [ ] Verify unilang workspace dependency versions
- [ ] Check for breaking changes in recent unilang versions
- [ ] Validate API compatibility between registration and execution

### 4. Pipeline Processing
- [ ] Trace command processing through unilang pipeline
- [ ] Identify where command name transformation occurs
- [ ] Verify `process_command_simple()` behavior

## Debugging Traces

### Registry State Verification
```rust
// Suggested debug code to add to setup_command_registry()
println!("DEBUG: Registering command: '{}'", chat_cmd.name);
registry.command_add_runtime(&chat_cmd, Box::new(handle_chat_command))?;
println!("DEBUG: Registration completed for: '{}'", chat_cmd.name);
```

### Runtime Resolution Tracing
```rust
// Suggested debug in main() before process_command_simple
println!("DEBUG: Processing command string: '{}'", command_str);
let result = pipeline.process_command_simple(&command_str);
println!("DEBUG: Command processing result: success={}", result.success);
```

## Workaround Attempts

### 1. Alternative Registration (Test)
Try registering without dot prefix:
```rust
let chat_cmd = CommandDefinition {
    name: "chat".to_string(), // Test without dot
    // ...
};
```

### 2. Direct Pipeline Testing
Create isolated test to verify registry functionality:
```rust
#[test] 
fn test_command_runtime_resolution() {
    let registry = setup_command_registry().unwrap();
    let pipeline = Pipeline::new(registry);
    let result = pipeline.process_command_simple(".chat");
    assert!(result.success, "Command should execute successfully");
}
```

## Files Involved

### Primary Files
- `/home/user1/pro/lib/llm_tools/module/assistant/src/bin/assistant.rs` - Main command registration and execution
- `/home/user1/pro/lib/llm_tools/module/assistant/Cargo.toml` - Dependency configuration

### Key Functions
- `setup_command_registry()` - Command registration logic
- `handle_chat_command()` - Example failing command handler  
- `main()` - Command processing pipeline

## Success Criteria

### Definition of Done
- [ ] `.chat` command executes successfully and launches TUI
- [ ] All other dot-prefixed commands execute properly
- [ ] Command registration and runtime resolution work consistently
- [ ] No regression in command discovery or help functionality
- [ ] Root cause documented and prevented for future commands

### Verification Tests
```bash
# All these should work after fix:
assistant .chat
assistant .version  
assistant .session.list
assistant .run prompts::"test"
```

## Notes

### Discovery Timeline
- **2025-08-10**: Issue discovered during comprehensive testing of assistant CLI
- **Confirmation**: Affects ALL commands, not just .chat
- **Validation**: Command registration code is correct, issue is in unilang runtime

### Related Issues
- This may affect other projects using unilang command system
- Similar symptoms might be seen in any dot-prefixed command implementations
- Could be related to recent unilang architectural changes

### Testing Context
- Issue discovered during systematic manual testing after automated tests passed
- Demonstrates critical gap between unit testing and integration testing
- Automated tests validate handler logic but miss command system integration

---

## References
- **Source**: Comprehensive testing initiative in `/home/user1/pro/lib/llm_tools/module/assistant`
- **Related**: Assistant CLI implementation using unilang command framework  
- **Context**: Multi-agent chat system with Initiative-based turn-taking