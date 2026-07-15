# Pitfall Doc Entity

### Scope

**Responsibilities:**
Documents confirmed design pitfalls discovered through implementation in `cli_fmt` — traps, failure modes, and mitigations for incorrect assumptions this codebase has already fallen into.

**In Scope:**
- Instance 001 — truncation boundary delegation
- Instance 002 — stream merge ordering
- Instance 003 — option field silent drop

**Out of Scope:**
- Correct solutions to apply (see `feature/` instances)
- Invariants that must always hold (see `invariant/` instances)

### Overview Table

| # | File | Name | Status |
|---|------|------|--------|
| 1 | [001_truncation_boundary_delegation.md](001_truncation_boundary_delegation.md) | Truncation Boundary Delegation | ✅ |
| 2 | [002_stream_merge_ordering.md](002_stream_merge_ordering.md) | Stream Merge Ordering | ✅ |
| 3 | [003_option_field_silent_drop.md](003_option_field_silent_drop.md) | Option Field Silent Drop | ✅ |
