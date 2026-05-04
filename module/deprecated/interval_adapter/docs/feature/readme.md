# Feature Doc Entity

### Scope

- **Purpose**: Document user-visible capabilities of `interval_adapter` — what the crate does and why each feature exists.
- **Responsibility**: Collect one doc instance per user-visible feature; each instance owns scope, design decisions, constraints, and usage examples.
- **In Scope**: Feature specifications — scope, design, constraints, and integration points.
- **Out of Scope**: Behavioral contracts (→ `invariant/`); API signatures (→ `api/`); design patterns (→ `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Generic Interval Parameter](001_generic_interval_parameter.md) | Write generic functions accepting any interval type | ✅ |
| 002 | [Non-Iterable Intervals](002_non_iterable_intervals.md) | Query bounds of unbounded intervals without iteration | ✅ |
| 003 | [no_std Support](003_no_std_support.md) | Zero-dependency embedded-compatible interval abstraction | ✅ |
