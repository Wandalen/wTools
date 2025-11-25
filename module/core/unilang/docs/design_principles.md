# Design Principles: Make Illegal States Unrepresentable

## Overview

This document explains unilang's governing principle: **"If a state is illegal in the domain, it must be unrepresentable in the type system."**

This principle was triggered by the wplan bug (Task 085), where `multiple:true` with non-List storage caused silent data loss. The solution: prevent illegal states at compile-time rather than catching them at runtime.

---

## The Principle

### Definition

**Make Illegal States Unrepresentable** means that invalid domain states should be impossible to construct, not just rejected after construction.

### Corollaries

1. **Parse, Don't Validate**: Accept only data that's already valid by type construction
2. **Prefer Compile-Time Errors Over Runtime Errors**: Catch bugs during `cargo build`, not during execution
3. **Prefer Runtime Errors Over Silent Failures**: If compile-time prevention isn't possible, fail loudly
4. **No Partial Initialization**: Every constructed value must be fully valid

---

## Application to Unilang

### The wplan Bug: A Case Study

**The Bug:**
```rust
// User command line:
$ wplan .plan command::"cargo build" command::"cargo test"

// Code that caused silent data loss:
let mut command = None;  // Option<String> - can only hold ONE value!

for arg in args {
  match key {
    "command" => command = Some(value.to_string()),  // Overwrites previous!
  }
}
// Result: Only "cargo test" stored, "cargo build" silently lost
```

**The Type Lie:**
- The code *accepts* multiple `command::` parameters
- The type `Option<String>` can only *store* one value
- The gap between acceptance and storage is **silent data loss**

**The Fix:**
```rust
// Correct type that matches behavior:
let mut commands = Vec<String>;  // Can store MULTIPLE values

for arg in args {
  match key {
    "command" => commands.push(value.to_string()),  // Collects all!
  }
}
// Result: Both commands stored, no data loss
```

**The Lesson:** If you accept N values, your type must store N values.

---

## Implementation Strategy

Unilang uses a **validation approach** rather than pure typestate because:

1. **Multiple Definition Approaches**: 21 ways to define commands (YAML, JSON, Rust DSL, etc.)
2. **Not All Approaches Support Typestate**: YAML/JSON are data formats, not Rust code
3. **Validation Works Everywhere**: Can be applied uniformly across all approaches

### Three-Layer Defense

| Layer | When | Mechanism | Example |
|-------|------|-----------|---------|
| **1. Build-Time** | `cargo build` | build.rs validates YAML/JSON | Duplicate names rejected |
| **2. Registration-Time** | Runtime API calls | `validate_command_for_registration()` | Missing dot prefix rejected |
| **3. Execution-Time** | Command execution | Interpreter checks | Missing handler rejected |

### Validation Points

```
YAML/JSON Files
      ↓
  build.rs ← Layer 1: Compile-time validation
      ↓      - Name/namespace validation
      ↓      - Duplicate detection
      ↓      - Parameter storage validation (wplan bug)
      ↓
Static Commands → Compiled Binary
      ↓
Runtime Registration ← Layer 2: Registration-time validation
      ↓                 - Same checks as build.rs
      ↓                 - Ensures consistency
      ↓
  Registry
      ↓
Command Execution ← Layer 3: Execution-time validation
      ↓              - Handler presence check
      ↓              - Final safety net
      ↓
  Result
```

---

## Illegal States Prevented

### 1. Commands Without Names (Item #1)
**Prevention:** build.rs validates at line 631
```rust
if let Err(e) = validate_command(name, namespace, version, "unilang.commands.yaml")
{
  panic!(/* Clear error with fix instructions */);
}
```

**Error Example:**
```
╔══════════════════════════════════════════════════════════════════════════════╗
║ BUILD ERROR: Invalid command definition                                       ║
╟──────────────────────────────────────────────────────────────────────────────╢
║ Command name cannot be empty                                                  ║
╟──────────────────────────────────────────────────────────────────────────────╢
║ Fix: Ensure command names start with '.' (e.g., '.help', '.chat')             ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

### 2. Commands Without Handlers (Item #2)
**Prevention:** Interpreter checks at execution (line 97)
```rust
let routine = self.registry.get_routine( &full_command_name ).ok_or_else( ||
{
  Error::Execution( ErrorData::new(
    ErrorCode::InternalError,
    format!( "Internal Error: No executable routine found for command '{}'.", name),
  ))
})?;
```

**Why Execution-Time:** Handlers are Rust functions, not data. YAML can't define handlers, only metadata.

---

### 3. Duplicate Command Names (Item #3)
**Prevention:** build.rs tracks seen names (lines 619-676)
```rust
let mut seen_command_names : std::collections::HashMap< String, usize > = std::collections::HashMap::new();

