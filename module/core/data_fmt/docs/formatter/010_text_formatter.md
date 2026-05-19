# Formatter: TextFormatter

### Scope

- **Purpose**: Render tabular data as human-readable plain text in one of six list or structured-text styles.
- **Responsibility**: Document the `TextFormatter` struct — its `Format` trait implementation, `TextVariant` enum selection, and 6 style variants.
- **In Scope**: Trait implementation, `TextVariant` enum values, runtime-only variant selection, shared feature flag.
- **Out of Scope**: Variant output details (see `../variant/028_text_bullets.md` through `033_text_cli_help.md`), operation signatures (see `../api/004_formatters.md`).

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
| [028_text_bullets.md](../variant/028_text_bullets.md) | Variant: bullets |
| [029_text_numbered.md](../variant/029_text_numbered.md) | Variant: numbered |
| [030_text_sections.md](../variant/030_text_sections.md) | Variant: sections |
| [031_text_keyvalue.md](../variant/031_text_keyvalue.md) | Variant: keyvalue |
| [032_text_compact.md](../variant/032_text_compact.md) | Variant: compact |
| [033_text_cli_help.md](../variant/033_text_cli_help.md) | Variant: cli_help |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | `TextFormatter` and `TextVariant` implementation |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

### Variants

Selection mechanism: pass a `TextVariant` enum value to `TextFormatter::new(variant)`. All 6 variants are **runtime-only** — they share the single `format_text` feature flag and cannot be individually excluded at compile time.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| bullets | `TextVariant::Bullets` | `format_text` |
| numbered | `TextVariant::Numbered` | `format_text` |
| sections | `TextVariant::Sections` | `format_text` |
| keyvalue | `TextVariant::KeyValue` | `format_text` |
| compact | `TextVariant::Compact` | `format_text` |
| cli_help | `TextVariant::CliHelp` | `format_text` |

Enabling `format_text` compiles all 6 variants into the binary. This differs from `TableFormatter`, `HtmlFormatter`, and `SqlFormatter` where each variant has its own feature flag.
