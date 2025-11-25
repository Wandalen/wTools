# Task: Add Argv-Based API to Unilang for Proper CLI Integration

**Priority:** ⚠️ **HIGH** - Architectural flaw affecting all CLI applications using unilang
**Status:** Feature Request
**Category:** API Design / Architecture
**Affects:** unilang v0.23.0 - v0.25.0 (current)
**Reporter:** willbe3 development team
**Date:** 2025-01-12

---

## Executive Summary

Unilang currently only provides string-based API (`process_command(command_str: &str)`), forcing CLI applications to reconstruct strings from OS-provided argv arrays. This lossy conversion destroys structural information and causes parse errors for perfectly valid shell input.

**Impact:** Every CLI application using unilang must implement complex workarounds to reconstruct the original argv structure through string manipulation with quotes.

**Solution:** Add argv-based API (`process_command_from_argv(argv: &[String])`) alongside the existing string API.

---

## The Problem

### Current API (String-Only)

```rust
pub fn process_command(&self, command_str: &str) -> CommandResult
```

### Information Loss Pipeline

```text
OS → Shell → Argv Array → [APPLICATION JOINS] → String → [UNILANG RE-TOKENIZES] → Argv Array
         ↑                                              ↑
    Original structure                      Re-created structure (LOSSY!)
```

**The flaw:** We start with perfect argv structure from OS, destroy it by joining, then try to recreate it by parsing. Information is **irreversibly lost** in this process.

---

## Minimal Reproducible Example (MRE)

### Setup

```bash
cd /path/to/mre
rustc --edition 2021 unilang_argv_api_request_mre.rs -o mre
./mre command::ls -la
```

**Shell processing:**
- User types: `./mre command::ls -la`
- Shell tokenizes into argv: `["command::ls", "-la"]`
- Program receives these as separate argv entries

### Current Behavior (BROKEN)

```rust
// Application must join argv
let command_str = argv.join(" ");  // "command::ls -la"

// Unilang re-tokenizes on spaces
unilang.process_command(&command_str);

// ❌ ERROR: "Unexpected token '-la'"
// Why: Unilang sees [command::ls, -la] as two tokens
// The '-' makes it look like a flag, not part of the value
```

### Expected Behavior

```rust
// Pass argv directly
unilang.process_command_from_argv(&argv);

// ✅ SUCCESS
// Why: Unilang knows ["command::ls", "-la"] were originally separate argv
// Can intelligently combine them: command = "ls -la"
```

### Test Output

See attached `unilang_argv_api_request_mre.rs` for full runnable example.

Expected output:
```
Method 1: Current String-Based API (BROKEN)
❌ ERROR: Unexpected token '-la' - looks like part of a value but was tokenized separately

Method 2: Proposed Argv-Based API (WORKS)
✅ Success: ParsedCommand { name: "command::ls", arguments: {"command": "ls -la"} }
```

---

## Real-World Impact

### Affected Use Case: willbe3 CLI Tool

```bash
# User wants to run this natural command:
w3 .crates.for.each command::"ls -la"

# Shell processing:
argv = ["w3", ".crates.for.each", "command::ls", "-la"]

# Current situation (BROKEN):
command_str = ".crates.for.each command::ls -la"
→ Parse error: "Unexpected token '-la'"

# Required workaround (BAD UX):
w3 .crates.for.each 'command::"ls -la"'
#                    ↑             ↑
#              Extra quotes required!
```

### Current Workarounds and Their Problems

**Workaround 1: Require user quoting**
```bash
w3 .crates.for.each 'command::"ls -la"'
```
- ❌ Poor UX - users must understand multi-layer shell quoting
- ❌ Error-prone - easy to get quotes wrong
- ❌ Documentation burden - needs extensive explanation

**Workaround 2: Intelligent argv reconstruction (current willbe3 fix)**
```rust
// 100+ lines of complex logic to reconstruct string with correct quoting
let reconstructed = reconstruct_with_smart_quoting(&argv);
unilang.process_command(&reconstructed);
```
- ❌ Complex - every application must reimplement
- ❌ Fragile - heuristics can fail on edge cases
- ❌ Maintenance burden - must update when unilang changes
- ❌ Duplicate logic - N applications × same reconstruction code

**Workaround 3: Shell script wrapper**
```bash
#!/bin/bash
# Wrap all argv with proper quotes
exec w3 .crates.for.each "command::\"$*\""
```
- ❌ Platform-specific - different for bash/zsh/fish/PowerShell
- ❌ Extra complexity - users need wrapper scripts
- ❌ Deployment burden - distribute scripts alongside binary

---

## Proposed Solution

### New API Method

Add to `Pipeline`:

