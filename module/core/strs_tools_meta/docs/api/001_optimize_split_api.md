# API: optimize_split! Macro

## Scope

- **In Scope**: Invocation syntax; parameter types and defaults; return type; compile-time error conditions; feature gate.
- **Out of Scope**: Strategy selection logic (see `invariant/001_split_strategy_thresholds.md`); behavioral design (see `feature/001_compile_time_split.md`).
- **Boundary**: Covers observable call-site contract; not implementation internals.
- **Status**: Stable.

## Abstract

`optimize_split!` is a procedural macro that expands a string split at compile time, selecting the most efficient splitting implementation based on delimiter properties. Returns `Vec<String>`. Requires the `optimize_split` feature.

## Operations

| Form | Return | Notes |
|------|--------|-------|
| `optimize_split!(src, "delim")` | `Vec<String>` | Single delimiter |
| `optimize_split!(src, ["d1", "d2", ...])` | `Vec<String>` | Multiple delimiters |
| `optimize_split!(src, delims, preserve_delimiters = bool)` | `Vec<String>` | Include delimiter tokens in output |
| `optimize_split!(src, delims, preserve_empty = bool)` | `Vec<String>` | Retain empty segments |
| `optimize_split!(src, delims, debug)` | `Vec<String>` | Emit compile-time diagnostics |

**Parameter contract:**
- `src`: any expression yielding `&str`
- delimiter arguments: string literals only (non-literals cause compile-time error)
- `preserve_delimiters`: bool literal; default `false`
- `preserve_empty`: bool literal; default `true` (matches `str::split()` stdlib semantics)
- `debug`: bare flag with no `= value`; absence means disabled; does not alter return

## Error Handling

| Condition | Error |
|-----------|-------|
| Non-literal delimiter | Compile error at macro expansion |
| Malformed keyword argument | Compile error via `macro_tools` parsing |

## Compatibility Guarantees

- Return type `Vec<String>` is stable.
- `preserve_empty = true` (default) matches Rust stdlib `str::split()` empty-segment behavior.
- `debug` presence does not alter return value.
- Strategy selection is an internal detail; observable output is identical regardless of which strategy is selected.

### Sources

| File | Relationship |
|------|-------------|
| `spec.md` (git `c13cf485~1`) | §Public API `optimize_split!` section. Feature Flags table reflected BLOCKING `default` violation — corrected in `Cargo.toml`; this instance documents post-fix defaults. |
| [`../../src/lib.rs`](../../src/lib.rs) | `optimize_split` entry point, `optimize_split_impl` |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/optimize_split_tests.rs`](../../tests/optimize_split_tests.rs) | TC1–TC10 |

### Features

| File | Relationship |
|------|-------------|
| [`../feature/001_compile_time_split.md`](../feature/001_compile_time_split.md) | Behavioral design and parameter semantics |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/001_split_strategy_thresholds.md`](../invariant/001_split_strategy_thresholds.md) | Strategy selection thresholds |
