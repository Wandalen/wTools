# 002 — Add compile-fail tests for layer boundary isolation

## Status: 📥 Ready

- **ID:** 002
- **Priority:** 4
- **Executor:** any
- **Advisability:** 216
- **Value:** 6 / Easiness:** 6 / Safety:** 6

## Purpose

The `use super::child` layer propagation (fixed in task 001) has positive tests verifying
items ARE accessible in correct layers, but no compile-fail tests verifying items are
NOT accessible in wrong layers. The commented-out assertions in `only_test/*.rs` document
expected non-accessibility but aren't enforced.

## Context

After the `record_use_implicit` fix, `layer_simple_only_test.rs` has 16 commented-out
assertions across 5 test sections showing expected layer isolation:

- `layer_a_own()` not accessible unqualified or via any parent layer (only via `own::layer_a::`)
- `layer_a_orphan()` not in `orphan::`, `exposed::`, `prelude::` (only in `own::` and root)
- `layer_a_exposed()` not in `prelude::` (only in `exposed::`, `orphan::`, `own::`, root)

These are compile-time properties — items that fail to resolve. Testing requires
`trybuild` or equivalent compile-fail framework.

## Scope

- `tests/inc/only_test/layer_simple_only_test.rs` — 16 commented-out assertions
- `tests/inc/only_test/use_non_layer_only_test.rs` — similar pattern
- `tests/inc/only_test/layer_single_only_test.rs` — similar pattern

## MOST Goals

1. Add `trybuild` compile-fail test cases for each commented-out assertion
2. All compile-fail tests correctly reject access to wrong layers
3. Existing positive tests continue passing

## Notes

- Low urgency — positive tests already cover correctness; this is defense-in-depth
- The `trybuild` crate may already be a workspace dependency (check before adding)
- Each commented-out line maps 1:1 to a compile-fail test case
