# Invariant: No cfg Gate on Namespace Modules

### Scope

- **Purpose**: Prevents widespread compilation failures caused by hiding public namespace modules behind feature cfg guards.
- **Responsibility**: Documents the invariant that namespace modules must remain unconditionally visible to tests.
- **In Scope**: The `own`, `orphan`, `exposed`, and `prelude` modules declared in `src/lib.rs`.
- **Out of Scope**: Feature gating of functionality within modules; only the module declarations themselves are constrained.

### Invariant Statement

The four namespace modules (`own`, `orphan`, `exposed`, `prelude`) must be declared with the `enabled` feature gate only — never with additional gates such as `not(doctest)` or similar conditions that would hide them during test compilation.

### Enforcement Mechanism

Code review of `src/lib.rs` module declarations. REGRESSION PREVENTION comments in `src/lib.rs` document the historical context and guard against re-introduction.

### Violation Consequences

Hiding a namespace module behind a doctest cfg gate causes the test runner (which enables the `doctest` feature via `.cargo/config.toml` rustdocflags) to make the cfg condition true, hiding the module. This triggers 100+ E0432 compilation errors across all aggregated tests that import from `the_module::exposed::*` and similar paths.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Namespace module declarations with REGRESSION PREVENTION comments |
| test | `tests/smoke_test.rs` | Verifies all namespace imports work |
| task | `task/completed/001_fix_test_compilation_failures.md` | Task that resolved 147 E0432 errors |
| doc | `docs/feature/001_test_aggregation_facade.md` | Feature that owns these namespace modules |
| doc | `docs/pattern/001_namespace_layers.md` | Pattern that applies this namespace invariant |
