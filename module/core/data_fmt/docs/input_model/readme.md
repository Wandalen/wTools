# Input Model Doc Entity

### Scope

- **Purpose**: Define the conceptual data shapes the library accepts as input.
- **Responsibility**: Registry and overview of all input model doc instances.
- **In Scope**: Abstract, pre-type-system conceptual data shapes — tabular and hierarchical — their structural rules and downstream connections.
- **Out of Scope**: Concrete Rust struct/enum type definitions (see `input_type/`), construction APIs (see `builder/`).

Required instance sections (in order):

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Typed References | `### InputTypes`, `### Sources`, `### Tests` | Per-type `| File | Relationship |` table; `### Sources` and `### Tests` always last |
| Data Shape | `### Data Shape` | Structural description of the model: what it carries, how it is organized, any structural rules |
| Downstream Connections | `### Downstream Connections` | Which input types represent this model; which formatters consume it |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Tabular](001_tabular.md) | Headers + rows of cells | ✅ |
| 002 | [Hierarchical](002_hierarchical.md) | Tree of named nodes with optional leaf data | ✅ |
