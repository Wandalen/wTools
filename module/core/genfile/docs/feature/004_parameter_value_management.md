# Feature: Parameter Value Management

### Scope

- **Purpose**: Provides commands to set, list, and clear runtime values for template parameters.
- **Responsibility**: Documents the three value management commands.
- **In Scope**: `.value.set`, `.value.list`, `.value.clear`.
- **Out of Scope**: Parameter definitions (→ 003), materialization (→ 006).

### Design

Values can be set for any defined parameter; setting an undefined parameter produces a validation error. Values persist in the archive's stored values map and are serialized with the archive. Clearing reverts all values to defaults (or unset for mandatory parameters). Values are validated against parameter definitions at set-time.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/value.rs` | Handler implementations for value commands |
| config | `commands/value.yaml` | Authoritative command specs for value group |
