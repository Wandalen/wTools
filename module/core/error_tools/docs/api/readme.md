# API Doc Entity

### Scope

- **Purpose**: Document the public interface exposed by `error_tools` for contributors reviewing API contracts.
- **Responsibility**: Master index for all api doc instances in this crate.
- **In Scope**: Instances covering one logical grouping of public entry points each — error-with-report trait, debug assertion macros.
- **Out of Scope**: Behavioral rationale and implementation design — see `feature/` and `invariant/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Error-With-Report Trait](001_err_with_trait.md) | Paired-result conversion trait and type alias | ✅ |
| 002 | [Debug Assertion Macros](002_debug_assertions.md) | Debug-only identity and non-identity assertions | ✅ |
