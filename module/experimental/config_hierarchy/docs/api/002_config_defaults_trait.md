# API: ConfigDefaults Trait

### Scope

- **Purpose**: Define the contract for supplying default configuration values to ConfigManager.
- **Responsibility**: Document the two required methods, their behavior, and compatibility notes.
- **In Scope**: Implementing ConfigDefaults; get_defaults() and get_parameter_names() methods.
- **Out of Scope**: Path configuration (→ api/001), validation (→ api/003).

### Abstract

`ConfigDefaults` is one of the three traits users implement to configure the manager type. It defines the lowest-priority source in the resolution hierarchy: the application's built-in fallback values. It also declares the set of known parameter names, which drives full-config resolution to enumerate the complete parameter space.

### Operations

#### `get_defaults()`

Returns a map of parameter names to their default typed values. Parameters not present in this map resolve to null with a default source label if no higher-priority source provides a value.

#### `get_parameter_names()`

Returns the canonical list of parameter names the application cares about. Full-config resolution iterates this list to resolve each parameter. Parameters in config files or runtime maps but not in this list are still resolved — they are picked up in a secondary scan of global and local config files.

### Error Handling

Neither method returns a `Result`. Both are expected to be pure and infallible. Panicking inside these methods will propagate to the caller of any resolution function.

### Compatibility Guarantees

- Adding new parameters to `get_defaults()` without adding them to `get_parameter_names()` means they are available as defaults but not enumerated by full-config resolution
- Removing a parameter from `get_parameter_names()` without removing it from `get_defaults()` means it is no longer enumerated but still available if directly resolved by name
- Changing a default value changes only the fallback — higher-priority sources are unaffected

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Companion required trait |
| [api/003_config_validator_trait.md](../api/003_config_validator_trait.md) | Companion optional trait |
| [api/004_config_manager.md](../api/004_config_manager.md) | Manager type that takes this trait as a type parameter |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature this trait is part of |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Defaults are the lowest-priority level (priority 6) |

### Sources

| File | Relationship |
|------|--------------|
| [src/traits.rs](../../src/traits.rs) | `ConfigDefaults` trait definition |
| [src/hierarchy.rs](../../src/hierarchy.rs) | Consumes get_defaults() at resolution level 6 |

### Tests

| File | Relationship |
|------|--------------|
| [tests/configurability_tests.rs](../../tests/configurability_tests.rs) | Custom default value implementation tests |
| [tests/basic_operations_tests.rs](../../tests/basic_operations_tests.rs) | Default value resolution tests |
