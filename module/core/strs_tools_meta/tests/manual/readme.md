# Manual Testing Plan

## Purpose

This directory contains documentation for manual testing procedures that complement automated tests.

## Test Files

| File | Responsibility |
|------|---------------|
| `readme.md` | Manual testing plan and procedures |

## Manual Testing Procedures

### Corner Case Validation

All corner case tests have been automated in `tests/corner_cases_test.rs`. Execute with:

```bash
cargo test --test corner_cases_test --all-features -- --nocapture
```

### Test Coverage Summary

**optimize_split! Corner Cases** (17 tests):
- Empty string handling
- Delimiter boundary cases (start, end, consecutive)
- Unicode and multi-byte UTF-8 (emoji, accents)
- Overlapping delimiter prioritization
- Optimization threshold edges (8, 9 delimiters)
- Whitespace delimiter handling (\n, \t)
- preserve_empty and preserve_delimiters options

**optimize_match! Corner Cases** (11 tests):
- Empty input/pattern handling
- Match position cases (start, middle, end)
- Multiple occurrence behavior (returns first)
- Unicode and multi-byte UTF-8 matching
- Overlapping pattern prioritization
- Optimization threshold edges (16, 17 patterns)
- Strategy parameter validation

### Known Behavior (Not Bugs)

1. **Empty segments included by default**: Matches Rust `str::split()` behavior
   - `",,,".split(",")` → `["", "", "", ""]`
   - This is CORRECT and consistent with Rust stdlib

2. **Empty pattern matches at position 0**: Matches Rust `str::find("")` behavior
   - `"test".find("")` → `Some(0)`
   - This is CORRECT and consistent with Rust stdlib

3. **Strategy parameter simplified**: `longest_match` and `all_matches` currently generate same code as `first_match`
   - Formally documented in `docs/invariant/003_strategy_param_no_op.md`
   - Not a bug — planned enhancement; behavior will differ when true strategy dispatch is implemented

### Manual Verification Steps

1. **Run all automated tests**:
   ```bash
   cd /home/user1/pro/lib/wip_core/wtools/dev/module/core/strs_tools_meta
   w3 .test l::3
   ```

2. **Verify documentation examples compile**:
   ```bash
   cargo test --doc --all-features
   ```

3. **Test in downstream consumer** (strs_tools crate):
   ```bash
   cd ../strs_tools
   cargo test --all-features
   ```

## Test Results

**Last Manual Test Session**: 2026-01-21

| Category | Tests | Passed | Failed | Notes |
|----------|-------|--------|--------|-------|
| Corner Cases (split) | 17 | 17 | 0 | All edge cases validated |
| Corner Cases (match) | 11 | 11 | 0 | All edge cases validated |
| UTF-8 Handling | 4 | 4 | 0 | Emoji and accents work correctly |
| Threshold Edges | 4 | 4 | 0 | Optimization transitions correct |

**Total**: 28 tests, 28 passed, 0 failed

**Issues Found**: 0 bugs
**Enhancement Opportunities**: Documentation clarity (preserve_empty parameter)

## Conclusion

All manual testing completed successfully. No bugs found. Implementation correctly matches Rust standard library semantics for `str::split()` and `str::find()`.