```rust
impl Pipeline {
  /// Process command from OS argv array (recommended for CLI applications).
  ///
  /// This method preserves the original argv structure from the OS, avoiding
  /// information loss from string joining and re-tokenization.
  ///
  /// # Arguments
  ///
  /// * `argv` - Command-line arguments from std::env::args()
  ///   - First element should be the command name
  ///   - Following elements are arguments as separate entries
  ///   - Preserves shell tokenization boundaries
  ///
  /// # Examples
  ///
  /// ```rust
  /// use std::env;
  ///
  /// let argv: Vec<String> = env::args().skip(1).collect();
  /// let result = pipeline.process_command_from_argv(&argv)?;
  /// ```
  ///
  /// # Implementation Notes
  ///
  /// The argv-based parser should:
  /// 1. Recognize key::value patterns
  /// 2. Combine following argv entries that belong to a value
  /// 3. Stop combining when it sees another :: or command prefix (.)
  /// 4. Preserve the original tokenization boundaries
  ///
  #[must_use]
  pub fn process_command_from_argv(
    &self,
    argv: &[String],
    context: ExecutionContext
  ) -> CommandResult {
    // Implementation uses argv-aware parser
    unimplemented!()
  }

  /// Simple variant without context
  #[must_use]
  pub fn process_command_from_argv_simple(&self, argv: &[String]) -> CommandResult {
    self.process_command_from_argv(argv, ExecutionContext::default())
  }
}
```

### Parser Changes

Add to `unilang_parser`:

```rust
impl Parser {
  /// Parse instruction from argv array (preserves OS tokenization).
  pub fn parse_from_argv(
    &self,
    argv: &[String]
  ) -> Result<GenericInstruction, ParseError> {
    // Implementation that uses argv boundaries
    unimplemented!()
  }
}
```

---

## Implementation Strategy

### Phase 1: Add Argv API (Non-Breaking)

1. Add `parse_from_argv()` to `unilang_parser::Parser`
2. Add `process_command_from_argv()` to `unilang::Pipeline`
3. Keep existing string-based methods unchanged
4. Comprehensive test suite for argv parsing

**Tests needed:**
- Simple commands: `["cmd", "arg1", "arg2"]`
- Key-value pairs: `["cmd", "key::value"]`
- Values with spaces: `["cmd", "key::multi", "word", "value"]`
- Values with dashes: `["cmd", "command::ls", "-la"]`
- Multiple parameters: `["cmd", "key1::val1", "key2::val2"]`
- Empty values: `["cmd", "key::"]`
- Edge cases: Commands starting with `.`, special chars

### Phase 2: Documentation and Migration

1. Update documentation to recommend argv API for CLI
2. Update examples to show both APIs
3. Add migration guide for existing applications
4. Document when to use string vs argv API

**When to use each API:**
- **Argv API** - CLI applications receiving OS argv
- **String API** - REPL, interactive shells, embedded DSL parsing

### Phase 3: Deprecation Path (Optional)

Consider adding lints to detect string API misuse:

```rust
// Detect when applications do argv.join(" ")
// Suggest using process_command_from_argv() instead
#[deprecated(
  since = "0.26.0",
  note = "Consider using process_command_from_argv() for CLI applications"
)]
pub fn process_command(&self, command_str: &str) -> CommandResult
```

---

## Benefits

### For Unilang

- ✅ **Architectural correctness** - API matches the problem domain
- ✅ **Eliminates workarounds** - Applications stop doing string gymnastics
- ✅ **Better separation of concerns** - Parsing vs tokenization clearly separated
- ✅ **Type safety** - Argv boundaries are explicit in type system

### For Applications

- ✅ **Simpler integration** - Direct argv pass-through
- ✅ **Better UX** - Natural shell syntax works
- ✅ **Less code** - No reconstruction logic needed
- ✅ **Fewer bugs** - No heuristic parsing failures
- ✅ **Maintainability** - Changes isolated to unilang

### For Users

- ✅ **Natural syntax** - Commands work as expected
- ✅ **Less documentation** - No special quoting rules
- ✅ **Fewer errors** - Shell handles tokenization correctly
- ✅ **Consistency** - Works like other CLI tools

---

## Alternative Considered: Fix Parser to Handle Joined Strings

**Rejected because:**

1. **Lossy operation** - Once argv is joined, information is gone
   - Can't distinguish `["command::ls", "-la"]` from `["command::ls -la"]`
   - Heuristics are fragile and fail on edge cases

2. **Wrong layer** - Parser shouldn't guess original tokenization
   - Shell already tokenized correctly
   - Re-tokenizing is redundant work
   - Parser operates on wrong abstraction level

3. **Complexity** - Would need complex heuristics
   - Detect when `-` starts a flag vs is part of value
   - Handle edge cases like `command::--help` vs `command::ls --help`
   - Different rules for different positions

4. **Backwards compatibility** - Would change parsing semantics
   - Existing apps rely on current tokenization
   - Can't add heuristics without breaking changes

**Conclusion:** The right fix is to operate at the correct abstraction level (argv), not patch over lossy conversion.

---

## Backward Compatibility

### Guarantee

- ✅ **100% backward compatible** - All existing code continues to work
- ✅ **No breaking changes** - String API unchanged
- ✅ **Opt-in adoption** - Applications choose when to migrate
- ✅ **Deprecation optional** - Can keep both APIs indefinitely

### Migration Path

Existing applications can migrate incrementally:

```rust
// Before
let args: Vec<String> = std::env::args().skip(1).collect();
let command_str = args.join(" ");
let result = pipeline.process_command_simple(&command_str);

