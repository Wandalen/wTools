# Pattern: Abstraction Layer

### Scope
- **Purpose**: Prevent direct dependency coupling between consumer crates and syn, quote, and proc-macro2.
- **Responsibility**: Document the structural pattern of a thin abstraction layer over proc-macro primitives.
- **In Scope**: Dependency centralization, version unification, higher-level API surface over raw primitives.
- **Out of Scope**: Specific utilities the layer provides → feature/; stability contracts → invariant/001.

### Problem
Ten or more meta crates each need the same proc-macro primitives. Without centralization,
every meta crate carries direct dependencies on syn, quote, and proc-macro2 independently.
Version upgrades require coordinated changes across the whole ecosystem. Common parsing and
code-generation patterns are duplicated independently in each crate. Error messages vary in
format across crates because each crate produces its own diagnostics without shared convention.

### Solution
One crate holds all direct dependencies on syn, quote, and proc-macro2. Consumer meta-crates
depend only on that crate. The layer re-exports the underlying primitives so consumers access
them through a single import point and benefit from a shared, higher-level API built on top.
Common patterns — attribute parsing, generic manipulation, span-aware errors — are implemented
once in the layer and available to all consumers without duplication. All consumers use the
same underlying library versions because they have no independent version declarations.

### Applicability
Apply when multiple crates in an ecosystem all need the same low-level library; when version
upgrades across that ecosystem would be costly if scattered; when common patterns would benefit
from centralization rather than per-crate reimplementation; when consistent user-facing behavior
such as error message format is required across all crates.

### Consequences
**Positive**: Single upgrade point for shared dependencies; consistent error messages and
conventions across all consumers; shared utilities eliminate duplication; reduced total
dependency declarations in the ecosystem.

**Negative**: An additional indirection layer between consumer and primitive; consumers cannot
independently upgrade proc-macro primitives when needed; the abstraction layer itself becomes
a critical dependency that all consumers must coordinate upgrades through.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Re-export and dependency module structure |
| config | `Cargo.toml` | Declares the sole syn, quote, and proc-macro2 dependency |
| doc | `docs/invariant/001_unified_versioning.md` | Invariant this pattern enforces |
| doc | `docs/api/001_attribute_component_api.md` | Higher-level trait API built on this pattern's re-exports |
