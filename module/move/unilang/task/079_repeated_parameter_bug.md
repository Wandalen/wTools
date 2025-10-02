# Task 079: Fix Repeated Parameter Name Bug - "Invalid boolean value" Error

## Overview

**Priority**: 🔴 Critical
**Status**: 🆕 New (Regression from Task 078)
**Affects**: Production CLI applications (wrun, willbe)
**Created**: 2025-09-30

## Problem Statement

Unilang fails to parse commands with repeated parameter names (e.g., `command::"x" command::"y" command::"z"`), throwing a confusing "Invalid boolean value" error instead of collecting values into a `Value::List` as documented.

### Critical Impact

This bug BLOCKS the intended CLI syntax for production applications:
- **wrun**: Cannot use clean syntax for multiple commands
- **willbe**: Similar issues with repeated parameters
- **Workaround required**: Forces use of numbered parameters (`command1::`, `command2::`, etc.)

## Error Message

```
Command failed: Semantic analysis error: Execution Error: Type Error: Invalid boolean value. Please provide a valid value for this type.
```

## Minimum Reproducible Example (MRE)

### Test Case 1: wrun CLI (Real Production Issue)

**Command that FAILS:**
```bash
cd /home/user1/pro/lib/wTools/module/core/former
/home/user1/pro/lib/willbe/module/wrun/target/debug/wrun .run \
  command::"cargo build" \
  command::"echo hello1" \
  command::"cargo test" \
  command::"echo hello2" \
  parallel::2
```

**Expected Result:**
- Parses 4 commands into `Value::List`
- Executes commands with parallel::2

**Actual Result:**
```
Command failed: Semantic analysis error: Execution Error: Type Error: Invalid boolean value. Please provide a valid value for this type.
```

**Workaround that WORKS (but is ugly):**
```bash
/home/user1/pro/lib/willbe/module/wrun/target/debug/wrun .run \
  command1::"cargo build" \
  command2::"echo hello1" \
  command3::"cargo test" \
  command4::"echo hello2" \
  parallel::2
```

### Test Case 2: Standalone Unilang Example

Create this minimal test program:

```rust
// File: /tmp/test_repeated_params.rs
use unilang::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define command with repeated parameter name
    let command_def = CommandDefinition::former()
        .name(".test")
        .arguments(vec![
            ArgumentDefinition::former()
                .name("command")
                .kind(Kind::String)
                .attributes(ArgumentAttributes {
                    optional: true,
                    multiple: true,  // ← Multiple values allowed
                    ..Default::default()
                })
                .description("Test commands")
                .end(),

            ArgumentDefinition::former()
                .name("parallel")
                .kind(Kind::Integer)
                .attributes(ArgumentAttributes {
                    optional: true,
                    default: Some("2".to_string()),
                    ..Default::default()
                })
                .description("Parallel count")
                .end(),
        ])
        .end();

    let registry = CommandRegistry::new(vec![command_def]);

    // Parse command with repeated parameter names
    let input = vec![
        ".test",
        "command::\"cmd1\"",
        "command::\"cmd2\"",
        "command::\"cmd3\"",
        "parallel::2"
    ];

    match unilang::parse_and_verify(&input, &registry) {
        Ok(verified) => {
            println!("✅ SUCCESS: Parsed successfully");
            println!("Commands: {:?}", verified.arguments.get("command"));
            println!("Parallel: {:?}", verified.arguments.get("parallel"));
        }
        Err(e) => {
            println!("❌ FAILURE: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
```

**How to run MRE:**
```bash
cd /home/user1/pro/lib/wTools/module/move/unilang
cargo run --example test_repeated_params
```

Expected: Parses successfully with `command` as `Value::List(["cmd1", "cmd2", "cmd3"])`
Actual: "Invalid boolean value" error

### Test Case 3: Behavior Analysis (Number of Parameters)

Testing shows the bug appears based on the NUMBER of repeated parameters:

**2 commands (WORKS but with tokenization bug):**
```bash
wrun .run command::"cargo build" command::"echo hello1" parallel::2
# Result: Executes 2 commands BUT splits on spaces ("echo build", "echo hello1")
```

**3 commands (WORKS correctly):**
```bash
wrun .run command::"cargo build" command::"echo hello1" command::"cargo test" parallel::2
# Result: ✅ Executes 3 commands correctly with spaces preserved
```

