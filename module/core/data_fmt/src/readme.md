# src

## Purpose
Library source files. Each file owns one cohesive responsibility; `lib.rs` re-exports the public API.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Crate root: module declarations, feature gates, public re-exports |
| `data.rs` | Core data types: `TreeNode`, `TableView` trait, `ColumnData` |
| `config.rs` | Configuration types: `TreeConfig`, `TableConfig`, `ExpandedConfig`, style enums |
| `builder.rs` | `TreeBuilder`: path-based tree construction from string slices |
| `table_tree.rs` | `RowBuilder`: fluent builder for table-shaped tree data |
| `conversions.rs` | Tree↔table conversions and `FlattenConfig` for selective column flattening |
| `ansi_str.rs` | ANSI-aware string utilities: `visual_len`, `pad_to_width`, `truncate_cell` |
| `wrap.rs` | Word-wrap utility: `WrapFormatter`, `WrapConfig`, `BreakStrategy`, `Overflow` |
| `themes.rs` | Visual theme definitions for HTML and table formatters |
| `formatters/` | One formatter module per output format family |
