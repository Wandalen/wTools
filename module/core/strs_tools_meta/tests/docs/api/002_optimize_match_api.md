# Test Surface: API — optimize_match! Macro

## Scope

- **In Scope**: API contract test cases — invocation forms, return type (`Option<usize>`), strategy parameter acceptance, compile-time error on non-literal pattern, `debug` flag neutrality.
- **Out of Scope**: Strategy selection thresholds (see `../invariant/002_match_strategy_thresholds.md`); strategy no-op invariant (see `../invariant/003_strategy_param_no_op.md`); behavioral output details (see `../feature/002_compile_time_match.md`).
- **Status**: Active.

## Cases

### AP-1: Single-pattern invocation form compiles and returns `Option<usize>`

- **Given**: A `&str` expression and a single string literal pattern.
- **When**: `optimize_match!( src, "pattern" )` is used in a `let` binding.
- **Then**: The expression compiles. The inferred type is `Option<usize>`. The value is `Some(n)` (byte position) or `None`.
- **Status**: ✅

### AP-2: Multi-pattern array form compiles and returns `Option<usize>`

- **Given**: A `&str` expression and an array of two or more string literal patterns.
- **When**: `optimize_match!( src, [ "p1", "p2" ] )` is used in a `let` binding.
- **Then**: The expression compiles. The inferred type is `Option<usize>`.
- **Status**: ✅

### AP-3: Non-literal pattern causes compile-time error

- **Given**: A variable or expression (not a string literal) supplied as a pattern.
- **When**: The macro is expanded at compile time.
- **Then**: Compilation fails with a macro expansion error referencing non-literal input.
- **Status**: ✅

### AP-4: Strategy values accepted without compile error

- **Given**: `strategy = "first_match"`, `strategy = "longest_match"`, and `strategy = "all_matches"` as separate invocations.
- **When**: Each invocation expands.
- **Then**: All three compile without error. Unknown strategy values are silently accepted per documented API (see `api/002_optimize_match_api.md § Error Handling`).
- **Status**: ✅

### AP-5: `debug` flag does not alter return value

- **Given**: The same source and pattern; one invocation with `debug`, one without.
- **When**: Both invocations expand.
- **Then**: Both return identical `Option<usize>` values.
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| AP-1 | `tc1_single_pattern` | `optimize_match_tests.rs` |
| AP-2 | `tc2_multiple_small_patterns` | `optimize_match_tests.rs` |
| AP-3 | `non_literal_pattern_rejected_at_compile_time` | `compile_fail_test.rs` |
| AP-4 | `tc3_first_match_strategy`, `tc4_longest_match_strategy`, `tc5_all_matches_strategy` | `optimize_match_tests.rs` |
| AP-5 | `tc6_debug_mode` | `optimize_match_tests.rs` |
