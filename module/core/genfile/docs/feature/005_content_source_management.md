# Feature: Content Source Management

### Scope

- **Purpose**: Provides commands to convert between portable (inline) and lightweight (reference) archive content.
- **Responsibility**: Documents the three content source commands.
- **In Scope**: `.content.internalize`, `.content.externalize`, `.content.list`.
- **Out of Scope**: Archive serialization to disk (→ 007), file addition (→ 002).

### Design

Internalization fetches all external content references (file paths, URLs) and embeds them inline in the archive, producing a fully self-contained portable archive. Externalization extracts inline content to external files and replaces it with references, producing a lightweight archive. Listing shows all content sources by type (inline, file reference, URL reference) with optional filtering.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/content.rs` | Handler implementations for content commands |
| config | `commands/content.yaml` | Authoritative command specs for content group |
