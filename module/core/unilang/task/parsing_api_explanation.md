# Unilang Parsing APIs: Complete Explanation

## Overview

Unilang exposes **two layers** of parsing APIs:

1. **Low-level:** `unilang_parser` crate - Pure parsing, no execution
2. **High-level:** `unilang` crate's `Pipeline` - Parse + validate + execute

---

## Low-Level API: unilang_parser

**Location:** `unilang_parser::Parser`

### Function 1: `parse_single_instruction`

```rust
pub fn parse_single_instruction(&self, input: &str)
  -> Result<GenericInstruction, ParseError>
```

**Purpose:** Parse a single command from a string

**Input:** String like `".video.search query::\"llm rust\""`

**Process:**
1. Validates quote completeness
2. Tokenizes using `strs_tools::string::split::split()` with `quoting(true)`
3. Splits on delimiters: space, tab, newline, `::`, etc.
4. **Respects quotes** - quoted strings are atomic tokens
5. Builds `GenericInstruction` with command path and arguments

**Example:**
```rust
let parser = unilang_parser::Parser::new(Default::default());
let instruction = parser.parse_single_instruction(
  ".video.search query::\"llm rust\""
)?;
// instruction.named_arguments["query"] = "llm rust"
```

**When to use:**
- REPL loops (user types strings interactively)
- Embedded DSL parsing
- Configuration files
- **When you already have a properly formatted string**

---

### Function 2: `parse_from_argv`

```rust
pub fn parse_from_argv(&self, argv: &[String])
  -> Result<GenericInstruction, ParseError>
```

**Purpose:** Parse from OS command-line arguments (argv array)

**Input:** Array like `[".video.search", "query::llm", "rust"]`

**Process:**
1. Iterates through argv elements
2. For `key::value` elements:
   - Combines subsequent elements until next `::` or `.`
   - Example: `["query::llm", "rust"]` → combines to `"llm rust"`
3. Adds quotes around multi-word values: `query::"llm rust"`
4. **BUG:** Joins tokens back to string and calls `parse_single_instruction()`
5. This **re-tokenizes** the string, breaking quoted boundaries!

**Current Code (BUGGY):**
```rust
// Lines 1133-1164
if value.contains(' ') {
  tokens.push(format!("{key}::\"{value}\""));  // ← Adds quotes
}
// ...
let command_str = tokens.join(" ");  // ← Joins to string
self.parse_single_instruction(&command_str)  // ← BUG: Re-tokenizes!
```

**Why it breaks:**
```rust
// After reconstruction:
tokens = ["query::\"llm rust\""]
command_str = "query::\"llm rust\""

// parse_single_instruction re-tokenizes:
split("..:") → ["query", "", "llm rust"]  // ← Splits on :: BEFORE quote processing!
// Result: "llm" and "rust" become separate tokens
```

**Example:**
```rust
let parser = unilang_parser::Parser::new(Default::default());

// Simulates: user types query::"llm rust"
// Bash outputs: ["query::llm rust"] (quotes removed by shell)
let instruction = parser.parse_from_argv(&[
  ".video.search".to_string(),
  "query::llm rust".to_string()  // ← Shell combined this
])?;

// EXPECTED: query = "llm rust"
// ACTUAL: query = "llm", then "rust" becomes orphan token
```

**When to use:**
- CLI applications using `std::env::args()`
- When you receive pre-tokenized argv from OS
- **Currently BROKEN for multi-word values**

---

### Function 3: `parse_multiple_instructions`

```rust
pub fn parse_multiple_instructions(&self, input: &str)
  -> Result<Vec<GenericInstruction>, ParseError>
```

**Purpose:** Parse multiple commands separated by `;;`

**Input:** String like `".cmd1 arg::val ;; .cmd2 arg::val2"`

**Process:**
1. Splits on `;;` delimiter
2. Calls `parse_single_instruction()` for each part
3. Returns vector of instructions

**Example:**
```rust
let parser = unilang_parser::Parser::new(Default::default());
let instructions = parser.parse_multiple_instructions(
  ".greet name::Alice ;; .greet name::Bob"
)?;
// instructions.len() == 2
```

**When to use:**
- Batch script processing
- Multiple commands in one input
- Pipeline automation

---

## High-Level API: unilang::Pipeline

**Location:** `unilang::pipeline::Pipeline`

The Pipeline combines: Parser → Semantic Analyzer → Interpreter

### Function 1: `process_command`

```rust
pub fn process_command(&self, command_str: &str, context: ExecutionContext)
  -> CommandResult
```

**Purpose:** Parse + validate + execute a command string

**Process:**
1. Calls `parser.parse_single_instruction(command_str)`
2. Calls `SemanticAnalyzer::analyze()` for validation
3. Calls `Interpreter::execute()` to run the command
4. Returns `CommandResult` with success/error/outputs

**Example:**
```rust
use unilang::pipeline::Pipeline;
use unilang::registry::CommandRegistry;

let registry = CommandRegistry::new();
let pipeline = Pipeline::new(registry);

let result = pipeline.process_command(
  ".video.search query::\"llm rust\"",
  ExecutionContext::default()
);

if result.success {
  println!("Output: {}", result.outputs[0].content);
} else {
  eprintln!("Error: {}", result.error.unwrap());
}
```

**When to use:**
- REPL loops
- Interactive shells
- When you have formatted strings

---

### Function 2: `process_command_simple`

```rust
pub fn process_command_simple(&self, command_str: &str) -> CommandResult
```

**Purpose:** Same as `process_command` but uses default context

