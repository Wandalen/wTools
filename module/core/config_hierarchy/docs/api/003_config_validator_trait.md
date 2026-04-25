# API: ConfigValidator Trait

### Scope

- **What**: Contract for validating resolved configuration values in `ConfigManager`
- **Who**: Application developers adding per-parameter or cross-parameter validation
- **When**: Implementing `ConfigValidator`; optional — use `NoValidator` stub when not needed
- **Out of scope**: Path configuration (→ api/001), default values (→ api/002)

### Abstract

`ConfigValidator` is the optional third trait users implement to configure `ConfigManager< D, P, V >`. It provides two validation hooks: single-parameter validation and cross-parameter validation. Applications that do not need validation use `ConfigManager< D, P, NoValidator >` instead of implementing this trait.

### Interface

```rust
pub trait ConfigValidator
{
  /// Validate a single resolved parameter value.
  ///
  /// Called individually per parameter after resolution.
  /// Return `Err(ValidationError)` to report an invalid value.
  fn validate_parameter
  (
    param_name : &str,
    value : &JsonValue,
  ) -> Result< (), ValidationError >;

  /// Validate the entire resolved configuration for cross-parameter constraints.
  ///
  /// Called once with the complete resolved map.
  /// Return all errors found — do not short-circuit.
  fn validate_all
  (
    config : &HashMap< String, ( JsonValue, ConfigSource ) >,
  ) -> Vec< ValidationError >;
}
```

### Operations

#### `validate_parameter()`

Called per parameter name and its resolved `JsonValue`. Use for type/range/format checks on individual values:

```rust
fn validate_parameter( param_name : &str, value : &JsonValue )
  -> Result< (), ValidationError >
{
  if param_name == "timeout"
  {
    if let Some( t ) = value.as_i64()
    {
      if !( 1..=300 ).contains( &t )
      {
        return Err( ValidationError::new( param_name, "must be between 1 and 300" ) );
      }
    }
  }
  Ok( () )
}
```

#### `validate_all()`

Called once with the complete resolved config map. Use for cross-parameter dependency checks. Must return all errors found — do not return early on first error:

```rust
fn validate_all( config : &HashMap< String, ( JsonValue, ConfigSource ) > )
  -> Vec< ValidationError >
{
  let mut errors = Vec::new();
  if let Some( ( retries, _ ) ) = config.get( "retries" )
  {
    if retries.as_u64().unwrap_or( 0 ) > 10
    {
      errors.push( ValidationError::new( "retries", "must not exceed 10" ) );
    }
  }
  errors
}
```

#### `NoValidator` stub

For applications that do not need validation:

```rust
type AppConfig = ConfigManager< AppDefaults, AppPaths, NoValidator >;
```

`NoValidator` implements `ConfigValidator` with both methods returning `Ok(())` / empty `Vec`.

### Error Handling

`ValidationError` carries the parameter name and an error message string. It implements `Display` and `Debug`. Errors from both validation methods are collected by the caller — neither method should panic.

### Compatibility Guarantees

- Adding new validation rules in `validate_parameter` is non-breaking for callers; existing configs that were valid remain valid unless the new rule rejects them
- Switching from `NoValidator` to a real validator is a non-breaking crate API change but may reject configs that were previously accepted

### Cross-References

| Type | Target | Relationship |
|------|--------|-------------|
| api | api/001_config_paths_trait.md | companion required trait |
| api | api/002_config_defaults_trait.md | companion required trait |
| feature | feature/001_config_hierarchy.md | validation is part of this feature |
