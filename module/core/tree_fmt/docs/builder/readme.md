# Builder Doc Entity

### Scope

- **Purpose:** Document construction helpers that produce input types from user data.
- **Responsibility:** Registry and overview of all builder doc instances.
- **In Scope:** `RowBuilder`, `TreeBuilder<T>`, their APIs and output types.
- **Out of Scope:** Input type internals (see `input_type/`), formatter behavior (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [RowBuilder](001_row_builder.md) | Construct tabular data (headers + rows) | ✅ |
| 002 | [TreeBuilder](002_tree_builder.md) | Construct hierarchical trees from flat path insertions | ✅ |
