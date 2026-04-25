# Builder Doc Entity

### Scope

- **Purpose**: Document construction helpers that produce input types from user data.
- **Responsibility**: Registry and overview of all builder doc instances.
- **In Scope**: `RowBuilder`, `TreeBuilder`, their APIs and output types.
- **Out of Scope**: Input type internals (see `input_type/`), formatter behavior (see `feature/`).

#### Type-Specific Requirements

Every builder doc instance must contain these sections in order:

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Cross-References | `### Cross-References` | Table: Type / File / Responsibility |
| Construction API | `### Construction API` | Constructor signatures, fluent methods, output type, minimal usage example |
| Invariants | `### Invariants` | Pre/post conditions enforced at construction time |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [RowBuilder](001_row_builder.md) | Construct tabular data (headers + rows) | ✅ |
| 002 | [TreeBuilder](002_tree_builder.md) | Construct hierarchical trees from flat path insertions | ✅ |
