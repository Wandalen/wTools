# Invariant: Backward Compatibility

### Scope

- **Purpose**: Governs API stability policy after version 1.0 release.
- **Responsibility**: Documents the semver compliance requirement and its scope.
- **In Scope**: All public API items in `src/`; minor and patch version change rules.
- **Out of Scope**: Pre-1.0 releases, internal implementation details.

### Invariant Statement

After version 1.0, all future minor and patch releases must maintain backward compatibility for the public API. Breaking changes are only permitted in major version bumps. Compliance is verified by `cargo-semver-checks`.

### Enforcement Mechanism

Run `cargo semver-checks` before publishing any minor or patch release. Any API removal or signature change that breaks existing callers must be preceded by a major version increment.

### Violation Consequences

Breaking changes in minor/patch releases invalidate dependent projects' `Cargo.lock` guarantees, cause compilation failures for downstream users, and erode trust in the library's stability.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Version declaration governing semver policy |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | NFR7 in original spec; combined source migrated to invariant/ |