**4 commands (FAILS with error):**
```bash
wrun .run command::"cargo build" command::"echo hello1" command::"cargo test" command::"echo hello2" parallel::2
# Result: ❌ Error: "Invalid boolean value"
```

**4 single-word commands (WEIRD behavior):**
```bash
wrun .run command::"pwd" command::"whoami" command::"date" command::"hostname" parallel::2
# Result: Only executes "hostname" (last command), ignores first 3
```

### Summary of Bugs

There are actually THREE related bugs:

1. **Count-based failure**: 4+ repeated parameters trigger "Invalid boolean value" error
2. **Silent data loss**: 4+ single-word commands only keep the last one
3. **Inconsistent tokenization**: 2 commands split on spaces, 3+ commands preserve spaces

## Root Cause Analysis Needed

### Hypotheses

1. **Semantic Analysis Bug**: The semantic analyzer may be incorrectly processing multiple arguments with the same name, possibly confusing them with boolean flags.

2. **Argument Collection Bug**: The code that collects multiple arguments with the same name into `Value::List` may not be working correctly.

3. **Type Inference Bug**: Type inference may be incorrectly determining that repeated parameters are boolean flags rather than a list of strings.

4. **Validation Bug**: Validation may be applying to the wrong parameter or misidentifying parameter types.

### Investigation Areas

Check these files in unilang:
- `src/semantic_analysis.rs` - Where `Vec<Argument>` converts to `HashMap<String, Value>`
- `src/parser.rs` - Where multiple arguments with same name are parsed
- `src/validation.rs` - Where type validation occurs
- `src/command_definition.rs` - Where `multiple: true` attribute is handled

## Contradiction with Task 078

Task 078 claims this functionality works correctly with passing tests. However:

1. **Production Evidence**: Real CLI application (wrun) fails with this exact error
2. **Workaround Required**: Production code had to implement numbered parameter workaround
3. **Regression**: This may be a regression introduced after Task 078 investigation

**Action Required:** Re-run Task 078's tests and verify they actually test the EXACT failing scenario.

## Acceptance Criteria

### Must Fix

1. ✅ Parse `command::"x" command::"y" command::"z"` into `Value::List(["x", "y", "z"])`
2. ✅ No "Invalid boolean value" errors for correctly typed parameters
3. ✅ wrun production command succeeds:
   ```bash
   wrun .run command::"cargo build" command::"echo hello" parallel::2
   ```

### Regression Prevention

4. ✅ Add test case matching exact wrun production usage
5. ✅ Test with real ArgumentDefinition from wrun (with `multiple: true`)
6. ✅ Test with mixed parameters (repeated string params + single integer param)

## Steps to Reproduce

### Quick Reproduction (wrun binary)

```bash
# Build wrun
cd /home/user1/pro/lib/willbe/module/wrun
cargo build

# Run failing command
target/debug/wrun .run \
  command::"cargo build" \
  command::"cargo test" \
  parallel::2
```

Expected: Executes both commands
Actual: "Invalid boolean value" error

### Detailed Reproduction (unilang tests)

```bash
cd /home/user1/pro/lib/wTools/module/move/unilang

# Create test file
cat > tests/repeated_parameter_regression_test.rs << 'EOF'
use unilang::*;

#[test]
fn test_repeated_parameter_multiple_commands() {
    let command_def = CommandDefinition::former()
        .name(".run")
        .arguments(vec![
            ArgumentDefinition::former()
                .name("command")
                .kind(Kind::String)
                .attributes(ArgumentAttributes {
                    optional: true,
                    multiple: true,
                    ..Default::default()
                })
                .description("Commands to execute")
                .end(),

            ArgumentDefinition::former()
                .name("parallel")
                .kind(Kind::Integer)
                .attributes(ArgumentAttributes {
                    optional: true,
                    default: Some("2".to_string()),
                    ..Default::default()
                })
                .description("Parallel count")
                .end(),
        ])
        .end();

    let registry = CommandRegistry::new(vec![command_def]);

    // This should work but currently fails
    let input = vec![
        ".run",
        "command::\"cargo build\"",
        "command::\"cargo test\"",
        "parallel::2"
    ];

    let result = unilang::parse_and_verify(&input, &registry);

    assert!(result.is_ok(), "Failed to parse repeated parameters: {:?}", result.err());

    let verified = result.unwrap();
    let commands = verified.arguments.get("command").expect("command parameter missing");

    match commands {
        Value::List(list) => {
            assert_eq!(list.len(), 2, "Expected 2 commands");
            assert_eq!(list[0], Value::String("cargo build".to_string()));
            assert_eq!(list[1], Value::String("cargo test".to_string()));
        }
        _ => panic!("Expected Value::List, got {:?}", commands),
    }
}
EOF

# Run test (will fail until bug is fixed)
cargo test repeated_parameter_regression_test -- --nocapture
```

