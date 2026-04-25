# API: ConfigValidator Trait

### Scope

- **Purpose**: Define the optional validation hooks contract for the configuration manager.
- **Responsibility**: Documents operations, error handling, and compatibility guarantees of ConfigValidator.
- **In Scope**: Per-parameter validation, cross-parameter validation, and the no-op implementation for applications without validation needs.
- **Out of Scope**: Path configuration (→ api/001), default value supply (→ api/002).

### Abstract

ConfigValidator is the optional third trait applications implement to configure the manager. It provides two validation hooks: per-parameter validation and cross-parameter validation. Applications that do not need validation use the provided no-op implementation instead of implementing this trait.

### Operations

#### `validate_parameter()`

Accepts a parameter name and its resolved value. Returns success when the value is valid, or a validation error identifying the parameter and describing the rejection reason. Use for type, range, and format checks on individual values. Called independently per parameter after resolution.

#### `validate_all()`

Accepts the complete resolved configuration map. Returns all validation errors found — must not return early on the first error. Use for cross-parameter dependency and relationship checks. Called once with the complete map.

#### No-Op Implementation

A built-in no-op implementation satisfies the contract with a per-parameter hook that always succeeds and a cross-parameter hook that always returns an empty error list. Use it as the third type parameter when validation is not needed.

### Error Handling

The validation error type carries the parameter name and a human-readable error message. Both hooks must collect all errors rather than returning on the first failure. Neither hook should throw an exception.

### Compatibility Guarantees

- Adding new validation rules in the per-parameter hook is non-breaking for callers; existing configs that were valid remain valid unless the new rule rejects them
- Switching from the no-op implementation to a real validator is a non-breaking API change but may reject configs that were previously accepted

### Cross-References

| Type | File                                  | Responsibility                                       |
|------|---------------------------------------|------------------------------------------------------|
| doc  | api/001_config_paths_trait.md         | Companion required trait for path configuration      |
| doc  | api/002_config_defaults_trait.md      | Companion required trait for default values          |
| doc  | feature/001_config_hierarchy.md       | Feature this validation contract is part of          |
