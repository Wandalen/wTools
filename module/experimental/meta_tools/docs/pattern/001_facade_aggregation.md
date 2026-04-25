# Pattern: Facade Aggregation

### Scope

- **Purpose**: Provide a single import point for a set of related but independently-releasable crates so consumers add one dependency entry and one `use` statement instead of many.
- **Responsibility**: Document the facade aggregation pattern as applied in `meta_tools`: problem context, structural solution, mandatory-vs-optional dependency distinction, naming conventions, and known trade-offs.
- **In Scope**: The aggregation structure of `meta_tools`, why `mod_interface_meta` is a mandatory dependency, feature-gate strategy, naming conventions for re-exported items.
- **Out of Scope**: Individual constituent crate designs, compile-time performance analysis, macro hygiene.

### Problem

The workspace has several related meta-programming crates (`for_each`, `impls_index`, `mod_interface`, `paste`) that are each independently useful but are almost always needed together by consumers. Requiring consumers to list all crates in `Cargo.toml`, manage individual version constraints, and write separate `use` statements for each creates friction and version skew risk as the workspace evolves.

### Solution

Introduce a single facade crate (`meta_tools`) that depends on all constituent crates and re-exports their public APIs under a unified namespace. Consumers add one `[dependencies]` entry and one `use meta_tools::*`.

Key structural decisions in this application:

**Mandatory vs optional dependencies**: Proc-macro crates (`impls_index_meta`, `mod_interface_meta`) are always linked because the facade itself uses `mod_interface!` in its own `meta` module. Declarative counterparts (`impls_index`, `mod_interface`) are optional and feature-gated because the facade does not use them internally.

**Naming consistency**: External crates are re-exported under workspace-consistent names when their original names do not fit conventions. `paste::paste` becomes `meta_idents_concat!` to follow the descriptive-name convention. `impls3` is aliased as `impls` for everyday use.

**Feature isolation**: Each sub-capability has its own feature flag so consumers can opt out of specific dependencies when compile time or binary size matters. All flags default to enabled.

### Applicability

Apply when:
- Multiple related crates are almost always used together by workspace consumers.
- Independent release cycles for constituent crates are needed.
- Consumers benefit from a single version constraint rather than individually managing several.

Do not apply when:
- The constituent crates are conceptually unrelated and grouping them creates a confusing API surface.
- Linking all constituents is costly and most consumers need only one.
- The facade would need to re-implement logic rather than purely re-export.

### Consequences

**Benefits:**
- Consumers manage one dependency entry and one import path.
- Consistent macro naming via re-export aliases.
- Feature flags let cost-sensitive consumers opt out of unused dependencies.

**Trade-offs:**
- Adding a new capability requires updating `Cargo.toml` and re-export entries in the facade — two locations instead of one.
- The current implementation duplicates re-export logic between `src/dependency.rs::exposed` and `src/exposed.rs` instead of delegating one to the other. This is a known smell arising from the two-path access design (`meta_tools::*` and `meta_tools::dependency::*`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Top-level feature-gated re-exports |
| Source | `src/dependency.rs` | All dependency imports and explicit re-exports |
| Source | `Cargo.toml` | Mandatory vs optional dependency declarations |
| Doc | `docs/feature/003_module_interface.md` | Why `mod_interface_meta` is a mandatory dependency |
| Doc | `docs/api/001_macros.md` | Complete macro API surface |
