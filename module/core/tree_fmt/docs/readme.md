# Documentation

> **Note:** `tree_fmt` has been renamed to [`data_fmt`](https://crates.io/crates/data_fmt).

## Purpose
Contains technical documentation for the data_fmt (formerly tree_fmt) library implementation.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `input_model/` | Conceptual data shapes: tabular, hierarchical |
| `input_type/` | Rust input types: TableView, TreeNode specializations |
| `builder/` | Construction helpers: RowBuilder, TreeBuilder |
| `trait/` | Interface contracts: Format, TableShapedFormatter, TableShapedView |
| `architecture.md` | Three-layer architecture, module structure, formatter design |
| `api/` | Public API surface: data types, builders, configs, formatters |
| `feature/` | Feature docs: table formatting, word wrap, format interface, themes |
| `invariant/` | Behavioral contracts: data model, ANSI/Unicode handling |
| `algorithm/` | Non-trivial algorithms: multiline cells, word wrapping, tree alignment |
| `variant/` | Per-variant output examples and format documentation |
| `variant_attributes.md` | Attribute reference for all format variant configurations |
| `development_notes.md` | Implementation decisions, trade-offs, and technical history |
| `feature_selection_guide.md` | Guide to choosing the right feature flags for use cases |
| `entities.md` | Master doc entity index: all 9 entity types, all 60 doc instances |
| `doc_graph.yml` | Cross-entity dependency graph: 60 nodes, 46 edges, 8 components |
