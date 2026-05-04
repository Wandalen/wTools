# Pattern: Facade Re-export

### Scope

- **Purpose**: Explain why data_type aggregates upstream re-exports rather than providing its own type implementations.
- **Responsibility**: Documents the facade re-export structural pattern — problem context, solution shape, applicability, and tradeoffs.
- **In Scope**: The aggregation model, feature-gated dependency structure, and design decisions behind the crate's existence.
- **Out of Scope**: The invariant contract that enforces pure re-export — that belongs in invariant/001.

### Cross-References

| Type   | File | Responsibility |
|--------|------|----------------|
| source | [`src/lib.rs`](../../src/lib.rs) | Facade entry point implementing the pattern |
| source | [`src/dt.rs`](../../src/dt.rs) | dt sub-namespace — re-export delegation chain |
| config | [`Cargo.toml`](../../Cargo.toml) | Feature flag declarations — one per aggregated dependency |
| doc    | [invariant/001_pure_aggregator.md](../invariant/001_pure_aggregator.md) | Invariant enforcing the pure re-export constraint |
| doc    | [api/001_namespace_chain.md](../api/001_namespace_chain.md) | Public namespace structure produced by this pattern |

### Problem

Workspace consumers that need intervals, collections, and sum types must manage three separate dependencies, pin their versions independently, and keep them mutually compatible. The per-consumer overhead scales with the number of crates that share these data types.

### Solution

A single facade crate aggregates re-exports from all three sources under one versioned dependency. Consumers declare one dependency; the facade guarantees compatible versions of its underlying crates. Each upstream source maps to one opt-in feature flag, all enabled by default, so consumers can exclude data types they do not need.

The re-export chain moves items from upstream crates through an internal dt sub-namespace, then into the top-level exposed namespace, making them accessible at the crate root.

### Applicability

Use this pattern when multiple data-type crates are used together across many workspace consumers, when version compatibility between the underlying crates must be centrally managed, and when consumers should be able to opt out of individual data types without switching to direct dependencies.

Avoid for library crates: libraries should declare direct dependencies on the specific crates they use to avoid transitive dependency over-reach in downstream consumers.

### Consequences

**Benefits:** Single import point for common data types, centrally managed upstream versions, modular opt-in via feature flags, full upstream APIs accessible through the dependency module when top-level re-exports are insufficient.

**Tradeoffs:** Extra indirection layer adds one compile-time dependency hop. Consumers cannot independently version the underlying crates. Reserved feature flags exist in the manifest for planned future integrations but are not yet implemented.
