# Feature: Async Toolkit Facade

### Scope

- **Purpose**: Provide a single entry point for async conversion utilities, eliminating the need for downstream crates to manage multiple direct dependencies.
- **Responsibility**: Documents the unified async toolkit surface — the traits, macros, and namespaces that async_tools aggregates and re-exports from its dependencies.
- **In Scope**: The async conversion trait family, the async_trait macro re-export, and the own/orphan/exposed/prelude namespace structure provided by this crate.
- **Out of Scope**: Individual trait contracts (see `async_from/docs/api/`), trait invariants, and async runtime specifics.

### Design

async_tools acts as a facade over the async conversion ecosystem. It re-exports all async conversion traits and the async_trait macro under a single unified namespace, so downstream crates depend only on async_tools rather than on multiple individual crates.

The crate provides four namespace layers — own, orphan, exposed, and prelude — following the standard wTools namespace convention. All exports are gated on the `enabled` feature flag, which callers activate explicitly.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Namespace structure and re-export declarations |
| test | `tests/tests.rs` | Facade re-export validation — all async conversion traits accessible via async_tools |
| doc | `../../../async_from/docs/feature/001_infallible_async_conversion.md` | Infallible async conversion feature documentation |
| doc | `../../../async_from/docs/feature/002_fallible_async_conversion.md` | Fallible async conversion feature documentation |
