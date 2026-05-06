# Feature: Archive Self-Containment

### Scope

- **Purpose**: Ensures a template archive carries all data needed for generation without external dependencies.
- **Responsibility**: Documents the self-contained archive model and its serialization guarantee.
- **In Scope**: Parameter values stored inside the archive, JSON/YAML round-trip, portability.
- **Out of Scope**: Content sources for file content (→ content_source module), archive operations API.

### Design

A template archive is self-contained: it stores parameter values alongside template files and metadata in a single JSON or YAML document. Loading an archive restores all parameter values, so no external state is needed for generation. External content references (file or URL references) are allowed only for file content, not for parameter values. The archive can be internalized — converting all external content references to inline — to produce a fully portable, single-file artifact.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/archive.rs` | Template archive struct and serialization logic |
| test | `tests/` | Archive self-containment and serialization tests |
| doc | `docs/feature/014_template_generation.md` | Generation using an archive's stored values |
