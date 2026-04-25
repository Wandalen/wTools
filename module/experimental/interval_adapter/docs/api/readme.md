# API Doc Entity

### Scope

- **Purpose**: Document the public traits and types of `interval_adapter` — signatures, contracts, and semver guarantees.
- **Responsibility**: Collect one doc instance per public API surface; each instance owns signatures, method tables, and compatibility guarantees.
- **In Scope**: Public trait signatures, method tables, type definitions, conversion contracts, and semver stability.
- **Out of Scope**: Feature design rationale (→ `feature/`); behavioral invariants (→ `invariant/`); design patterns (→ `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Interval Traits](001_interval_traits.md) | `NonIterableInterval` and `IterableInterval` — unified interval interface | ✅ |
| 002 | [Conversion Traits](002_conversion_traits.md) | `BoundExt`, `EndPointTrait`, `IntoInterval` — bound and endpoint helpers | ✅ |
