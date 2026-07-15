# Pitfall Test Specs

### Scope

**Responsibilities:**
Documents test cases verifying that cli_fmt pitfall mitigations hold — regression proof that each documented trap's failure mode does not recur.

**In Scope:**
- PF-prefixed specs for Pitfall 001 (truncation boundary delegation), Pitfall 002 (stream merge ordering), Pitfall 003 (option field silent drop)

**Out of Scope:**
- Feature behavioral tests beyond the specific trap (see `tests/docs/feature/`)

### Overview Table

| # | File | Name | Status |
|---|------|------|--------|
| 1 | [001_truncation_boundary_delegation.md](001_truncation_boundary_delegation.md) | Truncation Boundary Delegation | ✅ |
| 2 | [002_stream_merge_ordering.md](002_stream_merge_ordering.md) | Stream Merge Ordering | ✅ |
| 3 | [003_option_field_silent_drop.md](003_option_field_silent_drop.md) | Option Field Silent Drop | ✅ |
