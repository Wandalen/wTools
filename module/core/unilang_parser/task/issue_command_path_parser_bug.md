# ISSUE: Command Path Parser Incorrectly Consumes Named Arguments

## Status: ✅ **RESOLVED** - Fixed 2025-11-01

## Priority: **HIGH** - Affects `parse_single_instruction()` API

---

## Resolution Summary

**Fixed:** 2025-11-01
**Fix Location:** `src/parser_engine.rs:385-440`
**Test Coverage:** 12 comprehensive tests (all passing)
**Verification:** 172/172 unit tests pass, zero regressions

### What Was Fixed

Added lookahead logic to command path parser to detect `name::value` patterns before consuming identifiers:

```rust
// Clone iterator to look at next item without consuming current
let mut lookahead_iter = items_iter.clone();
lookahead_iter.next(); // Skip current item

if let Some( next_item ) = lookahead_iter.peek() {
  let is_named_arg_operator = match &next_item.kind {
    UnilangTokenKind::Operator( op ) => *op == "::" || *op == " :: ",
    _ => false,
  };

  if is_named_arg_operator {
    break; // Don't consume, let argument parser handle it
  }
}
```

### Results

**Before Fix:**
- `parse_single_instruction("cmd::value")` → ERROR (orphaned operator)
- `parse_single_instruction("arg::\"quoted\"")` → ERROR
- Named-only arguments completely broken

**After Fix:**
- `parse_single_instruction("cmd::value")` → ✅ SUCCESS
- `parse_single_instruction("arg::\"quoted\"")` → ✅ SUCCESS
- All 12 edge cases covered and working

### Test Results

```
All diagnostic tests: 12/12 PASS ✅
Full test suite: 172/172 PASS ✅
Clippy: Clean ✅
Regressions: 0 ✅
```

### Documentation

- **Source Code:** 3-field comment (Fix marker, Root cause, Pitfall) at parser_engine.rs:389-401
- **Test Suite:** 5-section documentation at tests/diagnostic_real_bug.rs:1-93
- **Coverage:** 12 tests covering named-only args, multiple args, quoted values, error detection, API consistency

### Bonus Fixes

Fixed 3 incorrectly-failing tests that expected errors for valid named-only arguments:
- `comprehensive_tests::ct5_1_single_str_no_path_named_arg_only`
- `error_reporting_tests::error_unexpected_delimiter_location_str`
- `error_reporting_tests::unexpected_colon_colon_no_name`

Also fixed `issue_084_mre::mre_direct_parse_with_escaped_quotes` (collateral benefit).

---

## Problem Statement

The command path parser in `parser_engine.rs` (lines 385-404) incorrectly consumes identifiers without checking if they're part of named argument patterns (`name::value`). This causes the parser to misinterpret named arguments as command path segments, leading to "orphaned operator" errors.

### Minimal Reproducible Example (MRE)

```rust
use unilang_parser::{ Parser, UnilangParserOptions };

fn main() {
  let parser = Parser::new(UnilangParserOptions::default());

  // This should parse as: named_arg "cmd" with value "value"
  let result = parser.parse_single_instruction("cmd::value");

  // ACTUAL BEHAVIOR: Fails with error:
  // ParseError {
  //   kind: Syntax("Named argument operator '::' cannot appear by itself"),
  //   location: Some(StrSpan { start: 3, end: 5 })
  // }

  // EXPECTED BEHAVIOR: Should parse successfully as:
  // - command_path: [] (empty)
  // - named_arguments: { "cmd": ["value"] }
}
```

### Error Manifestation

```
ParseError {
  kind: Syntax("Named argument operator '::' cannot appear by itself"),
  location: Some(StrSpan { start: 3, end: 5 })
}
```

The error message is misleading - `::` is NOT by itself, it's part of the named argument pattern `cmd::value`. The real issue is that the command path parser already consumed `cmd`, leaving only `::value` for the argument parser.

---

## Root Cause Analysis

### Location