// After
let argv: Vec<String> = std::env::args().skip(1).collect();
let result = pipeline.process_command_from_argv_simple(&argv);
```

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_argv_with_dashes() {
  let argv = vec!["command::ls".to_string(), "-la".to_string()];
  let instruction = parser.parse_from_argv(&argv).unwrap();
  assert_eq!(instruction.arguments["command"], "ls -la");
}

#[test]
fn test_argv_multiple_params() {
  let argv = vec![
    ".crates.for.each".to_string(),
    "command::cargo".to_string(),
    "build".to_string(),
    "filter::w*".to_string()
  ];
  let instruction = parser.parse_from_argv(&argv).unwrap();
  assert_eq!(instruction.arguments["command"], "cargo build");
  assert_eq!(instruction.arguments["filter"], "w*");
}
```

### Integration Tests

- Test with actual CLI binary
- Test with various shells (bash, zsh, fish)
- Test on different platforms (Linux, macOS, Windows)
- Test with complex real-world commands

### Regression Tests

- Ensure string API behavior unchanged
- Ensure existing applications still work
- Ensure REPL mode unaffected

---

## Documentation Requirements

### API Documentation

1. **Comprehensive doc comments** - When to use each API
2. **Examples** - Both simple and complex use cases
3. **Migration guide** - How to switch from string to argv API
4. **Troubleshooting** - Common issues and solutions

### User Documentation

1. **CLI integration guide** - How to integrate unilang in CLI apps
2. **Best practices** - Argv vs string API decision guide
3. **Shell compatibility** - Notes on different shell behaviors
4. **FAQ** - Common questions about argv vs string parsing

---

## Priority Justification

### Why HIGH Priority

1. **Affects all CLI applications** - Every app using unilang for CLI hits this
2. **Architectural flaw** - Operating at wrong abstraction level
3. **User experience** - Current workarounds create poor UX
4. **Maintenance burden** - Every app must implement complex workarounds
5. **Ecosystem health** - Blocking wider unilang adoption

### Current Workaround Cost

For willbe3 alone:
- **100+ lines** of argv reconstruction logic
- **5 integration tests** to verify reconstruction
- **500+ lines** of documentation explaining the issue
- **Ongoing maintenance** as unilang evolves

Multiply by N applications = significant ecosystem cost.

---

## Related Issues / PRs

- None yet (first report of this architectural issue)

---

## Attachments

### Files Included

1. **unilang_argv_api_request_mre.rs** - Runnable MRE demonstrating the problem
2. **willbe3/src/main.rs** - Real-world workaround implementation (100+ lines)
3. **willbe3/tests/argv_reconstruction_test.rs** - Test suite for workaround
4. **will_crates/spec.md** - User documentation explaining the issue

### How to Run MRE

```bash
# Compile
rustc --edition 2021 unilang_argv_api_request_mre.rs -o mre

# Test problematic case
./mre command::ls -la

# Expected: Shows string API failing, argv API working
```

---

## Contact / Discussion

**Reporter:** willbe3 development team
**Repository:** https://github.com/Wandalen/wTools
**Module:** unilang
**Version affected:** 0.25.0 (and earlier)

**Questions?** Comment on this task or open discussion in wTools repository.

---

## Checklist for Implementation

- [ ] Add `parse_from_argv()` to `unilang_parser::Parser`
- [ ] Add `process_command_from_argv()` to `unilang::Pipeline`
- [ ] Add `process_command_from_argv_simple()` helper
- [ ] Write comprehensive unit tests
- [ ] Write integration tests
- [ ] Add API documentation
- [ ] Add user guide for CLI integration
- [ ] Add migration guide
- [ ] Update examples
- [ ] Test on Linux
- [ ] Test on macOS
- [ ] Test on Windows
- [ ] Verify backward compatibility
- [ ] Add to changelog
- [ ] Update version number

---

## Conclusion

This is a fundamental architectural issue that affects every CLI application using unilang. The fix is straightforward (add argv-based API), fully backward compatible, and eliminates entire categories of workarounds.

**Recommendation:** Accept as HIGH priority feature request and include in next release.
