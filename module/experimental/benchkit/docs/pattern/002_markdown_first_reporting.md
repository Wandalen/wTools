# Pattern: Markdown-First Reporting

### Scope

- **Purpose**: Keep performance documentation permanently synchronized with benchmark results by writing output directly into version-controlled markdown files.
- **Responsibility**: Documents the architectural decision to treat markdown as the primary reporting output and its design implications.
- **In Scope**: The rationale for markdown-first output; how it shapes the update mechanism and section-targeting design; trade-offs relative to terminal or binary output.
- **Out of Scope**: The mechanics of safe section matching (→ invariant/002); the report generation API (→ api/001); the markdown reports feature scope (→ feature/003).

### Problem

Performance results from a benchmark run appear in terminal output, which is ephemeral — it vanishes when the session ends and cannot be reviewed later without re-running the benchmark. Developers who want to document performance characteristics manually copy terminal output into markdown files. This copy-paste workflow is error-prone, inconsistent across team members, and abandoned under time pressure. As a result, performance documentation quickly falls out of date and becomes untrustworthy.

### Solution

Generate report content as markdown and write it directly into the project's existing documentation files during the benchmark run. The benchmark itself becomes responsible for keeping its documentation current. Performance results are captured in version-controlled markdown alongside the code that produced them, making every commit's performance characteristics auditable via git history.

The update mechanism targets a named section within the markdown file, replacing only that section's content while preserving all surrounding text. This allows a single documentation file to contain human-authored context alongside auto-generated benchmark tables, with each maintaining its own update lifecycle.

### Applicability

Apply this pattern when:
- Performance documentation must stay synchronized with code without manual effort
- The project uses markdown files (readme, changelog, dedicated performance docs) as its primary documentation format
- Version control of performance results is desired — git blame and git log should surface when performance changed and by how much
- Multiple benchmark runs contribute results to a single documentation file that also contains manually authored content

Do not apply when:
- Output will be consumed by automated tooling that requires structured data (use JSON output instead)
- The target documentation system does not support markdown

### Consequences

**Positive**: Documentation is always current after a benchmark run; no manual copy-paste step; git history preserves performance evolution; diffs between commits show performance changes alongside code changes; any developer running benchmarks automatically refreshes the docs.

**Negative**: Requires exact-match discipline for section targeting — substring matching causes unbounded content duplication (→ invariant/002); the target file must exist before the benchmark runs; markdown formatting constraints apply to report content; concurrent benchmark processes writing to the same file require external coordination.

### Cross-References

| Type   | File                                    | Responsibility                                            |
|--------|-----------------------------------------|-----------------------------------------------------------|
| doc    | `docs/feature/003_markdown_reports.md`  | Feature that implements this pattern                      |
| doc    | `docs/invariant/002_exact_section_match.md` | Invariant this pattern depends on to avoid duplication |
| doc    | `docs/api/001_benchkit_api.md`          | API operations that expose markdown reporting             |
