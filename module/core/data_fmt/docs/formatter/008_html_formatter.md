# Formatter: HtmlFormatter

### Scope

- **Purpose**: Render tabular data as an HTML `<table>` element with optional CSS framework class attributes.
- **Responsibility**: Document the `HtmlFormatter` struct — its `Format` trait implementation, `HtmlVariant` enum selection, and 4 CSS theme variants.
- **In Scope**: Trait implementation, `HtmlVariant` enum values, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/020_html_minimal.md` through `023_html_custom.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/html.rs` | `HtmlFormatter` and `HtmlVariant` implementation |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | `Format` trait contract |
| doc | `../variant/020_html_minimal.md` | Variant: minimal |
| doc | `../variant/021_html_bootstrap.md` | Variant: bootstrap |
| doc | `../variant/022_html_tailwind.md` | Variant: tailwind |
| doc | `../variant/023_html_custom.md` | Variant: custom |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes `&TableView`, returns `Result<String, FormatError>` |

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

### Variants

Selection mechanism: pass an `HtmlVariant` enum value to `HtmlFormatter::with_variant(variant)`. Each variant has its own compile-time feature flag.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| minimal | `HtmlVariant::Minimal` | `html_minimal` |
| bootstrap | `HtmlVariant::Bootstrap` | `html_bootstrap` |
| tailwind | `HtmlVariant::Tailwind` | `html_tailwind` |
| custom | `HtmlVariant::Custom(class_string)` | `html_custom` |

`HtmlVariant::Custom` takes a `String` of CSS class names applied to the `<table>` element. Each variant is independently feature-gated; unused variants add no binary overhead.
