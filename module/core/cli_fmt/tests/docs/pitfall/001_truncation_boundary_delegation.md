# Pitfall Test: Truncation Boundary Delegation

### Scope

- **Purpose**: Verify the mitigation documented in `docs/pitfall/001_truncation_boundary_delegation.md` holds — the truncation boundary trap does not recur.
- **Responsibility**: Test spec proving `apply_width_filtering` gates truncation on `visible_len(line) > max_width`, not on unconditional invocation.
- **In Scope**: PF-1..PF-2 — exact-width passthrough (the historical failure boundary) and one-over-width truncation (confirms the fix is a precise boundary check, not a broadened no-op).
- **Out of Scope**: General width-truncation behavior beyond this specific boundary — see `tests/docs/feature/001_output_processing.md`.

### PF-1: Line at exact max_width is passed through untouched — the historical failure boundary

- **Given:** single-line input with visible length exactly equal to `max_width`
- **When:** `process_output` applies width filtering at that exact width
- **Then:** the line is not truncated and carries no suffix marker — `visible_len == max_width` is treated as a safe no-op, not a truncation trigger
- **Note:** Regression guard for BUG-005 — `apply_width_filtering` previously called truncation unconditionally, corrupting exact-fit lines.

### PF-2: Line one character over max_width is truncated — confirms the fix is a precise boundary, not a disabled check

- **Given:** single-line input with visible length exactly `max_width + 1`
- **When:** `process_output` applies width filtering at `max_width`
- **Then:** the line is truncated — proves the mitigation's `> max_width` gate still fires correctly just past the boundary; the BUG-005 fix narrowed the trigger condition without disabling truncation altogether

### Pitfalls

| File | Relationship |
|------|-------------|
| [`../../../docs/pitfall/001_truncation_boundary_delegation.md`](../../../docs/pitfall/001_truncation_boundary_delegation.md) | Authoritative trap/failure/mitigation for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | `apply_width_filtering` — boundary check under test |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | PF-1: `width_exact_boundary` (`bug_reproducer(BUG-005)`); PF-2: `width_one_over_boundary` |
