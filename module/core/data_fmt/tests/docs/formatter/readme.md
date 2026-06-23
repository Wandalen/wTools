# Formatter Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all formatter doc instances.
- **Responsibility**: Registry and overview of all formatter test spec instances.
- **In Scope**: FM-N formatter output contract cases in Given/When/Then format for all 10 formatter elements (`table`, `expanded`, `tree`, `logfmt`, `json`, `yaml`, `toml`, `html`, `sql`, `text`); minimum 4 cases per spec.
- **Out of Scope**: Feature-level behavior (see `../feature/`), variant rendering details (see `../variant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TableFormatter](001_table_formatter.md) | Output contract for table-style variants (plain, bordered, markdown, etc.) | ⏳ |
| 002 | [ExpandedFormatter](002_expanded_formatter.md) | Output contract for expanded record-per-row display | ⏳ |
| 003 | [TreeFormatter](003_tree_formatter.md) | Output contract for tree-style indented rendering | ⏳ |
| 004 | [LogfmtFormatter](004_logfmt_formatter.md) | Output contract for logfmt key=value line output | ⏳ |
| 005 | [JsonFormatter](005_json_formatter.md) | Output contract for JSON pretty/compact output | ⏳ |
| 006 | [YamlFormatter](006_yaml_formatter.md) | Output contract for YAML sequence-of-mappings output | ⏳ |
| 007 | [TomlFormatter](007_toml_formatter.md) | Output contract for TOML array-of-inline-tables output | ⏳ |
| 008 | [HtmlFormatter](008_html_formatter.md) | Output contract for HTML table with CSS variant classes | ⏳ |
| 009 | [SqlFormatter](009_sql_formatter.md) | Output contract for SQL INSERT statements with dialect quoting | ⏳ |
| 010 | [TextFormatter](010_text_formatter.md) | Output contract for 6 plain-text list/structured styles | ⏳ |
