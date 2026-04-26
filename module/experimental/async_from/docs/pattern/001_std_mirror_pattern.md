# Pattern: Std Mirror Pattern

### Scope

- **Purpose**: Design async conversion traits as direct counterparts to standard library conversion traits, minimising learning curve and enabling predictable API shape.
- **Responsibility**: Documents the decision to mirror the standard From/Into/TryFrom/TryInto traits and its consequences for API surface and blanket impl structure.
- **In Scope**: Trait naming, method naming, associated type naming, blanket impl derivation, and the async_trait macro bridge.
- **Out of Scope**: Runtime specifics, executor integration, and alternative async trait designs such as return-position impl Trait.

### Problem

Rust's standard conversion traits are pervasive, but none support async operations. Codebases that need async conversion — loading config from an environment, deserializing from a database, resolving identifiers through network calls — have no standard interface. Inventing ad-hoc patterns per project creates inconsistency and prevents generic code.

### Solution

Define AsyncFrom, AsyncInto, AsyncTryFrom, and AsyncTryInto with the same API shape as their standard library counterparts: same method naming convention, same associated type for the fallible pair, and the same blanket derivation model — the target-side traits derive from the implementor-side traits. The only additions are async execution and the async_trait macro attribute.

### Applicability

Apply this pattern when:
- Type conversion requires awaiting a future — IO-bound initialization, remote resolution, lazy loading
- The result is infallible — implement AsyncFrom and let AsyncInto derive automatically
- The result may fail — implement AsyncTryFrom and let AsyncTryInto derive automatically
- Callers should write generic async conversion code without learning new trait shapes

### Consequences

Callers familiar with the standard From/Into or TryFrom/TryInto traits transfer knowledge directly with no re-learning. Generic code can be parameterised over AsyncFrom in the same way standard code uses From.

The async_trait macro adds a heap allocation per call by boxing the returned future; this is the primary performance trade-off versus using a direct concrete async method. The thread-safety requirement on the blanket impls restricts types to those safe for multi-threaded runtimes; single-thread use still works via direct AsyncFrom implementations without relying on the blanket.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | All four trait definitions implementing this pattern |
| test | `tests/inc/basic_test.rs` | Validates all four async traits apply the std mirror pattern correctly |
| doc | `docs/feature/001_infallible_async_conversion.md` | Infallible conversion feature applying this pattern |
| doc | `docs/feature/002_fallible_async_conversion.md` | Fallible conversion feature applying this pattern |
| doc | `docs/api/001_async_from.md` | AsyncFrom API |
| doc | `docs/api/002_async_into.md` | AsyncInto blanket API |
| doc | `docs/api/003_async_try_from.md` | AsyncTryFrom API |
| doc | `docs/api/004_async_try_into.md` | AsyncTryInto blanket API |
