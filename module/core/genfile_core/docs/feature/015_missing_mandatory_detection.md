# Feature: Missing Mandatory Detection

### Scope

- **Purpose**: Detects mandatory parameters that have no value before generation begins.
- **Responsibility**: Documents the missing-mandatory check and its integration with generation.
- **In Scope**: Comparing mandatory parameter list against stored values; returning missing names.
- **Out of Scope**: Parameter definitions (→ 003, 004), value storage (→ 005).

### Design

Before generation, the system compares the mandatory parameter list from the parameter collection against keys present in the value map. Any mandatory parameter with no assigned value is collected and returned as a list of missing names. An empty list means all mandatory parameters are satisfied and generation can proceed. This check prevents partial or corrupted output from missing substitutions.

### Features

| File | Relationship |
|------|--------------|
| [`feature/004_parameter_collection.md`](004_parameter_collection.md) | List-mandatory method used for detection |
| [`feature/014_template_generation.md`](014_template_generation.md) | Generation step that this check precedes |

### Sources

| File | Relationship |
|------|--------------|
| [`src/template.rs`](../../src/template.rs) | Missing-mandatory detection implementation |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/template_error_test.rs`](../../tests/inc/template_error_test.rs) | Missing mandatory parameter detection tests |
