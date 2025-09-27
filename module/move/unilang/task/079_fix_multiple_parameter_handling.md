# Fix Multiple Parameter Handling and Quote Tokenization

## Description

Fix critical issues in unilang's parameter parsing system where the `multiple: true` attribute doesn't work correctly, and quoted parameters with spaces are incorrectly tokenized. These issues prevent CLI applications from accepting multiple instances of the same parameter in an intuitive way, forcing developers to implement complex workarounds.

This issue was discovered while implementing the `wrun` CLI tool, where users expect to be able to specify multiple commands using syntax like `command::"cargo build" command::"cargo test"` but instead get broken parsing behavior.

Links to related tasks: Core parameter parsing functionality, affects all CLI applications using unilang.

## Current Problematic Behavior

### Issue 1: Multiple Parameter Collection Failure

**Expected behavior:** When a parameter is defined with `multiple: true`, multiple instances should be collected into a `Value::List`.

**Actual behavior:** The last parameter overwrites previous ones, and remaining tokens get incorrectly assigned to other parameters.

**Example:**
```rust
// Parameter definition
ArgumentDefinition::former()
  .name("command")
  .kind(Kind::String)
  .attributes(ArgumentAttributes {
    optional: true,
    multiple: true,
    ..Default::default()
  })
  .end()
```

**Input:** `command::"cargo build" command::"cargo test"`

**Expected parsing:**
```rust
arguments = {
  "command": List([String("cargo build"), String("cargo test")])
}
```

**Actual parsing:**
```rust
arguments = {
  "command": List([String("build"), String("test")]),  // Missing "cargo"
  // Additional erroneous entries created
}
```

### Issue 2: Quote Tokenization Splitting

**Expected behavior:** Quoted parameter values should be treated as complete strings regardless of internal spaces.

**Actual behavior:** Quoted values get split on spaces and distributed across multiple parameters in unpredictable ways.

**Example:**

**Input:** `command1::"echo hello world" command2::"cargo test"`

**Expected parsing:**
```rust
arguments = {
  "command1": String("echo hello world"),
  "command2": String("cargo test")
}
```

**Actual parsing:**
```rust
arguments = {
  "command1": String("echo"),
  "command2": String("cargo"),
  "command": List([String("hello"), String("world"), String("test")])
}
```

### Issue 3: Inconsistent Quote Handling

**Current behavior:** Different quoting strategies produce different unexpected results, with no reliable way to pass multi-word values.

**Tested variations that all fail:**
- `command::"cargo build" command::"cargo test"`
- `command::'cargo build' command::'cargo test'`
- `'command::"cargo build"' 'command::"cargo test"'`

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

### Core Functionality

-   **Multiple parameter collection**: Parameters with `multiple: true` must collect all instances into a `Value::List`
-   **Quote preservation**: Quoted parameter values must be treated as atomic strings regardless of internal whitespace
-   **No cross-parameter contamination**: Tokens from one parameter must not leak into other parameters during parsing
-   **Consistent quoting behavior**: All standard shell quoting methods should work reliably

### Specific Test Cases

The following test cases must pass after implementation:

#### Test Case 1: Basic Multiple Parameters
```bash
# Input
myapp command::"cargo build" command::"cargo test" command::"cargo clippy"

# Expected arguments
{
  "command": List([
    String("cargo build"),
    String("cargo test"),
    String("cargo clippy")
  ])
}
```

#### Test Case 2: Mixed Single and Multiple
```bash
# Input
myapp command::"cargo build" parallel::4 command::"cargo test"

# Expected arguments
{
  "command": List([String("cargo build"), String("cargo test")]),
  "parallel": Integer(4)
}
```

#### Test Case 3: Complex Quoted Values
```bash
# Input
myapp command::"echo 'hello world'" command::"cargo test --verbose"

# Expected arguments
{
  "command": List([
    String("echo 'hello world'"),
    String("cargo test --verbose")
  ])
}
```

#### Test Case 4: Empty and Whitespace Values
```bash
# Input
myapp command::"" command::"  spaced  " command::"normal"

# Expected arguments
{
  "command": List([
    String(""),
    String("  spaced  "),
    String("normal")
  ])
}
```

### Performance Requirements

-   **No performance regression**: Parameter parsing performance must not degrade compared to current single-parameter handling
-   **Memory efficiency**: Multiple parameter collection should not cause excessive memory allocation
-   **Parse time complexity**: Should remain O(n) where n is the number of parameters

### Compatibility Requirements

-   **Backward compatibility**: Existing single-parameter usage must continue to work unchanged
-   **API stability**: No breaking changes to public parameter definition APIs
-   **Error handling**: Clear error messages for malformed parameter syntax

### Code Quality Requirements

-   **2-space indentation**: Following codestyle rules throughout implementation
-   **No clippy warnings**: `cargo clippy --all-targets --all-features -- -D warnings` must pass
-   **Comprehensive tests**: All edge cases must be covered with unit and integration tests
-   **Documentation updates**: Parameter handling documentation must reflect new capabilities

## Implementation Notes

### Root Cause Analysis

The issues appear to stem from:

1. **Tokenization order**: The parser may be splitting quoted strings before applying parameter assignment logic
2. **Parameter collection logic**: The `multiple: true` flag isn't properly implemented in the argument parsing pipeline
3. **Quote handling precedence**: Shell-level quote processing may be interfering with unilang-level quote handling

### Suggested Investigation Areas

-   **Tokenizer behavior**: How quoted strings are initially parsed and split
-   **ArgumentDefinition processing**: How the `multiple` attribute is handled during argument resolution
-   **Parameter assignment logic**: How values are assigned to parameter names during parsing
-   **Shell interaction**: How shell quote processing affects the tokens unilang receives

## Impact Assessment

### Critical Impact
This issue significantly affects the usability of unilang-based CLI applications, forcing developers to implement complex workarounds (like numbered parameters: `command1::`, `command2::`, etc.) that hurt user experience.

### Affected Applications
Any CLI application using unilang that needs to accept multiple instances of the same parameter type, including but not limited to:
- Build tools accepting multiple commands
- File processors accepting multiple inputs
- Configuration tools accepting multiple key-value pairs

### User Experience Impact
The current behavior violates user expectations and standard CLI conventions, making unilang-based tools feel broken compared to traditional CLI frameworks.

## Success Metrics

-   All test cases above pass reliably
-   No regressions in existing single-parameter functionality
-   Clean parameter syntax eliminates need for numbered parameter workarounds
-   Zero clippy warnings in parameter parsing code
-   Full compatibility with existing unilang-based applications