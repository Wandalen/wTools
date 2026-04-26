# Architecture

## Overview

This document explains the core architectural decisions for the multiline_input crate and the rationale behind them. These decisions were made based on comprehensive investigation conducted 2025-11-15 (see archived investigation files for full details).

---

## Core Design Decisions

### 1. Trait-Based Dependency Injection for Terminal Operations

**Decision**: Abstract all terminal operations behind the `TerminalOps` trait, with `RealTerminal` for production and `MockTerminal` for testing.

**Problem It Solves**:
The original architecture had hard dependencies on crossterm free functions:
```rust
// Original (fragile):
pub fn is_tty() -> bool {
  io::stdin().is_terminal()  // ❌ Not testable
}
```

This prevented:
- Testing error paths (NoTty, TerminalTooSmall)
- Integration testing without real TTY
- Deterministic test execution in CI/CD

**Solution Architecture**:
```rust
// Current (testable):
pub trait TerminalOps: Write {
  fn is_tty(&self) -> bool;
  fn size(&self) -> io::Result<(u16, u16)>;
  fn enable_raw_mode(&mut self) -> Result<(), Error>;
  // ... other operations
}

// Production
impl TerminalOps for RealTerminal { /* calls crossterm */ }

// Testing
impl TerminalOps for MockTerminal { /* simulates terminal */ }
```

**Benefits Realized**:
- ✅ 100% test coverage of error paths (10 error path tests)
- ✅ Deterministic integration tests (5 workflow tests)
- ✅ Zero environment dependencies in tests
- ✅ Fast test execution (no actual I/O)
- ✅ CI/CD compatibility (works without TTY)

**Tradeoffs**:
- Slightly more complex type signatures: `Editor<T: TerminalOps>`
- Additional trait implementation maintenance

**Why Tradeoffs Are Acceptable**:
The builder pattern hides generic complexity from users:
```rust
// Users see simple API:
let text = multiline_input::collect("Prompt:")?;

// Generic complexity hidden in builder:
Builder::new().build() // Returns Editor<RealTerminal>
```

**Alternative Approaches Considered** (Investigation 2025-11-15):

1. **Conditional Compilation** (`#[cfg(test)]`)
   - ❌ Creates separate code paths → higher bug risk
   - ❌ Can't test production code paths
   - ❌ Violates "test what you ship" principle

2. **Environment Variable Checks**
   - ❌ Fragile (tests depend on environment setup)
   - ❌ Silent failures if env vars not set
   - ❌ Violates non-fragile testing principle

3. **Mocking Framework** (e.g., `mockall`)
   - ❌ Test code depends on external mocking library
   - ❌ Adds complexity to test setup
   - ❌ Prefer real implementations per codebase_hygiene.rulebook.md

4. **Trait-Based Dependency Injection** (CHOSEN)
   - ✅ Clean separation of concerns
   - ✅ Real implementations (MockTerminal is real code, not mocking)
   - ✅ Single code path for production and tests
   - ✅ Idiomatic Rust pattern

**References**:
- Investigation: `-non_fragile_testing_solution.md` (2025-11-15, archived)
- Implementation: `src/terminal.rs` (TerminalOps trait)
- Test utilities: `tests/common/mock_terminal.rs`

---

### 2. MockTerminal: Real Implementation, Not Mocking

**Decision**: MockTerminal is a **complete implementation** of `TerminalOps`, not a mock/stub.

**Critical Distinction**:
```rust
// MockTerminal is NOT mocking - it's a real implementation
// with programmable behavior (test double pattern)
pub struct MockTerminal {
  is_tty: bool,              // Explicit state
  size: (u16, u16),          // Explicit state
  key_events: VecDeque<KeyEvent>,  // Programmable behavior
  output: String,            // Captured for assertions
}
```

**Why This Matters**:
- ✅ Complies with codebase_hygiene.rulebook.md "No Mocking" rule
- ✅ Tests exercise real code paths
- ✅ No dependencies on mocking frameworks
- ✅ Deterministic behavior (no mock setup/expectations)

**How It Works**:
1. Test pre-programs key events: `terminal.push_key(KeyCode::Char('a'))`
2. Editor calls `terminal.read_key()` → returns pre-programmed events in order
3. Editor writes to terminal → MockTerminal captures to `output` field
4. Test asserts on captured output

This is **dependency injection with a test double**, not mocking.

**References**:
- Pattern: Test Double (Martin Fowler)
- Implementation: `tests/common/mock_terminal.rs` (228 lines)
- Rulebook: `codebase_hygiene.rulebook.md` → No Mocking

---

### 3. Builder Pattern for Progressive API Disclosure

**Decision**: Three API levels to serve different user needs:
1. Simple: `collect(prompt)` - Zero config for 80% of users
2. Validated: `collect_validated(prompt, validator)` - Common validation case
3. Full: `Builder::new()...build()` - Complete customization

**Rationale**:
Users shouldn't pay complexity cost for features they don't use.

**Usage Distribution** (estimated):
- 80% of users: Simple text input, no validation
- 15% of users: Input validation (min/max length)
- 5% of users: Full customization (colors, initial text, custom prompts)

**Design**:
```rust
// Level 1: Zero-config (80% case)
pub fn collect(prompt: &str) -> Result<Option<String>, Error> {
  Builder::new().prompt(prompt).build().collect()
}

// Level 2: Validation (15% case)
pub fn collect_validated<F>(
  prompt: &str,
  validator: F,
) -> Result<Option<String>, Error>
where
  F: Fn(&str) -> Result<(), String> + 'static,
{
  Builder::new()
    .prompt(prompt)
    .validator(validator)
    .build()
    .collect()
}

// Level 3: Full customization (5% case)
let text = Builder::new()
  .prompt("Enter:")
  .min_length(5)
  .max_length(100)
  .initial_text("default")
  .build()
  .collect()?;
```

