# Test Surface: API — optimize_split! Macro

## Scope

- **In Scope**: API contract test cases — invocation forms, return type, parameter defaults, compile-time error on non-literal delimiter, `debug` flag neutrality.
- **Out of Scope**: Strategy selection logic (see `../invariant/001_split_strategy_thresholds.md`); behavioral output details (see `../feature/001_compile_time_split.md`).
- **Status**: Active.

## Cases

### AP-1: Single-delimiter invocation form compiles and returns `Vec<String>`

- **Given**: A `&str` expression and a single string literal delimiter.
- **When**: `optimize_split!( src, "delim" )` is used in a `let` binding.
- **Then**: The expression compiles. The inferred type is `Vec<String>`.
- **Status**: ✅

### AP-2: Multi-delimiter array form compiles and returns `Vec<String>`

- **Given**: A `&str` expression and an array of two or more string literal delimiters.
- **When**: `optimize_split!( src, [ "d1", "d2" ] )` is used in a `let` binding.
- **Then**: The expression compiles. The inferred type is `Vec<String>`.
- **Status**: ✅

### AP-3: Non-literal delimiter causes compile-time error

- **Given**: A variable or expression (not a string literal) supplied as a delimiter.
- **When**: The macro is expanded at compile time.
- **Then**: Compilation fails with a macro expansion error referencing non-literal input.
- **Status**: ✅

### AP-4: `preserve_delimiters` defaults to `false`

- **Given**: The same source and delimiter; one invocation without `preserve_delimiters`, one with `preserve_delimiters = false`.
- **When**: Both invocations expand.
- **Then**: Results are identical. Delimiter tokens do not appear as separate elements in either result.
- **Status**: ✅

### AP-5: `preserve_empty` defaults to `true`

- **Given**: A source string producing empty segments; one invocation without `preserve_empty`, one with `preserve_empty = true`.
- **When**: Both invocations expand.
- **Then**: Results are identical. Empty segments are present in both results, matching `str::split()` stdlib semantics.
- **Status**: ✅

### AP-6: `debug` flag does not alter return value

- **Given**: The same source and delimiter; one invocation with `debug`, one without.
- **When**: Both invocations expand.
- **Then**: Both return identical `Vec<String>` values.
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| AP-1 | `tc1_single_char_delimiter` | `optimize_split_tests.rs` |
| AP-2 | `tc3_multiple_delimiters` | `optimize_split_tests.rs` |
| AP-3 | `non_literal_delimiter_rejected_at_compile_time` | `compile_fail_test.rs` |
| AP-4 | `tc5_preserve_delimiters`, `tc10_default_value_equivalence` | `optimize_split_tests.rs` |
| AP-5 | `tc6_preserve_empty`, `tc10_default_value_equivalence` | `optimize_split_tests.rs` |
| AP-6 | `tc8_debug_mode` | `optimize_split_tests.rs` |
