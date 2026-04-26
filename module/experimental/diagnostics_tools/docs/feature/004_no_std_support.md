# Feature: No-Std Support

### Scope

- **Purpose**: Enable use of diagnostics_tools in embedded and constrained environments without the standard library.
- **Responsibility**: Documents the no_std and use_alloc feature flags and their effect on crate behavior.
- **In Scope**: The no_std and use_alloc feature flags and their interactions with assertion behavior.
- **Out of Scope**: Specific embedded targets, linker configuration, platform-specific concerns.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [invariant/004_alloc_requires_no_std.md](../invariant/004_alloc_requires_no_std.md) | use_alloc requires no_std to be enabled |

### Design

The no_std feature flag disables the standard library dependency, making diagnostics_tools usable in embedded and constrained environments. Compile-time assertions and memory layout assertions function identically under no_std, as they rely only on compiler intrinsics. Runtime assertions continue to work with reduced formatting capabilities.

The use_alloc feature extends no_std with heap allocation support and must not be enabled without no_std.
