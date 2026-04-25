# Feature: Configuration Validation

### Scope

**Purpose**: Validate configuration files against a JSON Schema derived from the target type before returning the deserialized value, catching structural errors before they cause runtime failures.
**Responsibility**: Load a config file, derive a JSON Schema from the target type, validate the raw content against that schema, and return detailed per-field errors on failure.
**In Scope**: Schema-validated configuration loading (requires `validation` feature, which also activates `serde`).
**Out of Scope**: Runtime value validation (business logic), schema authoring tools, non-JSON-Schema validation formats, validation of non-config data.

### Design

Validation is integrated into the load step, not a separate pass. The operation loads the file, deserializes to an intermediate representation, then validates against the generated schema before converting into the target structure. All schema violations are collected and returned as a single error with per-field detail, not just the first error.

Schema generation derives from the target type definition at runtime via an annotation, requiring no separate schema file on disk.

The `validation` feature depends on `serde` — enabling validation automatically enables configuration loading, ensuring callers do not need to manage feature dependency manually.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Validation impl block, `load_config_with_validation()` |
| Test | `tests/config_validation_tests.rs` | Schema-based configuration validation |
| Test | `tests/validation_boundary_tests.rs` | Input validation and boundary condition handling |
| Task | `task/completed/003_config_validation.md` | Initial schema validation implementation |
| Doc | `docs/api/001_workspace.md` | `load_config_with_validation()` method signature |
| Doc | `docs/feature/002_configuration_loading.md` | Underlying config loading (serde feature) |
