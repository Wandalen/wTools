# Feature: Template Generation

### Scope

- **Purpose**: Executes end-to-end file generation: render templates and write output files to disk.
- **Responsibility**: Documents the materialize and generate operations on the template holder and template archive.
- **In Scope**: Iterating file descriptors, rendering templates, writing output via the filesystem abstraction.
- **Out of Scope**: Missing mandatory detection (→ 015), individual component setup (→ 003-012).

### Design

Generation iterates all file descriptors. For each descriptor flagged as a template, the renderer substitutes parameter values; for static descriptors, content is copied verbatim. The computed output path joins the base directory with the descriptor's relative path. Each file is written via the file system abstraction using the descriptor's write mode. The operation returns a generation report listing all written files or the first error encountered.

### Features

| File | Relationship |
|------|--------------|
| [`feature/013_template_holder.md`](013_template_holder.md) | Struct that owns this operation |
| [`feature/015_missing_mandatory_detection.md`](015_missing_mandatory_detection.md) | Pre-generation validation step |

### Sources

| File | Relationship |
|------|--------------|
| [`src/template.rs`](../../src/template.rs) | Template holder materialize implementation |
| [`src/archive.rs`](../../src/archive.rs) | Template archive materialize implementation |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/template_test.rs`](../../tests/inc/template_test.rs) | Template generation end-to-end tests |
| [`tests/inc/integration_test.rs`](../../tests/inc/integration_test.rs) | Integration tests for template generation workflow |
