# API: ConfigValidator Trait

### Scope

- **Purpose**: Define the contract for validating resolved configuration values in ConfigManager.
- **Responsibility**: Document both validation hooks, the NoValidator stub, and compatibility notes.
- **In Scope**: Implementing ConfigValidator; optional — use NoValidator stub when not needed.
- **Out of Scope**: Path configuration (→ api/001), default values (→ api/002).

### Abstract

`ConfigValidator` is the optional third trait users implement to configure the manager type. It provides two validation hooks: single-parameter validation and cross-parameter validation. Applications that do not need validation use `NoValidator` as the third type parameter instead of implementing this trait.

### Operations

#### `validate_parameter()`

Called per parameter name and its resolved value. Return a `ValidationError` identifying the parameter and the rejection reason to reject it; return success to accept it. Use for type, range, and format checks on individual values. Called independently per parameter after resolution.

#### `validate_all()`

Called once with the complete resolved configuration map. Must return all errors found — must not return early on the first error. Use for cross-parameter dependency checks.

#### `NoValidator`

A built-in no-op type satisfying the `ConfigValidator` contract. Its per-parameter hook always succeeds; its cross-parameter hook always returns an empty error list. Use it as the third type parameter when validation is not needed.

### Error Handling

`ValidationError` carries the parameter name and an error message string. It implements `Display` and `Debug`. Errors from both validation methods are collected by the caller — neither method should panic.

### Compatibility Guarantees

- Adding new validation rules in `validate_parameter` is non-breaking for callers; existing configs that were valid remain valid unless the new rule rejects them
- Switching from `NoValidator` to a real validator is a non-breaking crate API change but may reject configs that were previously accepted

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Companion required trait |
| [api/002_config_defaults_trait.md](../api/002_config_defaults_trait.md) | Companion required trait |
| [api/004_config_manager.md](../api/004_config_manager.md) | Manager type that takes this trait as a type parameter |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that uses this validation contract |

### Sources

| File | Relationship |
|------|--------------|
| [src/traits.rs](../../src/traits.rs) | `ConfigValidator` trait and `NoValidator` implementation |
| [src/manager.rs](../../src/manager.rs) | validate_parameter() and validate_all() call sites |

### Tests

| File | Relationship |
|------|--------------|
| [tests/validator_tests.rs](../../tests/validator_tests.rs) | Full validator and NoValidator test coverage |
