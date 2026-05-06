# Data Structure Doc Entity

### Scope

- **Purpose**: Document data schemas and structural type definitions used across the library's variant system.
- **Responsibility**: Registry and overview of all data structure doc instances.
- **In Scope**: Attribute schemas, field inventories, canonical structural type descriptions used by multiple components.
- **Out of Scope**: Rust type signatures (see `api/`), behavioral contracts (see `invariant/`), individual variant specifications (see `variant/`).

### Infrastructure

| File | Responsibility |
|------|----------------|
| `procedure.md` | Operational procedure for creating and updating data structure doc instances |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Variant Attributes](001_variant_attributes.md) | 46-attribute schema describing all output variant properties | ✅ |
