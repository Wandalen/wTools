# API Doc Entity

### Scope

- **Purpose**: Document the public namespace surface contract that all consumers of wtools depend on.
- **Responsibility**: Define the five-level re-export namespace and module access semantics.
- **In Scope**: The dependency, own, orphan, exposed, and prelude namespace layers; module alias mapping; crate root re-export.
- **Out of Scope**: Individual sub-crate APIs (see each sub-crate's own docs/api/), feature flag details (see feature/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Namespace Hierarchy](001_namespace_hierarchy.md) | Five-level re-export namespace contract | ✅ |
