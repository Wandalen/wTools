# Feature: Content Source Model

### Scope

- **Purpose**: Allows file descriptors to reference content from external sources rather than embedding it inline.
- **Responsibility**: Documents the content source abstraction, helper types, and pluggable resolution and storage strategies.
- **In Scope**: `ContentSource` enum, `IntoContentSource` trait, `FileRef`/`UrlRef`/`InlineContent` helper types, `ContentResolver` and `ContentStorage` traits, `DefaultContentResolver`, `DefaultContentStorage`.
- **Out of Scope**: File descriptor structure (→ 008), archive self-containment invariant (→ 017), template rendering (→ 006).

### Design

A file descriptor may carry an optional external content reference via `ContentSource`. Three source variants exist: `Inline` (content embedded directly in the archive, the default), `File` (path to a local filesystem file), and `Url` (reference to a remote resource). Helper types `FileRef`, `UrlRef`, and `InlineContent` implement `IntoContentSource` for ergonomic construction.

Resolution is decoupled from the archive via the `ContentResolver` trait. Callers provide a resolver when invoking `materialize_with_resolver`; the archive fetches content for each file descriptor through the resolver just before writing. `DefaultContentResolver` handles inline and filesystem sources; URL resolution is intentionally left to custom implementations (it returns an explanatory error by default). Similarly, `ContentStorage` allows custom write destinations, with `DefaultContentStorage` writing to the local filesystem with automatic parent directory creation.

### Features

| File | Relationship |
|------|--------------|
| [`feature/008_file_descriptor.md`](008_file_descriptor.md) | File descriptor that carries the optional content source reference |
| [`feature/017_archive_self_containment.md`](017_archive_self_containment.md) | Archive portability model that permits external content references for file content |

### Invariants

| File | Relationship |
|------|--------------|
| [`invariant/002_memory_efficiency.md`](../invariant/002_memory_efficiency.md) | External references avoid loading content into memory until materialization time |

### Sources

| File | Relationship |
|------|--------------|
| [`src/content_source.rs`](../../src/content_source.rs) | Full content source abstraction: enum, traits, helper types, and default implementations |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/content_source_test.rs`](../../tests/inc/content_source_test.rs) | Unit and integration tests for content source creation, resolution, and materialization |
| [`tests/inc/content_source_example.rs`](../../tests/inc/content_source_example.rs) | Example-as-tests demonstrating HTTP resolver and database resolver custom implementations |
