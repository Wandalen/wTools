# Task 085: CLI Parser with Message Tail Collection

## Metadata

- **ID:** 085
- **Status:** ✅ (Completed)
- **Priority:** 4
- **Value:** 9
- **Easiness:** 7
- **Safety:** 8
- **Advisability:** 2016

## Summary

Add `cli_parser` module to `unilang_parser` that provides a convenience API for CLI tools needing to parse `param::value` arguments while collecting remaining tokens as a single message string. This enables AI assistants and similar tools to use the pattern:

```bash
command session::resume timeout::7200000 tell me about the code
```

Where parameters are extracted and "tell me about the code" becomes the message.

## Problem Statement

### Current Situation

`unilang_parser` already supports `param::value` syntax and produces `GenericInstruction` with:
- `named_arguments: HashMap<String, Vec<Argument>>`
- `positional_arguments: Vec<Argument>`

However, for AI CLI tools like `wplan_agent`, the remaining positional arguments need to be joined into a single message string. Currently, `wplan_agent` implements this with 1,053 LOC of custom parsing logic that duplicates functionality already in `unilang_parser`.

### Desired Outcome

- New `cli_parser` module in `unilang_parser` (~150 LOC)
- `CliParams` trait for type-safe parameter binding
- `parse_cli_args<T>()` function that returns `(params, message)`
- `unilang` crate re-exports this module
- `wplan_agent/src/params.rs` reduced from 1,053 LOC to ~200 LOC

## Technical Design

### API Surface

```rust
// In unilang_parser::cli_parser

/// Result of CLI parsing with separated params and message.
pub struct CliParseResult<T> {
  /// Parsed parameters from `param::value` tokens
  pub params: T,
  /// Remaining tokens joined as message string
  pub message: String,
}

/// Trait for parameter sets that can be parsed from CLI args.
pub trait CliParams: Default {
  /// Process a single `key::value` parameter.
  fn process_param(&mut self, key: &str, value: &str) -> Result<(), String>;

  /// Validate the complete parameter set after all params processed.
  fn validate(&self) -> Result<(), String> { Ok(()) }
}

/// Parse CLI arguments into typed params and message tail.
pub fn parse_cli_args<T: CliParams>(args: &[String]) -> Result<CliParseResult<T>, String>;

/// Convenience wrapper for &str slices.
pub fn parse_cli_str_args<T: CliParams>(args: &[&str]) -> Result<CliParseResult<T>, String>;
```

### Algorithm

Two-phase parsing leveraging existing `Parser`:

1. **Phase 1 (Syntax):** Use `Parser::parse_single_instruction()` to get `GenericInstruction`
2. **Phase 2 (Binding):**
   - Iterate `named_arguments`, call `T::process_param()` for each
   - Call `T::validate()`
   - Join `positional_arguments` into message string

### Integration Points

- Uses existing `unilang_parser::Parser` for syntax parsing
- No changes to existing parser logic
- Pure addition of convenience layer

## File Changes

### Files to Create

| File | Content | LOC |
|------|---------|-----|
| `src/cli_parser.rs` | Module with CliParams trait, parse functions | ~150 |
| `tests/cli_parser_tests.rs` | Comprehensive tests | ~200 |

### Files to Modify

| File | Change |
|------|--------|
| `src/lib.rs` | Add `pub mod cli_parser;` |
| `Cargo.toml` | No changes needed |

### In unilang (separate small change)

| File | Change |
|------|--------|
| `src/lib.rs` | Add `pub use unilang_parser::cli_parser;` |

## Acceptance Criteria

### Functional Requirements

- [ ] `CliParams` trait defined with `process_param()` and `validate()` methods
- [ ] `parse_cli_args<T>()` correctly separates named params from message
- [ ] Multiple values for same param collected (called multiple times)
- [ ] Empty message when no positional args
- [ ] Error on unknown parameter (via `process_param` returning Err)
- [ ] Error on validation failure (via `validate` returning Err)

### Test Coverage

- [ ] Basic parsing: `["timeout::5000", "hello", "world"]` → params + "hello world"
- [ ] No message: `["timeout::5000"]` → params + ""
- [ ] No params: `["hello", "world"]` → default params + "hello world"
- [ ] Multiple same param: `["tag::a", "tag::b"]` → process_param called twice
- [ ] Boolean flags: `["verbose::true", "dry::false"]`
- [ ] Unknown param error: `["unknown::value"]` → Err
- [ ] Validation error: timeout=0 → Err
- [ ] Empty input: `[]` → default params + ""
- [ ] Quoted values: `["msg::\"hello world\""]` → proper handling

### Documentation

- [ ] Module-level documentation with examples
- [ ] All public items have doc comments
- [ ] Example showing typical usage pattern

### Code Quality

- [ ] Follows unilang code style (2-space indents, etc.)
- [ ] No unwrap() in library code
- [ ] Proper error messages with context

## Implementation Steps

### Step 1: Create cli_parser module (2 hours)

1. Create `src/cli_parser.rs` with:
   - `CliParseResult<T>` struct
   - `CliParams` trait
   - `parse_cli_args<T>()` function
   - `parse_cli_str_args<T>()` convenience wrapper
2. Export in `src/lib.rs`

### Step 2: Write tests (2 hours)

