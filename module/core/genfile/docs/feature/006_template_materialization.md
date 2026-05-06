# Feature: Template Materialization

### Scope

- **Purpose**: Renders template archives into generated files at a target destination.
- **Responsibility**: Documents the `.materialize` and `.unpack` commands.
- **In Scope**: Parameter substitution via template engine, output file writing, mandatory parameter validation, dry-run mode.
- **Out of Scope**: Parameter value assignment (→ 004), archive serialization (→ 007).

### Design

Materialization validates that all mandatory parameters have values, then renders each template file by substituting parameter values via the template engine, writing outputs to the destination directory. Static files are copied verbatim. Unpack copies raw archive content without rendering, useful for inspecting or extracting archives without substitution. Both operations support dry-run mode for preview without writing.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/materialize.rs` | Handler implementations for materialize/unpack commands |
| config | `commands/materialize.yaml` | Authoritative command specs |
| test | `tests/materialization_test.rs` | Integration tests for materialize and unpack commands |
