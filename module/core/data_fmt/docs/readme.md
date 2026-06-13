# Docs

### Scope

**Responsibilities:** Define behavioral requirements and document data_fmt architecture, algorithms, and design decisions.
**In Scope:** Typed doc entity instances (feature/, invariant/, api/, algorithm/, formatter/, trait/, variant/, builder/, pattern/, data_structure/, input_model/, input_type/), architecture docs, development notes, entities index, and doc dependency graph.
**Out of Scope:** Test code (tests/), production source (src/), usage examples (examples/), task tracking (task/).

### Vocabulary

| Term | Definition |
|------|-----------|
| **caption** | The titled-rule line printed above a table header — includes lead rule, title, optional caption fields, and trailing rule |
| **title** | The primary text in a caption |
| **caption field** | An additional metadata item appended to the caption title with the field separator |
| **field separator** | The character placed between caption fields — fixed as `·` (U+00B7 MIDDLE DOT) |
| **rule character** | The horizontal fill character for rule sections — fixed as `─` (U+2500 BOX DRAWINGS LIGHT HORIZONTAL) |
| **lead width** | The number of rule characters emitted before the title text — fixed at 3 |
| **titled rule** | The visual pattern: `─── Title · Field1 ──────...` filling the resolved terminal width |

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `input_model/` | Documents conceptual data shapes for input |
| `input_type/` | Documents concrete input types for formatters |
| `builder/` | Documents construction helpers for input types |
| `trait/` | Documents formatter interface contracts |
| `architecture.md` | Documents three-layer architecture and module structure |
| `api/` | Documents the public API surface |
| `feature/` | Documents behavioral features of the library |
| `invariant/` | Documents behavioral invariants and contracts |
| `algorithm/` | Documents non-trivial formatter algorithms |
| `data_structure/` | Documents data schemas and structural type definitions |
| `pattern/` | Documents architectural patterns and design principles |
| `formatter/` | Documents individual formatter structs and dispatch |
| `variant/` | Documents per-variant output format and attributes |
| `variant_attributes.md` | Legacy reference listing all 46 variant attributes with examples |
| `development_notes.md` | Records implementation decisions and technical history |
| `feature_selection_guide.md` | Guides feature flag selection for use cases |
| `entities.md` | Indexes all doc entity types and instances |
| `doc_graph.yml` | Maps cross-entity dependencies as nodes and edges |