1. Create `tests/cli_parser_tests.rs`
2. Implement test parameter struct
3. Write tests for all acceptance criteria
4. Ensure 100% coverage of public API

### Step 3: Add re-export to unilang (30 min)

1. Add `pub use unilang_parser::cli_parser;` to unilang's lib.rs
2. Verify re-export works

### Step 4: Documentation (1 hour)

1. Add comprehensive module docs
2. Add examples showing real-world usage
3. Document error conditions

## Usage Example

After implementation, consumers can use:

```rust
use unilang::cli_parser::{parse_cli_args, CliParams, CliParseResult};

#[derive(Default)]
struct MyParams {
  timeout: u64,
  verbose: bool,
}

impl CliParams for MyParams {
  fn process_param(&mut self, key: &str, value: &str) -> Result<(), String> {
    match key {
      "timeout" => self.timeout = value.parse().map_err(|e| format!("{}", e))?,
      "verbose" => self.verbose = value.parse().map_err(|e| format!("{}", e))?,
      _ => return Err(format!("Unknown parameter: {}", key)),
    }
    Ok(())
  }

  fn validate(&self) -> Result<(), String> {
    if self.timeout == 0 {
      return Err("timeout must be > 0".into());
    }
    Ok(())
  }
}

fn main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().skip(1).collect();
  let result: CliParseResult<MyParams> = parse_cli_args(&args)?;

  println!("Timeout: {}", result.params.timeout);
  println!("Verbose: {}", result.params.verbose);
  println!("Message: {}", result.message);

  Ok(())
}
```

## Benefits

| Metric | Before | After |
|--------|--------|-------|
| wplan_agent/params.rs | 1,053 LOC | ~200 LOC |
| Parsing logic | Duplicated | Shared in unilang |
| Maintenance | Custom code | Maintained with unilang |
| Features | Fixed | Inherits unilang improvements |

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| API doesn't match wplan_agent needs | Low | High | Design reviewed against actual usage |
| Performance regression | Low | Medium | Uses existing optimized parser |
| Breaking existing tests | None | N/A | Pure addition, no changes to existing code |

## Dependencies

- None - uses only existing unilang_parser infrastructure

## Effort Estimate

- **Total:** 5.5 hours
- **Breakdown:**
  - Module implementation: 2 hours
  - Tests: 2 hours
  - Documentation: 1 hour
  - Re-export in unilang: 0.5 hours

## Related

- **Crates affected:** unilang_parser (primary), unilang (re-export)
- **Consumers:** wplan_agent (first adopter)
- **Specification:** Uses FR-ARG-3 (named argument binding) from unilang spec

## Outcomes

### Phase 1 Completed: 2025-11-18

**Files Created:**
- `src/cli_parser.rs` (~165 LOC) - Core module with CliParseResult, CliParams trait, parse functions
- `tests/cli_parser_tests.rs` (~550 LOC) - Comprehensive test suite with 29 tests

**Files Modified:**
- `src/lib.rs` - Added module export and prelude inclusion
- `unilang/src/lib.rs` - Added re-export

**Implementation Notes:**
- Used simple `split_once("::")` parsing instead of full Parser integration
- Parser integration was complex since Parser expects command structure
- Simple approach matches wplan_agent's current behavior exactly
- All 199 tests pass including 29 new cli_parser tests
- Doc tests and clippy pass

**Deviations from Plan:**
- Did not use `Parser::parse_single_instruction()` as originally planned
- Simple manual parsing is cleaner and matches the use case better
- Same algorithm as wplan_agent's current implementation

**Verified:**
- All acceptance criteria met
- Level 3 tests pass (nextest + doc tests + clippy)
- unilang crate compiles with re-export

### Phase 2 Completed: 2025-11-19

**Extended cli_parser with config defaults support (~323 LOC added):**

- `CliParamsAdvanced<C>` trait - Config-aware parameter parsing with explicit tracking
- `CliParser` builder - Fluent API with `.with_config().parse()`
- `CliParseResultAdvanced<T>` - Extended result with `explicit_params: BTreeSet<String>`
- 12 new tests for advanced features

**Applied to wplan_agent:**

- Added `unilang_parser` dependency to `wplan_agent/Cargo.toml`
- Refactored `wplan_agent/src/params.rs` using `CliParamsAdvanced` trait
- Implemented `apply_defaults()` for config hierarchy integration
- Implemented `finalize()` for smart defaults (interactive mode when no message)

**Results:**

| Component | Tests | Doc Tests | Clippy | LOC Change |
|-----------|-------|-----------|--------|------------|
| unilang_parser | 211 ✅ | 6 ✅ | ✅ | +323 |
| wplan_agent | 107 ✅ | 7 ✅ | ✅ | -622 |
| unilang | 722 ✅ | 75 ✅ | ✅ | 0 |
| **Net** | - | - | - | **-299 LOC** |

**Key Design Decisions:**
- Used `BTreeSet<String>` for no_std compatibility instead of `HashSet`
- Canonical names for alias tracking (e.g., "v" → "verbosity")
- Message validation in `parse_with_config()` after message assignment
- Config passed by reference, not owned

## Notes

This task enables consolidating CLI parameter parsing logic from wplan_agent into the unilang ecosystem, reducing code duplication and maintenance burden while leveraging unilang's existing parser infrastructure.
