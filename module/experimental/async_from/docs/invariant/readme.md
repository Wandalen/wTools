# Invariant Doc Entity

### Scope

- **Purpose**: State correctness properties that must hold across all async conversion implementations and their blanket impls.
- **Responsibility**: Document each invariant with a precise statement, enforcement mechanism, and consequences of violation.
- **In Scope**: Thread-safety requirement for blanket impls, blanket impl non-conflict invariant.
- **Out of Scope**: Test strategies for verifying invariants and runtime behaviour under violation.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Send Bounds on Async Conversions](001_send_bounds.md) | Thread-safety constraint ensuring blanket impls are safe across thread boundaries | ✅ |
| 002 | [Blanket Impl Non-Conflict](002_blanket_impl_chain.md) | Non-overlap guarantee between AsyncInto and AsyncTryInto blankets | ✅ |
