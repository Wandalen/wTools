# API: ConfigDefaults Trait

### Scope

- **Purpose**: Define the default configuration values contract for the configuration manager.
- **Responsibility**: Documents operations, error conditions, and compatibility guarantees of ConfigDefaults.
- **In Scope**: Default value supply, known parameter name enumeration, interaction with the resolution hierarchy.
- **Out of Scope**: Path configuration (→ api/001), validation hooks (→ api/003).

### Abstract

ConfigDefaults is one of the three traits applications implement to configure the manager. It defines the lowest-priority source in the resolution hierarchy: the application's built-in fallback values. It also declares the set of known parameter names, which drives the full-resolution operation to enumerate the complete parameter space.

### Operations

#### `get_defaults()`

Returns a map of parameter names to their default values. Parameters absent from this map resolve to a null value with Default provenance when no higher-priority source provides a value.

#### `get_parameter_names()`

Returns the canonical list of parameter names the application cares about. The resolve-all operation iterates this list to resolve each named parameter. Parameters in config files or runtime maps that are not in this list are still resolved — they are picked up in a secondary scan of global and local config files.

### Error Handling

Neither operation returns an error. Both are expected to be pure and infallible. An exception thrown inside these operations will propagate to the caller of the resolution function.

### Compatibility Guarantees

- Adding new parameters to the defaults map without adding them to the parameter names list means they are available as defaults but not enumerated by the resolve-all operation
- Removing a parameter from the names list without removing it from the defaults map means it is no longer enumerated but still available if directly resolved by name
- Changing a default value changes only the fallback — higher-priority sources are unaffected

### Cross-References

| Type | File                                  | Responsibility                                       |
|------|---------------------------------------|------------------------------------------------------|
| doc  | invariant/001_resolution_hierarchy.md | Defaults are the lowest-priority level (priority 6)  |
| doc  | api/001_config_paths_trait.md         | Companion required trait for path configuration      |
| doc  | api/003_config_validator_trait.md     | Companion optional trait for validation              |
