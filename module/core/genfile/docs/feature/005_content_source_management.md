# Feature: Content Source Management

### Scope

- **Purpose**: Provides commands to convert between portable (inline) and lightweight (reference) archive content.
- **Responsibility**: Documents the three content source commands.
- **In Scope**: `.content.internalize`, `.content.externalize`, `.content.list`.
- **Out of Scope**: Archive serialization to disk (→ 007), file addition (→ 002).

### Design

Internalization fetches all external content references (file paths, URLs) and embeds them inline in the archive, producing a fully self-contained portable archive. Externalization extracts inline content to external files and replaces it with references, producing a lightweight archive. Listing shows all content sources by type (inline, file reference, URL reference) with optional filtering.

### Features

| File | Relationship |
|------|--------------|
| [`feature/007_archive_serialization.md`](007_archive_serialization.md) | Pack operation that requires all content to be internalized |

### Sources

| File | Relationship |
|------|--------------|
| [`src/handlers/content.rs`](../../src/handlers/content.rs) | Handler implementations for content commands |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/content_commands_test.rs`](../../tests/content_commands_test.rs) | Integration tests for content source commands |
