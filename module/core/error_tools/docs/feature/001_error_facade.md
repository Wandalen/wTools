# Feature: Error Facade

### Scope

- **Purpose**: Provide a single unified entry point for all error handling primitives.
- **Responsibility**: Documents the error facade feature — its scope, design, and cross-references to all related artifacts.
- **In Scope**: Consolidating typed errors, untyped errors, error-with-report mechanism, and debug assertion macros under one namespace.
- **Out of Scope**: Defining new error types, custom formatting, serialization, or transport — consumer responsibility.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_typed_errors.md](002_typed_errors.md) | Typed error derivation via thiserror |
| doc | [feature/003_untyped_errors.md](003_untyped_errors.md) | Untyped dynamic errors via anyhow |
| doc | [feature/004_no_std_support.md](004_no_std_support.md) | Optional constrained-environment support |
| doc | [api/001_err_with_trait.md](../api/001_err_with_trait.md) | Error-with-report operation set and result shorthand |
| doc | [api/002_debug_assertions.md](../api/002_debug_assertions.md) | Debug-only identity assertion macros |
| doc | [invariant/001_exclusive_dependency.md](../invariant/001_exclusive_dependency.md) | Sole error framework mandate |
| doc | [invariant/002_zero_cost_facade.md](../invariant/002_zero_cost_facade.md) | Pass-through with no overhead |

### Design

**Pattern:** Facade — a thin pass-through layer that consolidates upstream library APIs behind a stable module boundary. Downstream code imports from `error_tools` rather than from `anyhow` or `thiserror` directly.

**Motivation:** When the underlying error library is imported by multiple crates in a workspace, version skew and API fragmentation become friction points. The facade pins the API surface and lets the workspace upgrade underlying libraries in one place.

**Opt-in Activation Model:** `default = []` — no API surface is active unless a feature is explicitly requested. This prevents accidental activation when `error_tools` appears as a transitive workspace dependency. The `enabled` feature is the master gate: all crate-root exports are compiled only when `enabled` is active. Consumers opt in via:
- `enabled` — activates all core API surface (ErrWith, ResultWithReport, debug assertion macros, ErrorTrait re-export)
- `error_typed` — activates typed error derivation via thiserror (requires `enabled` for namespace access)
- `error_untyped` — activates dynamic error handling via anyhow (requires `enabled` for namespace access)
- `full` — activates `enabled` + `error_typed` + `error_untyped`

**Component Structure:**
- Typed component: activated by the `error_typed` feature; exposes the error type derivation mechanism for structured errors with field capture
- Untyped component: activated by the `error_untyped` feature; exposes dynamic error creation, context chaining, and the heap-allocated error type
- Assert component: active when `enabled`; provides debug-only identity assertion macros stripped from release builds
- Core component: active when `enabled`; provides the error-with-report operation set and the paired-result return type shorthand
