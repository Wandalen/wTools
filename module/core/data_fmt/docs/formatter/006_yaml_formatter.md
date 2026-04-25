# Formatter: YamlFormatter

### Scope

- **Purpose**: Render tabular data as a YAML sequence of mappings, using serde_yaml for serialization.
- **Responsibility**: Document the `YamlFormatter` struct — its `Format` trait implementation, serde dependency, and single variant.
- **In Scope**: Trait implementation, serde dependency, feature flag.
- **Out of Scope**: Variant output details (see `../variant/018_yaml_standard.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/yaml.rs` | `YamlFormatter` implementation |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | `Format` trait contract |
| doc | `../variant/018_yaml_standard.md` | Variant: standard |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

`FormatError::Serialization` is only present when `serde_support` feature is enabled.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

Requires the `serde_support` feature (pulled in automatically by `format_yaml`).

### Variants

No selection mechanism — `YamlFormatter` has a single output style. Construct with `YamlFormatter::new()`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| standard | `YamlFormatter::new()` | `format_yaml` |
