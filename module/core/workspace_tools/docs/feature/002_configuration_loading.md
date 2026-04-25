# Feature: Configuration Loading

### Scope

**Purpose**: Load typed configuration from TOML, JSON, and YAML files located relative to the workspace root using serde deserialization.
**Responsibility**: Detect file format by extension, deserialize file content into caller-defined structs, and support layered merging where later files override earlier ones.
**In Scope**: `load_config()`, `load_config_from()`, `load_config_layered()`, `load_config_with_merge()`, `find_config()`, `merge_config()`, `save_config()` (all gated on the `serde` feature).
**Out of Scope**: Schema validation (see `feature/005_configuration_validation.md`), config file watching or hot-reload, remote configuration sources, encryption of config values.

### Design

Format detection is entirely extension-driven: `.toml` → TOML, `.json` → JSON, `.yaml`/`.yml` → YAML. No explicit format argument is required; `load_config("app")` tries each extension in order until a file is found.

Layered merging follows a last-wins rule: configs are deserialized into the same target type and merged in declaration order, with each subsequent file overriding keys from earlier files. This enables a `base.toml` + `dev.toml` pattern where environment-specific files override shared defaults.

`find_config()` implements a priority search order — workspace-local `config/` directory first, then the workspace root, allowing per-project overrides of shared defaults without moving files.

`save_config()` serializes a typed struct back to TOML at a workspace-relative path, enabling tools that mutate and persist configuration programmatically.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Serde impl block, `ConfigMerge`, `WorkspaceDeserializer` |
| Test | `tests/serde_integration_tests.rs` | Integration with serde for configuration deserialization |
| Test | `tests/comprehensive_test_suite.rs` | Full coverage matrix including config loading |
| Test | `tests/feature_combination_tests.rs` | Feature flag combination correctness |
| Task | `task/completed/005_serde_integration.md` | Initial serde configuration loading implementation |
| Doc | `docs/api/001_workspace.md` | Configuration loading method signatures |
| Doc | `docs/feature/005_configuration_validation.md` | Schema validation on top of config loading |