## Related Issues

- Task 078: Claimed this was fixed - needs re-verification
- wrun production issue: Documented workaround in CLI help text (lines 236-238, 271-274)
- willbe may have similar issues

## Priority Justification

**Critical** because:
1. Blocks clean CLI syntax in production applications
2. Forces ugly numbered parameter workarounds
3. Error message is misleading (says "boolean" when parameter is string)
4. User experience is severely degraded
5. Contradicts documented functionality

## Deliverables

1. 🔧 **Fix**: Correct handling of repeated parameter names with `multiple: true`
2. ✅ **Test**: Add regression test matching exact production usage
3. 📝 **Verification**: Confirm wrun production command works
4. 🔍 **Investigation**: Document why Task 078 tests didn't catch this
5. 📚 **Documentation**: Update any incorrect documentation about multiple parameter support

## Environment

- **OS**: Linux 6.8.0-84-generic
- **Rust**: 1.85+ (latest stable)
- **unilang**: Current master branch (commit 941f6567)
- **Affected Apps**: wrun v0.2.0, willbe (potentially)

## Verification Results (2025-09-30)

### Testing Performed

**Tested with published unilang v0.12.0 from crates.io:**

```bash
# Updated wrun to use published versions (no path dependencies)
error_tools = { version = "0.34", features = ["full"] }
former = { version = "2.28", features = ["full"] }
unilang = { version = "0.12", features = ["enabled"], optional = true }
```

**Tested with commit 941f6567 (latest stable git version):**

```bash
cd /home/user1/pro/lib/wTools/module/core/former
/home/user1/pro/lib/willbe/module/wrun/target/debug/wrun .run \
  command::"cargo build" \
  command::"echo hello1" \
  command::"cargo test" \
  command::"echo hello2" \
  parallel::2
```

**Result**: ❌ **BUG STILL EXISTS**
```
Command failed: Semantic analysis error: Execution Error: Type Error: Invalid boolean value. Please provide a valid value for this type.
```

### Attempted Update

Attempted to update to newer version (commit 8244a9ef) but found:
- **Critical Issue**: Bad merge in unilang_parser with duplicate struct/enum definitions
- **Build Errors**: 16 compilation errors in item_adapter.rs (duplicate `Split` and `SplitType`)
- **Status**: Latest master (8244a9ef) is **unbuildable**

**Testing with unilang v0.22.0 (latest published):**

```bash
# 4 commands with repeated parameter syntax
wrun .run command::"cargo build" command::"echo hello1" command::"cargo test" command::"echo hello2" parallel::2
Result: ❌ "Invalid boolean value" error (same as v0.12.0)

# 3 commands with repeated parameter syntax
wrun .run command::"cargo build" command::"echo hello1" command::"cargo test" parallel::2
Result: ❌ Executes but strips commands to single words ("build", "hello1", "test")

# 4 commands with numbered syntax
wrun .run command1::"cargo build" command2::"echo hello1" command3::"cargo test" command4::"echo hello2" parallel::2
Result: ❌ Only 3 commands, strips to single words ("cargo", "echo", "cargo"), 4th command lost
```

**Conclusion**:
1. Bug exists in **published version 0.12.0** from crates.io
2. Bug exists in **published version 0.22.0** from crates.io (latest)
3. Bug exists in stable git version (commit 941f6567)
4. Latest git version (8244a9ef) has merge conflicts and doesn't compile
5. v0.22.0 shows DIFFERENT bugs: tokenization splits quoted strings + silent data loss
6. Bug remains **UNFIXED** in all tested versions (both published and git)

## Notes

This bug is particularly frustrating because:
1. The workaround (numbered params) DOES NOT work in v0.22.0 - it has tokenization bugs too
2. The error message is misleading ("boolean value" for string parameter)
3. Task 078 claimed this was fixed, but production evidence shows it isn't
4. The `multiple: true` attribute exists but doesn't seem to work as documented
5. **Latest master has a bad merge and doesn't compile** (commit 8244a9ef)
6. **v0.22.0 introduced NEW bugs**: tokenization splits quoted strings on spaces
7. **v0.22.0 silent data loss**: 4th numbered command completely disappears

