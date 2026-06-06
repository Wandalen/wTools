# Feature: Compile-Time String Match Optimization

## Scope

- **In Scope**: Compile-time pattern analysis; strategy selection; generated match code behavior; `optimize_match!` parameter semantics.
- **Out of Scope**: Trie data structure implementation (unused); runtime performance.
- **Boundary**: Covers behavioral contract; call interface is in `api/002_optimize_match_api.md`.
- **Status**: Implemented.

## Design

`optimize_match!` analyzes pattern count and length at macro expansion time and selects one of three match strategies. The `strategy` parameter is syntactically accepted but has no effect on strategy selection or generated code; see `invariant/003_strategy_param_no_op.md`.

### Strategy Selection

Strategy is chosen at compile time. Thresholds defined in `invariant/002_match_strategy_thresholds.md`.

| Strategy | Condition | Generated implementation |
|----------|-----------|--------------------------|
| `SinglePattern` | 1 pattern | Single `str::find()` call |
| `TrieBasedMatch` | count ≤ 16 AND all lengths ≤ 8 | Linear `find()` loop (trie-named; true trie not implemented) |
| `SequentialMatch` | Otherwise | Linear `find()` loop |

`TrieBasedMatch` and `SequentialMatch` currently produce identical generated code. The naming distinction is reserved for a future implementation.

### Parameters

| Name | Type | Default | Notes |
|------|------|---------|-------|
| `source` | `&str` expression | — | String to search |
| `patterns` | string literal or `[...]` array | — | One or more patterns |
| `strategy` | string keyword arg | `"first_match"` | Accepted but has no effect; see `invariant/003_strategy_param_no_op.md` |
| `debug` | bare flag | absent | Emit compile-time diagnostics; does not alter return value |

### Return

`Option<usize>` — byte position of the first match, or `None`.

### Rationale

Compile-time optimization eliminates runtime pattern dispatch overhead:
1. Pattern count and length are known at build time — strategy selection is free.
2. Generated code is specialized for the exact pattern set — no runtime branching.
3. Compile-time errors catch malformed pattern expressions before deployment.

Note: `TrieBasedMatch` is named aspirationally. A true compile-time trie is a planned enhancement; current generated code is a linear `find()` loop identical to `SequentialMatch`.

### Sources

| File | Relationship |
|------|-------------|
| `spec.md` (git `c13cf485~1`) | Original spec; deleted without migration in `c13cf485`. Usage example in §Public API used incorrect pipe syntax (`"p1" \| "p2"`) — corrected to array literal in this instance and in `api/002`. `strategy` parameter no-op not documented in original — added as `invariant/003`. |
| [`../../src/lib.rs`](../../src/lib.rs) | `optimize_match_impl`, `analyze_match_pattern`, `generate_*_match` |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/optimize_match_tests.rs`](../../tests/optimize_match_tests.rs) | TC1–TC10 unit tests |
| [`../../tests/corner_cases_test.rs`](../../tests/corner_cases_test.rs) | Edge case and threshold boundary tests |

### APIs

| File | Relationship |
|------|-------------|
| [`../api/002_optimize_match_api.md`](../api/002_optimize_match_api.md) | Call interface |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/002_match_strategy_thresholds.md`](../invariant/002_match_strategy_thresholds.md) | Strategy selection thresholds |
| [`../invariant/003_strategy_param_no_op.md`](../invariant/003_strategy_param_no_op.md) | `strategy` parameter no-op invariant |
