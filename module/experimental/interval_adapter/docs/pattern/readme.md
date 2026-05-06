# Pattern Doc Entity

### Scope

- **Purpose**: Document architectural patterns and design decisions that govern `interval_adapter`.
- **Responsibility**: Collect one doc instance per architectural decision; each instance owns problem, solution, applicability, and consequences.
- **In Scope**: Design patterns — trait hierarchy structure, canonical type rationale, and architectural choices.
- **Out of Scope**: API signatures (→ `api/`); behavioral invariants (→ `invariant/`); feature specifications (→ `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Two-Trait Hierarchy](001_two_trait_hierarchy.md) | Why iterable and non-iterable are separate traits | ✅ |
| 002 | [Canonical Interval Type](002_canonical_interval_type.md) | Why the canonical type unifies all range representations | ✅ |
