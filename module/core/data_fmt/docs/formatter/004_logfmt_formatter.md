# Formatter: LogfmtFormatter

### Scope

- **Purpose**: Render tabular data as structured log lines in `key=value` format compatible with Logfmt parsers.
- **Responsibility**: Document the `LogfmtFormatter` struct — its `Format` trait implementation, input type, and single variant.
- **In Scope**: Trait implementation, input type, feature flag.
- **Out of Scope**: Variant output details (see `../variant/015_logfmt_standard.md`), operation signatures (see `../api/004_formatters.md`).

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
| [015_logfmt_standard.md](../variant/015_logfmt_standard.md) | Variant: standard |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/logfmt.rs`](../../src/formatters/logfmt.rs) | `LogfmtFormatter` implementation |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

### Variants

No selection mechanism — `LogfmtFormatter` has a single output style. Construct with `LogfmtFormatter::new()`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| standard | `LogfmtFormatter::new()` | `format_logfmt` |
