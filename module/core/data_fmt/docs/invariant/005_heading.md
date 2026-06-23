# Invariant: Heading Rendering

### Scope

- **Purpose**: Define the behavioral guarantees that heading rendering must maintain in all configurations and table styles.
- **Responsibility**: Documents three invariants: no-heading passthrough, width ceiling, and single-line output.
- **In Scope**: Absent heading behavior, table width ceiling, output line count guarantee.
- **Out of Scope**: Heading content format (see `feature/007_table_heading.md`), rendering algorithm (see `algorithm/007_heading_rendering.md`).

### Features

| File | Relationship |
|------|-------------|
| [007_table_heading.md](../feature/007_table_heading.md) | Feature whose invariants are defined here |

### Algorithms

| File | Relationship |
|------|-------------|
| [007_heading_rendering.md](../algorithm/007_heading_rendering.md) | Algorithm whose outputs these invariants constrain |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/row_rendering.rs`](../../src/formatters/table/row_rendering.rs) | `render_heading_if_present()` — heading rendering path |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Call site in `format_internal()` — passes `primary_widths` to heading renderer |
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading` optional field on `TableConfig` |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../tests/table_heading_test.rs) | Heading invariant tests |

### Invariant Statement

#### Invariant 1 — No-Heading Passthrough

A table with no heading attached produces output byte-identical to the output produced by the same table before the heading feature existed. The heading path is a strict additive extension — no code runs on the heading path when no heading is set.

#### Invariant 2 — Width Ceiling

The heading line never exceeds the rendered table width. When heading content (lead prefix + content + trailing space) equals or exceeds the rendered table width, the trailing rule width is clamped to zero. The heading line may be shorter than the rendered table width but never longer. When content alone exceeds the rendered table width, the heading line may exceed it — the invariant guarantee is that the trailing rule is clamped to zero and content is never truncated.

#### Invariant 3 — Single Output Line

The heading always occupies exactly one output line ending with a newline character. No combination of title length, heading field count, terminal width, or embedded line breaks can cause the heading to span multiple lines.

### Enforcement Mechanism

Invariant 1 is enforced by an early-exit guard in `render_heading_if_present()` that skips the heading path entirely when the heading field on `TableConfig` is absent (`None`). Invariant 2 is enforced by the `saturating_sub` clamp applied to the trailing rule width computation using the rendered table width (`compute_total_row_width(primary_widths)`), not the terminal width. Invariant 3 is enforced by `sanitize_line_breaks()` in `content_str()` which replaces all line-break sequences (`\r\n`, `\r`, `\n`) with spaces before assembly, and by the single-string assembly in the rendering algorithm which appends exactly one newline at the end.

### Violation Consequences

| Invariant | Consequence of Violation |
|-----------|------------------------|
| No-heading passthrough | Silent regression for all existing callers — every table output changes without any user action |
| Width ceiling | Heading line overflows the rendered table width; output wraps or truncates unexpectedly in terminal display |
| Single output line | Table layout corrupts; the header row and subsequent rows shift down by an unexpected number of lines |