**File:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/src/parser_engine.rs`
**Function:** `parse_command_path`
**Lines:** 385-404

### Technical Analysis

The command path parser processes tokens in this order:

1. **Line 385**: Matches on `UnilangTokenKind::Identifier`
2. **Line 397**: Adds identifier to `command_path_slices`
3. **Line 399**: **Consumes the token** via `items_iter.next()`
4. **Line 401-404**: Checks if next token is dot; if not, breaks

**The Bug:** The parser never checks if the identifier is followed by `::`, which would indicate it's a named argument, NOT a command path segment.

### Code Section (Lines 385-404)

```rust
UnilangTokenKind ::Identifier( ref s ) =>
{
  if command_path_slices.is_empty() || last_token_was_dot
  {
    if s.contains( '-' )
    {
      return Err( ParseError ::new(
        ErrorKind ::Syntax( format!( "Invalid character '-' in command path segment '{s}'" ) ),
        item.adjusted_source_location.clone(),
      ));
    }
    command_path_slices.push( s.to_string() );  // ← Adds to command path
    last_token_was_dot = false;
    items_iter.next(); // ← BUG: Consumes without checking for ::
  }
  else
  {
    break; // End of command path
  }
}
```

### Execution Flow for `cmd::value`

**Input after tokenization (from strs_tools):**
```rust
[
  Token { kind: Identifier("cmd"), ... },
  Token { kind: Operator("::"), ... },
  Token { kind: Identifier("value"), ... },
]
```

**Parser execution:**

1. Command path parser starts
2. Sees `Identifier("cmd")`
3. Checks: `command_path_slices.is_empty()` → `true`
4. Adds `"cmd"` to `command_path_slices` (line 397)
5. **Consumes token** (line 399) → iterator now points to `Operator("::")`
6. Peeks next token → sees `Operator("::")` (not dot)
7. Breaks from loop (line 401-404)
8. **Returns to argument parser** with remaining tokens: `["::", "value"]`
9. Argument parser sees orphaned `::` → **ERROR**

**What SHOULD happen:**

1. Command path parser starts
2. Sees `Identifier("cmd")`
3. **Peeks ahead** → sees `Operator("::")`
4. Recognizes pattern `identifier::` → **NOT a command path segment**
5. Does NOT consume token
6. Breaks from loop
7. **Returns to argument parser** with tokens: `["cmd", "::", "value"]`
8. Argument parser correctly parses named argument → **SUCCESS**

---

## Impact Assessment

### Affected APIs

**Broken:**
- ❌ `parse_single_instruction("cmd::value")` - Fails with orphaned operator error
- ❌ `parse_single_instruction("arg::\"quoted value\"")` - Same error
- ❌ Any direct parsing of named-only arguments (no command path)

**Working (via workaround):**
- ✅ `parse_from_argv(["cmd::value"])` - Works due to workaround in lines 1287-1341
- ✅ `parse_single_instruction(".test arg::value")` - Works because `.test` is consumed as command path first

### Current Workaround

**File:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/src/parser_engine.rs`
**Lines:** 1287-1341 (55 lines)

The workaround pre-processes argv input to escape quotes before passing to the tokenizer. This works around BOTH the (now-proven-incorrect) ISSUE-STRS-001 AND this bug, but only for the `parse_from_argv()` path.

**Why this is problematic:**
1. Doesn't fix `parse_single_instruction()` (direct parsing API)
2. Creates two different code paths with different behaviors
3. Violates architectural separation of concerns

---

## Recommended Fix

### Required Changes

