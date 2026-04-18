# Documentation

## Purpose
Contains technical documentation for the data_fmt library implementation.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `input_model/` | Conceptual data shapes: tabular, hierarchical |
| `input_type/` | Rust input types: TableView, TreeNode specializations |
| `builder/` | Construction helpers: RowBuilder, TreeBuilder |
| `trait/` | Interface contracts: Format, TableShapedFormatter, TableShapedView |
| `architecture.md` | Three-layer architecture, module structure, formatter design (source document) |
| `api/` | Public API surface: data types, builders, configs, formatters |
| `feature/` | Feature docs: table formatting, word wrap, format interface, themes |
| `invariant/` | Behavioral contracts: data model, ANSI/Unicode handling |
| `algorithm/` | Non-trivial algorithms: multiline cells, word wrapping, tree alignment |
| `data_structure/` | Data schemas: attribute schemas, structural type definitions |
| `pattern/` | Architectural patterns: three-layer architecture, design principles, formatter design |
| `variant/` | Per-variant output examples and format documentation |
| `variant_attributes.md` | Attribute reference for all format variant configurations (source document) |
| `development_notes.md` | Implementation decisions, trade-offs, and technical history |
| `feature_selection_guide.md` | Guide to choosing the right feature flags for use cases |
| `entities.md` | Master doc entity index: all entity types and doc instances |
| `doc_graph.yml` | Cross-entity dependency graph: nodes, edges, and components |
