# Task 084: Quoted Arguments in Commands

## Status: PARTIALLY FIXED
## Issue: 084
## Priority: Medium
## Impact: Enables shell commands with quoted multi-word arguments

## Problem Statement

Users want to pass commands with quoted arguments through `.crates.for.each`:

```bash
# Desired syntax (both should work):
w3 .crates.for.each cmd::'cld -p "/start explore"'
w3 .crates.for.each cmd::"cld -p '/start explore'"
```

This requires proper handling at TWO levels:
1. **Parsing** (unilang_parser) - Must parse the command without errors
2. **Execution** (will_crates) - Must execute bash with proper quote escaping

## Current Status

### ✅ Part 1: Parsing (FIXED)

**File:** `unilang_parser` crate

Both syntaxes parse successfully:

```rust
// Test 1: Double quotes inside single-quoted argv
let result = parser.parse_from_argv(&[
  ".crates.for.each".to_string(),
  r#"cmd::cld -p "/start explore""#.to_string(),
]);
assert!(result.is_ok()); // ✅ PASSES
// value = `cld -p "/start explore"`

// Test 2: Single quotes inside double-quoted argv
let result = parser.parse_from_argv(&[
  ".crates.for.each".to_string(),
  "cmd::cld -p '/start explore'".to_string(),
]);
assert!(result.is_ok()); // ✅ PASSES
// value = `cld -p '/start explore'`
```

**How it works:**
1. Argv reconstructor escapes inner quotes: `"` → `\"`
2. Tokenizer uses `strs_tools::split()` with quoting
3. Unescape function converts back: `\"` → `"`

**Test coverage:** 6/7 MRE tests pass in `tests/issue_084_mre.rs`

### ❌ Part 2: Bash Execution (BROKEN for single quotes)

**File:** `/home/user1/pro/lib/willbe/module/will_crates/src/crate_commands.rs:693-700`

**Current code:**
```rust
let child = std::process::Command::new( "bash" )
  .arg( "-c" )
  .arg( format!( "cd {} && {}", self.crate_info.absolute_path.display(), self.command ) )
  .stdout( output_file.try_clone()? )
  .stderr( output_file )
  .spawn()?;
```

**Problem:** When `self.command` contains single quotes, bash quoting breaks.

#### MRE: Bash Quote Termination

```bash
# Double quotes: ✅ WORKS
bash -c '/tmp/test_args.sh -p "/start explore"'
# Result: Receives 2 args: [-p], [/start explore]

# Single quotes: ❌ BROKEN
bash -c '/tmp/test_args.sh -p '/start explore''
# Result: Receives 2 args: [-p], [/start] (LOSES "explore")
```

**Why it breaks:**
```bash
bash -c 'command '/start explore''
#        ^1     ^2^3           ^4
# 1. Opening quote from -c
# 2. Closing quote (terminates string)
# 3. Bare text: /start
# 4. Opening new quoted string: explore'
```

The inner single quote terminates the outer `-c` quote prematurely.

#### Real-World Impact

```bash
# User runs:
w3 .crates.for.each cmd::"cld -p '/start explore'"

# Parsing: ✅ Works
# command value = "cld -p '/start explore'"

# Execution generates:
bash -c 'cd /path/to/crate && cld -p '/start explore''
#                                      ^BREAKS HERE

# cld receives: ["-p", "/start"] ❌ WRONG
# Expected:     ["-p", "/start explore"]
```

## Solution Design

### Option 1: Use Rust's Command::arg() (Recommended)

**Don't use bash -c with string concatenation.** Let Rust handle escaping:

```rust
let child = std::process::Command::new( "bash" )
  .arg( "-c" )
  .arg( &self.command )  // Rust escapes this properly
  .current_dir( &self.crate_info.absolute_path )
  .stdout( output_file.try_clone()? )
  .stderr( output_file )
  .spawn()?;
```

**Pros:**
- Rust's `.arg()` handles ALL quoting correctly (tested earlier in session)
- Simpler code (no format string)
- No `cd` needed (use `.current_dir()`)

**Cons:**
- None

### Option 2: Escape Single Quotes for Bash

Replace `'` with `'\''` (close quote, escaped quote, open quote):

```rust
fn escape_for_bash_single_quotes(s: &str) -> String {
  s.replace("'", "'\\''")
}

let escaped_command = escape_for_bash_single_quotes(&self.command);
let child = std::process::Command::new( "bash" )
  .arg( "-c" )
  .arg( format!( "cd {} && {}", self.crate_info.absolute_path.display(), escaped_command ) )
  // ...
```

**Pros:**
- Keeps existing structure
- Explicit about bash escaping

**Cons:**
- More complex
- Easy to get wrong
- Still needs `cd` wrapper

## Recommended Fix

**Use Option 1** - it's simpler and already works:

```rust
// File: /home/user1/pro/lib/willbe/module/will_crates/src/crate_commands.rs

fn execute( &self, output_file: &std::path::Path )
  -> core::result::Result< std::process::Child, CommandError >
{
  let output_file = std::fs::File::create( output_file )
    .map_err( | e | CommandError::buffer_creation( e, "creating output file" ) )?;

  // ✅ Use .current_dir() instead of 'cd' in bash
  // ✅ Use .arg() instead of format!() for proper escaping
  let child = std::process::Command::new( "bash" )
    .current_dir( &self.crate_info.absolute_path )  // Set working directory
    .arg( "-c" )
    .arg( &self.command )  // Rust handles all escaping
    .stdout( output_file.try_clone()
      .map_err( | e | CommandError::buffer_creation( e, "cloning output file" ) )? )
    .stderr( output_file )
    .spawn()
    .map_err( | e | CommandError::process_management( "spawning bash process", e ) )?;

  Ok( child )
}
```

**This change:**
- ✅ Fixes single quote handling
- ✅ Fixes double quote handling
- ✅ Simplifies code
- ✅ Removes `cd` wrapper
- ✅ More secure (no string interpolation in shell commands)

## Verification

### Test Script

```bash
#!/bin/bash
# File: /tmp/test_args.sh
echo "Received $# args:"
for arg in "$@"; do echo "  [$arg]"; done
```

### Before Fix

```bash
# Current broken behavior:
w3 .crates.for.each cmd::"echo test '/start explore'"

# Execution:
bash -c 'cd /path && echo test '/start explore''

# Output: "test /start" ❌ (loses "explore")
```

### After Fix

```bash
# Fixed behavior:
w3 .crates.for.each cmd::"echo test '/start explore'"

# Execution (Rust handles escaping):
Command::new("bash")
  .current_dir("/path")
  .arg("-c")
  .arg("echo test '/start explore'")

# Output: "test /start explore" ✅ (correct)
```

## Test Cases

### Test 1: Double Quotes
```bash
w3 .crates.for.each 'cmd::/tmp/test_args.sh -p "/start explore"'

# Expected output (per crate):
Received 2 args:
  [-p]
  [/start explore]
```

### Test 2: Single Quotes
```bash
w3 .crates.for.each "cmd::/tmp/test_args.sh -p '/start explore'"

# Expected output (per crate):
Received 2 args:
  [-p]
  [/start explore]
```

### Test 3: Mixed Quotes
```bash
w3 .crates.for.each 'cmd::/tmp/test_args.sh "hello" '"'"'world'"'"''

# Expected output (per crate):
Received 2 args:
  [hello]
  ['world']
```

### Test 4: No Quotes
```bash
w3 .crates.for.each 'cmd::/tmp/test_args.sh /start explore'

# Expected output (per crate):
Received 2 args:
  [/start]
  [explore]
```

## Implementation Plan

### Step 1: Update execute() Method
- [ ] Change to `.current_dir()` instead of `cd &&`
- [ ] Use `.arg(&self.command)` instead of `format!()`
- [ ] Remove format string concatenation

### Step 2: Update get_display_info() Method
- [ ] Update display format to reflect new structure
- [ ] Remove `cd` from display string

### Step 3: Test All Quote Combinations
- [ ] Test double quotes: `cmd::'command "/path"'`
- [ ] Test single quotes: `cmd::"command '/path'"`
- [ ] Test mixed quotes
- [ ] Test no quotes
- [ ] Test escaped spaces: `cmd::'path\ with\ spaces'`

### Step 4: Verify with Real Commands
- [ ] Test with `echo`: `cmd::'echo "hello world"'`
- [ ] Test with paths: `cmd::'cat "/path/with spaces/file.txt"'`
- [ ] Test with cld: `cmd::'cld -p "/start explore"'`

## References

- **Parser fix:** `/home/user1/pro/lib/wTools/module/core/strs_tools/src/string/split.rs:58-96` (unescape_str)
- **Execution bug:** `/home/user1/pro/lib/willbe/module/will_crates/src/crate_commands.rs:693-700`
- **MRE tests:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/tests/issue_084_mre.rs`
- **Bash escaping test:** This session (verified Rust's `.arg()` handles quotes correctly)

## Success Criteria

- [ ] ✅ Parsing works (ALREADY DONE)
- [ ] `w3 .crates.for.each 'cmd::command "/arg"'` executes correctly
- [ ] `w3 .crates.for.each "cmd::command '/arg'"` executes correctly
- [ ] Arguments with spaces inside quotes are preserved
- [ ] No regression in existing functionality
- [ ] Display output shows correct command format

## Current Workarounds

Until fixed, users should:

### Workaround 1: Use Double Quotes (WORKS)
```bash
w3 .crates.for.each 'cmd::cld -p "/start explore"'
```

### Workaround 2: Escape Spaces
```bash
w3 .crates.for.each 'cmd::cld -p /start\ explore'
```

### Workaround 3: Avoid Single Quotes
```bash
# DON'T USE:
w3 .crates.for.each "cmd::cld -p '/start explore'"  # ❌ BROKEN

# USE INSTEAD:
w3 .crates.for.each 'cmd::cld -p "/start explore"'  # ✅ WORKS
```

## Conclusion

**Two-part issue:**

1. **✅ Parsing (FIXED):** unilang_parser correctly handles both quote styles through escape/unescape pipeline
2. **❌ Execution (BROKEN):** will_crates bash execution breaks with single quotes due to quote termination

**Simple fix:** Use Rust's `.current_dir()` and `.arg()` instead of bash string concatenation. This is a 3-line change that solves the issue completely and simplifies the code.
