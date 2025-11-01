# Task: Make Illegal States Unrepresentable in Unilang Public API

**Status:** Open
**Priority:** Critical
**Category:** API Design, Type Safety, Correctness
**Created:** 2025-10-21
**Triggered By:** wplan CLI parameter parsing bug (multiple `command::` parameters silently overwritten)

---

## The Problem: Silent Data Loss Through Legal Types

### Real-World Incident That Exposed This Issue

**Location:** `wplan_client/src/cli/mod.rs:336-376`

**What Happened:**
```bash
# User executed:
wplan .plan command::"cargo build" command::"cargo test"

# Expected: 2 jobs created
# Actual: 1 job created (only "cargo test" - last command silently overwrote first)
```

**Root Cause - Illegal State Was Representable:**
```rust
// The type system ALLOWED this fundamentally broken design:
let mut command = None;  // Option<String> can only hold ONE value

for arg in args {
  match key {
    "command" => command = Some(value.to_string()),  // SILENT OVERWRITE!
    // ...
  }
}

// Compiler said: ✅ This is fine!
// Runtime behavior: ❌ Silently drops all commands except the last
// User experience: ❌ Confused why their jobs disappeared
```

**The Type Lie:**
- The code *accepts* multiple `command::` parameters (no error, no warning)
- The type `Option<String>` can only *store* one value
- The gap between acceptance and storage is **silent data loss**
- The type system was **complicit** in this bug

### The Correct Design - Illegal State Is Unrepresentable

```rust
// The type FORCES correct behavior:
let mut commands = Vec<String>;  // Vec can hold MULTIPLE values

for arg in args {
  match key {
    "command" => commands.push(value.to_string()),  // COLLECTS ALL!
    // ...
  }
}

// If you can accept N values, your type must store N values
// No gap = no silent data loss
```

---

## The Core Issue: Type-Semantic Mismatch

### What Is "Illegal State"?

An **illegal state** is data that violates domain invariants but is representable in the type system.

**Examples from the wild:**

1. **The wplan bug (just fixed):**
   - Domain invariant: "All provided commands must be executed"
   - Illegal state: "Some commands accepted but not stored"
   - Root cause: `Option<String>` when semantics require `Vec<String>`

2. **Unilang potential issues:**
   ```rust
   // Can unilang represent these illegal states?

   // Illegal: Command with help text but no name
   struct Command {
     name: Option<String>,
     help: String,
   }
   // Should be: name is REQUIRED, make it non-Option

   // Illegal: Command registered but not callable
   // Illegal: Duplicate command names in registry
   // Illegal: Command with no handler
   // Illegal: Command that can be both routine and subject
   ```

---

## Governing Principle (DEMANDED)

### Primary Directive

**"If a state is illegal in the domain, it must be unrepresentable in the type system."**

### Corollaries

1. **Parse, Don't Validate:**
   - Don't accept data then validate it
   - Accept only data that's already valid (by type construction)

2. **Make Illegal States Unrepresentable:**
   - If two fields must be set together, group them in a type
   - If a value must be non-empty, use `NonEmpty<Vec<T>>`
   - If a string must match a format, use a newtype with private constructor

3. **Type-Driven API Design:**
   - The type signature should make correct use easy, incorrect use impossible
   - Prefer compile-time errors over runtime errors
   - Prefer runtime errors over silent failures

4. **No Partial Initialization:**
   - Every constructed value must be fully valid
   - Use builder pattern with type-state for complex construction
   - Never expose `new()` that returns partially initialized state

---

## Minimal Reproducible Example (MRE)

### The Bug Pattern

```rust
// MRE: Demonstrating the antipattern that caused the wplan bug

use std::collections::HashMap;

// ❌ BAD: Illegal state is representable
pub struct ParameterParserBad {
  params: HashMap<String, String>,  // Can only store ONE value per key!
}

impl ParameterParserBad {
  pub fn new() -> Self {
    Self { params: HashMap::new() }
  }

  // This API LIES: it accepts multiple values but only keeps the last!
  pub fn parse(&mut self, key: &str, value: &str) {
    self.params.insert(key.to_string(), value.to_string());  // OVERWRITES!
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.params.get(key)
  }
}

#[test]
fn test_bad_parser_demonstrates_silent_data_loss() {
  let mut parser = ParameterParserBad::new();

  // User provides TWO commands:
  parser.parse("command", "cargo build");
  parser.parse("command", "cargo test");

  // Only ONE command is stored (silent data loss):
  assert_eq!(parser.get("command"), Some(&"cargo test".to_string()));
  // "cargo build" is gone! User never got an error!
}

// ✅ GOOD: Illegal state is unrepresentable
pub struct ParameterParserGood {
  params: HashMap<String, Vec<String>>,  // Can store MULTIPLE values!
}

impl ParameterParserGood {
  pub fn new() -> Self {
    Self { params: HashMap::new() }
  }

  // This API is HONEST: it accepts multiple values and stores them all!
  pub fn parse(&mut self, key: &str, value: &str) {
    self.params
      .entry(key.to_string())
      .or_insert_with(Vec::new)
      .push(value.to_string());  // COLLECTS ALL!
  }

  pub fn get_all(&self, key: &str) -> &[String] {
    self.params.get(key).map(|v| v.as_slice()).unwrap_or(&[])
  }
}

#[test]
fn test_good_parser_preserves_all_data() {
  let mut parser = ParameterParserGood::new();

  // User provides TWO commands:
  parser.parse("command", "cargo build");
  parser.parse("command", "cargo test");

  // BOTH commands are stored (no data loss):
  let commands = parser.get_all("command");
  assert_eq!(commands.len(), 2);
  assert_eq!(commands[0], "cargo build");
  assert_eq!(commands[1], "cargo test");
}
```

