# Input Model Doc Entity

### Scope

- **Purpose:** Define the conceptual data shapes the library accepts as input.
- **Responsibility:** Registry and overview of all input model doc instances.
- **In Scope:** Tabular and hierarchical data shapes, their invariants, and downstream connections.
- **Out of Scope:** Rust type details (see `input_type/`), construction APIs (see `builder/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Tabular](001_tabular.md) | Headers + rows of cells | ✅ |
| 002 | [Hierarchical](002_hierarchical.md) | Tree of named nodes with optional leaf data | ✅ |
