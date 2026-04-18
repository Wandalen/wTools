# Data Structure Doc Entity

### Scope

- **Purpose**: Document data schemas and structural type definitions used across the library's variant system.
- **Responsibility**: Registry and overview of all data structure doc instances.
- **In Scope**: Attribute schemas, field inventories, canonical structural type descriptions used by multiple components.
- **Out of Scope**: Rust type signatures (see `api/`), behavioral contracts (see `invariant/`), individual variant specifications (see `variant/`).

### Type-Specific Requirements

Every data_structure doc instance must contain these sections in order:

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Cross-References | `### Cross-References` | Table: Type / File / Responsibility |
| Schema | `### Schema` | Enumerated list of all fields/attributes with name, type, and example values |
| Sources | `### Sources` | Table: File / Notes — listing all contributing source documents |

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Variant Attributes](001_variant_attributes.md) | 46-attribute schema describing all output variant properties | ✅ |
