# Feature: Builder Trait Infrastructure

### Scope

- **Purpose**: Provide a reusable, compile-time trait infrastructure that any entity can use to acquire a type-safe builder.
- **Responsibility**: Navigate all artifacts — source files, tests, api docs, algorithm, and invariant — for the core builder pattern capability.
- **In Scope**: All definition, process, and storage traits; the formation lifecycle; the formation integrity invariant.
- **Out of Scope**: Collection-specific subformer support (see feature/002), derive macro code generation.

### Design

The builder pattern trait infrastructure solves a single problem: how to decouple builder construction from the entity being built, while preserving full type safety and zero runtime overhead.

The solution layers three groups of traits. The definition layer declares which types participate in formation — storage container, formed entity, context, mutator, and completion handler. The formation-process layer controls the temporal flow — beginning a sub-former, transforming storage before completion, and converting storage to the final result. The storage layer defines the intermediate container — a default-constructible type that accumulates field values and converts to a preformed entity.

The formation lifecycle sequences these three groups into four strictly ordered phases: begin, accumulation, mutation, completion.

The design choice of associated types over generic parameters means each entity has exactly one builder definition — no ambiguity from multiple parameterizations. All trait method calls resolve at compile time, resulting in zero-cost dispatch.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/definition.rs | Definition layer trait implementations |
| source | ../../src/forming.rs | Formation process trait implementations |
| source | ../../src/storage.rs | Storage trait implementations |
| test | ../../tests/inc/lifetime_mre_test.rs | Lifetime correctness reproducer for FormerBegin |
| doc | api/001_former_definition.md | Definition traits API contract |
| doc | api/002_formation_process.md | Formation process traits API contract |
| doc | api/003_storage.md | Storage traits API contract |
| doc | algorithm/001_formation_lifecycle.md | Four-step formation procedure |
| doc | invariant/001_formation_integrity.md | Formation correctness guarantee |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; overview, architecture, public API, design rationale, and usage patterns sections extracted here |
