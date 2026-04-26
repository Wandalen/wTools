# API: optimize_match! Macro

## Scope

- **In Scope**: Invocation syntax; parameter types and defaults; return type; compile-time error conditions; feature gate; `strategy` parameter semantics.
- **Out of Scope**: Trie construction (unused internally); threshold values (see `invariant/002_match_strategy_thresholds.md`).
- **Boundary**: Covers observable call-site contract; not internal strategy dispatch.
- **Status**: Stable.

## Abstract

`optimize_match!` is a procedural macro that expands a string match at compile time, selecting a matching implementation based on pattern properties. Returns `Option<usize>` (byte position of first match, or `None`). The `strategy` parameter is accepted but has no effect on generated code — see `invariant/003_strategy_param_no_op.md`. Requires the `optimize_match` feature.

## Operations

| Form | Return | Notes |
|------|--------|-------|
| `optimize_match!(src, "pattern")` | `Option<usize>` | Single pattern |
| `optimize_match!(src, ["p1", "p2", ...])` | `Option<usize>` | Multiple patterns |
| `optimize_match!(src, pats, strategy = "first_match")` | `Option<usize>` | Strategy: no-op; see invariant/003 |
| `optimize_match!(src, pats, debug)` | `Option<usize>` | Emit compile-time diagnostics |

**Parameter contract:**
- `src`: any expression yielding `&str`
- pattern arguments: string literals only
- `strategy`: string literal; accepted values `"first_match"`, `"longest_match"`, `"all_matches"`; all produce identical generated code (no-op)
- `debug`: bare flag with no `= value`; does not alter return

## Error Handling

| Condition | Error |
|-----------|-------|
| Non-literal pattern | Compile error at macro expansion |
| Unknown `strategy` value | No compile error — silently accepted |

## Compatibility Guarantees

- Return type `Option<usize>` is stable.
- `strategy` parameter values are API-stable but currently no-op; behavior may differ in a future version that implements real strategy dispatch.
- `debug` presence does not alter return value.

### Sources

| File | Notes |
|------|-------|
| `spec.md` (git `c13cf485~1`) | §Public API `optimize_match!` section. Original example used incorrect pipe syntax (`"pattern1" \| "pattern2"`) — corrected to array literal here. `strategy` no-op not documented in original — added as `invariant/003`. |

### Cross-References

| Type | File | Notes |
|------|------|-------|
| source | `../../src/lib.rs` | `optimize_match` entry point, `optimize_match_impl` |
| test | `../../tests/optimize_match_tests.rs` | TC1–TC10 |
| doc | `../feature/002_compile_time_match.md` | Behavioral design and parameter semantics |
| doc | `../invariant/002_match_strategy_thresholds.md` | Strategy selection thresholds |
| doc | `../invariant/003_strategy_param_no_op.md` | `strategy` parameter no-op invariant |
