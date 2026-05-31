# API: Generation

### Scope

- **Purpose**: Defines the contract for executing template generation via the template holder and archive interfaces.
- **Responsibility**: Documents the generation operation surface available to callers — both low-level holder and high-level archive.
- **In Scope**: Template holder construction and generation, archive loading and generation, output path semantics.
- **Out of Scope**: Renderer internals (→ `feature/007`), file system internals (→ `feature/011`, `feature/012`).

### Design

The library exposes two generation interfaces. The template holder is the low-level form: callers supply file descriptors, a parameter collection, a value map, a renderer, and a file system implementation. The template archive is the high-level form: callers load a self-contained archive (JSON or YAML) that carries all parameter values, descriptors, and content inline. Both interfaces expose a generate operation that runs the missing-mandatory check, renders templates, and writes output via the file system abstraction. The return value is either a list of written file paths or the first typed error encountered.

### Features

| File | Relationship |
|------|--------------|
| [feature/013_template_holder.md](../feature/013_template_holder.md) | Low-level holder interface being contracted here |
| [feature/014_template_generation.md](../feature/014_template_generation.md) | Generation operation contracted here |

### Sources

| File | Relationship |
|------|--------------|
| `src/template.rs` | Template holder and generation implementation |
| `src/archive.rs` | Archive generation implementation |
