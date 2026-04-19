# Formatter: LogfmtFormatter

### Scope

- **Purpose**: Render tabular data as structured log lines in `key=value` format compatible with Logfmt parsers.
- **Responsibility**: Document the `LogfmtFormatter` struct — its `Format` trait implementation, input type, and single variant.
- **In Scope**: Trait implementation, input type, feature flag.
- **Out of Scope**: Variant output details (see `../variant/015_logfmt_standard.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/logfmt.rs` | `LogfmtFormatter` implementation |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | `Format` trait contract |
| doc | `../variant/015_logfmt_standard.md` | Variant: standard |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes `&TableView`, returns `Result<String, FormatError>` |

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

### Variants

No selection mechanism — `LogfmtFormatter` has a single output style. Construct with `LogfmtFormatter::new()`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| standard | `LogfmtFormatter::new()` | `format_logfmt` |
