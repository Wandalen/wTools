# Document Panic Conditions and Improve Panic Messages

**Date**: 2025-11-22
**Priority**: MEDIUM
**Category**: Foundation - Quality
**Status**: 📥 (Backlog)
**Task ID**: 005
**Advisability**: 960 (Value: 10, Easiness: 8, Safety: 8, Priority: 1.5)

## Problem Statement

**Note**: This task improves panic messages and documentation, NOT error handling strategy. Functions will still panic on error (using `.expect()` instead of `.unwrap()`).

Current `now()` function uses `.unwrap()` which can panic if system time < UNIX epoch, but:
- Panic condition not documented in function docs
- No actionable panic message (just "called unwrap on None")
- No guidance for users who encounter this edge case

```rust
// Current implementation (src/now.rs)
pub fn now() -> i64 {
  time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis() as i64
  // ^^^^^^^ Can panic but not documented!
}
```

## Context

This task is part of the **Foundation Phase** (task F4). Must be completed after spec.md exists (task 002) which defines the error handling strategy.

See `-comprehensive_redefinition_plan.md` lines 422-474 for detailed options.

## Dependencies

**Blockers**:
- Task 002: Create specification (spec.md defines error handling strategy)

## Requirements

Improve panic documentation and messages in `src/now.rs`:

1. **Document Panic Conditions**
   - Add `# Panics` section to all functions that can panic
   - Explain when panic occurs (system time < epoch)
   - Provide actionable guidance

2. **Improve Panic Messages**
   - Replace `.unwrap()` with `.expect()` with descriptive message
   - Message should explain problem and suggest solution
   - Example: "System clock is before UNIX epoch (1970-01-01). Please check your system time configuration."

3. **Document Alternatives for Advanced Users**
   - Note that users needing error handling can use `std::time::SystemTime` directly
   - Document that panic is intentional design decision for this simple API

## Acceptance Criteria

- [ ] All functions in `src/now.rs` have `# Panics` documentation:
  - [ ] `now()`
  - [ ] `s::now()`
  - [ ] `ms::now()`
  - [ ] `ns::now()`
- [ ] Panic documentation includes:
  - [ ] When panic occurs
  - [ ] Why it's extremely rare
  - [ ] What user should do if they hit it
- [ ] `.unwrap()` replaced with `.expect()` with actionable message
- [ ] Alternative approaches documented for advanced users
- [ ] Doc comments follow Rust conventions
- [ ] Documentation builds without warnings (`cargo doc`)
- [ ] Tests passing after changes (`w3 .test l::3`)
- [ ] Clippy passes with no warnings

## Implementation Notes

**Recommended Approach** (Option A from plan lines 438-456):

```rust
/// Get current time. Units are milliseconds.
///
/// Returns the number of milliseconds elapsed since the UNIX epoch
/// (1970-01-01 00:00:00 UTC).
///
/// # Panics
///
/// Panics if the system clock is set to a time before the UNIX epoch
/// (1970-01-01 00:00:00 UTC). This indicates a misconfigured system and
/// is extremely rare in practice.
///
/// If your application must handle such systems, consider using
/// `std::time::SystemTime::now()` directly and handling the error.
///
/// # Performance
///
/// Typically <100ns per call, zero allocations.
///
/// # Examples
///
/// ```
/// use time_tools::now;
///
/// let timestamp = now();
/// assert!(timestamp > 0);
/// ```
#[must_use]
pub fn now() -> i64
{
  time::SystemTime::now()
    .duration_since(time::UNIX_EPOCH)
    .expect("System clock is before UNIX epoch (1970-01-01). Please check your system time configuration.")
    .as_millis() as i64
}
```

Apply similar pattern to all four functions.

**Alternative Approach** (Option B - defer to v0.3.0):

If spec.md indicates future return type change, document:
```rust
/// # Future Changes
///
/// In version 0.3.0, this function may return `Option<i64>` to handle
/// the edge case of system time before epoch gracefully.
```

## Estimated Effort

1 hour

## Blockers

- Task 002: Create specification (defines error handling strategy)

## Follow-up Tasks

None directly. Completes foundation phase prerequisites.

## Metrics

```
Value: 10      (Important for production quality)
Easiness: 8    (Straightforward documentation + .expect())
Safety: 8      (Low risk, behavior unchanged)
Priority: 1.5  (Medium-low - not blocking features)

Advisability = 10 × 8 × 8 × 1.5 = 960
```

## Success Criteria

**Definition of Done**:
- All panic conditions documented
- Panic messages provide actionable guidance
- Alternative approaches documented
- Documentation builds cleanly
- No behavior changes (tests still pass)

**Verification**:
```bash
# Build documentation
cargo doc --all-features --no-deps --open

# Verify no warnings
cargo doc --all-features --no-deps 2>&1 | grep -i warning

# Verify tests pass
w3 .test l::3

# Verify clippy happy
cargo clippy --all-targets --all-features -- -D warnings
```

## Quality Standards

Documentation must:
- Follow Rust doc comment conventions
- Use proper markdown formatting
- Include `# Panics` section (standard Rust convention)
- Provide actionable guidance (not just "may panic")
- Include examples demonstrating correct usage

## Open Questions

Resolved by task 002 (spec creation):
- Should we change return type to `Option<i64>` in v0.3.0?
- Is panic acceptable for this edge case?
- Should we expose a `try_now()` variant?

These questions will be answered in spec.md's "Open Questions" section.

## References

- `-comprehensive_redefinition_plan.md` - Lines 422-474 for implementation options
- `spec.md` - Error handling strategy (created in task 002)
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/documentation.html
- Current implementation: `src/now.rs`
