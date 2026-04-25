# Feature: CI/CD Environment Detection

### Scope

- **Purpose**: Let automation code adapt its behavior when running inside a CI/CD pipeline without adding dependencies.
- **Responsibility**: Owns `is_cicd()` as the single feature-gated check for pipeline environment detection.
- **In Scope**: Environment-variable-based detection logic, supported CI platform list, and feature-gate design.
- **Out of Scope**: Modifying environment variables; any process execution or lifecycle behavior.

### Status

- **Version introduced:** 0.5.0
- **Stability:** stable
- **Module path:** `process_tools::environment`
- **Feature gate:** `process_environment_is_cicd` — included in both `default` and `full`

### Design

Detection is purely environment-variable-based: the function calls `std::env::var()` for each known CI variable and returns `true` on the first hit. No process introspection, no filesystem reads, no external calls. This makes the function `O(n)` in the number of CI platforms checked and free of I/O side effects.

The feature is included in both `default` and `full`, so projects get `is_cicd()` without any special configuration. Callers that don't need CI detection can exclude it by using `default-features = false` and omitting `process_environment_is_cicd` from their explicit features list.

The supported variables (`CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `TRAVIS`, `CIRCLECI`, `JENKINS_URL`) are checked by presence — their value is irrelevant. Any truthy or empty value causes a match.

### Example

```rust
#[ cfg( feature = "process_environment_is_cicd" ) ]
{
  use process_tools::environment;

  // In a CI pipeline one of the known variables will be set
  if environment::is_cicd() {
    println!( "running in CI" );
  }
}
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/environment.rs](../../src/environment.rs) | `is_cicd()` implementation and CI variable list |
| test | [tests/inc/environment_is_cicd.rs](../../tests/inc/environment_is_cicd.rs) | CI variable detection tests |
| doc | [api/007_environment_api.md](../api/007_environment_api.md) | `is_cicd()` function signature and detection contract |
| doc | [feature/001_process_execution.md](001_process_execution.md) | Primary execution feature; CI detection complements it |