for (i, cmd_value) in command_definitions.iter().enumerate()
{
  let full_name = compute_full_name(namespace, name);

  if let Some(first_index) = seen_command_names.get(&full_name)
  {
    panic!(/* Error with both occurrences shown */);
  }

  seen_command_names.insert(full_name.clone(), i);
}
```

**Error Example:**
```
╔══════════════════════════════════════════════════════════════════════════════╗
║ BUILD ERROR: Duplicate command name detected                                  ║
╟──────────────────────────────────────────────────────────────────────────────╢
║ Command '.test' is defined multiple times in YAML manifest                   ║
║                                                                                ║
║ First occurrence: command index 0                                             ║
║ Duplicate found:  command index 2                                             ║
╟──────────────────────────────────────────────────────────────────────────────╢
║ Fix: Rename one of the commands or remove the duplicate entry.                ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

### 4. Registry Before Initialization (Item #4)
**Status:** Not applicable - static registries are always initialized (const)

**Why N/A:** `StaticCommandRegistry` is created from const data at compile-time. It's impossible to use before initialization.

---

### 5. Parameter Parsing Drops Values - wplan Bug (Item #5)
**Prevention:** build.rs validates parameter storage types (lines 678-743)
```rust
if multiple  // Parameter accepts multiple values
{
  // Check if kind is a List
  let is_list = if let Some(kind_str) = arg["kind"].as_str()
  {
    kind_str.contains("List")
  }
  else if let Some(_kind_map) = arg["kind"].as_mapping()
  {
    arg["kind"].as_mapping()
      .and_then(|m| m.keys().next())
      .and_then(|k| k.as_str())
      .is_some_and(|k| k == "List")
  }
  else
  {
    false
  };

  if !is_list
  {
    panic!(/* Error: wplan bug pattern detected */);
  }
}
```

**Error Example:**
```
╔══════════════════════════════════════════════════════════════════════════════╗
║ BUILD ERROR: Invalid parameter definition (wplan bug pattern)                 ║
╟──────────────────────────────────────────────────────────────────────────────╢
║ Command:   .plan                                                              ║
║ Parameter: command                                                            ║
║ Problem:   multiple:true but storage type is NOT List                         ║
║                                                                                ║
║ Current kind: String("String")                                                ║
║                                                                                ║
║ This causes silent data loss when multiple values overwrite each other.       ║
╟──────────────────────────────────────────────────────────────────────────────╢
║ Fix: Change parameter kind to List storage:                                   ║
║                                                                                ║
║   kind: {List: ["String", null]}  # For string values                        ║
║   kind: {List: ["Integer", null]} # For integer values                       ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

### 6-10. Other Items

| # | Illegal State | Status | Reason |
|---|---------------|--------|--------|
| 6 | Help text diverges from signature | N/A | Auto-generated from definition |
| 7 | Commands both routine AND subject | N/A | "Subject" concept doesn't exist |
| 8 | Commands executed before registration | Resolved | SemanticAnalyzer checks registry |
| 9 | Commands unregistered while referenced | Non-issue | Rust ownership prevents dangling refs |
| 10 | Verification fails silently | Documented | Rust's `Result` pattern - caller responsibility |

---

## Why Not Typestate?

**Typestate Pattern Example:**
```rust
// Phantom types encode state in the type system
pub struct Command<State> {
  name: String,
  state: std::marker::PhantomData<State>,
}

pub struct NoHandler;
pub struct WithHandler;

impl Command<NoHandler> {
  pub fn routine<F>(self, handler: F) -> Command<WithHandler>
  {
    // Transition from NoHandler -> WithHandler
  }
}

