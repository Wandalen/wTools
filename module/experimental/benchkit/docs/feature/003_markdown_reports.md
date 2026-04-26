# Feature: Markdown Reports

### Scope

- **Purpose**: Keep performance documentation in sync with code by updating markdown files directly from benchmark runs.
- **Responsibility**: Documents the section updater, section identification, update chains, and template rendering.
- **In Scope**: Section-targeted markdown updates, duplicate-prevention via exact section matching, update chains, templates.
- **Out of Scope**: Benchmark measurement (→ feature/001); statistical analysis of results (→ feature/004).

### Design

The central mechanism is the section updater: it locates a named section within a markdown file using exact match (trimmed comparison of the heading line against the configured section marker) and replaces the section body with new content. Exact matching is mandatory — substring matching caused the duplication defect where a section like "Results" would match inside "Algorithm Comparison Results", appending duplicate content on every run.

Update chains allow multiple sections in the same file to be updated sequentially in one pass, reducing file I/O round-trips when a benchmark run produces several independent results blocks.

Templates provide pre-structured markdown skeletons for common report patterns (comparison tables, scaling charts, regression summaries). A template is filled with measurement data and passed to the section updater.

### Cross-References

| Type      | File                                    | Responsibility                                          |
|-----------|-----------------------------------------|---------------------------------------------------------|
| source    | `src/reporting.rs`                      | Section updater and error types; section update logic   |
| source    | `src/update_chain.rs`                   | Multi-section sequential update chains                  |
| source    | `src/templates.rs`                      | Pre-built report template structures                    |
| source    | `src/documentation.rs`                  | Documentation integration helpers                       |
| test      | `tests/`                                | Round-trip update tests; duplicate prevention tests     |
| doc       | `docs/invariant/002_exact_section_match.md` | Exact match invariant preventing content duplication|
| doc       | `docs/api/001_benchkit_api.md`          | Public API surface including section update operations  |
| doc       | `docs/pattern/002_markdown_first_reporting.md` | Architectural pattern this feature implements    |
| doc       | `docs/pattern/001_toolkit_not_framework.md`    | Design principle this feature exemplifies        |
