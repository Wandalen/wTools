# Test Surface: Feature — Compile-Time Split

## Scope

- **In Scope**: Behavioral test cases for `optimize_split!` — delimiter forms, option parameters, debug mode, default equivalence.
- **Out of Scope**: API contract cases (see `../api/001_optimize_split_api.md`); strategy threshold enforcement (see `../invariant/001_split_strategy_thresholds.md`).
- **Status**: Active.

## Cases

### FT-1: Single-char delimiter produces correct segments

- **Given**: A source string containing a single-char delimiter (e.g. `"/"`).
- **When**: `optimize_split!( src, "/" )` expands.
- **Then**: Result is a `Vec<String>` where each element is a segment between occurrences of `"/"`. Delimiter characters are absent from result elements.
- **Status**: ✅

### FT-2: Multi-char single delimiter produces correct segments

- **Given**: A source string containing a multi-char delimiter (e.g. `"::"`).
- **When**: `optimize_split!( src, "::" )` expands.
- **Then**: Result is a `Vec<String>` split at each occurrence of `"::"`. The delimiter substring is absent from result elements.
- **Status**: ✅

### FT-3: Multiple delimiters split at any occurrence

- **Given**: A source string; two or more delimiters supplied as an array.
- **When**: `optimize_split!( src, [ "/", "\\" ] )` expands.
- **Then**: String is split at every occurrence of any listed delimiter. Result elements contain no delimiter characters.
- **Status**: ✅

### FT-4: `preserve_delimiters = true` includes delimiter tokens in output

- **Given**: A source string with at least one delimiter occurrence; `preserve_delimiters = true`.
- **When**: `optimize_split!( src, "/", preserve_delimiters = true )` expands.
- **Then**: Each delimiter occurrence appears as a separate element adjacent to its surrounding segments. Non-delimiter content is unchanged.
- **Status**: ✅

### FT-5: `preserve_empty = false` drops empty segments

- **Given**: A source string with consecutive delimiters producing empty segments; `preserve_empty = false`.
- **When**: `optimize_split!( src, "/", preserve_empty = false )` expands.
- **Then**: Empty segments are absent from the result. Non-empty segments are retained in order.
- **Status**: ✅

### FT-6: `debug` flag emits diagnostics without altering return value

- **Given**: Any valid `optimize_split!` invocation with the `debug` flag appended.
- **When**: The macro expands at compile time.
- **Then**: Compile-time diagnostics are emitted (e.g. strategy selection). The return value is identical to the same invocation without `debug`.
- **Status**: ✅

### FT-7: Explicit defaults produce identical output to implicit defaults

- **Given**: The same source and delimiter.
- **When**: Comparing `optimize_split!( src, delim )` vs. `optimize_split!( src, delim, preserve_delimiters = false, preserve_empty = true )`.
- **Then**: Both invocations produce identical `Vec<String>` results.
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| FT-1 | `tc1_single_char_delimiter` | `optimize_split_tests.rs` |
| FT-2 | `tc2_multi_char_single_delimiter` | `optimize_split_tests.rs` |
| FT-3 | `tc3_multiple_delimiters`, `tc4_complex_delimiters`, `tc7_multiple_delimiters_simple` | `optimize_split_tests.rs` |
| FT-4 | `tc5_preserve_delimiters` | `optimize_split_tests.rs` |
| FT-5 | `tc6_preserve_empty` | `optimize_split_tests.rs` |
| FT-6 | `tc8_debug_mode` | `optimize_split_tests.rs` |
| FT-7 | `tc9_explicit_parameters`, `tc10_default_value_equivalence` | `optimize_split_tests.rs` |
