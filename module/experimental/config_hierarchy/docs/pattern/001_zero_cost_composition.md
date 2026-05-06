# Pattern: Zero-Cost Composition

### Scope

- **Purpose**: Document the phantom type parameter pattern used to compose configurable traits without runtime cost.
- **Responsibility**: Define the pattern structure, trade-offs, and when to apply it.
- **In Scope**: The manager type composition pattern; phantom type parameter usage.
- **Out of Scope**: Individual trait contracts (→ api/001, api/002, api/003).

### Abstract

Composes multiple configurable behaviors into a single manager type with zero runtime overhead. The manager type carries no fields — all behavior comes from compile-time monomorphization of generic type parameters. This eliminates dynamic dispatch, vtable allocation, and any heap cost from the composition itself.

### Structure

1. Define three behavior traits (path config, defaults, validation)
2. Create a manager type with three generic type parameters, one per trait
3. The manager type has no fields — only zero-sized phantom markers
4. All manager methods are associated functions that call trait methods on the concrete types
5. At the call site, specify the three concrete types once and the compiler generates a specialized implementation

### When to Apply

Apply this pattern when:
- Behavior is known at compile time and does not need to change at runtime
- Multiple orthogonal behaviors need composition without coupling
- The cost of trait objects (dynamic dispatch, heap allocation) is undesirable
- The composition itself should have zero memory footprint

Do not apply when runtime polymorphism is needed (different implementations selected at runtime).

### Trade-offs

| Benefit | Cost |
|---------|------|
| Zero runtime overhead — no vtable, no heap allocation | Binary bloat — monomorphization generates one copy per type combination |
| Compile-time type checking of all three behaviors | Type signatures become verbose at call sites |
| No lifetime issues from trait objects | Cannot store different implementations in the same collection |

### APIs

| File | Relationship |
|------|--------------|
| [api/004_config_manager.md](../api/004_config_manager.md) | The type that implements this pattern |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that uses this composition pattern |

### Sources

| File | Relationship |
|------|--------------|
| [src/manager.rs](../../src/manager.rs) | Pattern implementation |

### Tests

| File | Relationship |
|------|--------------|
| [tests/configurability_tests.rs](../../tests/configurability_tests.rs) | Tests exercising different type parameter combinations |
