# Pattern Doc Entity

### Scope

- **Purpose**: Document the reusable design patterns applied in format_tools that inform its architecture and extension points.
- **Responsibility**: Index of pattern doc instances, each describing one design pattern — its problem, solution, applicability, and consequences.
- **In Scope**: Design patterns whose application explains key structural decisions in format_tools.
- **Out of Scope**: Behavioral requirements (→ feature/), implementation-level API contracts (→ api/), constraints (→ invariant/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Fallback Chain](001_fallback_chain.md) | Compile-time multi-level fallback dispatch via type-based strategy markers | ✅ |
| 002 | [Format Strategy](002_format_strategy.md) | Pluggable output format selection via a common interface | ✅ |
