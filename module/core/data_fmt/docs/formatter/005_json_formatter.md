# Formatter: JsonFormatter

### Scope

- **Purpose**: Render tabular data as a JSON array of row objects with string field values, using serde_json for serialization.
- **Responsibility**: Document the `JsonFormatter` struct — its `Format` trait implementation, serde dependency, and two variants.
- **In Scope**: Trait implementation, serde dependency, pretty/compact variant selection via constructor, feature flag.
- **Out of Scope**: Variant output details (see `../variant/016_json_pretty.md`, `017_json_compact.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/json.rs` | `JsonFormatter` implementation |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | `Format` trait contract |
| doc | `../variant/016_json_pretty.md` | Variant: pretty |
| doc | `../variant/017_json_compact.md` | Variant: compact |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

`FormatError::Serialization` is only present when `serde_support` feature is enabled. Without it, only `InvalidData` and `UnsupportedOperation` variants exist.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

Requires the `serde_support` feature (pulled in automatically by `format_json`).

### Variants

Selection mechanism: constructor function. Both variants compile under the same `format_json` feature flag — they are **runtime-only** (not individually feature-gated).

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| pretty | `JsonFormatter::new()` or `JsonFormatter::pretty()` | `format_json` |
| compact | `JsonFormatter::compact()` | `format_json` |

Note: enabling `format_json` compiles both variants. There is no way to exclude one variant at compile time.
