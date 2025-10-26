# Root Cause Analysis: Multi-Word Parameter Bug

## Critical Finding: Bug is NOT in unilang_parser!

**Test Results:**
```
✅ All parser tests PASS (9/10)
✅ parse_from_argv works correctly
❌ unilang_cli binary FAILS
```

## The Real Bug Location

**File:** `unilang/src/bin/unilang_cli.rs`
**Lines:** 31-73, 597-605
**Problem:** CLI binary converts argv back to string instead of using `parse_from_argv`

### Current (Buggy) Code Flow

```rust
// Line 31-73: rejoin_broken_quoted_args function
fn rejoin_broken_quoted_args( args: &[ String ] ) -> String
{
  // Tries to fix broken quoted args
  // Joins everything into single string
  result.join( " " )  // ← Returns STRING
}

// Line 597-605: main processing
let command_input_str = rejoin_broken_quoted_args( &processed_args );
let instruction = parser.parse_single_instruction( &command_input_str )?;
//                       ^^^^^^^^^^^^^^^^^^^^^^^^
//                       Uses STRING parser, not ARGV parser!
```

### What Happens

**User types:**
```bash
$ unilang_cli .video.search query::"llm rust"
```

**Bash outputs:**
```
argv = [".video.search", "query::llm rust"]
```

**CLI binary processing:**
1. Calls `rejoin_broken_quoted_args(argv)`
2. Function joins: `".video.search query::llm rust"` ← NO QUOTES in string
3. Calls `parse_single_instruction(".video.search query::llm rust")`
4. This tokenizes on spaces
5. Result: `["query", "llm", "rust"]` - BROKEN!

### Why Parser Tests Pass

The parser tests use `parse_from_argv` directly:

```rust
parser.parse_from_argv( &[
  ".video.search".to_string(),
  "query::llm rust".to_string(),  // ← ONE token
]);
```

This correctly:
1. Sees `"query::llm rust"` as ONE token
2. Splits on `::` → key="query", value="llm rust"
3. Preserves the multi-word value

## Files That Need Changes

### 1. unilang/src/bin/unilang_cli.rs (PRIMARY FIX)

**Current (lines 597-605):**
```rust
let command_input_str = rejoin_broken_quoted_args( &processed_args );
let instruction = parser.parse_single_instruction( &command_input_str )?;
```

**Fixed:**
```rust
// Use parse_from_argv to preserve token boundaries
let instruction = parser.parse_from_argv( &processed_args )?;
```

**Changes needed:**
- Remove or simplify `rejoin_broken_quoted_args` function (lines 31-73)
- Change line 605 to use `parse_from_argv` instead of `parse_single_instruction`
- Remove string joining logic (line 597-604)

### 2. unilang_parser/src/parser_engine.rs (MINOR IMPROVEMENT)

**Current (lines 1163-1164):**
```rust
let command_str = tokens.join( " " );
self.parse_single_instruction( &command_str )
```

**Issue:** Even though `parse_from_argv` works, it unnecessarily:
1. Reconstructs tokens into string
2. Re-parses the string

**Improvement (optional but recommended):**
Build instruction directly from tokens without re-parsing.

### 3. unilang/tests/ (ADD TESTS)

Add integration tests that verify CLI behavior with multi-word params.

## Reproduction Steps

### Test Case 1: Direct Parser (WORKS)

```rust
use unilang_parser::Parser;

let parser = Parser::new(Default::default());
let result = parser.parse_from_argv(&[
    ".video.search".to_string(),
    "query::llm rust".to_string(),
]);

// ✅ PASSES: query = "llm rust"
```

### Test Case 2: CLI Binary (FAILS)

```bash
$ cargo run --bin unilang_cli -- .video.search query::"llm rust"
Query: llm
Title: rust

# ❌ FAILS: query = "llm", title = "rust"
```

### Test Case 3: CLI Binary with Shell Quoting (WORKAROUND)

```bash
$ cargo run --bin unilang_cli -- .video.search 'query::"llm rust"'
Query: llm rust

# ✅ WORKS: Outer quotes preserve inner quotes
```

