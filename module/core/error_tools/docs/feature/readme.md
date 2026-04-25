# Feature Doc Entity

### Scope

- **Purpose**: Document capabilities provided by `error_tools` for contributors and consumers.
- **Responsibility**: Master index for all feature doc instances in this crate.
- **In Scope**: Instances covering one cohesive behavioral area each — error facade, typed errors, untyped errors, no_std support.
- **Out of Scope**: API contracts and behavioral invariants — see `api/` and `invariant/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Error Facade](001_error_facade.md) | Unified re-export namespace for all error primitives | ✅ |
| 002 | [Typed Errors](002_typed_errors.md) | Structured named error types via thiserror | ✅ |
| 003 | [Untyped Errors](003_untyped_errors.md) | Dynamic context-chained errors via anyhow | ✅ |
| 004 | [No-Std Support](004_no_std_support.md) | Feature flags for constrained environments | ✅ |
