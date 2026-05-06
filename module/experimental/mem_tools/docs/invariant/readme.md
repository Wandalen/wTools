# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts for mem_tools that must hold regardless of code path.
- **Responsibility**: Documents correctness properties — invariant statements, enforcement mechanisms, and violation consequences for this crate's behavioral contracts.
- **In Scope**: Instances covering one correctness property each — heterogeneous type support and size-guarded byte comparison safety.
- **Out of Scope**: Feature design and API interface details — see feature/ and api/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Comparison Functions Work Across Heterogeneous Types](001_type_agnostic_comparison.md) | Functions accept independently typed references without a shared type constraint | ✅ |
| 002 | [same_data Validates Sizes Before Byte Comparison](002_size_guarded_data_comparison.md) | Byte comparison executes only after size equality is confirmed, preventing out-of-bounds reads | ✅ |
