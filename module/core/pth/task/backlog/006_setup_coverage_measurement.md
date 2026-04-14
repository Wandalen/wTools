# Task 006: Set Up Coverage Measurement

## Overview

Configure code coverage measurement tooling to establish baseline coverage metrics and enable ongoing coverage tracking for the pth crate.

## Status

🔄 Planned

## Priority Metrics

- **Value**: 7/10 (quality baseline, required for v1.0.0)
- **Easiness**: 6/10 (tooling setup, moderately straightforward)
- **Priority**: 4/5 (needed for v1.0.0)
- **Safety**: 5/5 (safe, just adding tooling)
- **Advisability**: 840

## Problem Statement

**From Spec** (Success Criteria #6):
> **6. Test Coverage**
> - **Specific**: ≥95% line coverage, ≥90% branch coverage measured by llvm-cov
> - **Current Status**: ~92% estimated (246 tests); formal measurement pending

Currently:
- No formal coverage measurement configured
- Estimated coverage is ~92% but not verified
- No CI integration for coverage regression detection
- Cannot identify uncovered code paths systematically

## Solution

Set up `cargo-llvm-cov` for coverage measurement and reporting.

## Implementation Strategy

### Step 1: Tool Installation

```bash
cargo install cargo-llvm-cov
```

### Step 2: Generate Coverage Report

```bash
cargo llvm-cov --all-features --workspace --html
```

This generates HTML report in `target/llvm-cov/html/`

### Step 3: Analyze Current Coverage

1. Review generated report
2. Identify uncovered lines
3. Categorize gaps:
   - Unreachable error paths
   - Missing edge case tests
   - Defensive code not exercised
4. Document findings

### Step 4: Set Coverage Baseline

Create `.cargo/coverage-baseline.txt`:
```
Current Coverage: XX.X%
Target: ≥95% line, ≥90% branch
Date: 2025-10-29
```

### Step 5: CI Integration (optional)

Add to GitHub Actions workflow:
```yaml
- name: Install llvm-cov
  run: cargo install cargo-llvm-cov

- name: Generate coverage
  run: cargo llvm-cov --all-features --lcov --output-path lcov.info

- name: Upload to codecov
  uses: codecov/codecov-action@v3
  with:
    files: lcov.info
    fail_ci_if_error: true
```

### Step 6: Add Coverage Badge (optional)

Update `readme.md` with coverage badge:
```markdown
[![codecov](https://codecov.io/gh/Wandalen/wTools/branch/master/graph/badge.svg?flag=pth)](https://codecov.io/gh/Wandalen/wTools)
```

## Acceptance Criteria

- [ ] `cargo-llvm-cov` installed and working
- [ ] Coverage report generated successfully
- [ ] Baseline coverage documented (should be ~92-95%)
- [ ] Coverage gaps identified and categorized
- [ ] Decision made on uncovered code:
  - Add tests for important paths
  - Mark defensive code as intentionally uncovered
  - Remove dead code
- [ ] Coverage meets spec targets (≥95% line, ≥90% branch) OR gaps documented with justification
- [ ] Optional: CI integration configured
- [ ] Optional: Coverage badge added to readme

## Estimated Effort

2-3 hours
- Tool setup: 30 min
- Generate and analyze: 1 hour
- Add missing tests: 1-2 hours (depends on gaps)
- CI integration: 30 min (optional)

## Target Milestone

v0.29.0 to v1.0.0 (P1 - HIGH)

## Related Issues

- Discovery issue 4.2: Missing Test Coverage Measurement
- Discovery issue 5.1: No Tests for NormalizedPath Edge Cases
- Discovery issue 5.2: No Tests for UTF-8 Panic Scenarios
- Spec Success Criteria #6: Test Coverage

## Implementation Notes

### Alternative: cargo-tarpaulin

If llvm-cov doesn't work well:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all-features --out Html
```

### Coverage Exclusion Markers

For intentionally uncovered defensive code:
```rust
// coverage:ignore-start
if unlikely_condition {
  panic!("Should never happen");
}
// coverage:ignore-end
```

### Expected Gaps

Based on discovery report, likely uncovered areas:
1. Error paths in conversion traits (TryFrom implementations)
2. Windows-specific code when running on Unix (and vice versa)
3. Defensive panics for "impossible" states
4. Some edge cases in `NormalizedPath` (issue 5.1)

### Documentation Output

Create `docs/coverage_report.md`:
```markdown
# Coverage Report

**Date**: 2025-10-29
**Tool**: cargo-llvm-cov
**Coverage**: XX.X% line, XX.X% branch

## Summary

- Total Lines: XXXX
- Covered Lines: XXXX
- Uncovered Lines: XX

## Uncovered Code Analysis

### Category 1: Intentional (defensive code)
- src/path.rs:123 - Impossible state panic
- ...

### Category 2: Platform-specific
- src/path/windows.rs:45-50 - Windows-only (tested in Windows CI)
- ...

### Category 3: Needs Tests
- src/path/normalized_path.rs:112 - Edge case missing test
- ...
```
