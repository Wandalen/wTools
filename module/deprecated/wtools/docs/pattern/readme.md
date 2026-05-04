# Pattern Doc Entity

### Scope

- **Purpose**: Document the architectural design patterns that shape how wtools aggregates the ecosystem.
- **Responsibility**: Capture the problem, solution, applicability, and consequences for each reusable design pattern applied in this crate.
- **In Scope**: Ecosystem aggregation pattern, feature flag composition pattern, module aliasing rationale.
- **Out of Scope**: Constituent crate patterns (see each sub-crate's own docs/pattern/), feature-specific details (see feature/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Ecosystem Aggregation](001_ecosystem_aggregation.md) | Single-dependency entry point to the wTools ecosystem | ✅ |
| 002 | [Feature Flag Composition](002_feature_flag_composition.md) | Per-category namespaced feature flags for granular control | ✅ |
