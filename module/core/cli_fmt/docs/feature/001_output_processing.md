# Feature: Output Processing

### Scope

- **Purpose**: Describe the CLI output processing capability and its design boundaries.
- **Responsibility**: Document the three-stage pipeline — stream selection, line filtering, and width truncation.
- **In Scope**: Head/tail filtering, ANSI-aware width truncation, stream merging, and result metadata.
- **Out of Scope**: Public API contract — see `api/001_output_api.md`.

### Design

CLI output processing for command-line applications. This feature encodes CLI-specific
policy decisions that are intentionally excluded from general-purpose string utilities.

Processing applies three sequential stages:

**Stream selection** — merges stdout and stderr according to a configurable stream filter.
The default ordering places stderr before stdout, following the CLI convention that errors
should be visible without scrolling past normal output.

**Line filtering** — applies head and tail limits to the combined output. Head retains
the first N lines; tail retains the last N lines. When both limits are set, the union
of both windows is preserved. The count of omitted lines is always tracked accurately,
including the case where head and tail windows overlap.

**Width truncation** — truncates any line whose visible character count exceeds the
configured maximum. ANSI escape sequences are excluded from visible width measurement.
A configurable suffix marks truncated lines. Width of zero disables truncation entirely.

The result always includes metadata: count of lines removed by head/tail filtering, and
a flag indicating whether any line was truncated by the width limit.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [`../api/001_output_api.md`](../api/001_output_api.md) | Public contract for the output processing function and types |
| doc | [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Why CLI-specific policy belongs here, not in strs_tools |
