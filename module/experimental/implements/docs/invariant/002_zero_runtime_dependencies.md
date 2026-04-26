# Invariant: Zero Runtime Dependencies

### Scope

- **Purpose**: Preserve the crate's zero-cost integration guarantee — it must be addable to any project without introducing third-party runtime dependencies.
- **Responsibility**: Documents the zero runtime dependency invariant — its precise statement, how it is enforced, and what breaks if it is violated.
- **In Scope**: Runtime dependencies declared in the dependencies section of Cargo.toml.
- **Out of Scope**: Dev-only dependencies, build-time tooling, transitive dependencies introduced by the caller.

### Invariant Statement

For all released versions: the dependencies section of Cargo.toml is empty. The crate introduces zero runtime transitive dependencies into any project that depends on it. The macro mechanism relies exclusively on core standard library facilities — specifically the zero-sized marker type used for phantom type construction — with no third-party crates required.

### Enforcement Mechanism

The dependencies table in Cargo.toml is kept empty by structural policy. Any addition to dependencies is immediately visible in the manifest and in the published crate graph — no tooling automation is needed because the constraint is trivially auditable by inspection.

### Violation Consequences

Any addition to dependencies makes the crate unsuitable as a zero-cost diagnostic utility. Callers that added the crate specifically to avoid transitive dependency weight would acquire unwanted third-party crates, potentially introducing version conflicts, compile-time bloat, or license obligations incompatible with their project.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Authoritative dependencies declaration — must remain empty |
| doc | `docs/feature/001_trait_implementation_check.md` | Feature whose lightweight design this invariant protects |
| doc | `docs/api/001_implements.md` | Primary macro this invariant constrains |
| doc | `docs/api/002_instance_of.md` | Alias macro equally constrained |
