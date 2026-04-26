# Invariant: Alloc Feature Requires No-Std

### Scope

- **Purpose**: Guarantee that use_alloc is never enabled in a standard-library build where it would be redundant and misleading.
- **Responsibility**: Documents the feature dependency contract: use_alloc implies and requires no_std.
- **In Scope**: The use_alloc and no_std feature flags and their declared dependency relationship.
- **Out of Scope**: Specific allocator configuration, heap sizing, or embedded linker setup.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/004_no_std_support.md](../feature/004_no_std_support.md) | No-std feature subject to this invariant |

### Invariant Statement

The use_alloc feature flag always implies and enables no_std. It is not valid to enable use_alloc without no_std — doing so would have no meaningful effect and would misrepresent the build configuration.

### Enforcement Mechanism

- Cargo feature declaration: use_alloc = [ "no_std" ] — enabling use_alloc automatically activates no_std.

### Violation Consequences

Removing the no_std dependency from use_alloc would allow consumers to enable use_alloc in standard-library builds, creating a misleading configuration that silently does nothing.
