# Pattern: Ecosystem Aggregation

### Scope

- **Purpose**: Provide a single-dependency entry point to the wTools ecosystem of utility crates.
- **Responsibility**: Document the aggregation design pattern: one crate, feature-gated optional dependencies, and unified namespace.
- **In Scope**: Module aliasing rationale, single-entry-point design, independent-crate preservation, version coherence.
- **Out of Scope**: Individual sub-crate designs (see each sub-crate's docs/), feature flag details (see feature/).

### Problem

Downstream projects that use multiple wTools utility crates face three friction points:

1. Multiple dependency declarations in the build manifest, each requiring independent version management.
2. Version coherence risk — different crates at different versions may have incompatible transitive dependencies.
3. Import boilerplate — each crate requires separate prelude or exposed imports, with no unified access point.

### Solution

A zero-implementation aggregation crate that:

1. Declares each constituent crate as an optional dependency gated behind a named feature flag.
2. Re-exports each dependency under a short module alias through a five-level namespace hierarchy (dependency, own, orphan, exposed, prelude).
3. Aggregates all enabled crates' exposed and prelude items into unified access points.

The crate contains no implementation logic. All functionality lives in the constituent crates. The aggregator provides organization, aliasing, and a single manifest entry.

### Applicability

**Apply when:**
- A project needs three or more wTools crates and values having a single dependency entry.
- Version coherence across the ecosystem matters more than minimal dependency footprint.
- The project wants a unified prelude combining items from multiple utility crates.

**Do not apply when:**
- Only one or two specific crates are needed — prefer direct dependencies for minimal footprint.
- Fine-grained version control per crate is required beyond what the aggregator pins.
- The project is a library that must minimize transitive dependencies for its own consumers.

### Consequences

**Benefits:**
- Single manifest entry replaces 3-10 individual dependency lines.
- All constituent crates are tested together, ensuring version coherence.
- Short module aliases (iter, meta, mem, etc.) provide cleaner import paths than full crate names.
- Unified prelude reduces import boilerplate for projects using multiple categories.

**Liabilities:**
- Feature flag explosion — the aggregator exposes 60+ feature flags across 10 categories.
- Compile time increases proportionally with the number of enabled features.
- Documentation is fragmented across constituent crates; the aggregator docs serve as a navigation layer.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/lib.rs` | Namespace hierarchy implementation |
| config | `../../Cargo.toml` | Optional dependency declarations |
| doc | `../api/001_namespace_hierarchy.md` | Public namespace surface contract |
| doc | `002_feature_flag_composition.md` | Companion pattern for feature flag structure |
