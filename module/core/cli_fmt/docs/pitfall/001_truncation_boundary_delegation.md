# Pitfall: Truncation Boundary Delegation

### Scope

- **Purpose**: Document the exact-width truncation trap discovered in width filtering.
- **Responsibility**: Trap, failure mode, and mitigation for calling truncation unconditionally instead of gating on visible length.
- **In Scope**: `apply_width_filtering`'s boundary between "pass through" and "truncate."
- **Out of Scope**: Truncation implementation itself — see `strs_tools`.

### Trap

Calling the width-truncation function unconditionally on every line, on the assumption that a line already within `max_width` is a safe no-op to pass through it.

### Failure

When a line's visible length exactly equals `max_width`, the truncation function still reserves space for the truncation suffix and shortens the line — producing a truncated result with a marker even though the line fit exactly. No panic or error signals the mistake; only the boundary case (`visible_len == max_width`) is affected, so ordinary shorter/longer test inputs pass while the exact-fit case silently corrupts output.

### Mitigation

Always check `visible_len(line) > max_width` before calling the truncation function. Only invoke truncation when the line is strictly longer than the limit; pass exact-fit and shorter lines through unchanged.

### Features

| File | Relationship |
|------|-------------|
| [`../feature/001_output_processing.md`](../feature/001_output_processing.md) | Width-truncation stage this pitfall applies to |

### Sources

| File | Relationship |
|------|-------------|
| `src/output.rs` | `apply_width_filtering` — boundary check enforcing this mitigation |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/docs/pitfall/001_truncation_boundary_delegation.md`](../../tests/docs/pitfall/001_truncation_boundary_delegation.md) | Test specification verifying this mitigation holds |
| `tests/output.rs` | Exact-width boundary regression test (`bug_reproducer(BUG-005)`) |
