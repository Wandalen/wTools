# Invariant: Exact Section Match

### Scope

- **Purpose**: Prevent duplicate content from accumulating in markdown files when the section updater runs multiple times.
- **Responsibility**: States the matching rule that section markers must follow to avoid content duplication defects.
- **In Scope**: How the section updater locates the target section heading within a markdown file.
- **Out of Scope**: Report content formatting (→ feature/003); file I/O error handling (→ api/001).

### Invariant Statement

Section identification in the section updater MUST use exact trimmed string equality: each scanned line is trimmed and compared directly against the trimmed section marker. Substring-containment matching is unconditionally forbidden.

### Enforcement Mechanism

The section updater stores the section marker as a validated string at construction time (validated for emptiness, length limit, and absence of newline characters). During update, the file is scanned line by line; a line is accepted as the section boundary only when trimmed equality holds. Construction-time validation also detects potential conflicts where one configured marker is a strict substring of another, rejecting such configurations with a section conflict error at startup.

### Violation Consequences

Using substring matching causes the updater to match any heading that contains the marker string, not just the exact heading. On the second and subsequent runs, the content that was inserted on the previous run contains the section marker text within it, causing the updater to insert the section body again inside itself. The result is unbounded duplication: each run appends another copy of the section content. This was the root cause of the CBF-1 bug observed during strs_tools benchmarking, where `benchmark_results.md` accumulated five duplicate "Algorithm Comparison Results" sections.

### Cross-References

| Type   | File                                  | Responsibility                                            |
|--------|---------------------------------------|-----------------------------------------------------------|
| source | `src/reporting.rs`                    | Section updater exact match implementation                |
| test   | `tests/`                              | Duplicate prevention round-trip tests                     |
| doc    | `docs/feature/003_markdown_reports.md` | Feature that relies on this invariant                    |
