# Feature Doc Entity

### Scope

- **Purpose**: Collects all user-facing capabilities of the former macro as individually navigable hubs pointing to their source, test, and design artifacts.
- **Responsibility**: Master index for feature doc instances — their identifiers, names, and status.
- **In Scope**: All behaviorally distinct, user-facing capabilities of the macro.
- **Out of Scope**: Algorithmic procedures (→ algorithm/), design patterns (→ pattern/), attribute contracts (→ api/), correctness constraints (→ invariant/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Struct Former](001_struct_former.md) | Builder derivation for struct types | ✅ |
| 002 | [Enum Former](002_enum_former.md) | Builder derivation for enum types | ✅ |
| 003 | [Scalar Subformer](003_subform_scalar.md) | Nested builder for scalar field types | ✅ |
| 004 | [Collection Subformer](004_subform_collection.md) | Aggregate builder for collection fields | ✅ |
| 005 | [Entry Subformer](005_subform_entry.md) | Per-entry builder for collection fields | ✅ |
| 006 | [Standalone Constructors](006_standalone_constructors.md) | Top-level constructor function generation | ✅ |
| 007 | [Debug Attribute](007_debug_attribute.md) | Compile-time diagnostic output for generated code | ✅ |
