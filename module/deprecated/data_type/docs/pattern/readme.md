# Pattern Doc Entity

### Scope

- **Purpose**: Document reusable design decisions that explain why data_type is structured as it is.
- **Responsibility**: Master index for all pattern doc instances in this crate.
- **In Scope**: Structural design patterns applied in this crate with their rationale and tradeoffs.
- **Out of Scope**: Per-feature behavioral contracts or implementation algorithms — those belong in invariant/ or algorithm/.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Facade Re-export](001_facade_reexport.md) | Why data_type uses aggregation rather than implementation | ✅ |