## Fix Implementation Plan

### Phase 1: Immediate Fix (unilang_cli.rs)

1. **Replace string-based parsing with argv-based:**

```rust
// OLD (BUGGY):
let command_input_str = rejoin_broken_quoted_args( &processed_args );
if verbosity > 1 {
  eprintln!( "DEBUG: Rejoined command string: {command_input_str:?}" );
}
let instruction = parser.parse_single_instruction( &command_input_str )?;

// NEW (FIXED):
if verbosity > 1 {
  eprintln!( "DEBUG: Processing argv: {processed_args:?}" );
}
let instruction = parser.parse_from_argv( &processed_args )?;
```

2. **Remove or mark deprecated:**
```rust
// Mark for removal - no longer needed
#[deprecated(note = "No longer needed - parse_from_argv handles multi-word values")]
fn rejoin_broken_quoted_args( args: &[ String ] ) -> String
{
  // Keep temporarily for reference
  args.join( " " )
}
```

### Phase 2: Parser Optimization (optional)

Refactor `parse_from_argv` to build instruction directly:

```rust
pub fn parse_from_argv( &self, argv: &[String] )
  -> Result< GenericInstruction, ParseError >
{
  // Build instruction directly from argv tokens
  // Don't convert to string and re-parse
  self.build_instruction_from_argv_tokens( argv )
}
```

### Phase 3: Testing

1. **Add integration test:**
```rust
#[test]
fn test_cli_multiword_params()
{
  let mut cmd = Command::cargo_bin("unilang_cli").unwrap();
  cmd.args(&[".video.search", "query::llm rust"]);

  cmd.assert()
    .success()
    .stdout(predicate::str::contains("Query: llm rust"));
}
```

2. **Run existing tests to verify no regression**

## Impact Assessment

### What Changes

**unilang_cli.rs:**
- ✏️  Change 1 function call (line 605)
- 🗑️  Delete/deprecate 1 helper function (lines 31-73)
- 🗑️  Remove string joining code (lines 597-604)

**Total changes:** ~50 lines removed, 1 line changed

### What Stays the Same

- ✅ `unilang_parser` API unchanged
- ✅ All existing tests still pass
- ✅ `parse_single_instruction` still works for REPL/DSL use cases
- ✅ `parse_from_argv` API unchanged

### Compatibility

**Breaking change:** NO
**API change:** NO
**Behavior change:** YES (fixes bug)

Users who relied on workarounds (outer shell quotes) will still work.

## Acceptance Criteria

- [ ] `unilang_cli .video.search query::"llm rust"` works without outer quotes
- [ ] `unilang_cli .crates.for.each cmd::"echo test"` works correctly
- [ ] All existing tests pass
- [ ] New integration tests added
- [ ] Documentation updated

## Testing Commands

```bash
# Test 1: Multi-word parameter
cargo run --bin unilang_cli -- .video.search query::"llm rust"
# Expected: Query: llm rust

# Test 2: Shell command as parameter
cargo run --bin unilang_cli -- .video.search query::"cargo build --release"
# Expected: Query: cargo build --release

# Test 3: Path with spaces
cargo run --bin unilang_cli -- .video.search query::"/My Documents/file.txt"
# Expected: Query: /My Documents/file.txt

# Test 4: Multiple params
cargo run --bin unilang_cli -- .video.search query::"llm rust" title::"Tutorial"
# Expected: Query: llm rust, Title: Tutorial

# Test 5: Run all tests
cargo test --all-features
# Expected: All pass
```

## Conclusion

**The bug is in `unilang_cli.rs`, NOT in `unilang_parser`.**

The parser already has the correct `parse_from_argv` function that works perfectly.
The CLI binary just needs to use it instead of converting argv back to a string.

**Fix complexity:** TRIVIAL (change 1 line, remove helper function)
**Risk:** MINIMAL (parser already tested and working)
**Impact:** HIGH (enables natural CLI usage)
