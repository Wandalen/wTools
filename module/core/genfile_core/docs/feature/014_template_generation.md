# Feature: Template Generation

### Scope

- **Purpose**: Executes end-to-end file generation: render templates and write output files to disk.
- **Responsibility**: Documents the materialize and generate operations on the template holder and template archive.
- **In Scope**: Iterating file descriptors, rendering templates, writing output via the filesystem abstraction.
- **Out of Scope**: Missing mandatory detection (→ 015), individual component setup (→ 003-012).

### Design

Generation begins with the missing-mandatory check: the parameter collection's mandatory list is compared against the value storage map; any missing names surface as a typed error before any file is written. The operation then iterates all file descriptors. For each descriptor flagged as a template, the renderer substitutes values from the value storage; for static descriptors, content is copied verbatim. The computed output path joins the base directory with the descriptor's relative path. Each file is written via the file system abstraction using the descriptor's write mode. The operation returns a list of written paths on success or the first typed error encountered.

### APIs

| File | Relationship |
|------|--------------|
| [api/003_generation_api.md](../api/003_generation_api.md) | API contract for the generation operation |

### Features

| File | Relationship |
|------|--------------|
| [feature/005_value_storage.md](005_value_storage.md) | Value storage consumed during generation |
| [feature/006_template_renderer_trait.md](006_template_renderer_trait.md) | Renderer consumed during generation |
| [feature/008_file_descriptor.md](008_file_descriptor.md) | File descriptors iterated during generation |
| [feature/013_template_holder.md](013_template_holder.md) | Struct that owns this operation |
| [feature/015_missing_mandatory_detection.md](015_missing_mandatory_detection.md) | Pre-generation validation step |
| [feature/016_typed_errors.md](016_typed_errors.md) | Error types returned by this operation |
| [feature/017_archive_self_containment.md](017_archive_self_containment.md) | Archive that also exposes this operation |

### Sources

| File | Relationship |
|------|--------------|
| `src/template.rs` | Template holder materialize implementation |
| `src/archive.rs` | Template archive materialize implementation |

### Tests

| File | Relationship |
|------|--------------|
| `tests/inc/template_test.rs` | Template generation end-to-end tests |
| `tests/inc/integration_test.rs` | Full generation pipeline integration tests |
