# Formatter: HtmlFormatter

### Scope

- **Purpose**: Render tabular data as an HTML `<table>` element with optional CSS framework class attributes.
- **Responsibility**: Document the `HtmlFormatter` struct — its `Format` trait implementation, `HtmlVariant` enum selection, and 4 CSS theme variants.
- **In Scope**: Trait implementation, `HtmlVariant` enum values, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/020_html_minimal.md` through `023_html_custom.md`), operation signatures (see `../api/004_formatters.md`).

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
| [020_html_minimal.md](../variant/020_html_minimal.md) | Variant: minimal |
| [021_html_bootstrap.md](../variant/021_html_bootstrap.md) | Variant: bootstrap |
| [022_html_tailwind.md](../variant/022_html_tailwind.md) | Variant: tailwind |
| [023_html_custom.md](../variant/023_html_custom.md) | Variant: custom |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/html.rs`](../../src/formatters/html.rs) | `HtmlFormatter` and `HtmlVariant` implementation |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

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
| custom | `HtmlVariant::Custom` (with a CSS class string) | `html_custom` |

`HtmlVariant::Custom` takes a string of CSS class names applied to the `<table>` element. Each variant is independently feature-gated; unused variants add no binary overhead.
