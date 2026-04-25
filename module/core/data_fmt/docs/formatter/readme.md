# Formatter Doc Entity

### Scope

- **Purpose**: Document each formatter — the rendering component that converts structured data into a specific output format family.
- **Responsibility**: Registry and overview of all formatter doc instances.
- **In Scope**: Trait implementations, accepted input types, variant selection mechanism, feature flag, deprecation status.
- **Out of Scope**: Per-variant preset details (see `../variant/`), operation signatures (see `../api/004_formatters.md`), format output semantics (see `../variant/`).

#### Type-Specific Requirements

Every formatter doc instance must contain these sections in order:

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Cross-References | `### Cross-References` | Table: Type / File / Responsibility |
| Trait | `### Trait` | Which trait(s) implemented; deprecation status of each |
| Input | `### Input` | Accepted Rust input type(s); which path each serves |
| Variants | `### Variants` | Variant list with selection mechanism per variant |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TableFormatter](001_table_formatter.md) | Horizontal tabular display; 9 style variants via `TableConfig` presets | ✅ |
| 002 | [ExpandedFormatter](002_expanded_formatter.md) | Vertical key-value records; deprecated `TableShapedFormatter` path only | ✅ |
| 003 | [TreeFormatter](003_tree_formatter.md) | Hierarchical box-drawing; no trait — direct method dispatch | ✅ |
| 004 | [LogfmtFormatter](004_logfmt_formatter.md) | Structured log `key=value` output; single variant | ✅ |
| 005 | [JsonFormatter](005_json_formatter.md) | JSON array of row objects via serde; pretty and compact variants | ✅ |
| 006 | [YamlFormatter](006_yaml_formatter.md) | YAML sequence of mappings via serde; single variant | ✅ |
| 007 | [TomlFormatter](007_toml_formatter.md) | TOML array of tables via serde; single variant | ✅ |
| 008 | [HtmlFormatter](008_html_formatter.md) | HTML table with CSS theme variants via `HtmlVariant` enum | ✅ |
| 009 | [SqlFormatter](009_sql_formatter.md) | SQL INSERT statements across 4 dialect variants via `SqlVariant` enum | ✅ |
| 010 | [TextFormatter](010_text_formatter.md) | Plain text output across 6 style variants via `TextVariant` enum | ✅ |

### Cross-Doc Entity Dependencies

- [variant/](../variant/) — per-variant preset docs (one doc instance per formatter variant)
- [api/004_formatters.md](../api/004_formatters.md) — formatter operation signatures and error handling
- [trait/001_format.md](../trait/001_format.md) — `Format` trait contract and implementor matrix
- [trait/002_table_shaped_formatter.md](../trait/002_table_shaped_formatter.md) — deprecated `TableShapedFormatter` trait
- [input_type/001_table_view.md](../input_type/001_table_view.md) — `TableView` input type
- [input_type/002_tree_node.md](../input_type/002_tree_node.md) — `TreeNode` input type
