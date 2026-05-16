# Formatter: TomlFormatter

### Scope

- **Purpose**: Render tabular data as a TOML array of inline tables, using the `toml` crate for serialization.
- **Responsibility**: Document the `TomlFormatter` struct — its `Format` trait implementation, serde dependency, and single variant.
- **In Scope**: Trait implementation, serde dependency, feature flag.
- **Out of Scope**: Variant output details (see `../variant/019_toml_standard.md`), operation signatures (see `../api/004_formatters.md`).

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | `Format` trait contract |

### APIs

| File | Relationship |
|------|-------------|
| [004_formatters.md](../api/004_formatters.md) | Operation signatures |

### Variants

| File | Relationship |
|------|-------------|
| [019_toml_standard.md](../variant/019_toml_standard.md) | Variant: standard |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/toml_fmt.rs`](../../src/formatters/toml_fmt.rs) | `TomlFormatter` implementation |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

`FormatError::Serialization` is only present when `serde_support` feature is enabled.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

Requires the `serde_support` feature (pulled in automatically by `format_toml`).

### Variants

No selection mechanism — `TomlFormatter` has a single output style. Construct with `TomlFormatter::new()`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| standard | `TomlFormatter::new()` | `format_toml` |
