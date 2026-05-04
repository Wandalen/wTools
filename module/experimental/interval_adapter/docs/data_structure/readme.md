# Data Structure Doc Entity

### Scope

- **Purpose**: Document canonical data types that store and iterate over interval values.
- **Responsibility**: Collect one doc instance per canonical struct; each instance owns field layout, construction, and iteration contracts.
- **In Scope**: Struct definitions, field inventories, construction APIs, and iteration behavior.
- **Out of Scope**: Trait signatures (→ `api/`); behavioral contracts (→ `invariant/`); design rationale (→ `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Interval](001_interval.md) | Canonical interval representation storing a pair of bound values | ✅ |
| 002 | [IntervalIterator](002_interval_iterator.md) | Closed-interval iterator produced from the canonical interval type | ✅ |