**Example:**
```rust
let result = pipeline.process_command_simple(".help");
```

**When to use:**
- Simple CLI tools
- When you don't need custom execution context

---

### Function 3: `process_command_from_argv`

```rust
pub fn process_command_from_argv(&self, argv: &[String], context: ExecutionContext)
  -> CommandResult
```

**Purpose:** Parse argv + validate + execute

**Process:**
1. Calls `parser.parse_from_argv(argv)`  ← **BUG IS HERE**
2. Semantic analysis
3. Execution
4. Returns result

**Example:**
```rust
let argv: Vec<String> = std::env::args().collect();
let result = pipeline.process_command_from_argv(&argv, context);
```

**When to use:**
- CLI applications
- **Currently BROKEN for multi-word params**

---

### Function 4: `process_command_from_argv_simple`

```rust
pub fn process_command_from_argv_simple(&self, argv: &[String]) -> CommandResult
```

**Purpose:** Same as `process_command_from_argv` with default context

---

### Function 5: `process_batch`

```rust
pub fn process_batch(&self, commands: &[&str], context: ExecutionContext)
  -> BatchResult
```

**Purpose:** Process multiple commands, continue on errors

**Returns:** `BatchResult` with success rate and individual results

---

### Function 6: `process_sequence`

```rust
pub fn process_sequence(&self, commands: &[&str], context: ExecutionContext)
  -> BatchResult
```

**Purpose:** Process commands until first failure

---

## The Bug Explained

### Where the Bug Is

**File:** `unilang_parser/src/parser_engine.rs`
**Function:** `parse_from_argv` (lines 1076-1165)
**Specific lines:** 1163-1164

```rust
let command_str = tokens.join( " " );  // ← Line 1163
self.parse_single_instruction( &command_str )  // ← Line 1164: BUG
```

### How the Bug Manifests

**User types:**
```bash
$ mycli .video.search query::"llm rust"
```

**Shell processes:**
```
Bash removes quotes → argv = [".video.search", "query::llm rust"]
```

**`parse_from_argv` executes:**
```rust
// Step 1: Reconstruct (CORRECT)
arg = "query::llm rust"
split_once("::") → key="query", value="llm rust"
value.contains(' ') → true
tokens.push("query::\"llm rust\"")  // ← Adds quotes back

// Step 2: Join tokens (UNNECESSARY)
command_str = "query::\"llm rust\""

// Step 3: Re-parse (BUG)
parse_single_instruction("query::\"llm rust\"")
  ↓
  Uses strs_tools::split() which tokenizes on ::
  ↓
  BEFORE processing quotes, splits on ::
  ↓
  Results in broken tokens
```

**What `parse_single_instruction` sees:**
```
Input: "query::\"llm rust\""
Tokenizer splits on :: FIRST → ["query", "", "\"llm rust\""]
Then processes quotes → ["query", "", "llm rust"]  ← Lost the association!
```

### Why Tests Pass

The existing tests use:
```rust
cmd.args(vec![".video.search", r#"query::"llm rust""#]);
                               ^^^^^^^^^^^^^^^^^^^
                               Quotes in the string!
```

This is equivalent to shell command:
```bash
mycli .video.search 'query::"llm rust"'
                    ^^^^^^^^^^^^^^^^^^^
                    Outer quotes preserve inner quotes
```

So argv receives: `['query::"llm rust"']` with quotes intact.

But normal users type:
```bash
mycli .video.search query::"llm rust"
                    ^^^^^^^^^^^^^^^^^
                    No outer quotes!
```

And argv receives: `["query::llm rust"]` without quotes.

---

## The Fix

**Don't re-parse.** Build instruction directly from reconstructed argv.

**Replace lines 1161-1164:**

```rust
// OLD (BUGGY):
let command_str = tokens.join( " " );
self.parse_single_instruction( &command_str )

// NEW (FIXED):
// Build instruction directly from parsed argv
self.build_instruction_from_tokens( tokens )
```

**New helper method:**
```rust
fn build_instruction_from_tokens(&self, tokens: Vec<String>)
  -> Result<GenericInstruction, ParseError>
{
  // Parse command name
  let command_name = &tokens[0];

  // Parse arguments directly from tokens
  let mut named_arguments = BTreeMap::new();

  for token in &tokens[1..] {
    if let Some((key, value)) = token.split_once("::") {
      // Strip surrounding quotes if present
      let final_value = strip_quotes(value);
      named_arguments.insert(key.to_string(), vec![final_value]);
    }
  }

  Ok(GenericInstruction {
    command_path_slices: parse_command_path(command_name),
    named_arguments,
    positional_arguments: vec![],
    help_requested: false,
    overall_location: SourceLocation::None,
  })
}
```

This eliminates the double-parsing and respects argv boundaries.

---

## Summary Table

| Function | Input Type | Use Case | Current Status |
|----------|-----------|----------|----------------|
| `parse_single_instruction` | `&str` | REPL, DSL | ✅ Works |
| `parse_from_argv` | `&[String]` | CLI apps | ❌ **BROKEN** |
| `parse_multiple_instructions` | `&str` | Batch scripts | ✅ Works |
| `process_command` | `&str` | REPL + execute | ✅ Works |
| `process_command_from_argv` | `&[String]` | CLI + execute | ❌ **BROKEN** |
| `process_batch` | `&[&str]` | Multiple cmds | ✅ Works |

**Root Cause:** `parse_from_argv` reconstructs argv correctly but then re-parses, breaking it.

**Solution:** Build instruction directly without string reconstruction/re-parsing.
