# API: ConfigDefaults Trait

### Scope

- **Purpose**: Define the contract for supplying default configuration values to ConfigManager.
- **Responsibility**: Document the two required methods, their behavior, and compatibility notes.
- **In Scope**: Implementing ConfigDefaults; get_defaults() and get_parameter_names() methods.
- **Out of Scope**: Path configuration (→ api/001), validation (→ api/003).

### Abstract

`ConfigDefaults` is one of the three traits users implement to configure `ConfigManager< D, P, V >`. It defines the lowest-priority source in the resolution hierarchy: the application's built-in fallback values. It also declares the set of known parameter names, which drives `resolve_all_config` to enumerate the full parameter space.

### Operations

#### `get_defaults()`

Returns a `HashMap< String, JsonValue >` mapping parameter names to their default `JsonValue`. Parameters not present in this map resolve to `JsonValue::Null` with `ConfigSource::Default` if no higher-priority source provides a value.

#### `get_parameter_names()`

Returns the canonical list of parameter names the application cares about. `resolve_all_config< D, P >()` iterates this list to resolve each parameter. Parameters in config files or runtime maps but not in this list are still resolved — they are picked up in a secondary scan of global and local config files by `resolve_all_config`.

### Error Handling

Neither method returns a `Result`. Both are expected to be pure and infallible. Panicking inside these methods will propagate to the caller of `resolve_config_value` or `resolve_all_config`.

### Compatibility Guarantees

- Adding new parameters to `get_defaults()` without adding them to `get_parameter_names()` means they are available as defaults but not enumerated by `resolve_all_config`
- Removing a parameter from `get_parameter_names()` without removing it from `get_defaults()` means it is no longer enumerated but still available if directly resolved by name
- Changing a default value changes only the fallback — higher-priority sources are unaffected

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Companion required trait |
| [api/003_config_validator_trait.md](../api/003_config_validator_trait.md) | Companion optional trait |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature this trait is part of |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Defaults are the lowest-priority level (priority 6) |
