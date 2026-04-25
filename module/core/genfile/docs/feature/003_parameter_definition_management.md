# Feature: Parameter Definition Management

### Scope

- **Purpose**: Provides commands to define, list, and remove template parameter metadata.
- **Responsibility**: Documents the three parameter management commands.
- **In Scope**: `.parameter.add`, `.parameter.list`, `.parameter.remove`.
- **Out of Scope**: Setting runtime values (→ 004), content rendering (→ 006).

### Design

Parameters are defined with a name, optional description, a mandatory flag, and an optional default value. Parameter names are validated as alphanumeric with underscores. Listing supports filtering to mandatory-only. Removal validates the parameter exists before deletion. Parameters persist inside the archive and are serialized with it.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/parameter.rs` | Handler implementations for parameter commands |
| config | `commands/parameter.yaml` | Authoritative command specs for parameter group |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | FR3 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