**File:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/src/parser_engine.rs`
**Function:** `parse_command_path`
**Lines:** 385-404

### Implementation Strategy

Add lookahead logic to check if identifier is followed by `::` before consuming:

```rust
UnilangTokenKind ::Identifier( ref s ) =>
{
  if command_path_slices.is_empty() || last_token_was_dot
  {
    // NEW: Check if this identifier is part of a named argument pattern
    if let Some( next_item ) = items_iter.peek()
    {
      if matches!( next_item.kind, UnilangTokenKind::Operator( "::" | " :: " ) )
      {
        // This is a named argument, not a command path segment
        break;
      }
    }

    if s.contains( '-' )
    {
      return Err( ParseError ::new(
        ErrorKind ::Syntax( format!( "Invalid character '-' in command path segment '{s}'" ) ),
        item.adjusted_source_location.clone(),
      ));
    }
    command_path_slices.push( s.to_string() );
    last_token_was_dot = false;
    items_iter.next(); // Now safe to consume
  }
  else
  {
    break; // End of command path
  }
}
```

### Test Cases to Verify

All test cases are already implemented in `/home/user1/pro/lib/wTools/module/core/unilang_parser/tests/diagnostic_real_bug.rs`:

1. **Test 1: Simple named argument (no command path)**
   ```rust
   parse_single_instruction("cmd::value")
   // Expected: command_path=[], named_arguments={"cmd": ["value"]}
   ```

2. **Test 2: Command path + named argument**
   ```rust
   parse_single_instruction(".test arg::value")
   // Expected: command_path=["test"], named_arguments={"arg": ["value"]}
   ```

3. **Test 3: Named argument with quoted value**
   ```rust
   parse_single_instruction(r#"cmd::"value with \"inner\" quotes""#)
   // Expected: command_path=[], named_arguments={"cmd": ["value with \"inner\" quotes"]}
   ```

4. **Test 4: Proof that tokenization is correct**
   ```rust
   // Verifies strs_tools produces correct tokens: ["cmd", "::", "value"]
   ```

---

## Success Criteria

### Definition of Done

1. ✅ All 4 tests in `tests/diagnostic_real_bug.rs` pass without `#[should_panic]`
2. ✅ Existing test suite continues to pass (no regressions)
3. ✅ `parse_single_instruction("cmd::value")` works correctly
4. ✅ Can remove or simplify workaround in lines 1287-1341
5. ✅ Both API paths (`parse_from_argv` and `parse_single_instruction`) behave consistently

### Verification Commands

```bash
# Run diagnostic tests (currently 3 fail, 1 passes)
cargo test --test diagnostic_real_bug

# After fix, all should pass:
# test diagnostic_simple_named_arg_no_quotes ... ok
# test diagnostic_command_plus_named_arg ... ok
# test diagnostic_escaped_quotes_shows_same_bug ... ok
# test diagnostic_strs_tools_is_correct ... ok

# Run full test suite
w3 .test l::3
```

---

## Related Issues

### Upstream (Resolved)

**ISSUE-STRS-001** in strs_tools was filed based on incorrect diagnosis. Deep investigation revealed that strs_tools correctly handles escape sequences and produces correct tokens. The issue has been marked as resolved.

**Evidence:**
- File: `/home/user1/pro/lib/wTools/module/core/strs_tools/task/issue_001_escaped_quotes_in_quoted_strings.md`
- Status: ✅ RESOLVED - Already Implemented
- Tests: `/home/user1/pro/lib/wTools/module/core/strs_tools/tests/issue_001_mre.rs` (5/5 PASS)

### Downstream (This Issue)

This is the ACTUAL bug causing the parsing failures originally attributed to strs_tools.

---

## Timeline

**Discovered:** 2025-11-01 (during deep investigation of ISSUE-084)
**Priority:** HIGH
**Complexity:** MEDIUM (simple lookahead logic)
**Estimated Fix Time:** 1-2 hours (implementation + testing)

---

## Reproduction Steps

### Step 1: Navigate to Project

```bash
cd /home/user1/pro/lib/wTools/module/core/unilang_parser
```

### Step 2: Run Diagnostic Tests

```bash
# Run all diagnostic tests
cargo test --test diagnostic_real_bug

# You'll see:
# - diagnostic_simple_named_arg_no_quotes: FAILED (documents bug)
# - diagnostic_command_plus_named_arg: FAILED (documents bug)
# - diagnostic_escaped_quotes_shows_same_bug: FAILED (documents bug)
# - diagnostic_strs_tools_is_correct: PASSED (proves strs_tools works)
```

### Step 3: View Test Output

The tests show:
```
❌ Parse failed: ParseError {
  kind: Syntax("Named argument operator '::' cannot appear by itself"),
  location: Some(StrSpan { start: 3, end: 5 })
}
```

This proves the command path parser consumed `cmd`, leaving orphaned `::`.

---

## Additional Context

### Discovery Process

This bug was discovered during a deep investigation of ISSUE-084 (escaped quotes handling). The investigation revealed:

1. **Initial hypothesis (WRONG):** strs_tools can't handle escaped quotes
2. **Filed ISSUE-STRS-001** based on incorrect diagnosis
3. **User correction:** Proved strs_tools works perfectly
4. **Deep investigation:** Found escape handling at split.rs:462-498
5. **Real bug discovered:** Command path parser at parser_engine.rs:385-404

### Why This Was Missed

1. The `parse_from_argv()` path has a workaround that masks the bug
2. Most test cases use command paths (`.test arg::value`), not bare named arguments
3. The error message is misleading (blames `::` instead of command path logic)
4. Initial investigation focused on tokenization layer, not semantic layer

---

## Contact

**Reporter:** Deep investigation following user correction of ISSUE-STRS-001
**Date Reported:** 2025-11-01
**Diagnostic Tests:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/tests/diagnostic_real_bug.rs`

---

**Absolute Path to this File:**
`/home/user1/pro/lib/wTools/module/core/unilang_parser/task/issue_command_path_parser_bug.md`
