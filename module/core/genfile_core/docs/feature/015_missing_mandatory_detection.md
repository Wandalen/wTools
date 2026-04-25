# Feature: Missing Mandatory Detection

### Scope

- **Purpose**: Detects mandatory parameters that have no value before generation begins.
- **Responsibility**: Documents the missing-mandatory check and its integration with generation.
- **In Scope**: Comparing mandatory parameter list against stored values; returning missing names.
- **Out of Scope**: Parameter definitions (→ 003, 004), value storage (→ 005).

### Design

Before generation, the system compares the mandatory parameter list from the parameter collection against keys present in the value map. Any mandatory parameter with no assigned value is collected and returned as a list of missing names. An empty list means all mandatory parameters are satisfied and generation can proceed. This check prevents partial or corrupted output from missing substitutions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/template.rs` | Missing-mandatory detection implementation |
| test | `tests/` | Missing mandatory parameter detection tests |
| doc | `docs/feature/004_parameter_collection.md` | List-mandatory method used here |
| doc | `docs/feature/014_template_generation.md` | Triggered before generation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR15 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
