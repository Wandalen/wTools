# Add Comprehensive Edge Case Tests

**Date**: 2025-11-22
**Priority**: HIGH
**Category**: Foundation - Testing
**Status**: ✅ (Completed)
**Task ID**: 004
**Advisability**: 1120 (Value: 10, Easiness: 4, Safety: 7, Priority: 4)

## Problem Statement

Current test coverage (~70%) only covers basic happy paths. Missing:
- Edge cases (time near epoch, overflow conditions)
- Error conditions (system clock issues)
- Platform-specific behavior (32-bit overflow, no_std)
- Cross-unit consistency over extended samples
- Monotonic property verification

## Context

This task is part of the **Foundation Phase** (task F3). Must be completed after spec.md exists (task 002) to know which edge cases to test.

See `-comprehensive_redefinition_plan.md` lines 342-419 for detailed test cases.

## Dependencies

**Blockers**:
- Task 002: Create specification (spec.md defines edge cases to test)

## Requirements

Add comprehensive edge case tests to `tests/time_tests.rs`:

1. **Epoch Boundary Tests**
   - Verify behavior near UNIX epoch
   - Document limitations (can't test < epoch without mocking)

2. **Overflow Tests**
   - Test ns::now() overflow on 32-bit systems
   - Document expected behavior

3. **Extended Cross-Unit Consistency**
   - Verify consistency over multiple samples
   - Test with larger time windows

4. **Monotonic Property**
   - Verify time advances monotonically
   - Test with sleep delays

5. **Platform-Specific Tests**
   - no_std compilation check
   - Architecture-specific behavior

## Acceptance Criteria

- [ ] Edge case tests added to `tests/time_tests.rs`
- [ ] Epoch boundary test implemented:
  - [ ] `test_epoch_boundary()` - verifies positive time
- [ ] Overflow test implemented:
  - [ ] `test_ns_overflow_32bit()` - documents 32-bit behavior
- [ ] Extended consistency test implemented:
  - [ ] `test_cross_unit_consistency_extended()` - multiple samples
- [ ] Monotonic property test implemented:
  - [ ] `test_monotonic_property()` - time advances correctly
- [ ] Platform-specific test implemented:
  - [ ] `test_no_std_compilation()` - no_std works
- [ ] All new tests passing
- [ ] Coverage >90% (target from spec)
- [ ] All edge cases documented in test doc comments
- [ ] Tests use proper assertions with clear messages
- [ ] Full test suite passes (`w3 .test l::3`)

## Implementation Notes

**Test Templates** (from plan lines 350-407):

```rust
/// Test behavior near UNIX epoch
#[test]
fn test_epoch_boundary()
{
  // Verify we can get time without panic
  let now = the_module::now();
  assert!(now > 0, "Time should be positive (past 1970)");
}

/// Test cross-unit consistency with larger time windows
#[test]
fn test_cross_unit_consistency_extended()
{
  // Verify consistency over multiple samples
  for _ in 0..10 {
    let ms = the_module::now();
    let s = the_module::s::now();
    let ns = the_module::ns::now();

    assert!((ms / 1000 - s).abs() <= 1, "ms and s should be consistent");
    assert!((ns / 1_000_000 - ms).abs() <= 1, "ns and ms should be consistent");
  }
}

/// Test that time is monotonic within same process
#[test]
fn test_monotonic_property()
{
  let t1 = the_module::now();
  std::thread::sleep(std::time::Duration::from_millis(10));
  let t2 = the_module::now();

  assert!(t2 >= t1, "Time should be monotonic (t2={}, t1={})", t2, t1);
  assert!(t2 - t1 >= 10, "Time should advance (delta={} ms)", t2 - t1);
}
```

## Estimated Effort

2-3 hours

## Blockers

- Task 002: Create specification (defines which edge cases need coverage)

## Follow-up Tasks

- Task 005: Improve error handling (can run in parallel)
- Any feature development tasks (blocked until foundation complete)

## Metrics

```
Value: 10      (Critical foundation work, prevents bugs)
Easiness: 4    (Moderate complexity - some tricky edge cases)
Safety: 7      (Medium risk - tests can be flaky)
Priority: 4    (High urgency - needed before features)

Advisability = 10 × 4 × 7 × 4 = 1120
```

## Success Criteria

**Definition of Done**:
- All required edge case tests implemented
- Test coverage >90%
- All edge cases documented
- Tests pass reliably
- No flaky tests
- Full test suite passes

**Verification**:
```bash
# Run full test suite
w3 .test l::3

# Check coverage (if tarpaulin available)
cargo tarpaulin --out Stdout --all-features
# Target: >90%

# Verify test count increased
cargo test --all-features -- --list | grep -c "test "
# Should be significantly more than current 6
```

**Coverage Goal**: >90% line coverage (from spec requirements)

## Quality Standards

All tests must:
- Have clear doc comments explaining what they test
- Use descriptive assertion messages
- Be deterministic (no random failures)
- Follow wTools code style (2-space indents, space before `::`)
- Run quickly (<1s per test typically)

## References

- `-comprehensive_redefinition_plan.md` - Lines 342-419 for complete test templates
- `spec.md` - Defines edge cases to cover (created in task 002)
- `test_organization.rulebook.md` - Test organization standards
- Current tests in `tests/time_tests.rs` - Follow existing patterns
