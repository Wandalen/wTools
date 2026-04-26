# API: CI/CD Environment Detection

### Scope

- **Purpose**: Define the complete public surface of `environment::is_cicd()` — the sole function exported by the `environment` module.
- **Responsibility**: Documents the function signature, feature gate requirement, detection mechanism, the six CI platforms checked, and presence-not-value semantics.
- **In Scope**: `is_cicd()` signature, `process_environment_is_cicd` feature gate, the six env vars checked in order, presence-based detection semantics, and return contract.
- **Out of Scope**: Modifying environment variables; process execution (→ `api/001`); any CI-specific execution logic.

### Abstract

`process_tools::environment::is_cicd()` detects whether the current process is running inside a CI/CD pipeline. Detection is purely environment-variable-based: six well-known CI variables are checked for presence (not value). The function is gated behind the `process_environment_is_cicd` feature, which is included in both `default` and `full`.

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `is_cicd()` | free fn | `#[must_use]`; requires `process_environment_is_cicd` feature |

**Detection table** — variables checked, in order:

| Variable | CI/CD System |
|----------|-------------|
| `CI` | Common across many CI systems |
| `GITHUB_ACTIONS` | GitHub Actions |
| `GITLAB_CI` | GitLab CI |
| `TRAVIS` | Travis CI |
| `CIRCLECI` | CircleCI |
| `JENKINS_URL` | Jenkins |

Detection is presence-based: `std::env::var(var).is_ok()`. Variable value is irrelevant — `CI=0` and `CI=true` both return `true`.

### Error Handling

`is_cicd()` is infallible and returns only `bool`. No `Result`, no panics, no I/O, no filesystem access.

### Compatibility Guarantees

- **Platform:** all targets — no `#[cfg(unix)]` restriction.
- **Feature gate:** `process_environment_is_cicd`, included in both `default` and `full`. To opt out: `default-features = false` and omit this feature from the explicit list.
- **Detection order:** variables are checked in the order listed above; first match returns `true`. Order is stable.
- **Presence semantics:** only variable existence is checked, not value. This matches the convention of all major CI systems.
- **`#[must_use]`:** unused return value is a compile-time warning.
- **Stability:** stable since 0.5.0.

### Example

```rust
#[ cfg( feature = "process_environment_is_cicd" ) ]
{
  use process_tools::environment;

  if environment::is_cicd() {
    println!( "running in CI — skipping interactive steps" );
  }
}
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/environment.rs](../../src/environment.rs) | `is_cicd()` implementation and CI variable list |
| test | [tests/inc/environment_is_cicd.rs](../../tests/inc/environment_is_cicd.rs) | CI variable detection tests |
| doc | [feature/003_environment_detection.md](../feature/003_environment_detection.md) | Design rationale and feature gate design |
