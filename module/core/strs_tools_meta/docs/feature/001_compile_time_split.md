# Feature: Compile-Time String Split Optimization

## Scope

- **In Scope**: Compile-time delimiter analysis; strategy selection logic; generated split code behavior; `optimize_split!` parameter semantics.
- **Out of Scope**: Runtime performance benchmarks; regex engine internals; SIMD dispatch (removed).
- **Boundary**: Covers macro-level behavioral contract; call interface is in `api/001_optimize_split_api.md`.
- **Status**: Implemented.

## Design

`optimize_split!` analyzes delimiter count and length at macro expansion time and selects one of three split strategies, eliminating runtime strategy dispatch. The selected strategy is embedded directly into the generated code.

### Strategy Selection

Strategy is chosen from delimiter properties at compile time. Thresholds defined in `invariant/001_split_strategy_thresholds.md`.

| Strategy | Condition | Generated implementation |
|----------|-----------|--------------------------|
| `SingleCharDelimiter` | 1 delimiter AND length == 1 | `str::split(char)` |
| `MultipleCharDelimiters` | count ≤ 8 AND all lengths ≤ 4 | Chained `find()` loop |
| `ComplexPattern` | Otherwise | Regex-based split |

### Parameters

| Name | Type | Default | Notes |
|------|------|---------|-------|
| `source` | `&str` expression | — | String to split |
| `delimiters` | string literal or `[...]` array | — | One or more delimiters |
| `preserve_delimiters` | `bool` keyword arg | `false` | Include delimiter tokens in output |
| `preserve_empty` | `bool` keyword arg | `true` | Retain empty segments; matches `str::split()` stdlib semantics |
| `debug` | bare flag | absent | Emit compile-time diagnostics; does not alter return value |

### Return

`Vec<String>` — collected split segments.

### Rationale

Compile-time optimization eliminates runtime strategy dispatch overhead:
1. Delimiter properties are known at build time — strategy selection is free.
2. Generated code is specialized for the exact delimiter set — no runtime branch on strategy.
3. Compile-time errors catch malformed delimiter expressions before deployment.

### Sources

| File | Notes |
|------|-------|
| `spec.md` (git `c13cf485~1`) | Original spec; deleted without migration in `c13cf485`. §Public API covered both macros combined — split into separate instances. Module structure diagram retained in `readme.md` only. |

### Cross-References

| Type | File | Notes |
|------|------|-------|
| source | `../../src/lib.rs` | `optimize_split_impl`, `analyze_split_pattern`, `generate_*_split` |
| test | `../../tests/optimize_split_tests.rs` | TC1–TC10 unit tests |
| test | `../../tests/corner_cases_test.rs` | Edge case and threshold boundary tests |
| doc | `../api/001_optimize_split_api.md` | Call interface |
| doc | `../invariant/001_split_strategy_thresholds.md` | Strategy selection thresholds |
