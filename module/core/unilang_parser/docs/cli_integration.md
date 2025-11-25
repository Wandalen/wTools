# CLI Integration Guide: Avoiding the Argv Misuse Pitfall

## Overview

This guide explains the critical difference between parsing shell arguments and parsing strings, and why using the wrong parsing method leads to broken quote handling in CLI applications.

---

## The Problem: Shell Arguments Are Pre-Tokenized

When your CLI application receives arguments from the shell, the shell has **already performed tokenization**, handling quotes, escapes, and whitespace. The `argv` you receive is a vector of **pre-split tokens**.

### Example

```bash
$ my-app .deploy --name "Production Server" --region "us-east-1"
```

The shell passes to your app:
```rust
argv = [
  "my-app",
  ".deploy",
  "--name",
  "Production Server",    // ← Single token (quotes removed by shell)
  "--region",
  "us-east-1"             // ← Single token
]
```

Notice:
- Quotes are **removed** by the shell
- Values with spaces are **single tokens**
- Whitespace inside quotes is **preserved**

---

## The Pitfall: Re-Tokenizing Pre-Tokenized Arguments

### ❌ Incorrect Pattern (Breaks Quote Handling)

```rust
use unilang_parser :: { Parser, UnilangParserOptions };

fn main() -> Result< (), Box<dyn std ::error ::Error >> {
  let argv : Vec<String> = std ::env ::args().collect();

  // MISTAKE #1: Join pre-tokenized argv back into a string
  let joined = argv.join(" ");
  // joined = "my-app .deploy --name Production Server --region us-east-1"

  // MISTAKE #2: Use parse_single_instruction (which splits on whitespace)
  let parser = Parser ::new( UnilangParserOptions ::default() );
  let instruction = parser.parse_single_instruction(&joined)?;

  // Result: "Production Server" becomes TWO separate arguments: "Production" and "Server"
  // User expectation: BROKEN ❌

  Ok(())
}
```

### What Went Wrong

1. **Shell tokenized:** `"Production Server"` → single token `Production Server`
2. **You joined:** `["--name", "Production Server"]` → `"--name Production Server"`
3. **You re-split:** `"--name Production Server"` → `["--name", "Production", "Server"]`
4. **Result:** Original intent lost - user's quoted argument is now split incorrectly

### Why split_whitespace() Fails

`split_whitespace()` has **no quote handling**. It doesn't know that `"Production Server"` was originally quoted. All it sees is whitespace to split on.

---

## The Solution: Use parse_from_argv()

### ✅ Correct Pattern (Preserves Shell Tokenization)

```rust
use unilang_parser :: { Parser, UnilangParserOptions };

fn main() -> Result< (), Box<dyn std ::error ::Error >> {
  let argv : Vec<String> = std ::env ::args().collect();

  // CORRECT: Pass pre-tokenized argv directly
  let parser = Parser ::new( UnilangParserOptions ::default() );
  let instruction = parser.parse_from_argv(&argv)?;

  // Result: "Production Server" remains a SINGLE argument
  // User expectation: PRESERVED ✅

  Ok(())
}
```

### Why This Works

`parse_from_argv()` treats each argv element as a **pre-tokenized unit**. It doesn't re-split on whitespace because the shell already did that work correctly.

---

## When to Use Each Method

### Use `parse_from_argv(&argv)` When:

✅ Building a CLI application that receives arguments from the shell
✅ Receiving `std ::env ::args()` or similar
✅ User invokes your app like: `my-app .command "value with spaces"`

**Why:** Shell has already tokenized. Re-tokenizing breaks quote handling.

### Use `parse_single_instruction(input)` When:

✅ Parsing instruction strings from configuration files
✅ Accepting user input from a REPL or interactive prompt
✅ Processing embedded instructions in source code or scripts
✅ Input is a **raw string** that hasn't been shell-tokenized

**Why:** String needs tokenization. Quotes need to be parsed.

---

## Technical Deep Dive

### What parse_from_argv() Does

1. Takes pre-tokenized argv slice
2. Skips the first element (program name)
3. Treats each element as a complete token (no re-splitting)
4. Applies unilang syntax rules (command paths, named args, etc.)

```rust
pub fn parse_from_argv< S >( &self, argv : &[ S ] ) -> Result< GenericInstruction >
where
  S : AsRef< str > + core ::fmt ::Debug,
{
  // Convert argv to ParsedItem slices (no re-tokenization)
  let items = argv
    .iter()
    .skip( 1 )  // Skip program name
    .enumerate()
    .map( | ( idx, token ) | ParsedItem {
      raw : token.as_ref(),
      source_location : SourceLocation ::SliceSegment { slice_index : idx + 1 },
    })
    .collect();

  self.parse_instruction_from_items( items )
}
```

### What parse_single_instruction() Does

1. Takes a raw string
2. Uses `strs_tools` to tokenize (split on whitespace, handle quotes)
3. Applies unilang syntax rules

