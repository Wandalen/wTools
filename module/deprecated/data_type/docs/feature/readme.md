# Feature Doc Entity

### Scope

- **Purpose**: Document capabilities provided by data_type for contributors and consumers.
- **Responsibility**: Master index for all feature doc instances in this crate.
- **In Scope**: Instances covering one upstream integration each — Either type, interval types, collection types.
- **Out of Scope**: Behavioral invariants governing the aggregator design — see invariant/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Either Type](001_either_type.md) | Pass-through re-export of Either from the either crate | ✅ |
| 002 | [Interval Integration](002_interval_integration.md) | Pass-through re-export of interval_adapter items | ✅ |
| 003 | [Collection Integration](003_collection_integration.md) | Pass-through re-export of collection_tools items | ✅ |
