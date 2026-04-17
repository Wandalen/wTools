# Feature: CI/CD Environment Detection

### Scope

- **Purpose**: Let automation code adapt its behavior when running inside a CI/CD pipeline without adding dependencies.
- **Responsibility**: Owns `is_cicd()` as the single opt-in check for pipeline environment detection, behind its own feature flag.
- **In Scope**: Environment-variable-based detection logic, supported CI platform list, and feature-gate design.
- **Out of Scope**: Modifying environment variables; any process execution or lifecycle behavior.

### Status

- **Version introduced:** 0.5.0
- **Stability:** stable
- **Module path:** `process_tools::environment`
- **Feature gate:** `process_environment_is_cicd`

### Design

Detection is purely environment-variable-based: the function calls `std::env::var()` for each known CI variable and returns `true` on the first hit. No process introspection, no filesystem reads, no external calls. This makes the function `O(n)` in the number of CI platforms checked (currently six) and free of I/O side effects.

The function is feature-gated behind `process_environment_is_cicd` to avoid pulling unused CI-detection logic into binaries that don't need it. Projects that only use the process execution layer pay no cost.

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
| api | [api/001_run_api.md](../api/001_run_api.md) | Process execution API; callers often skip certain behaviors in CI |
| feature | [feature/001_process_execution.md](001_process_execution.md) | Primary execution feature; CI detection complements it |