**Status**: **CONFIRMED BUG - NOT FIXED - GETTING WORSE**

**Critical Regression in v0.22.0**:
- Quoted strings with spaces are now SPLIT on spaces (e.g., "cargo build" → "cargo")
- 4th command silently lost when using numbered syntax
- Both repeated parameter AND numbered syntax are broken

**Urgent Actions Required**:
1. Fix bad merge in commit 8244a9ef (item_adapter.rs has duplicate definitions)
2. Fix tokenization regression in v0.22.0 (quoted strings being split)
3. Fix silent data loss of 4th command in v0.22.0
4. Reproduce and fix the 4-command repeated parameter bug
5. Add regression tests for repeated parameter syntax with 2, 3, 4+ parameters
6. Add regression tests for quoted string preservation

**Request**: Please reproduce this bug, investigate the root cause, and fix it so that the clean syntax `command::"x" command::"y"` works as intended. Also fix the bad merge in commit 8244a9ef.

## Investigation Results (2025-10-01)

### wrun API Correction

**Issue Found**: wrun was using incorrect ArgumentDefinition pattern for command parameter.

**Incorrect Usage** (in wrun src/cli/mod.rs:494-504):
```rust
ArgumentDefinition::former()
  .name( "command" )
  .kind( Kind::List( Box::new( Kind::String ), None ) )  // ❌ WRONG
  .attributes( ArgumentAttributes {
    optional: true,
    multiple: true,
    ..Default::default()
  } )
```

**Correct Usage** (matching unilang test examples):
```rust
ArgumentDefinition::former()
  .name( "command" )
  .kind( Kind::String )  // ✅ CORRECT: Use Kind::String with multiple: true
  .attributes( ArgumentAttributes {
    optional: true,
    multiple: true,  // This makes unilang collect values into Value::List
    ..Default::default()
  } )
```

**Reference**: Pattern verified in `/home/user1/pro/lib/wTools_3/module/move/unilang/tests/inc/phase2/complex_types_and_attributes_test.rs:187-198`

### Tokenization Bug Confirmed with Debug Output

After correcting wrun's API usage, added debug output to extract_commands function to see what unilang is actually parsing.

**Test Command**:
```bash
wrun .run command::"echo a" command::"echo b" command::"echo c" command::"echo d" parallel::2
```

**Debug Output**:
```
DEBUG: arguments keys: ["command2", "timeout", "command3", "verbose", "dry", "output", "command", "command1", "working_dir", "command4", "parallel"]
DEBUG: command = Some(List([String("echo"), String("echo"), String("echo"), String("echo")]))
DEBUG: command1 = Some(String("a"))
DEBUG: command2 = Some(String("b"))
DEBUG: command3 = Some(String("c"))
DEBUG: command4 = Some(String("d"))
```

**Analysis**:
1. unilang v0.22.0 is splitting each quoted string on spaces
2. First word goes into `command` list (4 instances of "echo")
3. Second word somehow gets assigned to `command1`, `command2`, `command3`, `command4`
4. This is completely wrong - each `command::"echo a"` should be ONE value, not split

**Execution Result**: Only 3 commands ran ("echo a", "echo b", "echo c"), 4th command lost

### Conclusion

**Root Cause**: The bug is in **unilang's tokenization**, NOT in wrun's usage.

Even after correcting wrun to use the proper `Kind::String` with `multiple: true` pattern (matching unilang's own test examples), unilang v0.22.0 still incorrectly tokenizes quoted strings by splitting them on spaces.

**Evidence**:
- wrun's ArgumentDefinition now matches unilang test examples exactly
- Debug output proves unilang is misparsing: `command::"echo a"` → `command="echo"` + `command1="a"`
- Bug exists across all tested versions (0.12.0, 0.22.0, git 941f6567)
- v0.22.0 has WORSE tokenization bugs than v0.12.0

**Status**: **BUG CONFIRMED IN UNILANG - WRUN USAGE CORRECTED BUT BUG REMAINS**

**Files Modified**:
- `/home/user1/pro/lib/willbe/module/wrun/src/cli/mod.rs` - corrected ArgumentDefinition
- `/home/user1/pro/lib/willbe/module/wrun/Cargo.toml` - updated to unilang 0.22.0
- `/home/user1/pro/lib/willbe/module/wrun_core/Cargo.toml` - published versions
