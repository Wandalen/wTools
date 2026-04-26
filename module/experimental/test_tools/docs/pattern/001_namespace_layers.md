# Pattern: Namespace Layers

### Scope

- **Purpose**: Provides controlled API surface with graduated visibility for aggregator crates.
- **Responsibility**: Documents the four-layer nested namespace module pattern used in `test_tools`.
- **In Scope**: The own/orphan/exposed/prelude module hierarchy and visibility semantics.
- **Out of Scope**: Feature-gating rules within namespaces; see `docs/invariant/`.

### Problem

Aggregator crates re-exporting from multiple constituent crates need a structured way to control API visibility at different levels of access.

### Solution

Four nested namespace modules with graduated visibility: `own` (widest), `orphan`, `exposed` (primary for aggregated tests), `prelude` (essentials for glob imports).

### Applicability

Apply in any crate that aggregates APIs from multiple constituent crates and needs graduated visibility for different consumer contexts.

### Consequences

Consistent `the_module::exposed::*` import paths across all aggregated test files. Namespace module visibility must never be conditionally hidden (see `docs/invariant/001_no_cfg_gate_on_namespace.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Own/orphan/exposed/prelude module implementations |
| test | `tests/mod_interface_aggregation_tests.rs` | Tests namespace aggregation correctness |
| doc | `docs/feature/001_test_aggregation_facade.md` | Feature that applies this pattern |
| doc | `docs/invariant/001_no_cfg_gate_on_namespace.md` | Invariant protecting namespace module visibility |
