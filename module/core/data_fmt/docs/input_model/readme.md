# Input Model Doc Entity

### Scope

- **Purpose**: Define the conceptual data shapes the library accepts as input.
- **Responsibility**: Registry and overview of all input model doc instances.
- **In Scope**: Tabular and hierarchical data shapes, their invariants, and downstream connections.
- **Out of Scope**: Rust type details (see `input_type/`), construction APIs (see `builder/`).

### Type-Specific Requirements

Every input_model doc instance must contain these sections in order:

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Cross-References | `### Cross-References` | Table: Type / File / Responsibility |
| Data Shape | `### Data Shape` | Structural description of the model: what it carries, how it is organized, any structural rules |
| Downstream Connections | `### Downstream Connections` | Which input types represent this model; which formatters consume it |

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Tabular](001_tabular.md) | Headers + rows of cells | ✅ |
| 002 | [Hierarchical](002_hierarchical.md) | Tree of named nodes with optional leaf data | ✅ |