```rust
pub fn parse_single_instruction( &self, input : &str ) -> Result< GenericInstruction >
{
  // Tokenize the input string (handles quotes, escapes)
  let items = self.itemize_string( input )?;

  self.parse_instruction_from_items( items )
}
```

### Key Difference

- **parse_from_argv:** No tokenization (assumes pre-tokenized)
- **parse_single_instruction:** Full tokenization (assumes raw string)

---

## Common Scenarios

### Scenario 1: Standard CLI Application

```rust
// User runs: my-app .build --output "dist/my app"

fn main() -> Result< (), Box<dyn std ::error ::Error >> {
  let argv : Vec<String> = std ::env ::args().collect();

  let parser = Parser ::new( UnilangParserOptions ::default() );
  let instruction = parser.parse_from_argv(&argv)?;  // ✅ Correct

  // instruction.named_args["output"] = "dist/my app" (single value, space preserved)
  Ok(())
}
```

### Scenario 2: Configuration File Parsing

```rust
// Config file contains: .deploy region::"us-east-1" name::"Production Server"

fn parse_config( config_content : &str ) -> Result< GenericInstruction, Box<dyn std ::error ::Error >> {
  let parser = Parser ::new( UnilangParserOptions ::default() );
  let instruction = parser.parse_single_instruction(config_content)?;  // ✅ Correct

  // instruction.named_args["name"] = "Production Server" (quotes parsed correctly)
  Ok(())
}
```

### Scenario 3: Interactive REPL

```rust
// User types: .search query::"rust parser" limit::10

fn handle_repl_input( input : &str ) -> Result< GenericInstruction, Box<dyn std ::error ::Error >> {
  let parser = Parser ::new( UnilangParserOptions ::default() );
  let instruction = parser.parse_single_instruction(input)?;  // ✅ Correct

  Ok(())
}
```

### Scenario 4: ❌ WRONG - Re-tokenizing Shell Argv

```rust
// User runs: my-app .deploy --name "Production Server"

fn main() -> Result< (), Box<dyn std ::error ::Error >> {
  let argv : Vec<String> = std ::env ::args().collect();
  let joined = argv[1..].join(" ");  // ❌ WRONG: Loses token boundaries

  let parser = Parser ::new( UnilangParserOptions ::default() );
  let instruction = parser.parse_single_instruction(&joined)?;

  // BROKEN: instruction now has TWO args: "Production" and "Server"
  Ok(())
}
```

---

## Summary: Decision Tree

```
Is your input coming from the shell (std::env::args)?
│
├─ YES → Use parse_from_argv(&argv)
│         Reason: Shell already tokenized
│
└─ NO → Is your input a raw string?
        │
        ├─ YES → Use parse_single_instruction(input)
        │         Reason: String needs tokenization
        │
        └─ Are you converting argv to a string then re-parsing?
                  → ❌ STOP! This is the pitfall.
                    Use parse_from_argv(&argv) instead.
```

---

## Real-World Production Bug

This pitfall caused a real production bug in the `wflow` project:

**User Command:**
```bash
$ wflow .languages --path "src/my project"
```

**Expected Behavior:**
- `--path` argument receives single value: `"src/my project"`

**Actual Behavior (Bug):**
- argv was joined: `".languages --path src/my project"`
- Re-split on whitespace: `[".languages", "--path", "src/my", "project"]`
- Parser saw TWO positional args: `"src/my"` and `"project"`
- Command failed: "unexpected positional argument 'project'"

**Root Cause:**
Code used `argv.join(" ")` followed by `parse_single_instruction()`, destroying the shell's tokenization.

**Fix:**
Changed to `parse_from_argv(&argv)`, preserving shell tokenization.

---

## Prevention Checklist

When integrating unilang_parser into a CLI application:

- [ ] Are you receiving arguments from the shell (`std ::env ::args()`)?
- [ ] Are you using `parse_from_argv(&argv)` (NOT `parse_single_instruction`)?
- [ ] Are you avoiding `argv.join(" ")` or similar string concatenation?
- [ ] Have you tested with arguments containing spaces (e.g., `--name "foo bar"`)?
- [ ] Have you verified quote handling is preserved end-to-end?

---

## Related Documentation

- [Task 086: Prevent Argv Misuse Pitfall](../task/086_prevent_argv_misuse_pitfall.md) - Original task specification
- [Parser Engine Source](../src/parser_engine.rs) - Implementation of parse_from_argv() and parse_single_instruction()
- [Argv Multiword Bug Test](../tests/argv_multiword_bug_test.rs) - Regression test for this pitfall

---

## Questions?

If you're unsure which method to use, ask yourself:

**"Has the input already been tokenized by something else (like the shell)?"**

- **YES** → Use `parse_from_argv()` (don't re-tokenize)
- **NO** → Use `parse_single_instruction()` (needs tokenization)