### The Pattern Applied to Unilang

**Question:** Does unilang's current API allow illegal states?

**Investigation Required:**

```rust
// Example potential issues (INVESTIGATE):

// 1. Can you create a Command with no name?
let cmd = Command::default();  // Is name Option<String>?

// 2. Can you register duplicate commands?
registry.register("build", handler1);
registry.register("build", handler2);  // Silent overwrite?

// 3. Can you create a command with no handler?
let cmd = Command::new("build").help("Build the project");
// Forgot to set .routine()! What happens?

// 4. Can the registry be in a partially initialized state?
let mut registry = CommandRegistry::new();
// What if someone calls registry.execute() before registering any commands?
```

---

## The Question: Can Unilang Reach This Goal?

**Goal:** Make misuse of unilang's public API impossible through type design.

### Technical Feasibility: YES

Rust's type system provides all necessary tools:

1. **Phantom Types:** Encode state in type parameters
2. **Typestate Pattern:** Make invalid transitions impossible
3. **Sealed Traits:** Prevent external implementations
4. **Private Fields:** Force construction through validated builders
5. **Non-Exhaustive Enums:** Allow internal invariants
6. **Const Generics:** Encode compile-time constraints
7. **Associated Types:** Link related types together

### Example: Typestate Builder

```rust
// Using typestate pattern to make illegal states unrepresentable

pub struct Command<State> {
  name: String,
  help: String,
  state: std::marker::PhantomData<State>,
}

// Type states
pub struct NoHandler;
pub struct WithHandler;

// Only commands WITHOUT handlers can have handlers added
impl Command<NoHandler> {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      help: String::new(),
      state: std::marker::PhantomData,
    }
  }

  pub fn help(mut self, help: impl Into<String>) -> Self {
    self.help = help.into();
    self
  }

  // Adding handler transitions state from NoHandler -> WithHandler
  pub fn routine<F>(self, handler: F) -> Command<WithHandler>
  where F: Fn() + 'static
  {
    Command {
      name: self.name,
      help: self.help,
      state: std::marker::PhantomData,
    }
  }
}

// Only commands WITH handlers can be registered
impl<State> Command<State> {
  // This method is only available on Command<WithHandler>
  // due to the trait bound below
}

pub struct Registry {
  commands: Vec<Command<WithHandler>>,  // Can ONLY store valid commands!
}

impl Registry {
  pub fn register(&mut self, cmd: Command<WithHandler>) {
    // Type system GUARANTEES this command has a handler!
    self.commands.push(cmd);
  }
}

#[test]
fn test_typestate_prevents_illegal_states() {
  let mut registry = Registry::new();

  // ✅ This compiles: command has handler
  let cmd = Command::new("build")
    .help("Build the project")
    .routine(|| println!("Building..."));
  registry.register(cmd);

  // ❌ This DOES NOT compile: command has no handler
  // let cmd = Command::new("test").help("Run tests");
  // registry.register(cmd);  // ERROR: expected Command<WithHandler>, found Command<NoHandler>
}
```

### Implementation Effort: HIGH

**Challenges:**

1. **Breaking Changes:** Existing API must be redesigned
2. **Complexity:** Typestate APIs are more complex to implement
3. **Documentation:** Users need to understand the safety guarantees
4. **Migration:** Existing code must be updated

**But:** The payoff is **compile-time correctness** instead of runtime bugs.

---

## Concrete Demands

### 1. Audit Current API for Illegal States

**Task:** Systematically identify every way unilang's public API allows illegal states.

**Checklist:**

- [ ] Can Commands be created without names?
- [ ] Can Commands be created without handlers?
- [ ] Can Commands be registered with duplicate names?
- [ ] Can Registry be used before initialization?
- [ ] Can parameter parsing silently drop values?
- [ ] Can help text diverge from actual command signature?
- [ ] Can Commands be both routine and subject simultaneously?
- [ ] Can Commands be executed before registration?
- [ ] Can Commands be unregistered while still referenced?
- [ ] Can verification fail silently?

### 2. Redesign API Using Type-Driven Principles

**Requirements:**

1. **Every public constructor must return fully valid values**
   - No `new()` that requires subsequent initialization
   - Use builder pattern with typestate if construction is complex

2. **Every illegal domain state must be unrepresentable**
   - If command must have name: `name: String` not `Option<String>`
   - If registry must have commands: `NonEmpty<Vec<Command>>`
   - If parameter can appear multiple times: `Vec<Value>` not `Option<Value>`

