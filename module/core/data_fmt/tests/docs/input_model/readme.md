# Input Model Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all input model doc instances.
- **Responsibility**: Registry and overview of all input model test spec instances.
- **In Scope**: IM-N input model contract cases in Given/When/Then format for all 2 input model elements (`tabular`, `hierarchical`); minimum 4 cases per spec.
- **Out of Scope**: Data type invariants (see `../invariant/`), builder API contracts (see `../builder/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Tabular](001_tabular.md) | Header schema, row-length invariant, row details, column order | ⏳ |
| 002 | [Hierarchical](002_hierarchical.md) | Single root, leaf vs directory, node names, specializations | ⏳ |