**Tradeoff**:
Slight code duplication in simple functions (`collect` and `collect_validated` wrap builder).

**Why Tradeoff Is Acceptable**:
- Better UX for 95% of users (no builder syntax needed)
- Duplication is minimal (2 wrapper functions)
- Zero maintenance burden (functions are stable)

**References**:
- Pattern: Progressive Disclosure (UX design principle)
- Implementation: `src/lib.rs` (public API), `src/builder.rs` (builder pattern)

---

### 4. Domain-Based Test Organization

**Decision**: Tests organized by functional domain (buffer, keys, validation) not by methodology (unit, integration).

**Structure**:
```
tests/
├── buffer_operations_test.rs   # ← Domain: buffer
├── key_handling_test.rs        # ← Domain: keys
├── validation_test.rs          # ← Domain: validation
├── integration_workflows_test.rs  # ← Domain: workflows
├── error_paths_test.rs         # ← Domain: errors
└── common/mock_terminal.rs     # ← Shared utilities
```

**Rationale**:
Developers think "I'm working on buffer operations" not "I'm writing integration tests."

**Benefits**:
- Easier to find relevant tests when modifying code
- Tests co-located with their domain
- Mirrors mental model of development

**Alternative** (rejected):
```
tests/
├── unit/           # ❌ Methodology-based
├── integration/    # ❌ Methodology-based
└── e2e/            # ❌ Methodology-based
```

**References**:
- Rulebook: `files_structure.rulebook.md` → Domain-Based Test Organization
- Implementation: `tests/readme.md` (organization documentation)

---

## Historical Context

### Investigation Timeline (2025-11-12 to 2025-11-15)

**2025-11-12**: Initial test coverage audit revealed:
- 27 unit tests embedded in `src/**/*.rs` (rulebook violation)
- Zero integration tests
- Zero error path tests
- Cannot test without real TTY

**2025-11-15**: Comprehensive investigation conducted:
- Identified root cause: hard dependencies on crossterm
- Designed trait-based DI solution
- Created MockTerminal test double
- Implemented complete test migration
- Achieved 44 tests with 100% non-fragile coverage

**Outcome**: Solution fully implemented and validated.

**Archived Investigation Files** (moved to `./-knowledge/`, then deleted after extraction):
- `-testing_strategy_summary.md` (13KB)
- `-non_fragile_testing_solution.md` (38KB)
- `-test_coverage_analysis.md` (29KB)
- `-hypothesis_confidence_investigation.md` (20KB)
- `-corner_cases_summary.md` (16KB)
- `-test_coverage_audit.md` (18KB)
- `-test_coverage_extension_plan.md` (18KB)
- `-development_plan.md` (32KB)

All valuable knowledge extracted to permanent locations per files_structure.rulebook.md requirements.

---

## Design Principles

### 1. Testability Through Abstraction

Abstract hard dependencies behind traits to enable testing without mocking frameworks.

### 2. Real Implementations Over Mocks

Use test doubles that implement real interfaces (MockTerminal) rather than mocking frameworks.

### 3. Progressive API Disclosure

Simple APIs for common cases, builder pattern for advanced customization.

### 4. Zero Environment Dependencies

Tests must be deterministic and environment-independent (no TTY, no file system, no network).

### 5. Domain-Based Organization

Organize code and tests by functional domain, not by technical methodology.

---

## Known Limitations

### 1. Terminal Size Validation Timing

**Issue**: Terminal size is validated in `render()` AFTER raw mode enabled and cursor hidden.

**Impact**: Brief flicker of hidden cursor if terminal too small.

**Proper Fix**: Move size check to `Editor::collect()` before modifying terminal state.

**Priority**: P2 (minor UX issue, doesn't affect correctness)

### 2. Performance Unvalidated for Large Texts

**Issue**: Spec claims 1MB text support (NFR1.2), but no performance tests exist.

**Root Cause**: `grapheme_to_byte_index()` is O(n) and called on every cursor operation.

**Risk**: Unknown - might work fine, might lag on 10,000+ char lines.

**Proper Fix**: Add performance benchmarks to `benches/` directory.

**Priority**: P3 (works for typical use cases <100KB)

### 3. Unicode Edge Cases Untested

**Issue**: Combining characters, RTL text, complex emoji sequences not explicitly tested.

**Assumption**: `unicode-segmentation` crate handles correctly.

**Risk**: Low (crate is battle-tested), but should validate assumptions.

**Proper Fix**: Add comprehensive Unicode test suite.

**Priority**: P3 (works for common Unicode, edge cases TODO)

---

## Future Architectural Decisions

### Potential: Syntax Highlighting

**If Added**: Would need trait-based abstraction for syntax highlighters to maintain testability.

**Pattern**:
```rust
pub trait SyntaxHighlighter {
  fn highlight(&self, text: &str) -> Vec<StyledSegment>;
}
```

### Potential: Clipboard Integration

**If Added**: Would need trait-based abstraction for clipboard operations.

**Pattern**:
```rust
pub trait ClipboardOps {
  fn copy(&mut self, text: &str) -> Result<(), Error>;
  fn paste(&mut self) -> Result<String, Error>;
}
```

**Principle**: Maintain testability through abstraction, avoid hard dependencies.

---

## References

- Feature doc: `../docs/feature/001_multiline_input.md`
- Testing strategy: `../tests/readme.md`
- TerminalOps trait: `../src/terminal.rs`
- MockTerminal implementation: `../tests/common/mock_terminal.rs`
- Rulebooks:
  - `files_structure.rulebook.md` → Test Organization
  - `codebase_hygiene.rulebook.md` → No Mocking
  - `code_design.rulebook.md` → Trait-Based Design
