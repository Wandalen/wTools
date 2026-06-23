# BUG-002: Trailing Delimiter Missing Empty Segment

## Status: ✅ RESOLVED (2026-06-23)

- **Severity:** Major
- **First Seen:** 2026-06-23
- **Regressions:** 0

## Problem Statement

`SplitFastIterator` did not yield a trailing empty content segment after the final delimiter when `preserving_empty(true)` was set. For example, `"a,b,"` with `preserving_empty(true)` yielded `["a", "b"]` instead of `["a", "b", ""]`, deviating from `str::split(",")` semantics which produces `["a", "b", ""]`.

## Root Cause

The early termination guard in `SplitFastIterator::next()` returned `None` unconditionally when `iterable.is_empty() && counter > 0`, without checking whether the iterator was in a content phase (odd counter) or delimiter phase (even counter). After the final delimiter was yielded (even counter), the iterator state left `iterable` empty. The next call hit the early return before incrementing the counter to the content phase, preventing the trailing empty content segment from being emitted.

## Fix Applied

Added a `done: bool` field to `SplitFastIterator`. When `iterable` is empty and `counter > 0`:
- If `counter % 2 == 0` (last yielded was delimiter), emit one trailing empty content segment before setting `done = true`
- If `counter % 2 == 1` (last yielded was content), set `done = true` and return `None`

## Testing Gap

The existing test `test_m_t3_3_leading_trailing_space_preserve_all` expected the trailing empty segment in its `expected` array but used `for (i, split) in iter.enumerate()` which only checks items the iterator yields. The assertion never verified total segment count, silently masking the missing trailing segment. Fixed by collecting into a Vec and asserting `result.len() == expected.len()`.

## Prevention

- Always assert total segment count in iterator tests, not just per-element correctness
- Compare split behavior against `str::split()` for reference semantics