// Registry only accepts Command<WithHandler>
impl Registry {
  pub fn register(&mut self, cmd: Command<WithHandler>) {
    // Type system GUARANTEES this command has a handler
  }
}
```

**Why We Didn't Use It:**
1. **Limited Scope**: Only works for Rust DSL approaches (#7, #8, #9, #10)
2. **Breaking Change**: Would require redesigning entire public API
3. **No Benefit for YAML/JSON**: 90% of users use YAML (Approach #2)
4. **Validation Sufficient**: Achieves the same goal without breaking changes

**Future Consideration**: Could add typestate for Rust DSL users as opt-in enhancement without breaking existing API.

---

## Consistency Across All 21 Approaches

The validation strategy ensures uniform behavior across all CLI definition approaches:

| Approach | Validation Point | Duplicate Detection | wplan Bug Prevention |
|----------|------------------|---------------------|----------------------|
| #1 YAML Single Static | build.rs | ✅ | ✅ |
| #2 YAML Multi Static | build.rs | ✅ | ✅ |
| #3 YAML Runtime | Registration | ✅ (HashMap) | ✅ |
| #4 JSON Single Static | build.rs | ✅ | ✅ |
| #5 JSON Multi Static | build.rs | ✅ | ✅ |
| #6 JSON Runtime | Registration | ✅ (HashMap) | ✅ |
| #7 Rust DSL Dynamic | Registration | ✅ (HashMap) | ✅ |
| #8 Rust DSL Static | build.rs | ✅ | ✅ |
| #18 Hybrid | Both | ✅ | ✅ |
| #9-17, #19-21 | TBD | Will use same | Will use same |

**Key Insight:** All approaches validate AFTER transformations, so both YAML formats work:
- Format 1: `name: ".version"`, `namespace: ""` → `.version` ✅
- Format 2: `name: "version"`, `namespace: ""` → `.version` ✅

---

## Best Practices for Contributors

### When Adding New Features

1. **Ask:** Can this feature be misused?
2. **Ask:** What illegal states could arise?
3. **Implement:** Add validation to prevent those states
4. **Validate:** Ensure validation works across ALL approaches
5. **Document:** Update this file and Task 085
6. **Test:** Add compile-fail tests for YAML/JSON, runtime tests for Rust DSL

### When Fixing Bugs

1. **Identify:** What illegal state was reached?
2. **Analyze:** Why wasn't it prevented?
3. **Fix:** Add validation at the earliest possible point
4. **Document:** Add to Task 085 with "Fix(issue-NNN)" comment
5. **Prevent:** Ensure the same pattern can't occur elsewhere

### When Reviewing Code

**Red Flags:**
- `Option<T>` when `T` is required
- `Vec::new()` without checking if empty is valid
- Mutable state without validation
- `unwrap()` on user-provided data
- Silent data loss (overwriting without warning)

**Green Flags:**
- Private fields with public validated constructors
- `Result<T, E>` for fallible operations
- Clear error messages with fix instructions
- Validation at compile-time when possible
- Loud failures instead of silent ones

---

## References

### Theory

1. **"Parse, Don't Validate"** - Alexis King
   https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

2. **"Making Illegal States Unrepresentable"** - Yaron Minsky
   https://blog.janestreet.com/effective-ml-revisited/

3. **"The Typestate Pattern in Rust"** - Ana Hobden
   https://cliffle.com/blog/rust-typestate/

### Unilang Implementation

- **Task 085**: `task/085_make_illegal_states_unrepresentable.md`
- **Specification**: `spec.md` (FR-REG-9)
- **Validation Code**: `src/validation_core.rs`, `src/command_validation.rs`
- **Build Script**: `build.rs` (lines 619-743)
- **Tests**: 833 tests pass with `w3 .test l::3`

---

## Conclusion

Unilang's implementation of "Make Illegal States Unrepresentable" demonstrates that:

1. **Validation is practical**: Works across 21 different CLI definition approaches
2. **Compile-time is preferred**: Catches errors during `cargo build`, not at runtime
3. **Consistency is achievable**: Same rules apply regardless of how commands are defined
4. **Silent failures are unacceptable**: Every illegal state fails loudly with clear errors

The wplan bug taught us: **If you accept N values, your type must store N values.** This principle now governs all of unilang's design decisions.
