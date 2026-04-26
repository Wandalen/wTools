# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts for mem_tools that must hold regardless of code path.
- **Responsibility**: Master index for all invariant doc instances in this crate.
- **In Scope**: Instances covering one correctness property each — heterogeneous type support.
- **Out of Scope**: Feature design and API interface details — see feature/ and api/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Comparison Functions Work Across Heterogeneous Types](001_type_agnostic_comparison.md) | Functions accept independently typed references without a shared type constraint | ✅ |
