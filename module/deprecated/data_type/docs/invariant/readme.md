# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts for data_type that must hold regardless of code path.
- **Responsibility**: Master index for all invariant doc instances in this crate.
- **In Scope**: Instances covering one correctness property each — the pure aggregator contract.
- **Out of Scope**: Feature design — see feature/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Pure Aggregator](001_pure_aggregator.md) | data_type adds no items — all exports are pass-throughs from upstream | ✅ |
