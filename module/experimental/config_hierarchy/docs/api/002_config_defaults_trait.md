# API: ConfigDefaults Trait

### Scope

- **What**: Contract for supplying default configuration values to `ConfigManager`
- **Who**: Application developers providing fallback values for all known parameters
- **When**: Implementing `ConfigDefaults`; always required
- **Out of scope**: Path configuration (→ api/001), validation (→ api/003)

### Abstract

`ConfigDefaults` is one of the three traits users implement to configure `ConfigManager< D, P, V >`. It defines the lowest-priority source in the resolution hierarchy: the application's built-in fallback values. It also declares the set of known parameter names, which drives `resolve_all_config` to enumerate the full parameter space.

### Interface

```rust
pub trait ConfigDefaults
{
  /// Returns application default values as key-value pairs.
  fn get_defaults() -> HashMap< String, JsonValue >;

  /// Returns the list of all known parameter names.
  ///
  /// Used by `resolve_all_config` to enumerate parameters.
  /// Parameters not in this list are only resolved if found in a file or runtime map.
  fn get_parameter_names() -> Vec< &'static str >;
}
```

### Operations

#### `get_defaults()`

Returns a `HashMap< String, JsonValue >` mapping parameter names to their default `JsonValue`. Parameters not present in this map resolve to `JsonValue::Null` with `ConfigSource::Default` if no higher-priority source provides a value.

#### `get_parameter_names()`

Returns the canonical list of parameter names the application cares about. `resolve_all_config< D, P >()` iterates this list to resolve each parameter. Parameters in config files or runtime maps but not in this list are still resolved — they are picked up in a secondary scan of global and local config files by `resolve_all_config`.

### Example

```rust
use config_hierarchy::ConfigDefaults;
use std::collections::HashMap;
use serde_json::Value as JsonValue;

struct AppDefaults;

impl ConfigDefaults for AppDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "timeout".to_string(), JsonValue::Number( 30.into() ) );
    map.insert( "retries".to_string(), JsonValue::Number( 3.into() ) );
    map.insert( "debug".to_string(), JsonValue::Bool( false ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "retries", "debug" ]
  }
}
```

### Error Handling

Neither method returns a `Result`. Both are expected to be pure and infallible. Panicking inside these methods will propagate to the caller of `resolve_config_value` or `resolve_all_config`.

### Compatibility Guarantees

- Adding new parameters to `get_defaults()` without adding them to `get_parameter_names()` means they are available as defaults but not enumerated by `resolve_all_config`
- Removing a parameter from `get_parameter_names()` without removing it from `get_defaults()` means it is no longer enumerated but still available if directly resolved by name
- Changing a default value changes only the fallback — higher-priority sources are unaffected

### Cross-References

| Type | Target | Relationship |
|------|--------|-------------|
| doc | invariant/001_resolution_hierarchy.md | defaults are the lowest-priority level (priority 6) |
| doc | api/001_config_paths_trait.md | companion required trait |
| doc | api/003_config_validator_trait.md | companion optional trait |