3. **Every state transition must be type-checked**
   - Use typestate pattern for complex state machines
   - Make invalid transitions fail at compile time

4. **Every invariant must be enforced by construction**
   - No runtime checks for things that could be compile-time checks
   - Private fields + public validated constructors

### 3. Write Tests Demonstrating Impossibility

**Demand:** Create test file `tests/illegal_states_impossible.rs` with compile-fail tests.

```rust
// These tests should FAIL TO COMPILE (not fail at runtime!)

#[test]
fn cannot_create_command_without_name() {
  // This should not compile:
  // let cmd = Command::new("").routine(|| {});  // Empty name not allowed
}

#[test]
fn cannot_register_command_without_handler() {
  // This should not compile:
  // let cmd = Command::new("test");
  // registry.register(cmd);  // No handler!
}

#[test]
fn cannot_parse_multiple_values_into_single_slot() {
  // If API accepts multiple values, it must store multiple values
  // This should not compile (type error):
  // let mut single: Option<String> = None;
  // parser.parse_multiple("command", &mut single);  // Type mismatch!
}
```

### 4. Document the Governing Principle

**Demand:** Add section to unilang documentation:

**File:** `docs/design_principles.md` (create if not exists)

**Section:** "Making Illegal States Unrepresentable"

**Content:**
- Explain the principle
- Show examples from unilang API
- Demonstrate how type system enforces correctness
- Provide migration guide for users

### 5. Performance Budget: Zero Runtime Overhead

**Constraint:** Type safety must be zero-cost abstraction.

**Verification:**
```rust
// These should compile to identical assembly:

// Safe version (with typestates)
let cmd = Command::new("build").routine(|| {});
registry.register(cmd);

// Unsafe version (with runtime checks)
let cmd = UnsafeCommand::new("build", || {});
unsafe_registry.register_unchecked(cmd);
```

Use `cargo asm` or `cargo llvm-ir` to verify zero overhead.

---

## Success Criteria

### The API is considered successful when:

1. **Compile-time Correctness:**
   - All illegal states from audit fail to compile
   - No runtime checks for compile-time invariants
   - Type errors guide users to correct usage

2. **Zero Runtime Overhead:**
   - Typestate pattern compiles to same code as unsafe version
   - No performance penalty for safety

3. **Ergonomic:**
   - Correct usage is natural and fluent
   - Error messages are helpful and actionable
   - Documentation clearly explains safety guarantees

4. **Proven:**
   - Comprehensive test suite demonstrates impossibility
   - Real-world usage confirms no misuse possible
   - Bug reports related to API misuse drop to zero

---

## References

### Theory

1. **"Parse, Don't Validate"** - Alexis King
   https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

2. **"Making Illegal States Unrepresentable"** - Yaron Minsky
   https://blog.janestreet.com/effective-ml-revisited/

3. **"Type-Driven Development with Idris"** - Edwin Brady
   (Demonstrates extreme version of this principle)

4. **"The Typestate Pattern in Rust"** - Ana Hobden
   https://cliffle.com/blog/rust-typestate/

### Real-World Examples

1. **wplan bug (this incident):**
   - File: `wplan_client/src/cli/mod.rs`
   - Fix: Changed `Option<String>` to `Vec<String>`
   - Tests: `wplan_client/tests/multiple_commands_bug_reproducer.rs`

2. **Hyper's Builder API:**
   - Uses typestate to prevent creating invalid HTTP requests
   - Separate types for `RequestBuilder` vs `Request`

3. **Tokio's Mutex:**
   - Type system prevents data races
   - Can't access data without holding lock guard

---

## Open Questions for Unilang Team

1. **How far should we go?**
   - Simple validation (non-empty names)?
   - Full typestate (builder pattern with states)?
   - Dependent types (GADTs, const generics)?

2. **Breaking changes acceptable?**
   - Is this worth a major version bump?
   - Can we provide migration path?

3. **Documentation burden?**
   - Are users willing to learn typestate pattern?
   - Can we hide complexity behind macros?

4. **Testing strategy?**
   - How to test that things DON'T compile?
   - Use `trybuild` for compile-fail tests?

---

## Conclusion

The wplan bug demonstrated a **fundamental API design flaw**: the type system allowed illegal states (multiple commands accepted but only one stored).

**The fix was simple:** Change one type from `Option<T>` to `Vec<T>`.

**The lesson is profound:** Every time we choose a type, we're making a statement about legal states. Choose wisely.

**The demand is clear:** Unilang must adopt the governing principle of making illegal states unrepresentable. Every public API must be audited, redesigned if necessary, and proven correct through compile-fail tests.

**Can unilang reach this goal?** YES. Rust provides the tools. The question is: will the team invest the effort?

The answer should be YES, because the alternative is more bugs like the wplan incident.

---

**Next Steps:**

1. Acknowledge this task
2. Audit current unilang API (create issues for each illegal state found)
3. Prioritize fixes (breaking vs non-breaking)
4. Implement typestate pattern for complex state machines
5. Write compile-fail tests
6. Update documentation
7. Release new major version with safety guarantees

**Status:** OPEN - Awaiting triage and assignment
