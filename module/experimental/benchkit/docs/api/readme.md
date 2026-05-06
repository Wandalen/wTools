# API Doc Entity

### Scope

- **Purpose**: Document the public programmatic interface that benchkit exposes to benchmark authors.
- **Responsibility**: Collects API surface documentation: operations, error types, and compatibility guarantees.
- **In Scope**: Types and operations available via `benchkit::prelude::*` and feature-gated public modules.
- **Out of Scope**: Internal implementation details (→ src/); usage patterns and worked examples (→ readme.md, usage.md).

### Overview Table

| ID  | Name                                       | Purpose                                          | Status |
|-----|--------------------------------------------|--------------------------------------------------|--------|
| 001 | [benchkit Public API](001_benchkit_api.md) | Full public interface across all feature flags   | ✅ |
