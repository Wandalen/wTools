# Task 002: Remove cfg(no_std) from Test Files

## Overview

Remove all `#[cfg(not(feature = "no_std"))]` and `#[cfg(feature = "no_std")]` attributes from test files. These references cause clippy errors because the `no_std` feature was removed during the no_std cleanup (Task 001).

## Status

🔄 Planned

## Priority Metrics

- **Value**: 8/10 (blocks clippy, causes warnings)
- **Easiness**: 9/10 (simple search/replace)
- **Priority**: 5/5 (blocking)
- **Safety**: 5/5 (safe, removing dead code)
- **Advisability**: 1800

## Problem Statement

After abandoning the no_std refactoring (Task 001), test files still contain conditional compilation attributes referencing the removed `no_std` feature. This causes clippy errors:

```
error: unexpected `cfg` condition value: `no_std`
  --> tests/inc/absolute_path_test/basic_test.rs:32:12
   |
32 | #[ cfg(not(feature = "no_std")) ]
   |            ^^^^^^^^^^^^^^^^^^
```

## Affected Files

From exploration discovery (issue 2.2):
1. `tests/inc/absolute_path_test/basic_test.rs` (lines 32, 41, 74, 83)
2. `tests/inc/current_path.rs` (line 4)
3. Potentially others (needs grep search)

## Solution

1. Search all test files for `cfg.*no_std` patterns
2. Remove all `#[cfg(not(feature = "no_std"))]` attributes
3. Remove all `#[cfg(feature = "no_std")]` attributes
4. Verify clippy passes cleanly

## Acceptance Criteria

- [ ] No `cfg(feature = "no_std")` references in test files
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] All 232+ tests still pass
- [ ] Zero clippy warnings about unexpected cfg conditions

## Estimated Effort

30 minutes

## Target Milestone

v0.29.0 (P0 - IMMEDIATE)

## Related Issues

- Task 001: no_std refactoring (ABANDONED - root cause)
- Discovery issue 2.2: Configuration Error - Missing no_std Feature

## Implementation Notes

Commands to find all occurrences:
```bash
cd /home/user1/pro/lib/willbe/module/pth
grep -r "cfg.*no_std" tests/
```

Simple find-and-delete operation - no logic changes needed.
