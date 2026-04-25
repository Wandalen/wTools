# Feature: Error Facade

### Scope

**Purpose:** Provide a single unified entry point for all error handling primitives.

**Responsibility:** Aggregate typed and untyped error handling under one namespace, eliminating direct dependencies on upstream error libraries from consumer crates.

**In Scope:**
- Re-exporting typed error handling primitives under the crate namespace
- Re-exporting untyped error handling primitives under the crate namespace
- Exposing the error-with-report mechanism for paired result handling
- Exposing debug assertion macros for identity and non-identity checks

**Out of Scope:**
- Defining new error types (consumer responsibility)
- Custom error formatting or rendering
- Error serialization or transport

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Feature | feature/002_typed_errors.md | Typed error derivation via thiserror |
| Feature | feature/003_untyped_errors.md | Untyped dynamic errors via anyhow |
| Feature | feature/004_no_std_support.md | Optional constrained-environment support |
| API | api/001_err_with_trait.md | Error-with-report trait and result alias |
| API | api/002_debug_assertions.md | Debug-only identity assertion macros |
| Invariant | invariant/001_exclusive_dependency.md | Sole error framework mandate |
| Invariant | invariant/002_zero_cost_facade.md | Pure re-export with no overhead |

### Design

**Pattern:** Facade — a thin re-export layer that consolidates upstream library APIs behind a stable module boundary. Downstream code imports from `error_tools` rather than from `anyhow` or `thiserror` directly.

**Motivation:** When the underlying error library is imported by multiple crates in a workspace, version skew and API fragmentation become friction points. The facade pins the API surface and lets the workspace upgrade underlying libraries in one place.

**Component Structure:**
- Typed component: activated by the `error_typed` feature; re-exports the derive macro for structured errors with field capture
- Untyped component: activated by the `error_untyped` feature; re-exports dynamic error creation, context chaining, and the boxed error type
- Assert component: always active; provides debug-only identity assertion macros stripped from release builds
- Core component: always active when `enabled`; provides the error-with-report trait and the paired-result type alias
